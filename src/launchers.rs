use anyhow::{Context, Result};
use console::style;
use std::{
    env, fmt, fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

const MAX_CAPTURE_BYTES: usize = 12_000;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LauncherId {
    ClaudeCode,
    CodexApp,
    OpenClaw,
    HermesAgent,
    CodexCli,
    OpenCode,
    #[allow(dead_code)]
    Custom(String),
}

impl LauncherId {
    pub fn slug(&self) -> &str {
        match self {
            LauncherId::ClaudeCode => "claude-code",
            LauncherId::CodexApp => "codex-app",
            LauncherId::OpenClaw => "openclaw",
            LauncherId::HermesAgent => "hermes-agent",
            LauncherId::CodexCli => "codex",
            LauncherId::OpenCode => "opencode",
            LauncherId::Custom(value) => value,
        }
    }
}

impl fmt::Display for LauncherId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.slug())
    }
}

#[derive(Clone, Debug)]
pub enum LaunchCandidate {
    Binary { command: String, args: Vec<String> },
    MacApp { name: String, paths: Vec<PathBuf> },
}

impl LaunchCandidate {
    pub fn binary(command: &str, args: &[&str]) -> Self {
        Self::Binary {
            command: command.to_string(),
            args: args.iter().map(|arg| arg.to_string()).collect(),
        }
    }

    fn mac_app(name: &str, paths: &[&str]) -> Self {
        Self::MacApp {
            name: name.to_string(),
            paths: paths.iter().map(PathBuf::from).collect(),
        }
    }

    fn display_command(&self) -> String {
        match self {
            LaunchCandidate::Binary { command, args } => {
                if args.is_empty() {
                    command.clone()
                } else {
                    format!("{} {}", command, args.join(" "))
                }
            }
            LaunchCandidate::MacApp { name, .. } => format!("open -Ra {}", shell_quote(name)),
        }
    }

