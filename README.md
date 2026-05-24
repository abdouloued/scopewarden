# 🛡️ AgentScope

<div align="center">

**Your AI agent did exactly what you asked. AgentScope proves it.**

[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](LICENSE)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square)](CONTRIBUTING.md)

*Scope firewall and audit layer for AI coding agents.*
*Works with Claude Code, Codex, Cursor, Gemini CLI, OpenCode, Hermes, and more.*

</div>

---

```
You gave Claude Code write access to your repo.
It fixed the bug. It also rewrote your auth module.
AgentScope tells you — before you commit.
```

---

## The Problem

You run an AI coding agent. You ask it to "fix the rate-limit bug." It does that.
It also quietly rewrites your authentication module, modifies `.env`, and touches 14 other files.

You don't notice until production breaks.

## The Solution

```bash
agentscope check
```

```
  session  01KSDYJN1V3X  ·  claude-code
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

    ✕  src/auth/session.ts  — auth files are protected
    ✕  .env.local           — env files always blocked

  LLM judge  (ollama / qwen3.5:2b)

    DRIFT DETECTED  —  38% confidence changes match mission
    "The agent addressed rate-limiting but made unexplained changes
     to authentication logic unrelated to the reported bug."

  2 in scope  ·  2 unasked  ·  2 blocked
```

AgentScope is **not another AI coding agent**. It is the **safety layer** that watches all of them.

---

## ⚡ Install

```bash
cargo install agentscope
```

Or build from source:

```bash
git clone https://github.com/abdouloued/agentscopev2
cd agentscopev2
cargo build --release
cp target/release/agentscope ~/.local/bin/
```

## 🚀 Quick Start

```bash
# 1. Initialize in your repo
agentscope init

# 2. Tell it what you're asking the agent to do
agentscope start "Fix the rate-limit bug in api/middleware.ts"

# 3. Run your agent as normal
#    Claude Code, Codex, Cursor, Gemini CLI — any of them

# 4. Check what it actually did
agentscope check
```

That's it. Exit code `0` = clean. Exit code `1` = blocked files found.

---

## 📋 Commands

| Command | What it does |
|---|---|
| `agentscope init` | Create `agentscope.yaml` in current repo |
| `agentscope start "mission"` | Begin a session with a stated mission |
| `agentscope check` | Check current changes against mission + policy |
| `agentscope check --json` | Machine-readable output for CI pipelines |
| `agentscope audit last-5` | Review the last 5 sessions |
| `agentscope watch` | Live TUI dashboard — see changes in real time |
| `agentscope use claude` | Write agent integration file |
| `agentscope status` | One-line current session summary |

---

## 🤖 Supported Agents

AgentScope works with **any agent that writes to your filesystem**. First-class integrations:

| Agent | Launch command | Integration |
|---|---|---|
| **Claude Code** | `ollama launch claude --model qwen3.5` | `agentscope use claude` |
| **Codex** | `ollama launch codex --model qwen3.5` | `agentscope use codex` |
| **Codex App** | `ollama launch codex-app --model qwen3.5` | `agentscope use codex-app` |
| **Cursor** | *Open in Cursor IDE* | `agentscope use cursor` |
| **Gemini CLI** | *Run gemini in terminal* | `agentscope use gemini` |
| **OpenCode** | `ollama launch opencode --model qwen3.5` | `agentscope use opencode` |
| **OpenClaw** | `ollama launch openclaw --model qwen3.5` | `agentscope use openclaw` |
| **Hermes Agent** | `ollama launch hermes --model qwen3.5` | `agentscope use hermes` |
| **Copilot CLI** | *Run copilot in terminal* | `agentscope use copilot` |
| **Droid** | `ollama launch droid --model qwen3.5` | `agentscope use droid` |
| **Custom** | *Any agent* | `agentscope use custom` |

---

## 🔒 Policy

Edit `agentscope.yaml` to define what's **never allowed**:

```yaml
policy:
  blocked:
    - ".env*"
    - "src/auth/**"
    - "**/migrations/**"
    - "**/*.pem"
    - "**/*.key"
  warn:
    - "package-lock.json"
    - "yarn.lock"
    - "Cargo.lock"
  max_files_changed: 20
```

