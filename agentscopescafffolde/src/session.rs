use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use ulid::Ulid;

use crate::cli::AgentKind;
use crate::config::{self, ACTIVITY_LOG, SESSION_DIR};
use crate::git;
use crate::judge;
use crate::output::{CheckReport, Printer};
use crate::policy::PolicyEngine;

pub const ACTIVE_SESSION_FILE: &str = ".agentscope/session.json";

// ── Session data ──────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    pub id: String,
    pub mission: String,
    pub agent: String,
    pub git_baseline: String,
    pub started_at: String,
    pub repo_root: PathBuf,
}

// ── start ─────────────────────────────────────────────────────────────────────

pub async fn start(mission: String, agent: AgentKind, watch: bool) -> Result<()> {
    let p = Printer::new();
    let config = config::load_or_default();

    let repo = git::open_repo()?;
    let baseline = git::capture_baseline(&repo)?;
    let repo_root = repo.workdir().unwrap_or_else(|| std::path::Path::new(".")).to_path_buf();

    std::fs::create_dir_all(SESSION_DIR)?;

    let session = Session {
        id: Ulid::new().to_string(),
        mission: mission.clone(),
        agent: agent.to_string(),
        git_baseline: baseline.clone(),
        started_at: Utc::now().to_rfc3339(),
        repo_root: repo_root.clone(),
    };

    save_session(&session)?;
    append_activity("session_start", &session)?;

    p.session_started(&session);

    if watch {
        crate::tui::run_watch().await?;
    }

    Ok(())
}

// ── check ─────────────────────────────────────────────────────────────────────

pub async fn check(session_id: Option<String>, json: bool, share: bool) -> Result<()> {
    let p = Printer::new();
    let config = config::load_or_default();

    let session = match session_id {
        Some(id) => load_session_by_id(&id)?,
        None => load_active_session()?,
    };

    let repo = git::open_repo()?;
    let diff = git::working_tree_diff(&repo)?;

    let engine = PolicyEngine::from_config(&config.policy)?;
    let annotated = engine.annotate(&diff.files, &session.mission);
    let limit_warnings = engine.check_limits(diff.files.len(), diff.total_lines_changed());

    // LLM judge (async, optional)
    let judge_result = if config.judge.enabled && !json {
        judge::evaluate(&session.mission, &annotated, &config.judge).await.ok()
    } else {
        None
    };

    let report = CheckReport {
        session: session.clone(),
        annotated: annotated.clone(),
        limit_warnings,
        judge_result: judge_result.clone(),
    };

    if json {
        println!("{}", serde_json::to_string_pretty(&report.to_json())?);
        return Ok(());
    }

    p.print_check_report(&report);

    if share {
        let markdown = report.to_markdown();
        // In a real build: use arboard crate for clipboard
        println!("\n--- Markdown summary (copy to clipboard) ---\n{}", markdown);
    }

    // Append to activity log
    append_activity("session_check", &session)?;

    // Exit code: 1 if any BLOCKED files
    let has_blocks = annotated.iter().any(|f| f.verdict.is_blocked());
    if has_blocks {
        std::process::exit(1);
    }

    Ok(())
}

// ── status ────────────────────────────────────────────────────────────────────

pub async fn status() -> Result<()> {
    let p = Printer::new();

    match load_active_session() {
        Ok(session) => p.session_one_liner(&session),
        Err(_) => p.hint("No active session. Run: agentscope start \"your mission\""),
    }

    Ok(())
}

// ── Persistence ───────────────────────────────────────────────────────────────

fn save_session(session: &Session) -> Result<()> {
    let json = serde_json::to_string_pretty(session)?;
    std::fs::write(ACTIVE_SESSION_FILE, json)?;
    Ok(())
}

pub fn load_active_session() -> Result<Session> {
    let path = std::path::Path::new(ACTIVE_SESSION_FILE);
    if !path.exists() {
        anyhow::bail!("No active session. Run: agentscope start \"your mission\"");
    }
    let json = std::fs::read_to_string(path)?;
    let session: Session = serde_json::from_str(&json)?;
    Ok(session)
}

fn load_session_by_id(id: &str) -> Result<Session> {
    // Look through activity log for session with matching id
    let log_path = std::path::Path::new(ACTIVITY_LOG);
    if !log_path.exists() {
        anyhow::bail!("No activity log found");
    }

    let content = std::fs::read_to_string(log_path)?;
    for line in content.lines().rev() {
        if let Ok(entry) = serde_json::from_str::<ActivityEntry>(line) {
            if entry.session.id == id {
                return Ok(entry.session);
            }
        }
    }

    anyhow::bail!("Session {} not found", id);
}

fn append_activity(event: &str, session: &Session) -> Result<()> {
    let entry = ActivityEntry {
        event: event.to_string(),
        timestamp: Utc::now().to_rfc3339(),
        session: session.clone(),
    };
    let line = serde_json::to_string(&entry)?;
    use std::io::Write;
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(ACTIVITY_LOG)?;
    writeln!(file, "{}", line)?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct ActivityEntry {
    event: String,
    timestamp: String,
    session: Session,
}
