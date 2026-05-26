---
name: scope-guard
description: Monitor and enforce AgentScope mission scope. Use before starting repository edits, while checking changed files, before handoff, and before committing. Triggers on phrases like "check scope", "am I in scope", "scope check", or "check my changes".
tools: Bash
user-invocable: true
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
