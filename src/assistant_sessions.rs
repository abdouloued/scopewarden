use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};
use ulid::Ulid;

use crate::{agents, chat, cli::SessionsAction, config};

pub const AGENT_SESSION_CACHE: &str = ".agentscope/cache/agent-sessions.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AssistantSession {
    pub id: String,
    pub agent: String,
    pub path: PathBuf,
    pub modified_at: String,
    pub mission: Option<String>,
    pub confidence: f32,
    pub message_count: usize,
    pub preview: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct AssistantSessionCache {
    sessions: Vec<AssistantSession>,
}

pub async fn sessions_command(action: SessionsAction) -> Result<()> {
    let config = config::load_or_default();
    let sessions = index_sessions(&config)?;
    write_cache(&sessions)?;
    match action {
        SessionsAction::List { agent } => {
            let sessions = filter_sessions(sessions, agent.as_deref())?;
            for session in sessions.iter().take(80) {
                println!(
                    "  {}  {:<12} {:<20} {}",
                    session.id,
                    session.agent,
                    session.modified_at,
                    chat::truncate(
                        session
                            .mission
                            .as_deref()
                            .unwrap_or(session.preview.as_str()),
                        70
                    )
                );
            }
        }
        SessionsAction::Latest { agent } => {
            let session = filter_sessions(sessions, agent.as_deref())?
                .into_iter()
                .next()
                .context("No matching assistant sessions found")?;
            print_session(&session);
        }
        SessionsAction::Show { agent, session_id } => {
            let normalized = agents::normalize_agent(&agent)?;
            let session = sessions
                .into_iter()
                .find(|session| session.agent == normalized && session.id == session_id)
                .context("Assistant session not found")?;
            print_session(&session);
        }
    }
    Ok(())
}

pub fn index_sessions(config: &config::Config) -> Result<Vec<AssistantSession>> {
    let mut sessions = Vec::new();
    for agent in agents::supported_agents() {
        sessions.extend(index_agent_sessions(config, agent)?);
    }
    sessions.sort_by(|a, b| b.modified_at.cmp(&a.modified_at));
    Ok(sessions)
}

pub fn index_agent_sessions(config: &config::Config, agent: &str) -> Result<Vec<AssistantSession>> {
    let normalized = agents::normalize_agent(agent)?;
    let mut sessions = Vec::new();
    for source in agents::source_paths(config, normalized) {
        let expanded = agents::expand_path(&source);
        if !expanded.exists() {
            continue;
        }
        let files = if expanded.is_file() {
            vec![expanded]
        } else {
            collect_context_files(normalized, &expanded)?
        };
        for path in files {
            if let Ok(session) = session_from_file(normalized, path) {
                sessions.push(session);
            }
        }
    }
    sessions.sort_by(|a, b| b.modified_at.cmp(&a.modified_at));
    Ok(sessions)
}

#[allow(dead_code)]
pub fn load_cached_sessions() -> Result<Vec<AssistantSession>> {
    let path = Path::new(AGENT_SESSION_CACHE);
    if !path.exists() {
        return Ok(Vec::new());
    }
    let text = fs::read_to_string(path)?;
    Ok(serde_json::from_str::<AssistantSessionCache>(&text)
        .unwrap_or_default()
        .sessions)
}

pub fn write_cache(sessions: &[AssistantSession]) -> Result<()> {
    if let Some(parent) = Path::new(AGENT_SESSION_CACHE).parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(
        AGENT_SESSION_CACHE,
        serde_json::to_string_pretty(&AssistantSessionCache {
            sessions: sessions.to_vec(),
        })?,
    )?;
    Ok(())
}

pub fn latest_session(config: &config::Config, agent: Option<&str>) -> Result<AssistantSession> {
    filter_sessions(index_sessions(config)?, agent)?
        .into_iter()
        .next()
        .context("No matching assistant sessions found")
}

#[allow(dead_code)]
pub fn find_session(
    config: &config::Config,
    agent: &str,
    session_id: &str,
) -> Result<AssistantSession> {
    let normalized = agents::normalize_agent(agent)?;
    index_sessions(config)?
        .into_iter()
        .find(|session| session.agent == normalized && session.id == session_id)
        .context("Assistant session not found")
}

pub fn filter_sessions(
    sessions: Vec<AssistantSession>,
    agent: Option<&str>,
) -> Result<Vec<AssistantSession>> {
    if let Some(agent) = agent {
        let normalized = agents::normalize_agent(agent)?;
        Ok(sessions
            .into_iter()
            .filter(|session| session.agent == normalized)
            .collect())
    } else {
        Ok(sessions)
    }
}

fn session_from_file(agent: &str, path: PathBuf) -> Result<AssistantSession> {
    let metadata = fs::metadata(&path)?;
    let modified = metadata.modified().unwrap_or(std::time::UNIX_EPOCH);
    let modified_at = DateTime::<Utc>::from(modified).to_rfc3339();
    let contents = fs::read_to_string(&path).unwrap_or_default();
    let mission = agents::extract_mission(&contents);
    let preview = preview_text(&contents).unwrap_or_else(|| path.display().to_string());
    let message_count = message_count(&contents);
    let confidence = mission
        .as_ref()
        .map(|mission| {
            if mission.split_whitespace().count() >= 3 {
                0.85
            } else {
                0.45
            }
        })
        .unwrap_or(0.0);
    Ok(AssistantSession {
        id: stable_session_id(agent, &path, &modified_at),
        agent: agent.into(),
        path,
        modified_at,
        mission,
        confidence,
        message_count,
        preview,
    })
}

