# ScopeWarden Plugin

ScopeWarden is a scope firewall and audit cockpit for AI coding agents.
It enforces mission policy, monitors Git changes, and stops scope drift before it happens.

## Quick reference

```bash
scopewarden init                         # one-time repo setup
scopewarden start "your mission"         # record what you're doing
scopewarden watch                        # live TUI cockpit
scopewarden check                        # policy check + scope audit
scopewarden check --json                 # machine-readable output
scopewarden diff --problems              # show suspicious/blocked files only
scopewarden judge                        # ask the LLM judge
scopewarden attach --agent auto --apply  # infer mission from this session's logs
```

## Status labels

| Badge | Meaning |
|-------|---------|
| `EXPECTED` | File matches the active mission scope |
| `SUSPICIOUS` | Changed but no mission rule matches |
| `BLOCKED` | Matched a blocked policy path — hard stop |
| `IGNORED` | Clean, stale, or explicitly excluded |

**BLOCKED files are a hard stop.** Do not commit them. Report to the user.

## MCP tools available

| Tool | What it does |
|------|-------------|
| `scope_status` | Returns the current session JSON |
| `scope_check` | Hint to run `scopewarden check` for full policy output |
| `scope_start` | Hint to start a new mission |
| `agent_attach` | Hint to attach an inferred session |

## Policy config (`scopewarden.yaml`)

```yaml
policy:
  blocked:
    - ".env"
    - "**/.env.*"
    - "**/secrets/**"
    - "**/*.pem"
    - "**/*.key"
  warn:
    - "package-lock.json"
    - "yarn.lock"
    - "Cargo.lock"
  max_files_changed: 20
  max_lines_changed: 800
```

## Judge providers

```bash
scopewarden config set judge.provider ollama      # local, private
scopewarden config set judge.provider claude      # requires ANTHROPIC_API_KEY
scopewarden config set judge.provider openai      # requires OPENAI_API_KEY
scopewarden config set judge.provider gemini      # requires GEMINI_API_KEY
scopewarden config set judge.provider openrouter  # requires OPENROUTER_API_KEY
```

## TUI keyboard shortcuts

| Key | Action |
|-----|--------|
| `1`–`5` | Switch mode (Review/Chat/Dashboard/Sessions/Live) |
| `Enter` | Open diff overlay for selected file |
| `j` | Run judge on selected file |
| `a` / `b` | Allow / block selected file |
| `t` | Cycle themes |
| `?` | Help overlay |
| `q` | Quit |

Run `scopewarden --help` or visit https://github.com/abdouloued/scopewarden