    fn with_binary_command(&self, resolved_path: &Path) -> Self {
        match self {
            LaunchCandidate::Binary { args, .. } => Self::Binary {
                command: resolved_path.display().to_string(),
                args: args.clone(),
            },
            LaunchCandidate::MacApp { .. } => self.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct LauncherSpec {
    pub id: LauncherId,
    pub name: String,
    pub candidates: Vec<LaunchCandidate>,
}

impl LauncherSpec {
    pub fn new(id: LauncherId, name: &str, candidates: Vec<LaunchCandidate>) -> Self {
        Self {
            id,
            name: name.to_string(),
            candidates,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InstallStatus {
    Installed,
    Missing,
}

#[derive(Clone, Debug)]
pub struct DetectedLauncher {
    pub id: LauncherId,
    pub name: String,
    pub status: InstallStatus,
    pub resolved_command: Option<String>,
    pub smoke_candidate: Option<LaunchCandidate>,
    pub note: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SmokeStatus {
    Passed,
    Failed,
    TimedOut,
    Skipped,
}

#[derive(Clone, Debug)]
pub struct SmokeResult {
    pub id: LauncherId,
    pub name: String,
    pub status: SmokeStatus,
    pub command: String,
    pub exit_code: Option<i32>,
    pub launched: bool,
    pub stdout: String,
    pub stderr: String,
    pub duration_ms: u128,
    pub error: Option<String>,
}

pub async fn list_command() -> Result<()> {
    let detected = detect_launchers()?;
    println!(
        "{}",
        format_launcher_table(&detected, console::colors_enabled())
    );
    Ok(())
}

pub async fn test_command(app: Option<String>, timeout_secs: u64, summary: bool) -> Result<()> {
    let timeout = Duration::from_secs(timeout_secs.max(1));
    let detected = detect_launchers()?;
    let selected = select_launchers(detected, app.as_deref())?;

    if selected.is_empty() {
        println!(
            "{}",
            style("No launchers matched. Run `scopewarden launchers list`.").yellow()
        );
        return Ok(());
    }

    let mut results = Vec::with_capacity(selected.len());
    for launcher in selected {
        results.push(smoke_test_launcher(&launcher, timeout)?);
    }

    if summary {
        println!("{}", format_summary(&results, console::colors_enabled()));
    } else {
        println!(
            "{}",
            format_smoke_report(&results, console::colors_enabled())
        );
    }

    Ok(())
}

pub fn default_catalog() -> Vec<LauncherSpec> {
    vec![
        LauncherSpec::new(
            LauncherId::ClaudeCode,
            "Claude Code",
            vec![LaunchCandidate::binary("claude", &["--version"])],
        ),
        LauncherSpec::new(
            LauncherId::CodexApp,
            "Codex App",
            vec![
                LaunchCandidate::mac_app(
                    "Codex",
                    &[
                        "/Applications/Codex.app",
                        "/System/Applications/Codex.app",
                        "~/Applications/Codex.app",
                    ],
                ),
                LaunchCandidate::binary("codex-app", &["--version"]),
            ],
        ),
        LauncherSpec::new(
            LauncherId::OpenClaw,
            "OpenClaw",
            vec![LaunchCandidate::binary("openclaw", &["--version"])],
        ),
        LauncherSpec::new(
            LauncherId::HermesAgent,
            "Hermes Agent",
            vec![
                LaunchCandidate::binary("hermes-agent", &["--version"]),
                LaunchCandidate::binary("hermes", &["--version"]),
            ],
        ),
        LauncherSpec::new(
            LauncherId::CodexCli,
            "Codex",
            vec![LaunchCandidate::binary("codex", &["--version"])],
        ),
        LauncherSpec::new(
            LauncherId::OpenCode,
            "OpenCode",
            vec![LaunchCandidate::binary("opencode", &["--version"])],
        ),
    ]
}

pub fn detect_launchers() -> Result<Vec<DetectedLauncher>> {
    let path = env::var_os("PATH").unwrap_or_default();
    detect_launchers_with_path(&default_catalog(), path)
}

pub fn detect_launchers_with_path<P>(
    catalog: &[LauncherSpec],
    path: P,
) -> Result<Vec<DetectedLauncher>>
where
    P: AsRef<std::ffi::OsStr>,
{
    let path_value = path.as_ref();
    catalog
        .iter()
        .map(|spec| detect_one(spec, path_value))
        .collect()
}

pub fn smoke_test_launcher(launcher: &DetectedLauncher, timeout: Duration) -> Result<SmokeResult> {
    let start = Instant::now();
    let Some(candidate) = &launcher.smoke_candidate else {
        return Ok(SmokeResult {
            id: launcher.id.clone(),
            name: launcher.name.clone(),
            status: SmokeStatus::Skipped,
            command: launcher
                .resolved_command
                .clone()
                .unwrap_or_else(|| "not installed".to_string()),
            exit_code: None,
            launched: false,
            stdout: String::new(),
            stderr: String::new(),
            duration_ms: 0,
            error: Some(launcher.note.clone()),
        });
    };

    let command_label = candidate.display_command();
    let mut command = match candidate {
        LaunchCandidate::Binary { command, args } => {
            let mut cmd = Command::new(command);
            cmd.args(args);
            cmd
        }
        LaunchCandidate::MacApp { name, .. } => {
            let mut cmd = Command::new("/usr/bin/open");
            cmd.args(["-Ra", name]);
            cmd
        }
    };

    command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .env("OLLAMA_MODEL", "qwen3.5:2b")
        .env("SCOPEWARDEN_SMOKE_TEST", "1");

    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(err) => {
            return Ok(SmokeResult {
                id: launcher.id.clone(),
                name: launcher.name.clone(),
                status: SmokeStatus::Failed,
                command: command_label,
                exit_code: None,
                launched: false,
                stdout: String::new(),
                stderr: String::new(),
                duration_ms: start.elapsed().as_millis(),
                error: Some(err.to_string()),
            });
        }
    };

    loop {
        if let Some(status) = child.try_wait()? {
            let output = child
                .wait_with_output()
                .context("failed to collect launcher smoke-test output")?;
            let stdout = trim_capture(String::from_utf8_lossy(&output.stdout).as_ref());
            let stderr = trim_capture(String::from_utf8_lossy(&output.stderr).as_ref());
            let code = status.code();
            return Ok(SmokeResult {
                id: launcher.id.clone(),
                name: launcher.name.clone(),
                status: if status.success() {
                    SmokeStatus::Passed
                } else {
                    SmokeStatus::Failed
                },
                command: command_label,
                exit_code: code,
                launched: status.success(),
                stdout,
                stderr,
                duration_ms: start.elapsed().as_millis(),
                error: None,
            });
        }

        if start.elapsed() >= timeout {
            let _ = child.kill();
            let output = child
                .wait_with_output()
                .context("failed to collect timed-out launcher output")?;
            return Ok(SmokeResult {
                id: launcher.id.clone(),
                name: launcher.name.clone(),
                status: SmokeStatus::TimedOut,
                command: command_label,
                exit_code: None,
                launched: false,
                stdout: trim_capture(String::from_utf8_lossy(&output.stdout).as_ref()),
                stderr: trim_capture(String::from_utf8_lossy(&output.stderr).as_ref()),
                duration_ms: start.elapsed().as_millis(),
                error: Some(format!("timed out after {}ms", timeout.as_millis())),
            });
        }

        thread::sleep(Duration::from_millis(20));
    }
}

pub fn format_launcher_table(launchers: &[DetectedLauncher], color: bool) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "  {}\n",
        paint(
            "╭─ AI Launcher Lab ─────────────────────────────────────────╮",
            "cyan",
            color
        )
    ));
    out.push_str(&format!(
        "  {} qwen3.5:2b · safe startup checks · zero project writes      {}\n",
        paint("│", "cyan", color),
        paint("│", "cyan", color)
    ));
    out.push_str(&format!(
        "  {}\n\n",
        paint(
            "╰────────────────────────────────────────────────────────────╯",
            "cyan",
            color
        )
    ));
    out.push_str("  APP            STATUS       SAFE TEST COMMAND\n");
    out.push_str("  -------------  -----------  -------------------------------\n");
    for launcher in launchers {
        let status_plain = match launcher.status {
            InstallStatus::Installed => "installed",
            InstallStatus::Missing => "missing",
        };
        let status = match launcher.status {
            InstallStatus::Installed => paint(&format!("{:<11}", status_plain), "green", color),
            InstallStatus::Missing => paint(&format!("{:<11}", status_plain), "yellow", color),
        };
        let command = launcher
            .resolved_command
            .as_deref()
            .unwrap_or("not available");
        out.push_str(&format!(
            "  {:<13}  {}  {}\n",
            launcher.name, status, command
        ));
    }
    out.push_str("\n  Next:\n");
    out.push_str("    scopewarden launchers test\n");
    out.push_str("    scopewarden launchers test codex\n");
    out
}

pub fn format_smoke_report(results: &[SmokeResult], color: bool) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "  {}\n",
        paint(
            "╭─ AI Launcher Smoke Test ──────────────────────────────────╮",
            "cyan",
            color
        )
    ));
    out.push_str(&format!(
        "  {} safe startup checks · timeout cleanup · qwen3.5:2b ready    {}\n",
        paint("│", "cyan", color),
        paint("│", "cyan", color)
    ));
    out.push_str(&format!(
        "  {}\n\n",
        paint(
            "╰────────────────────────────────────────────────────────────╯",
            "cyan",
            color
        )
    ));

