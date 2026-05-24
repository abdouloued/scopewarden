use globset::{Glob, GlobSet, GlobSetBuilder};
use std::path::Path;

use crate::config::PolicyConfig;
use crate::git::FileDiff;

/// How AgentScope classifies a changed file relative to the mission
#[derive(Debug, Clone, PartialEq)]
pub enum FileVerdict {
    /// Matches stated mission + not blocked
    InScope,
    /// Not in mission scope but not blocked — warn only
    Unasked,
    /// Matched a blocked glob — halt session
    Blocked { policy: String },
    /// No changes (included for completeness)
    #[allow(dead_code)]
    Clean,
}

impl FileVerdict {
    pub fn is_blocked(&self) -> bool {
        matches!(self, FileVerdict::Blocked { .. })
    }

    pub fn label(&self) -> &'static str {
        match self {
            FileVerdict::InScope => "IN SCOPE",
            FileVerdict::Unasked => "UNASKED",
            FileVerdict::Blocked { .. } => "BLOCKED",
            FileVerdict::Clean => "CLEAN",
        }
    }
}

/// A file diff annotated with its policy verdict
#[derive(Debug, Clone)]
pub struct AnnotatedFile {
    pub diff: FileDiff,
    pub verdict: FileVerdict,
}

/// The compiled, ready-to-evaluate policy engine
pub struct PolicyEngine {
    blocked_set: GlobSet,
    warn_set: GlobSet,
    blocked_patterns: Vec<String>,
    #[allow(dead_code)]
    warn_patterns: Vec<String>,
    pub max_files: usize,
    pub max_lines: usize,
}

impl PolicyEngine {
    pub fn from_config(config: &PolicyConfig) -> anyhow::Result<Self> {
        let mut blocked_builder = GlobSetBuilder::new();
        for pattern in &config.blocked {
            blocked_builder.add(Glob::new(pattern)?);
        }

        let mut warn_builder = GlobSetBuilder::new();
        for pattern in &config.warn {
            warn_builder.add(Glob::new(pattern)?);
        }

        Ok(Self {
            blocked_set: blocked_builder.build()?,
            warn_set: warn_builder.build()?,
            blocked_patterns: config.blocked.clone(),
            warn_patterns: config.warn.clone(),
            max_files: config.max_files_changed,
            max_lines: config.max_lines_changed,
        })
    }

    /// Check a single file path against policy
    pub fn check_path(&self, path: &Path) -> FileVerdict {
        if self.blocked_set.is_match(path) {
            // Find which pattern matched for the error message
            let policy = self
                .blocked_patterns
                .iter()
                .enumerate()
                .find(|(i, _)| {
                    let matches = self.blocked_set.matches(path);
                    matches.contains(i)
                })
                .map(|(_, p)| p.clone())
                .unwrap_or_else(|| "blocked-path policy".into());

            return FileVerdict::Blocked { policy };
        }

        if self.warn_set.is_match(path) {
            // Treat warn-listed files as Unasked (yellow) for now
            return FileVerdict::Unasked;
        }

        // Default: Unasked — the mission classifier upgrades this to InScope
        FileVerdict::Unasked
    }

    /// Annotate a list of diffs using the mission text to infer scope
    pub fn annotate(
        &self,
        diffs: &[FileDiff],
        mission: &str,
    ) -> Vec<AnnotatedFile> {
        let scope_hints = extract_scope_hints(mission);

        diffs
            .iter()
            .map(|diff| {
                let verdict = match self.check_path(&diff.path) {
                    blocked @ FileVerdict::Blocked { .. } => blocked,
                    _ => {
                        if is_in_scope(&diff.path, &scope_hints) {
                            FileVerdict::InScope
                        } else {
                            FileVerdict::Unasked
                        }
                    }
                };
                AnnotatedFile {
                    diff: diff.clone(),
                    verdict,
                }
            })
            .collect()
    }

    /// Check limit policies
    pub fn check_limits(&self, file_count: usize, line_count: usize) -> Vec<LimitWarning> {
        let mut warnings = Vec::new();

        if self.max_files > 0 && file_count > self.max_files {
            warnings.push(LimitWarning::TooManyFiles {
                actual: file_count,
                limit: self.max_files,
            });
        }

        if self.max_lines > 0 && line_count > self.max_lines {
            warnings.push(LimitWarning::TooManyLines {
                actual: line_count,
                limit: self.max_lines,
            });
        }

        warnings
    }
}

#[derive(Debug)]
pub enum LimitWarning {
    TooManyFiles { actual: usize, limit: usize },
    TooManyLines { actual: usize, limit: usize },
}

/// Extract file/path hints from a natural language mission string.
/// E.g. "Fix rate-limit bug in api/middleware.ts" → ["api", "middleware", "middleware.ts"]
fn extract_scope_hints(mission: &str) -> Vec<String> {
    // Grab anything that looks like a path segment or filename
    let re_path = regex_lite::Regex::new(r"[\w.-]+\.(?:ts|js|rs|py|go|rb|java|c|cpp|h|css|html|json|yaml|toml|md)")
        .unwrap();
    let re_dir = regex_lite::Regex::new(r"[\w-]+/[\w./:-]+").unwrap();

    let mut hints: Vec<String> = Vec::new();

    for m in re_path.find_iter(mission) {
        hints.push(m.as_str().to_lowercase());
    }
    for m in re_dir.find_iter(mission) {
        hints.push(m.as_str().to_lowercase());
    }

    // Also add individual words longer than 3 chars as fuzzy hints
    for word in mission.split_whitespace() {
        let w = word.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase();
        if w.len() > 3 {
            hints.push(w);
        }
    }

    hints
}

/// Fuzzy match: does this path seem related to what was asked?
fn is_in_scope(path: &Path, hints: &[String]) -> bool {
    let path_str = path.to_string_lossy().to_lowercase();
    let stem = path
        .file_stem()
        .map(|s| s.to_string_lossy().to_lowercase())
        .unwrap_or_default();

    hints.iter().any(|hint| {
        path_str.contains(hint.as_str()) || hint.contains(stem.as_str())
    })
}