fn collect_context_files(agent: &str, root: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(path) = stack.pop() {
        let Ok(entries) = fs::read_dir(path) else {
            continue;
        };
        for entry in entries {
            let entry = entry?;
            let entry_path = entry.path();
            if entry_path.is_dir() {
                stack.push(entry_path);
            } else if is_context_file_for_agent(agent, &entry_path) {
                files.push(entry_path);
            }
        }
    }
    Ok(files)
}

fn is_context_file_for_agent(agent: &str, path: &Path) -> bool {
    let path_text = path.to_string_lossy();
    if agent == "copilot-cli" {
        return path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name == "events.jsonl")
            || path_text.contains("GitHub.copilot-chat/transcripts") && is_context_file(path)
            || path_text.contains(".copilot/session-state")
                && path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| name == "events.jsonl");
    }

    if agent == "openclaw" {
        return path_text.contains(".openclaw/agents")
            || path_text.contains(".openclaw/sessions")
            || is_context_file(path);
    }

    if agent == "hermes" {
        return path_text.contains(".hermes/agents")
            || path_text.contains(".hermes/sessions")
            || is_context_file(path);
    }

    if agent == "antigravity" {
        return path_text.contains(".gemini/antigravity")
            || path_text.contains("Antigravity")
            || path_text.contains(".antigravity")
            || is_context_file(path);
    }

    is_context_file(path)
}

fn is_context_file(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|ext| ext.to_str()),
        Some("jsonl" | "json" | "txt" | "md" | "yaml" | "yml")
    ) || path
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| {
            name.contains("transcript") || name.contains("chat") || name.contains("rollout")
        })
}

fn preview_text(contents: &str) -> Option<String> {
    contents
        .lines()
        .rev()
        .filter_map(line_text)
        .find(|line| !line.trim().is_empty())
        .map(|line| chat::truncate(line.trim(), 160))
}

fn line_text(line: &str) -> Option<String> {
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(line) {
        json_text(&value)
    } else {
        Some(line.to_string())
    }
}

fn json_text(value: &serde_json::Value) -> Option<String> {
    match value {
        serde_json::Value::String(text) => Some(text.clone()),
        serde_json::Value::Array(items) => items.iter().find_map(json_text),
        serde_json::Value::Object(map) => [
            "text",
            "content",
            "message",
            "prompt",
            "lastPrompt",
            "transformedContent",
            "data",
        ]
        .iter()
        .filter_map(|key| map.get(*key))
        .find_map(json_text),
        _ => None,
    }
}

fn message_count(contents: &str) -> usize {
    let count = contents
        .lines()
        .filter(|line| line_text(line).is_some_and(|text| !text.trim().is_empty()))
        .count();
    count.max(usize::from(!contents.trim().is_empty()))
}

fn stable_session_id(agent: &str, path: &Path, modified_at: &str) -> String {
    let input = format!("{}:{}:{}", agent, path.display(), modified_at);
    let hash = input.bytes().fold(0xcbf29ce484222325u64, |hash, byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x100000001b3)
    });
    format!("{:016x}", hash)
}

fn print_session(session: &AssistantSession) {
    println!("  id          {}", session.id);
    println!("  agent       {}", session.agent);
    println!("  modified    {}", session.modified_at);
    println!("  path        {}", session.path.display());
    println!("  confidence  {:.0}%", session.confidence * 100.0);
    println!("  messages    {}", session.message_count);
    if let Some(mission) = &session.mission {
        println!("  mission     {}", mission);
    }
    println!("  preview     {}", session.preview);
}

#[allow(dead_code)]
fn new_unstable_id() -> String {
    Ulid::new().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn indexes_multiple_fake_codex_sessions() {
        let dir = tempdir().unwrap();
        let source = dir.path().join("codex");
        fs::create_dir_all(&source).unwrap();
        fs::write(
            source.join("a.jsonl"),
            r#"{"message":{"content":[{"type":"text","text":"Implement chat mode"}]}}"#,
        )
        .unwrap();
        fs::write(source.join("b.txt"), "Fix policy precedence").unwrap();
        let mut config = config::Config::default();
        config.agents.sources.insert(
            "codex".into(),
            config::AgentSourceConfig {
                enabled: true,
                paths: vec![source.display().to_string()],
            },
        );

        let sessions = index_agent_sessions(&config, "codex").unwrap();
        assert_eq!(sessions.len(), 2);
        assert!(sessions.iter().any(|session| {
            session
                .mission
                .as_deref()
                .is_some_and(|mission| mission.contains("Implement chat"))
        }));
    }

    #[test]
    fn filters_sessions_by_normalized_agent() {
        let sessions = vec![
            AssistantSession {
                id: "1".into(),
                agent: "claude-code".into(),
                path: "a".into(),
                modified_at: "2026".into(),
                mission: None,
                confidence: 0.0,
                message_count: 1,
                preview: String::new(),
            },
            AssistantSession {
                id: "2".into(),
                agent: "codex".into(),
                path: "b".into(),
                modified_at: "2026".into(),
                mission: None,
                confidence: 0.0,
                message_count: 1,
                preview: String::new(),
            },
        ];
        let filtered = filter_sessions(sessions, Some("claude")).unwrap();
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].agent, "claude-code");
    }
}
