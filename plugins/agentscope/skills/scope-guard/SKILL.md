---
description: Use AgentScope to keep Claude Code work inside the active mission. Trigger before starting repository edits, while checking changed files, before handoff, and before committing.
---

# AgentScope Scope Guard

This repository uses AgentScope as a scope firewall for AI coding sessions.

Before starting work in a Git repository, run:

```bash
agentscope status
```

If there is no active session, ask for the mission or start one:

```bash
agentscope start "your mission"
```

While working, use the live cockpit when useful:

```bash
agentscope watch
```

Before handoff, run:

```bash
agentscope check
```

Before committing, review problem files only:

```bash
agentscope diff --problems
```

Blocked paths are hard stops. Suspicious or unasked files need an explicit reason before they are included.
