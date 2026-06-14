---
name: scope-check
description: Run a scope policy check and report whether any changed files are BLOCKED or SUSPICIOUS. Use before committing, when asked to verify scope compliance, or when the user asks "is this in scope?".
tools: Bash
user-invocable: true
---

# Scope Check

Run `scopewarden check` to audit all current Git changes against the active mission policy.

```bash
scopewarden check              # policy check + scope audit
scopewarden check --json       # machine-readable output
scopewarden diff --problems    # show only suspicious/blocked files
```

## Interpreting results

- **EXPECTED** — file matches the active mission. Safe to commit.
- **SUSPICIOUS** — changed but no mission rule matches. Needs review.
- **BLOCKED** — matched a hard-stop policy path. **Do not commit. Report to user immediately.**
- **IGNORED** — clean or stale, no action needed.

## If files are BLOCKED

Stop. Do not include them in the commit. Tell the user:
> "The file `<path>` is BLOCKED by ScopeWarden policy. It matches a blocked pattern in `scopewarden.yaml`. This is a hard stop — the file must be removed from the change set."

## If files are SUSPICIOUS

Run the judge and report back:
```bash
scopewarden judge
```

Then tell the user what the judge said before proceeding.