- **Blocked** paths → `agentscope check` exits with code `1`. Deterministic. No AI judgment.
- **Warn** paths → printed as warnings, don't block.
- **Limits** → flag sessions where the agent changed too many files.

---

## 🧠 LLM Judge

AgentScope optionally asks a **local LLM** whether the changes match your mission.
No data leaves your machine.

```yaml
judge:
  enabled: true
  provider: ollama        # ollama | claude | openai | none
  model: "qwen3.5:2b"
  endpoint: "http://localhost:11434"
```

Requires [Ollama](https://ollama.ai) running locally.

```bash
# Pull the default judge model
ollama pull qwen3.5:2b
```

Supported providers:

| Provider | Model | Privacy |
|---|---|---|
| **Ollama** (default) | `qwen3.5:2b`, `gemma4:e2b`, any | 🟢 100% local |
| **Claude** | `claude-haiku-4-5` | 🟡 Cloud API |
| **OpenAI** | `gpt-4o-mini` | 🟡 Cloud API |

---

## 📺 Live Watch Mode

```bash
agentscope watch
```

Opens a **live TUI dashboard** that refreshes every 500ms. See exactly what your agent is touching in real time — before you commit anything.

The dashboard shows:
- Session info (ID, agent, mission)
- Every changed file with its verdict (IN SCOPE / UNASKED / BLOCKED)
- Live diff stats (+lines / −lines)
- Summary counts

Press `q` or `Ctrl+C` to exit.

---

## 🏗️ CI / CD Integration

```bash
# In your CI pipeline:
agentscope check --json | jq '.blocked'

# Or just use the exit code:
agentscope check || echo "Agent touched blocked files — review required"
```

```yaml
# GitHub Actions example
- name: Audit agent changes
  run: |
    agentscope check --json > report.json
    if [ $? -ne 0 ]; then
      echo "::error::AgentScope detected blocked file modifications"
      cat report.json | jq '.files[] | select(.verdict == "BLOCKED")'
      exit 1
    fi
```

---

## 🧩 How It Works

```
┌──────────────────────────────────────────────────────┐
│  agentscope start "Fix the bug in middleware.ts"     │
│  ↓                                                    │
│  Records: mission, git baseline (HEAD SHA), agent    │
│  Writes: .agentscope/session.json                    │
└──────────────────────────────────────────────────────┘
          │
          ▼  (you run your AI agent normally)
          │
┌──────────────────────────────────────────────────────┐
│  agentscope check                                     │
│  ↓                                                    │
│  1. Git diff: working tree vs baseline                │
│  2. Policy engine: glob match → BLOCKED / WARN        │
│  3. Scope hints: mission text → file path matching    │
│  4. LLM judge: "do changes match the mission?"        │
│  ↓                                                    │
│  Output: IN SCOPE / UNASKED / BLOCKED per file        │
│  Exit code: 0 (clean) or 1 (blocked files found)      │
└──────────────────────────────────────────────────────┘
```

---

## 💡 Philosophy

| Principle | What it means |
|---|---|
| **Deterministic first** | Blocked paths use glob matching, not AI. No false negatives on `.env`. |
| **Agent-agnostic** | Works with any agent that writes files. Not tied to any vendor. |
| **Privacy-first** | LLM judge runs locally via Ollama by default. Nothing leaves your machine. |
| **Git-native** | Diffs against your git baseline. No proprietary snapshots. |
| **Zero config** | `agentscope init` + `agentscope start` — that's all you need. |

---

## 🛠️ Development

```bash
# Clone
git clone https://github.com/abdouloued/agentscopev2
cd agentscopev2

# Build
cargo build

# Run tests
cargo test

# Run with debug logging
RUST_LOG=debug cargo run -- check

# Release build (optimized + stripped)
cargo build --release
```

---

## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

MIT — see [LICENSE](LICENSE)

---

<div align="center">

**Built with 🦀 Rust**

*AgentScope doesn't replace your AI agent. It makes sure your AI agent only does what you asked.*

</div>
