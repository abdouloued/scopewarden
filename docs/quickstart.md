# Quickstart

Use this when you want ScopeWarden running in a real coding session.

## Manual mission

```bash
scopewarden init
scopewarden start "Fix checkout button loading state" --agent codex
```

Run your coding agent normally.

```bash
scopewarden watch
scopewarden check
```

## Agent-aware mission

```bash
scopewarden init
scopewarden agents doctor
scopewarden agents detect
scopewarden attach --agent auto
```

If the dry run looks right:

```bash
scopewarden attach --agent auto --apply
scopewarden monitor --agent auto
```

## Before commit

```bash
scopewarden diff --problems
scopewarden check
```

Optional:

```bash
scopewarden judge -m qwen3.5:2b
scopewarden report --markdown
```

## If something is missing

```bash
scopewarden agents doctor
```

Then either update `scopewarden.yaml` with a custom source path, or skip detection for this session:

```bash
scopewarden start "the exact mission" --agent codex
```
