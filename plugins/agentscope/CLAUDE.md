# AgentScope Plugin

AgentScope is a scope firewall and audit cockpit for AI coding agents.
It enforces mission policy, monitors Git changes, and stops scope drift before it happens.

## Quick reference

```bash
agentscope init                         # one-time repo setup
agentscope start "your mission"         # record what you're doing
agentscope watch                        # live TUI cockpit
agentscope check                        # policy check + scope audit
agentscope check --json                 # machine-readable output
agentscope diff --problems              # show suspicious/blocked files only
agentscope judge                        # ask the LLM judge
agentscope attach --agent auto --apply  # infer mission from this session's logs
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
| `scope_check` | Hint to run `agentscope check` for full policy output |
| `scope_start` | Hint to start a new mission |
| `agent_attach` | Hint to attach an inferred session |

## Policy config (`agentscope.yaml`)

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
agentscope config set judge.provider ollama      # local, private
agentscope config set judge.provider claude      # requires ANTHROPIC_API_KEY
agentscope config set judge.provider openai      # requires OPENAI_API_KEY
agentscope config set judge.provider gemini      # requires GEMINI_API_KEY
agentscope config set judge.provider openrouter  # requires OPENROUTER_API_KEY
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

Run `agentscope --help` or visit https://github.com/abdouloued/agentscopev2