    for result in results {
        let status = smoke_label(&result.status, color);
        let code = result
            .exit_code
            .map(|code| code.to_string())
            .unwrap_or_else(|| "-".to_string());
        let launch_state = if result.launched {
            "launch ok"
        } else {
            "no launch"
        };
        out.push_str(&format!(
            "  {:<13} {:<15} {}  exit {:<3}  {:>5}ms  {}\n",
            result.name,
            format!("({})", result.id.slug()),
            status,
            code,
            result.duration_ms,
            launch_state
        ));
        out.push_str(&format!("    command {}\n", result.command));
        if !result.stdout.trim().is_empty() {
            out.push_str(&format_capture("stdout", &result.stdout));
        }
        if !result.stderr.trim().is_empty() {
            out.push_str(&format_capture("stderr", &result.stderr));
        }
        if let Some(error) = &result.error {
            out.push_str(&format!("    error   {}\n", error));
        }
        out.push('\n');
    }

    out.push_str(&format_summary(results, color));
    out
}

pub fn format_summary(results: &[SmokeResult], color: bool) -> String {
    let passed = results
        .iter()
        .filter(|result| result.status == SmokeStatus::Passed)
        .count();
    let failed = results
        .iter()
        .filter(|result| result.status == SmokeStatus::Failed)
        .count();
    let skipped = results
        .iter()
        .filter(|result| result.status == SmokeStatus::Skipped)
        .count();
    let timed_out = results
        .iter()
        .filter(|result| result.status == SmokeStatus::TimedOut)
        .count();
    let total_ms: u128 = results.iter().map(|result| result.duration_ms).sum();
    let names = if results.is_empty() {
        "none".to_string()
    } else if results.len() > 2 {
        format!("{} launchers", results.len())
    } else {
        results
            .iter()
            .map(|result| result.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    };

    format!(
        "  Summary  {} | {} passed | {} failed | {} timed out | {} skipped | {}\n",
        names,
        paint(&passed.to_string(), "green", color),
        paint(&failed.to_string(), "red", color),
        paint(&timed_out.to_string(), "yellow", color),
        paint(&skipped.to_string(), "dim", color),
        paint(&format!("{}ms", total_ms), "cyan", color),
    )
}

fn detect_one(spec: &LauncherSpec, path: &std::ffi::OsStr) -> Result<DetectedLauncher> {
    for candidate in &spec.candidates {
        match candidate {
            LaunchCandidate::Binary { command, .. } => {
                if let Some(found) = find_in_path(command, path) {
                    let resolved_candidate = candidate.with_binary_command(&found);
                    return Ok(DetectedLauncher {
                        id: spec.id.clone(),
                        name: spec.name.clone(),
                        status: InstallStatus::Installed,
                        resolved_command: Some(command.clone()),
                        smoke_candidate: Some(resolved_candidate),
                        note: format!("found at {}", found.display()),
                    });
                }
            }
            LaunchCandidate::MacApp { name: _, paths } => {
                if let Some(found) = paths.iter().find_map(|p| expand_existing_path(p.as_path())) {
                    return Ok(DetectedLauncher {
                        id: spec.id.clone(),
                        name: spec.name.clone(),
                        status: InstallStatus::Installed,
                        resolved_command: Some(candidate.display_command()),
                        smoke_candidate: Some(candidate.clone()),
                        note: format!("found at {}", found.display()),
                    });
                }
            }
        }
    }

    Ok(DetectedLauncher {
        id: spec.id.clone(),
        name: spec.name.clone(),
        status: InstallStatus::Missing,
        resolved_command: None,
        smoke_candidate: None,
        note: format!("not found; tried {}", tried_candidates(&spec.candidates)),
    })
}

fn find_in_path(command: &str, path: &std::ffi::OsStr) -> Option<PathBuf> {
    let command_path = Path::new(command);
    if command_path.components().count() > 1 {
        return is_executable(command_path).then(|| command_path.to_path_buf());
    }

    env::split_paths(path)
        .map(|dir| dir.join(command))
        .find(|path| is_executable(path))
}

fn is_executable(path: &Path) -> bool {
    fs::metadata(path)
        .map(|metadata| metadata.is_file())
        .unwrap_or(false)
}

fn expand_existing_path(path: &Path) -> Option<PathBuf> {
    let expanded = if let Some(stripped) = path.to_string_lossy().strip_prefix("~/") {
        env::var_os("HOME")
            .map(PathBuf::from)
            .map(|home| home.join(stripped))
            .unwrap_or_else(|| path.to_path_buf())
    } else {
        path.to_path_buf()
    };
    expanded.exists().then_some(expanded)
}

fn tried_candidates(candidates: &[LaunchCandidate]) -> String {
    candidates
        .iter()
        .map(LaunchCandidate::display_command)
        .collect::<Vec<_>>()
        .join(", ")
}

fn select_launchers(
    detected: Vec<DetectedLauncher>,
    app: Option<&str>,
) -> Result<Vec<DetectedLauncher>> {
    let Some(app) = app else {
        return Ok(detected);
    };
    let normalized = normalize(app);
    let selected = detected
        .into_iter()
        .filter(|launcher| {
            normalize(&launcher.name) == normalized || normalize(launcher.id.slug()) == normalized
        })
        .collect::<Vec<_>>();
    Ok(selected)
}

fn normalize(value: &str) -> String {
    value
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

fn trim_capture(value: &str) -> String {
    if value.len() <= MAX_CAPTURE_BYTES {
        value.to_string()
    } else {
        format!("{}…\n[truncated]", &value[..MAX_CAPTURE_BYTES])
    }
}

fn format_capture(label: &str, value: &str) -> String {
    let mut out = String::new();
    for line in value.lines().take(5) {
        out.push_str(&format!("    {:<7} {}\n", label, line));
    }
    if value.lines().count() > 5 {
        out.push_str("            ...\n");
    }
    out
}

fn smoke_label(status: &SmokeStatus, color: bool) -> String {
    match status {
        SmokeStatus::Passed => paint("PASS", "green", color),
        SmokeStatus::Failed => paint("FAIL", "red", color),
        SmokeStatus::TimedOut => paint("TIMEOUT", "yellow", color),
        SmokeStatus::Skipped => paint("SKIP", "dim", color),
    }
}

fn paint(value: &str, color_name: &str, color: bool) -> String {
    if !color {
        return value.to_string();
    }

    match color_name {
        "green" => style(value).green().bold().to_string(),
        "red" => style(value).red().bold().to_string(),
        "yellow" => style(value).yellow().bold().to_string(),
        "cyan" => style(value).cyan().to_string(),
        "dim" => style(value).dim().to_string(),
        _ => value.to_string(),
    }
}

fn shell_quote(value: &str) -> String {
    if value
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || ch == '-')
    {
        value.to_string()
    } else {
        format!("'{}'", value.replace('\'', "'\\''"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, os::unix::fs::PermissionsExt, path::Path, time::Duration};

    fn make_bin(dir: &Path, name: &str, body: &str) {
        let path = dir.join(name);
        fs::write(&path, body).unwrap();
        let mut perms = fs::metadata(&path).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(path, perms).unwrap();
    }

    #[test]
    fn detects_binary_from_path() {
        let tmp = tempfile::tempdir().unwrap();
        make_bin(tmp.path(), "claude", "#!/bin/sh\necho claude\n");
        let catalog = vec![LauncherSpec::new(
            LauncherId::ClaudeCode,
            "Claude Code",
            vec![LaunchCandidate::binary("claude", &["--version"])],
        )];

        let detected = detect_launchers_with_path(&catalog, tmp.path()).unwrap();

        assert_eq!(detected[0].status, InstallStatus::Installed);
        assert_eq!(detected[0].resolved_command.as_deref(), Some("claude"));
    }

    #[test]
    fn reports_missing_binary_without_error() {
        let tmp = tempfile::tempdir().unwrap();
        let catalog = vec![LauncherSpec::new(
            LauncherId::OpenCode,
            "OpenCode",
            vec![LaunchCandidate::binary("opencode", &["--version"])],
        )];

        let detected = detect_launchers_with_path(&catalog, tmp.path()).unwrap();

        assert_eq!(detected[0].status, InstallStatus::Missing);
        assert!(detected[0].note.contains("not found"));
    }

    #[test]
    fn smoke_test_captures_output_and_exit_code() {
        let tmp = tempfile::tempdir().unwrap();
        make_bin(
            tmp.path(),
            "ok-agent",
            "#!/bin/sh\necho startup-ok\necho warning-line >&2\nexit 0\n",
        );
        let spec = LauncherSpec::new(
            LauncherId::Custom("ok".to_string()),
            "OK Agent",
            vec![LaunchCandidate::binary("ok-agent", &[])],
        );
        let detected = detect_launchers_with_path(&[spec], tmp.path()).unwrap();

        let result = smoke_test_launcher(&detected[0], Duration::from_secs(2)).unwrap();

        assert_eq!(result.exit_code, Some(0));
        assert!(result.launched);
        assert!(result.stdout.contains("startup-ok"));
        assert!(result.stderr.contains("warning-line"));
    }

    #[test]
    fn smoke_test_times_out_and_marks_process_unsuccessful() {
        let tmp = tempfile::tempdir().unwrap();
        make_bin(tmp.path(), "slow-agent", "#!/bin/sh\nsleep 2\n");
        let spec = LauncherSpec::new(
            LauncherId::Custom("slow".to_string()),
            "Slow Agent",
            vec![LaunchCandidate::binary("slow-agent", &[])],
        );
        let detected = detect_launchers_with_path(&[spec], tmp.path()).unwrap();

        let result = smoke_test_launcher(&detected[0], Duration::from_millis(100)).unwrap();

        assert_eq!(result.status, SmokeStatus::TimedOut);
        assert!(!result.launched);
    }

    #[test]
    fn summary_is_concise_and_status_colored_when_enabled() {
        let result = SmokeResult {
            id: LauncherId::CodexCli,
            name: "Codex".to_string(),
            status: SmokeStatus::Passed,
            command: "codex --version".to_string(),
            exit_code: Some(0),
            launched: true,
            stdout: "codex 1.0.0\n".to_string(),
            stderr: String::new(),
            duration_ms: 42,
            error: None,
        };

        let summary = format_summary(&[result], false);

        assert!(summary.contains("1 passed"));
        assert!(summary.contains("Codex"));
        assert!(summary.contains("42ms"));
    }
}
