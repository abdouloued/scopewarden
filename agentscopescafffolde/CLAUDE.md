# CLAUDE.md — Instructions for Claude Code

Read AGENTS.md first. This file adds Claude-specific notes.

## Before you start any task

1. Read AGENTS.md — architecture rules, file map, colour constants
2. Read docs/TUI_VISUAL_SPEC.md if your task involves tui.rs
3. Check the active session: cat .agentscope/session.json (if it exists)

## Files you may NOT touch without explicit instruction

- README.md hero section (first 8 lines after the title)
- The agentscope.yaml config schema (field renames break existing installs)
- Cargo.toml dependency versions unless fixing a build error

## Files you should always update together

- If you change policy.rs (FileVerdict enum) → update output.rs + tui.rs to match
- If you add a new Command to cli.rs → add handler in main.rs + implement in correct module
- If you change session.rs Session struct → check audit.rs deserialization still works

## Code style

- Use anyhow::Result<()> for all fallible functions, never unwrap()
- All pub functions must have a doc comment (/// Description)
- Constants in SCREAMING_SNAKE_CASE, local variables in snake_case
- No clippy warnings (run cargo clippy before considering task done)
- Run cargo fmt before considering task done

## TUI colour rule (CRITICAL)

Never use Color::White, Color::Yellow, Color::Blue.
Always use Color::Rgb(r, g, b) from the constants defined at top of tui.rs.
If you need a new colour, add it to both tui.rs constants AND docs/TUI_VISUAL_SPEC.md.

## Output style rule

Never call println! directly in business logic.
All terminal output goes through crate::output::Printer methods.
Exception: tui.rs uses ratatui rendering only.
