use anyhow::{Context, Result};
use git2::{DiffOptions, Repository};
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Snapshot of a single file's diff stats
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FileDiff {
    pub path: PathBuf,
    pub additions: usize,
    pub deletions: usize,
    pub status: DiffStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DiffStatus {
    Modified,
    Added,
    Deleted,
    Renamed { from: PathBuf },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffLineKind {
    Add,
    Delete,
    Context,
    Header,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffContentLine {
    pub kind: DiffLineKind,
    pub content: String,
    /// Line number in the old file (before change). None for hunk headers.
    pub old_lineno: Option<u32>,
    /// Line number in the new file (after change). None for hunk headers.
    pub new_lineno: Option<u32>,
}

/// The current working-tree diff against HEAD (or a baseline commit)
#[derive(Debug, Default)]
pub struct WorkingTreeDiff {
    pub files: Vec<FileDiff>,
}

impl WorkingTreeDiff {
    pub fn total_additions(&self) -> usize {
        self.files.iter().map(|f| f.additions).sum()
    }

    pub fn total_deletions(&self) -> usize {
        self.files.iter().map(|f| f.deletions).sum()
    }

    pub fn total_lines_changed(&self) -> usize {
        self.total_additions() + self.total_deletions()
    }
}

/// Open the git repo at or above cwd
pub fn open_repo() -> Result<Repository> {
    let cwd = std::env::current_dir()?;
    Repository::discover(&cwd).context("Not inside a git repository. AgentScope requires git.")
}

/// Capture the current HEAD commit SHA as a baseline
pub fn capture_baseline(repo: &Repository) -> Result<String> {
    let head = repo
        .head()
        .context("No commits yet — make at least one commit first")?;
    let commit = head.peel_to_commit()?;
    Ok(commit.id().to_string())
}

/// Diff working tree (including staged changes) against HEAD
#[allow(dead_code)]
pub fn working_tree_diff(repo: &Repository) -> Result<WorkingTreeDiff> {
    working_tree_diff_from(repo, None)
}

/// Diff working tree against a baseline commit SHA (or HEAD when `baseline` is None).
///
/// Using a session baseline captures both committed *and* uncommitted changes
/// since the session started — so agent commits (Codex, Claude Code) are never missed.
pub fn working_tree_diff_from(
    repo: &Repository,
    baseline: Option<&str>,
) -> Result<WorkingTreeDiff> {
    let base_tree = match baseline {
        Some(sha) => {
            let oid = git2::Oid::from_str(sha)
                .with_context(|| format!("invalid baseline SHA: {}", sha))?;
            let commit = repo
                .find_commit(oid)
                .with_context(|| format!("could not find baseline commit {}", sha))?;
            commit.tree()?
        }
        None => {
            let head = repo.head()?;
            let commit = head.peel_to_commit()?;
            commit.tree()?
        }
    };
    // delegate to the shared impl
    _working_tree_diff_impl(repo, &base_tree)
}

fn _working_tree_diff_impl(repo: &Repository, base_tree: &git2::Tree) -> Result<WorkingTreeDiff> {
    let head_tree = base_tree;

    let mut diff_opts = DiffOptions::new();
    diff_opts.include_untracked(true);
    diff_opts.recurse_untracked_dirs(true);

    let diff = repo
        .diff_tree_to_workdir_with_index(Some(head_tree), Some(&mut diff_opts))
        .context("Failed to compute diff")?;

    // Collect file paths and statuses
    let mut file_map: HashMap<PathBuf, FileDiff> = HashMap::new();

    // First pass: gather all changed files
    let stats = diff.stats()?;
    let _ = stats; // just to force computation

    diff.foreach(
        &mut |delta, _progress| {
            let path = delta
                .new_file()
                .path()
                .or_else(|| delta.old_file().path())
                .map(PathBuf::from)
                .unwrap_or_default();

            let status = match delta.status() {
                git2::Delta::Added => DiffStatus::Added,
                git2::Delta::Deleted => DiffStatus::Deleted,
                git2::Delta::Renamed => DiffStatus::Renamed {
                    from: delta
                        .old_file()
                        .path()
                        .map(PathBuf::from)
                        .unwrap_or_default(),
                },
                _ => DiffStatus::Modified,
            };

            file_map.entry(path.clone()).or_insert(FileDiff {
                path,
                additions: 0,
                deletions: 0,
                status,
            });
            true
        },
        None,
        None,
        None,
    )?;

    // Second pass: count line additions/deletions
    let diff2 = repo
        .diff_tree_to_workdir_with_index(Some(head_tree), Some(&mut diff_opts))
        .context("Failed to compute diff for line stats")?;

    diff2.foreach(
        &mut |_delta, _progress| true,
        None,
        None,
        Some(&mut |delta, _hunk, line| {
            let path = delta
                .new_file()
                .path()
                .map(PathBuf::from)
                .unwrap_or_default();
            if let Some(entry) = file_map.get_mut(&path) {
                match line.origin() {
                    '+' => entry.additions += 1,
                    '-' => entry.deletions += 1,
                    _ => {}
                }
            }
            true
        }),
    )?;

    let mut files: Vec<FileDiff> = file_map.into_values().collect();
    files.sort_by(|a, b| a.path.cmp(&b.path));

    Ok(WorkingTreeDiff { files })
}

pub fn file_diff_content(repo: &Repository, path: &Path) -> Result<Vec<DiffContentLine>> {
    let head_tree = {
        let head = repo.head()?;
        let commit = head.peel_to_commit()?;
        commit.tree()?
    };

    let mut diff_opts = DiffOptions::new();
    diff_opts.include_untracked(true);
    diff_opts.recurse_untracked_dirs(true);
    diff_opts.pathspec(path);

    let diff = repo
        .diff_tree_to_workdir_with_index(Some(&head_tree), Some(&mut diff_opts))
        .context("Failed to compute file diff")?;

    let lines = RefCell::new(Vec::new());
    diff.foreach(
        &mut |_delta, _progress| true,
        None,
        Some(&mut |_delta, hunk| {
            let header = std::str::from_utf8(hunk.header())
                .unwrap_or("@@ binary or unreadable hunk @@")
                .trim_end()
                .to_string();
            lines.borrow_mut().push(DiffContentLine {
                kind: DiffLineKind::Header,
                content: header,
                old_lineno: None,
                new_lineno: None,
            });
            true
        }),
        Some(&mut |_delta, _hunk, line| {
            let kind = match line.origin() {
                '+' => DiffLineKind::Add,
                '-' => DiffLineKind::Delete,
                ' ' => DiffLineKind::Context,
                _ => DiffLineKind::Header,
            };
            let prefix = match kind {
                DiffLineKind::Add => "+",
                DiffLineKind::Delete => "-",
                DiffLineKind::Context => " ",
                DiffLineKind::Header => "",
            };
            let text = std::str::from_utf8(line.content())
                .map(|s| s.trim_end_matches(['\r', '\n']).to_string())
                .unwrap_or_else(|_| "[binary or non-utf8 content]".into());
            lines.borrow_mut().push(DiffContentLine {
                kind,
                content: format!("{}{}", prefix, text),
                old_lineno: line.old_lineno(),
                new_lineno: line.new_lineno(),
            });
            true
        }),
    )?;

    let mut lines = lines.into_inner();
    let has_text_changes = lines
        .iter()
        .any(|line| matches!(line.kind, DiffLineKind::Add | DiffLineKind::Delete));
    if !has_text_changes {
        if let Some(workdir) = repo.workdir() {
            let full_path = workdir.join(path);
            if full_path.is_file() {
                if let Ok(contents) = std::fs::read_to_string(&full_path) {
                    lines.clear();
                    lines.push(DiffContentLine {
                        kind: DiffLineKind::Header,
                        content: format!("@@ untracked {} @@", path.display()),
                        old_lineno: None,
                        new_lineno: None,
                    });
                    lines.extend(
                        contents
                            .lines()
                            .enumerate()
                            .map(|(i, line)| DiffContentLine {
                                kind: DiffLineKind::Add,
                                content: format!("+{}", line),
                                old_lineno: None,
                                new_lineno: Some(i as u32 + 1),
                            }),
                    );
                }
            }
        }
    }

    if lines.is_empty() {
        lines.push(DiffContentLine {
            kind: DiffLineKind::Context,
            content: format!("No text diff available for {}", path.display()),
            old_lineno: None,
            new_lineno: None,
        });
    }

    Ok(lines)
}

/// Diff between two commit SHAs — used by audit
#[allow(dead_code)]
pub fn diff_between(repo: &Repository, from: &str, to: &str) -> Result<WorkingTreeDiff> {
    let from_commit = repo.find_commit(git2::Oid::from_str(from)?)?;
    let to_commit = repo.find_commit(git2::Oid::from_str(to)?)?;

    let from_tree = from_commit.tree()?;
    let to_tree = to_commit.tree()?;

    let diff = repo.diff_tree_to_tree(Some(&from_tree), Some(&to_tree), None)?;

    let mut files: Vec<FileDiff> = Vec::new();

    diff.foreach(
        &mut |delta, _| {
            let path = delta
                .new_file()
                .path()
                .or_else(|| delta.old_file().path())
                .map(PathBuf::from)
                .unwrap_or_default();

            let status = match delta.status() {
                git2::Delta::Added => DiffStatus::Added,
                git2::Delta::Deleted => DiffStatus::Deleted,
                _ => DiffStatus::Modified,
            };

            files.push(FileDiff {
                path,
                additions: 0,
                deletions: 0,
                status,
            });
            true
        },
        None,
        None,
        None,
    )?;

    Ok(WorkingTreeDiff { files })
}

/// Get recent commits (for audit range)
#[allow(dead_code)]
pub fn recent_commits(repo: &Repository, n: usize) -> Result<Vec<CommitInfo>> {
    let mut walk = repo.revwalk()?;
    walk.push_head()?;
    walk.set_sorting(git2::Sort::TIME)?;

    let commits = walk
        .take(n)
        .filter_map(|oid| {
            let oid = oid.ok()?;
            let commit = repo.find_commit(oid).ok()?;
            Some(CommitInfo {
                sha: oid.to_string(),
                sha_short: oid.to_string()[..7].to_string(),
                message: commit.summary().unwrap_or("").to_string(),
                timestamp: commit.time().seconds(),
            })
        })
        .collect();

    Ok(commits)
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CommitInfo {
    pub sha: String,
    pub sha_short: String,
    pub message: String,
    pub timestamp: i64,
}

#[cfg(test)]
mod diff_content_tests {
    use super::*;
    use std::fs;
    use std::process::Command;
    use tempfile::TempDir;

    fn repo_with_initial_file() -> (TempDir, Repository) {
        let tmp = TempDir::new().unwrap();
        Command::new("git")
            .args(["init"])
            .current_dir(tmp.path())
            .output()
            .unwrap();
        Command::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(tmp.path())
            .output()
            .unwrap();
        Command::new("git")
            .args(["config", "user.name", "Test User"])
            .current_dir(tmp.path())
            .output()
            .unwrap();
        fs::write(tmp.path().join("src.txt"), "one\nold\nthree\n").unwrap();
        Command::new("git")
            .args(["add", "."])
            .current_dir(tmp.path())
            .output()
            .unwrap();
        Command::new("git")
            .args(["commit", "-m", "init"])
            .current_dir(tmp.path())
            .output()
            .unwrap();

        let repo = Repository::open(tmp.path()).unwrap();
        (tmp, repo)
    }

    #[test]
    fn file_diff_content_classifies_changed_lines() {
        let (tmp, repo) = repo_with_initial_file();
        fs::write(tmp.path().join("src.txt"), "one\nnew\nthree\n").unwrap();

        let lines = file_diff_content(&repo, PathBuf::from("src.txt").as_path()).unwrap();

        assert!(lines.iter().any(|line| line.kind == DiffLineKind::Header));
        assert!(lines
            .iter()
            .any(|line| line.kind == DiffLineKind::Delete && line.content.contains("old")));
        assert!(lines
            .iter()
            .any(|line| line.kind == DiffLineKind::Add && line.content.contains("new")));
    }

    #[test]
    fn file_diff_content_handles_untracked_file() {
        let (tmp, repo) = repo_with_initial_file();
        fs::write(tmp.path().join("new.txt"), "fresh\n").unwrap();

        let lines = file_diff_content(&repo, PathBuf::from("new.txt").as_path()).unwrap();

        assert!(lines
            .iter()
            .any(|line| line.kind == DiffLineKind::Add && line.content.contains("fresh")));
    }
}
