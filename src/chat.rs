use anyhow::{Context, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{BufRead, Write},
    path::{Path, PathBuf},
};
use ulid::Ulid;

use crate::config;

pub const CHATS_DIR: &str = ".scopewarden/chats";
pub const CHAT_INDEX: &str = ".scopewarden/chats/index.json";
pub const CHAT_ARCHIVE_DIR: &str = ".scopewarden/chats/archive";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatSessionMeta {
    pub id: String,
    pub title: String,
    pub created_at: String,
    pub updated_at: String,
    pub provider: String,
    pub model: String,
    pub repo_root: PathBuf,
    #[serde(default)]
    pub last_message_preview: String,
    #[serde(default)]
    pub archived: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatMessage {
    pub id: String,
    pub chat_id: String,
    pub role: String,
    pub content: String,
    pub timestamp: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct ChatIndex {
    #[serde(default)]
    chats: Vec<ChatSessionMeta>,
}


pub fn create_chat(title: Option<String>, config: &config::Config) -> Result<ChatSessionMeta> {
    ensure_chat_root()?;
    let now = Utc::now().to_rfc3339();
    let id = Ulid::new().to_string();
    let repo_root = std::env::current_dir()?;
    let title = title
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "ScopeWarden chat".into());
    let meta = ChatSessionMeta {
        id: id.clone(),
        title,
        created_at: now.clone(),
        updated_at: now,
        provider: provider_label(&config.judge.provider).into(),
        model: config.judge.model.clone(),
        repo_root,
        last_message_preview: String::new(),
        archived: false,
    };
    fs::create_dir_all(chat_dir(&id, false))?;
    write_json(
        chat_dir(&id, false).join("context.json"),
        &serde_json::json!({
            "provider": meta.provider,
            "model": meta.model,
            "repo_root": meta.repo_root,
        }),
    )?;
    let mut index = read_index()?;
    index.chats.push(meta.clone());
    write_index(&index)?;
    append_chat_activity("chat_new", &id, &meta.title)?;
    Ok(meta)
}

pub fn list_chats(include_archived: bool) -> Result<Vec<ChatSessionMeta>> {
    let mut chats = read_index()?.chats;
    if !include_archived {
        chats.retain(|chat| !chat.archived);
    }
    chats.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    Ok(chats)
}

pub fn get_chat(chat_id: &str, include_archived: bool) -> Result<ChatSessionMeta> {
    read_index()?
        .chats
        .into_iter()
        .find(|chat| chat.id == chat_id && (include_archived || !chat.archived))
        .with_context(|| format!("Chat {} not found", chat_id))
}

pub fn append_message(chat_id: &str, role: &str, content: &str) -> Result<ChatMessage> {
    let mut meta = get_chat(chat_id, false)?;
    let message = ChatMessage {
        id: Ulid::new().to_string(),
        chat_id: chat_id.into(),
        role: role.into(),
        content: content.into(),
        timestamp: Utc::now().to_rfc3339(),
    };
    let path = chat_dir(chat_id, false).join("messages.jsonl");
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    writeln!(file, "{}", serde_json::to_string(&message)?)?;

    meta.updated_at = message.timestamp.clone();
    meta.last_message_preview = truncate(content, 80);
    update_meta(meta)?;
    append_chat_activity("chat_message", chat_id, role)?;
    Ok(message)
}

#[allow(dead_code)]
pub fn load_messages(chat_id: &str, archived: bool) -> Result<Vec<ChatMessage>> {
    let path = chat_dir(chat_id, archived).join("messages.jsonl");
    if !path.exists() {
        return Ok(Vec::new());
    }
    let file = fs::File::open(path)?;
    std::io::BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .filter(|line| !line.trim().is_empty())
        .map(|line| serde_json::from_str::<ChatMessage>(&line).map_err(Into::into))
        .collect()
}

pub fn soft_delete_chat(chat_id: &str) -> Result<()> {
    let mut meta = get_chat(chat_id, false)?;
    fs::create_dir_all(CHAT_ARCHIVE_DIR)?;
    let source = chat_dir(chat_id, false);
    let dest = chat_dir(chat_id, true);
    if source.exists() {
        if dest.exists() {
            fs::remove_dir_all(&dest)?;
        }
        fs::rename(source, dest)?;
    }
    meta.archived = true;
    meta.updated_at = Utc::now().to_rfc3339();
    update_meta(meta)?;
    append_chat_activity("chat_delete", chat_id, "")?;
    Ok(())
}

#[allow(dead_code)]
pub fn restore_chat(chat_id: &str) -> Result<()> {
    let mut meta = get_chat(chat_id, true)?;
    if !meta.archived {
        return Ok(());
    }
    fs::create_dir_all(CHATS_DIR)?;
    let source = chat_dir(chat_id, true);
    let dest = chat_dir(chat_id, false);
    if source.exists() {
        if dest.exists() {
            fs::remove_dir_all(&dest)?;
        }
        fs::rename(source, dest)?;
    }
    meta.archived = false;
    meta.updated_at = Utc::now().to_rfc3339();
    update_meta(meta)?;
    append_chat_activity("chat_restore", chat_id, "")?;
    Ok(())
}

#[allow(dead_code)]
pub fn purge_chat(chat_id: &str) -> Result<()> {
    let mut index = read_index()?;
    let meta = index
        .chats
        .iter()
        .find(|chat| chat.id == chat_id)
        .cloned()
        .with_context(|| format!("Chat {} not found", chat_id))?;
    let dir = chat_dir(chat_id, meta.archived);
    if dir.exists() {
        fs::remove_dir_all(dir)?;
    }
    index.chats.retain(|chat| chat.id != chat_id);
    write_index(&index)?;
    append_chat_activity("chat_purge", chat_id, "")?;
    Ok(())
}

fn ensure_chat_root() -> Result<()> {
    fs::create_dir_all(CHATS_DIR)?;
    fs::create_dir_all(CHAT_ARCHIVE_DIR)?;
    if !Path::new(CHAT_INDEX).exists() {
        write_index(&ChatIndex::default())?;
    }
    Ok(())
}

fn read_index() -> Result<ChatIndex> {
    ensure_parent(CHAT_INDEX)?;
    if !Path::new(CHAT_INDEX).exists() {
        return Ok(ChatIndex::default());
    }
    let text = fs::read_to_string(CHAT_INDEX)?;
    Ok(serde_json::from_str(&text).unwrap_or_default())
}

fn write_index(index: &ChatIndex) -> Result<()> {
    ensure_parent(CHAT_INDEX)?;
    write_json(CHAT_INDEX, index)
}

fn update_meta(meta: ChatSessionMeta) -> Result<()> {
    let mut index = read_index()?;
    if let Some(existing) = index.chats.iter_mut().find(|chat| chat.id == meta.id) {
        *existing = meta;
    } else {
        index.chats.push(meta);
    }
    write_index(&index)
}

fn chat_dir(chat_id: &str, archived: bool) -> PathBuf {
    if archived {
        Path::new(CHAT_ARCHIVE_DIR).join(chat_id)
    } else {
        Path::new(CHATS_DIR).join(chat_id)
    }
}

fn write_json(path: impl AsRef<Path>, value: &impl Serialize) -> Result<()> {
    let path = path.as_ref();
    ensure_parent(path)?;
    fs::write(path, serde_json::to_string_pretty(value)?)?;
    Ok(())
}

fn ensure_parent(path: impl AsRef<Path>) -> Result<()> {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

fn provider_label(provider: &config::JudgeProvider) -> &'static str {
    match provider {
        config::JudgeProvider::Ollama => "ollama",
        config::JudgeProvider::Claude => "claude",
        config::JudgeProvider::Openai => "openai",
        config::JudgeProvider::Gemini => "gemini",
        config::JudgeProvider::Openrouter => "openrouter",
        config::JudgeProvider::None => "none",
    }
}

pub(crate) fn truncate(text: &str, max: usize) -> String {
    if text.chars().count() <= max {
        return text.into();
    }
    let keep = max.saturating_sub(1);
    format!("{}…", text.chars().take(keep).collect::<String>())
}

fn append_chat_activity(event: &str, chat_id: &str, detail: &str) -> Result<()> {
    fs::create_dir_all(config::SESSION_DIR)?;
    let entry = serde_json::json!({
        "event": event,
        "timestamp": Utc::now().to_rfc3339(),
        "chat_id": chat_id,
        "detail": detail,
    });
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(config::ACTIVITY_LOG)?;
    writeln!(file, "{}", serde_json::to_string(&entry)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn create_append_delete_restore_and_purge_chat() {
        let dir = tempdir().unwrap();
        let old = std::env::current_dir().unwrap();
        std::env::set_current_dir(dir.path()).unwrap();
        let config = config::Config::default();

        let chat = create_chat(Some("Scope chat".into()), &config).unwrap();
        assert_eq!(list_chats(false).unwrap().len(), 1);
        append_message(&chat.id, "user", "hello from tests").unwrap();
        assert_eq!(load_messages(&chat.id, false).unwrap().len(), 1);

        soft_delete_chat(&chat.id).unwrap();
        assert!(list_chats(false).unwrap().is_empty());
        assert_eq!(list_chats(true).unwrap().len(), 1);

        restore_chat(&chat.id).unwrap();
        assert_eq!(list_chats(false).unwrap().len(), 1);

        soft_delete_chat(&chat.id).unwrap();
        purge_chat(&chat.id).unwrap();
        assert!(list_chats(true).unwrap().is_empty());
        std::env::set_current_dir(old).unwrap();
    }
}
