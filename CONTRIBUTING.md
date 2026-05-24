# Contributing to AgentScope

Thank you for your interest in contributing to AgentScope! This project aims to be the universal safety layer for AI coding agents, and we welcome contributions from the community.

## Getting Started

```bash
# Fork and clone the repo
git clone https://github.com/YOUR_USERNAME/agentscopev2
cd agentscopev2

# Build
cargo build

# Run tests
cargo test

# Run with debug output
RUST_LOG=debug cargo run -- check
```

## Development Setup

**Requirements:**
- Rust 1.75+ (`rustup update`)
- Git
- [Ollama](https://ollama.ai) (optional, for LLM judge testing)

```bash
# Pull the default judge model (optional)
ollama pull qwen3.5:2b
```

## How to Contribute

### Reporting Bugs

Open an issue with:
- What you expected to happen
- What actually happened
- Steps to reproduce
- `agentscope --version` output
- Your OS and Rust version

### Suggesting Features

Open an issue tagged `enhancement` with:
- The problem you're solving
- Your proposed solution
- Why this belongs in core vs. a plugin

### Pull Requests

1. **Fork** the repo and create a branch from `main`
2. **Write tests** for any new functionality
3. **Run `cargo test`** — all tests must pass
4. **Run `cargo clippy`** — no new warnings
5. **Format with `cargo fmt`**
6. **Open a PR** with a clear description of what and why

### What We're Looking For

| Area | Examples |
|---|---|
| **New agent integrations** | Aider, Continue, Windsurf, etc. |
| **Policy engine features** | Regex path matching, file-type rules, custom validators |
| **LLM judge providers** | Groq, local llama.cpp, Mistral API |
| **Output formats** | SARIF, GitHub annotations, Slack webhooks |
| **TUI improvements** | File preview, diff view, keyboard shortcuts |
| **Documentation** | Tutorials, blog posts, video walkthroughs |

## Project Structure

```
src/
├── main.rs      # Entry point, command dispatch
├── cli.rs       # Clap CLI definitions
├── config.rs    # YAML config, agent integration templates
├── git.rs       # git2 integration (diffs, baselines)
├── policy.rs    # Glob-based policy engine, scope hints
├── session.rs   # Session lifecycle (start, check, status)
├── judge.rs     # LLM judge (Ollama, Claude, OpenAI)
├── output.rs    # Terminal formatting, CheckReport
├── tui.rs       # Ratatui live dashboard
└── audit.rs     # Activity log and session history
```

## Code Style

- Follow standard Rust conventions (`cargo fmt`)
- Use `anyhow::Result` for error handling in application code
- Use `thiserror` for library-style error types
- Keep functions small and well-documented
- Prefer clarity over cleverness

## Testing

```bash
# Run all tests
cargo test

# Run a specific test
cargo test test_blocked_paths

# Run with output
cargo test -- --nocapture
```

## Release Process

Releases are tagged from `main`:

```bash
cargo build --release
# Binary at target/release/agentscope
```

## Code of Conduct

Be kind. Be constructive. We're all here to make AI agents safer.

## License

By contributing, you agree that your contributions will be licensed under the [MIT License](LICENSE).
