---
name: scope-guard
description: Monitor and enforce ScopeWarden mission scope. Use before starting repository edits, while checking changed files, before handoff, and before committing. Triggers on phrases like "check scope", "am I in scope", "scope check", or "check my changes".
tools: Bash
user-invocable: true
---

# ScopeWarden Scope Guard

This repository uses ScopeWarden as a scope firewall for AI coding sessions.

Before starting work in a Git repository, run:

```bash
scopewarden status
```

If there is no active session, ask for the mission or start one:

```bash
scopewarden start "your mission"
```

While working, use the live cockpit when useful:

```bash
scopewarden watch
```

Before handoff, run:

```bash
scopewarden check
```

Before committing, review problem files only:

```bash
scopewarden diff --problems
```

Blocked paths are hard stops. Suspicious or unasked files need an explicit reason before they are included.

This repository uses ScopeWarden as a scope firewall for AI coding sessions.

Before starting work in a Git repository, run:

```bash
scopewarden status
```

If there is no active session, ask for the mission or start one:

```bash
scopewarden start "your mission"
```

While working, use the live cockpit when useful:

```bash
scopewarden watch
```

Before handoff, run:

```bash
scopewarden check
```

Before committing, review problem files only:

```bash
scopewarden diff --problems
```

Blocked paths are hard stops. Suspicious or unasked files need an explicit reason before they are included.
