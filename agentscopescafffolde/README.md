# AgentScope

```
You gave Claude Code write access to your repo.
It fixed the bug. It also rewrote your auth module.
AgentScope tells you — before you commit.
```

**Your AI agent did exactly what you asked. AgentScope proves it.**

---

## What it does

`agentscope check` compares every file your agent touched against the mission you
gave it, enforces your blocked-path policy deterministically, and optionally asks
a local LLM whether the changes match your intent.

```
  session  sess_4f8a2c  ·  claude-code
  mission  "Fix the rate-limit bug in api/middleware.ts"

  ──────────────────────────────────────────────────────
  scanning working tree against git baseline...

  IN SCOPE   src/api/middleware.ts        +47 −12
  IN SCOPE   src/api/middleware.test.ts   +31 −0
  UNASKED    src/api/router.ts            +18 −3
  UNASKED    src/lib/ratelimit.ts         +62 −0
  BLOCKED    src/auth/session.ts          +9 −41
  BLOCKED    .env.local                   +2 −0
  CLEAN      14 other files unchanged

  ──────────────────────────────────────────────────────

  ╔═══════════════════════════════════════════╗
  ║  BLOCK — session halted                   ║
  ║  2 violations of declared scope policy    ║
  ╚═══════════════════════════════════════════╝

    ✕  src/auth/session.ts  — auth files are protected (policy: no-auth-edits)
    ✕  .env.local           — env files always blocked (policy: no-env-writes)

  LLM judge  (ollama / llama3)

    DRIFT DETECTED  —  38% confidence changes match mission
    "The agent addressed rate-limiting but made unexplained changes
     to authentication logic unrelated to the reported bug."

  2 in scope  ·  2 unasked  ·  2 blocked
```

---

## Install

```bash
cargo install agentscope
```

Or build from source:

```bash
git clone https://github.com/yourusername/agentscope
cd agentscope
cargo build --release
cp target/release/agentscope ~/.local/bin/
```

## Quick start

```bash
# 1. Initialize in your repo (creates agentscope.yaml)
agentscope init

# 2. Tell it what you're asking the agent to do
agentscope start "Fix the rate-limit bug in api/middleware.ts"

# 3. Run your agent as normal (Claude Code, Codex, Cursor, etc.)
# ...

# 4. Check what it actually did
agentscope check
```

That's it. Exit code `0` = clean. Exit code `1` = blocked files found (safe for CI).

---

## Commands

| Command | What it does |
|---|---|
| `agentscope init` | Create `agentscope.yaml` in current repo |
| `agentscope start "mission"` | Begin a session with a stated mission |
| `agentscope check` | Check current changes against mission + policy |
| `agentscope audit last-5` | Review the last 5 sessions |
| `agentscope watch` | Live TUI dashboard |
| `agentscope use claude` | Write `CLAUDE.md` integration file |
| `agentscope use cursor` | Write `.cursor/rules/agentscope.md` |
| `agentscope status` | One-line current session summary |

---

## Policy

Edit `agentscope.yaml` to customize blocked paths:

```yaml
policy:
  blocked:
    - ".env*"
    - "src/auth/**"
    - "**/migrations/**"
    - "**/*.pem"
  warn:
    - "package-lock.json"
  max_files_changed: 20
```

Blocked paths cause `agentscope check` to exit with code `1`.
Warn paths print a warning but don't block.

---

## LLM Judge

AgentScope can ask a local LLM whether the changes actually match your mission.
No data leaves your machine.

```yaml
judge:
  enabled: true
  provider: ollama      # ollama | claude | openai | none
  model: llama3
  endpoint: "http://localhost:11434"
```

Requires [Ollama](https://ollama.ai) running locally. `ollama pull llama3` to get the model.

---

## Agent integrations

```bash
agentscope use claude    # writes CLAUDE.md
agentscope use cursor    # writes .cursor/rules/agentscope.md
agentscope use gemini    # writes GEMINI.md
```

These files instruct the agent to check `.agentscope/session.json` before starting
and run `agentscope check` when done.

---

## CI / CD

```bash
# In your CI pipeline:
agentscope check --json | jq '.blocked'

# Or just use the exit code:
agentscope check || echo "Agent touched blocked files — review required"
```

`--json` outputs machine-readable results for pipeline integration.

---

## Philosophy

AgentScope is not another AI coding agent. It is a **safety rail and audit layer**
that sits on top of all of them.

- **Deterministic policy engine** — blocked paths are enforced with globset matching,
  not AI judgment. No false negatives on `.env` files.
- **Agent-agnostic** — works with Claude Code, Codex, Cursor, Gemini CLI, OpenCode,
  or any agent that writes to your filesystem.
- **Privacy-first** — the LLM judge runs locally via Ollama by default.
  Nothing is sent to the cloud unless you configure it.
- **Git-native** — diffs against your git baseline, not a proprietary snapshot.

---

## Contributing

Issues and PRs welcome. See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT
