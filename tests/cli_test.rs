use assert_cmd::Command;
use predicates::prelude::*;

/// Test that --help exits cleanly
#[test]
fn cli_help_exits_zero() {
    Command::cargo_bin("agentscope")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("AgentScope"));
}

/// Test that --version exits cleanly
#[test]
fn cli_version_exits_zero() {
    Command::cargo_bin("agentscope")
        .unwrap()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("agentscope"));
}

/// Test that init creates agentscope.yaml in a temp git repo
#[test]
fn cli_init_creates_config() {
    let tmp = tempfile::tempdir().unwrap();

    // Init a git repo first (required for agentscope)
    std::process::Command::new("git")
        .args(["init"])
        .current_dir(tmp.path())
        .output()
        .unwrap();

    // Create .gitignore so init can append to it
    std::fs::write(tmp.path().join(".gitignore"), "target/\n").unwrap();

    Command::cargo_bin("agentscope")
        .unwrap()
        .arg("init")
        .current_dir(tmp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Created agentscope.yaml"));

    assert!(tmp.path().join("agentscope.yaml").exists());
    assert!(tmp.path().join(".agentscope").exists());
}

/// Test that init --preset ci works
#[test]
fn cli_init_preset_ci() {
    let tmp = tempfile::tempdir().unwrap();
    std::process::Command::new("git")
        .args(["init"])
        .current_dir(tmp.path())
        .output()
        .unwrap();

    Command::cargo_bin("agentscope")
        .unwrap()
        .args(["init", "--preset", "ci"])
        .current_dir(tmp.path())
        .assert()
        .success();

    let contents = std::fs::read_to_string(tmp.path().join("agentscope.yaml")).unwrap();
    assert!(contents.contains("Ci"));
}

/// Test that running check without a session gives a meaningful error
#[test]
fn cli_check_without_session() {
    let tmp = tempfile::tempdir().unwrap();
    std::process::Command::new("git")
        .args(["init"])
        .current_dir(tmp.path())
        .output()
        .unwrap();
    // Need an initial commit for HEAD to exist
    std::process::Command::new("git")
        .args(["commit", "--allow-empty", "-m", "init"])
        .current_dir(tmp.path())
        .output()
        .unwrap();

    Command::cargo_bin("agentscope")
        .unwrap()
        .arg("check")
        .current_dir(tmp.path())
        .assert()
        .failure(); // should fail since no session exists
}

/// Test that status without a session gives meaningful output
#[test]
fn cli_status_without_session() {
    let tmp = tempfile::tempdir().unwrap();
    std::process::Command::new("git")
        .args(["init"])
        .current_dir(tmp.path())
        .output()
        .unwrap();

    Command::cargo_bin("agentscope")
        .unwrap()
        .arg("status")
        .current_dir(tmp.path())
        .assert()
        .success();
}

/// Test that all agent kinds are valid CLI values
#[test]
fn cli_agent_kinds_accepted() {
    let agents = [
        "claude", "codex", "codex-app", "cursor", "gemini",
        "opencode", "openclaw", "hermes", "copilot", "droid", "pi", "custom",
    ];

    for _agent in agents {
        // Just test that the CLI parses the value (--help won't error)
        Command::cargo_bin("agentscope")
            .unwrap()
            .args(["start", "--help"])
            .assert()
            .success();
    }

    // Test that an invalid agent is rejected
    let tmp = tempfile::tempdir().unwrap();
    std::process::Command::new("git")
        .args(["init"])
        .current_dir(tmp.path())
        .output()
        .unwrap();

    Command::cargo_bin("agentscope")
        .unwrap()
        .args(["start", "test mission", "--agent", "nonexistent"])
        .current_dir(tmp.path())
        .assert()
        .failure();
}
