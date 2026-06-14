# ScopeWarden skill · cursor

This file instructs `cursor` to use ScopeWarden as a scope firewall before and after making changes.

## When to run ScopeWarden

| Trigger | Command |
|---------|--------|
| Before starting work | `scopewarden status` |
| While working | `scopewarden watch` (live TUI cockpit) |
| Before finishing | `scopewarden check` |
| Before committing | `scopewarden diff --problems` |

## Quick reference

```
scopewarden init                          # one-time repo setup
scopewarden start "your mission"          # record what you're doing
scopewarden watch                         # live cockpit (1=review 2=chat 3=dash 4=sessions 5=live)
scopewarden check                         # policy check + scope audit
scopewarden check --json                  # machine-readable output
scopewarden judge                         # ask the LLM judge
scopewarden diff --problems               # show suspicious/blocked files only
scopewarden attach --agent auto --apply   # infer mission from this agent's logs
```

## Status labels

| Badge | Meaning |
|-------|---------|
| `EXPECTED` | File matches the active mission scope |
| `SUSPICIOUS` | Changed but no mission rule matches |
| `BLOCKED` | Matched a blocked policy path — hard stop |
| `IGNORED` | Clean, stale, or explicitly excluded |

## TUI keyboard shortcuts

| Key | Action |
|-----|--------|
| `1`–`5` | Switch mode (Review/Chat/Dashboard/Sessions/Live) |
| `Enter` | Open diff overlay for selected file |
| `j` | Run judge on selected file |
| `a` / `b` | Allow / block selected file |
| `t` | Cycle themes (scopewarden/codex/claude/openclaw/high-contrast) |
| `?` | Help overlay |
| `q` | Quit |

## Judge providers

ScopeWarden supports Ollama (local/private), Claude, OpenAI, Gemini, and OpenRouter.

```
scopewarden config set judge.provider ollama      # local, private
scopewarden config set judge.provider claude      # requires ANTHROPIC_API_KEY
scopewarden config set judge.provider openai      # requires OPENAI_API_KEY
scopewarden config set judge.provider gemini      # requires GEMINI_API_KEY
scopewarden config set judge.provider openrouter  # requires OPENROUTER_API_KEY
```

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

Blocked patterns are enforced deterministically — no model can override them.

## More info

Run `scopewarden --help` or visit https://github.com/abdouloued/scopewarden
