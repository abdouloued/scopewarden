# AgentScope TUI Redesign Plan

## Purpose

Redesign AgentScope’s terminal UI from a debug-style file watcher into a polished coding-agent cockpit.

AgentScope already has the important product features:
- agent missions
- Codex / Claude tracking
- scope classification
- in-scope vs unasked file detection
- judge provider support
- dashboard/stat views
- chat area
- theme switching

Do not rebuild the product. Focus on design, visual hierarchy, interaction model, theme tokens, and chat usability.

---

## Product Positioning

AgentScope is not “another coding agent.”

AgentScope is:

> A mission-control cockpit for AI coding agents that shows which code changes were expected, suspicious, or blocked.

The UI must make this sentence obvious.

Every screen should help the user answer:

1. What changed?
2. Was it expected?
3. Why did AgentScope classify it that way?
4. What action should I take?
5. Which agent caused it?

---

## Current Problems Seen in Screenshots

### Problem 1: Too much raw internal state

The current screen exposes many low-level items at the same visual weight:
- `IN SCOPE`
- `UNASKED`
- `UNMATCHED`
- file paths
- agents
- diff counts
- mission text
- chat text
- charts
- judge status
- footer commands

This makes the UI feel like a debug console instead of a polished product.

### Problem 2: No clear default workflow

The user is not sure whether they are:
- reviewing files
- chatting
- watching agents
- looking at dashboard stats
- running a judge
- managing missions

The default screen should be focused on review decisions.

### Problem 3: Chat feels like a log panel

The current `Scope Chat` area looks like metadata/log output. It does not feel like a conversation or command surface.

Chat should be its own full mode, not a small bottom section.

### Problem 4: Dashboard charts compete with the review task

The charts are useful, but they should not compete with the main file-review workflow.

Charts belong in Dashboard mode only.

### Problem 5: Keyboard shortcuts are ambiguous

Current shortcuts like `d=dash` conflict mentally with `diff`.

Use numbered modes and `enter` for diff to avoid ambiguity.

---

## Design Principles

### 1. Review-first

The default screen should be `Review`.

The user should immediately see:
- mission summary
- file changes
- classification
- selected file details
- recommended actions

### 2. Use semantic status names

Replace current internal labels with productized labels.

| Old Label | New Label |
|---|---|
| `IN SCOPE` | `EXPECTED` |
| `UNASKED` | `SUSPICIOUS` |
| `BLOCKED` | `BLOCKED` |
| `STALE` / ignored items | `IGNORED` |
| `UNMATCHED` | reason text only, not primary badge |

Primary user-facing statuses:

```text
EXPECTED
SUSPICIOUS
BLOCKED
IGNORED
```

Technical reasons can still appear in detail panels:

```text
Reason: unmatched by active mission
Policy: no matching allow rule
Agent: Codex
```

### 3. Color only the badge, not the whole row

Do not make the full file path bright green/yellow/red.

Bad:

```text
UNASKED agentscope/src/git.rs +205 -5 UNMATCHED
```

Better:

```text
SUSPICIOUS  agentscope/src/git.rs     +205 -5   CODEX
```

Only `SUSPICIOUS` should be amber. The path should remain neutral.

### 4. Reduce visual noise

Most text should be muted or neutral.

Use strong color only for:
- focused panel border
- selected row
- status badge
- destructive action
- critical warning

### 5. Every panel needs a job

Do not show a panel unless it has a clear purpose.

Default Review screen panels:
- Header: mission and state summary
- Left: change list
- Right: decision/details panel
- Bottom: command bar

Dashboard screen panels:
- scope distribution
- agent health
- judge status
- mission stats

Chat screen panels:
- message history
- input composer
- context sidebar or compact mission header

---

## New App Modes

Implement four main app modes.

```text
1 Review
2 Chat
3 Dashboard
4 Sessions
```

Mode switching:
- `1` opens Review
- `2` opens Chat
- `3` opens Dashboard
- `4` opens Sessions
- `tab` switches focus within the current mode
- `shift+tab` switches focus backward

The footer should always show the active mode.

---

# Mode 1: Review

## Goal

Review mode is the default and most important screen.

It answers:

> Which AI changes should I accept, inspect, or revert?

## Layout

```text
┌ agentscope ───────────────────────────────────────────────────────────────┐
│ Mission: Create CLAUDE.md for future Claude Code instances                │
│ 11 expected   22 suspicious   0 blocked   2 agents   judge: ollama/gemma  │
├─────────────────────────────────────────────┬─────────────────────────────┤
│ Changes                                     │ Decision                    │
│                                             │                             │
│ › EXPECTED    CLAUDE.md             +0 -0   │ CLAUDE.md                   │
│   EXPECTED    agentscope.html       +0 -0   │ Verdict: Expected           │
│   EXPECTED    agentscope.yaml       +3 -30  │ Agent: Claude               │
│   SUSPICIOUS  src/git.rs            +205 -5 │ Policy: matched mission     │
│   SUSPICIOUS  src/judge.rs          +82 -0  │                             │
│   SUSPICIOUS  src/main.rs           +0 -0   │ Why                         │
│                                             │ This file was part of the   │
│                                             │ requested documentation     │
│                                             │ mission.                    │
│                                             │                             │
│                                             │ Actions                     │
│                                             │ [enter] diff  [a] allow     │
│                                             │ [r] revert   [b] block      │
├─────────────────────────────────────────────┴─────────────────────────────┤
│ / search   tab focus   1 review   2 chat   3 dash   4 sessions   ? help   │
└────────────────────────────────────────────────────────────────────────────┘
```

## Header Requirements

The header should show:
- product name
- active mode
- mission title
- active agents count
- status summary
- current theme
- judge provider/model

Example:

```text
agentscope  review  ·  2 active  ·  11 expected  22 suspicious  0 blocked  ·  theme agentScope
```

Use muted separators.

## Change List Requirements

Each file row should show:

```text
STATUS_BADGE  path  +added -removed  AGENT_BADGE
```

Example:

```text
EXPECTED    CLAUDE.md                         +0 -0    CLAUDE
SUSPICIOUS  agentscope/src/git.rs             +205 -5  CODEX
BLOCKED     src/auth/session.ts               +12 -2   CLAUDE
IGNORED     target/debug/.fingerprint         +0 -0    SYSTEM
```

### Row Styling

Selected row:
- use `selection_bg`
- status badge keeps its semantic color
- file path becomes high contrast

Unselected rows:
- file path neutral
- diff counts muted except large changes may be highlighted subtly

Status badge colors:
- EXPECTED = green
- SUSPICIOUS = amber
- BLOCKED = red
- IGNORED = gray

### Sorting

Default sort order:
1. BLOCKED
2. SUSPICIOUS
3. EXPECTED
4. IGNORED

Within each group:
- larger diffs first
- then alphabetical path

Allow future filter support:

```text
/show suspicious
/show blocked
/show expected
```

## Decision Panel Requirements

The right panel should show the selected file and explain the classification.

Fields:
- File
- Verdict
- Agent
- Diff stat
- Policy result
- Matched rule
- Reason
- Recommended action
- Available commands

Example:

```text
File
agentscope/src/git.rs

Verdict
SUSPICIOUS

Agent
CODEX

Diff
+205 -5

Policy
No matching allow rule

Reason
The active mission asks for CLAUDE.md documentation, but this file changes git detection logic.

Recommendation
Review diff before allowing. Consider reverting if unrelated.

Actions
[enter] diff
[a] allow once
[b] block pattern
[r] revert file
```

## Review Mode Keyboard Shortcuts

```text
j / down       move selection down
k / up         move selection up
enter          open diff for selected file
a              allow selected file once
A              allow selected file pattern
r              revert selected file
b              block selected file
B              block selected file pattern
/              search/filter files
tab            focus next panel
shift+tab      focus previous panel
1              review mode
2              chat mode
3              dashboard mode
4              sessions mode
?              help
q              quit
```

Do not use `d` for dashboard because users associate `d` with diff. Use `enter` for diff and `3` for dashboard.

---

# Mode 2: Chat

## Goal

Chat is a mission-aware decision assistant, not a general chatbot.

It helps the user ask:
- Why did this file change?
- Should I allow this?
- Ask Claude to justify the change.
- Ask Codex to review suspicious files.
- Generate a mission report.
- Create a policy rule.

## Layout

```text
┌ agentscope chat ──────────────────────────────────────────────────────────┐
│ Mission: Create CLAUDE.md for future Claude Code instances                │
│ Context: selected file agentscope/src/git.rs · verdict suspicious          │
├────────────────────────────────────────────────────────────────────────────┤
│ You                                                                        │
│ Why is src/git.rs suspicious?                                              │
│                                                                            │
│ AgentScope                                                                 │
│ src/git.rs is suspicious because it was modified by Codex but not matched  │
│ by the active mission. It changed git detection logic with +205 -5 lines.  │
│                                                                            │
│ Codex                                                                      │
│ I modified src/git.rs to improve untracked-file detection for the report.  │
│                                                                            │
├────────────────────────────────────────────────────────────────────────────┤
│ > ask selected agent...                                                    │
└────────────────────────────────────────────────────────────────────────────┘
```

## Chat Sender Types

Support these sender labels:

```text
YOU
AGENTSCOPE
JUDGE
CLAUDE
CODEX
SYSTEM
```

Sender styling:
- YOU = text/accent
- AGENTSCOPE = accent
- JUDGE = violet/cyan depending theme
- CLAUDE = `agent_claude`
- CODEX = `agent_codex`
- SYSTEM = muted

## Chat Commands

Support slash commands:

```text
/explain selected
/explain <path>
/ask claude <question>
/ask codex <question>
/ask judge <question>
/report
/allow selected
/block selected
/revert selected
/filter suspicious
```

Example:

```text
> /explain selected
```

Expected response:

```text
AgentScope:
The selected file is suspicious because it was changed by Codex, but the active mission only requested documentation output. The file changes source logic, so it requires manual review.
```

## Chat Keyboard Shortcuts

```text
i              focus composer
enter          send message when composer focused
esc            leave composer
up/down        scroll messages
ctrl+u         clear composer
@claude        target Claude
@codex         target Codex
@judge         target local judge
@scope         target AgentScope
1              review
2              chat
3              dashboard
4              sessions
?              help
```

## Chat Acceptance Criteria

- Chat mode is full screen or near full screen.
- Chat is not hidden in a small bottom panel.
- Composer must be visually obvious.
- Selected file context should appear near the header.
- User should be able to ask about the selected file without typing the full path.
- Chat history should persist for the current session.
- `i` should enter compose mode reliably.
- `esc` should exit compose mode without leaving Chat mode.
- `2` should always switch back to Chat mode.

---

# Mode 3: Dashboard

## Goal

Dashboard shows mission health and aggregate stats. It is not the default screen.

Use it for:
- scope distribution
- agent contribution
- suspicious ratio
- blocked file count
- judge provider health
- mission timing
- active sessions

## Layout

```text
┌ agentscope dashboard ─────────────────────────────────────────────────────┐
│ Mission health                                                             │
│                                                                            │
│ Scope distribution                                                         │
│ ███████████ expected 33%   ███████████████████ suspicious 66%   blocked 0  │
│                                                                            │
│ Agents                                                                     │
│ Claude   85%   active 3h    6 expected   9 suspicious                      │
│ Codex    85%   active 26m   5 expected   13 suspicious                     │
│                                                                            │
│ Judge                                                                      │
│ Provider: ollama / gemma4:2b   last run: 59s   health: degraded 33%        │
└────────────────────────────────────────────────────────────────────────────┘
```

## Dashboard Requirements

Show:
- total files changed
- expected count
- suspicious count
- blocked count
- ignored count
- percentage distribution
- total added/removed lines
- per-agent breakdown
- judge provider/model
- judge health
- last judge run time
- active watch duration

Do not show full file details in Dashboard. Keep that in Review.

## Dashboard Keyboard Shortcuts

```text
r              run judge
R              refresh stats
1              review
2              chat
3              dashboard
4              sessions
?              help
```

---

# Mode 4: Sessions

## Goal

Sessions mode shows current and previous agent missions.

It should answer:
- Which agents are active?
- What are they doing?
- How long have they been running?
- Which mission owns which changes?
- Which sessions are stale?

## Layout

```text
┌ agentscope sessions ──────────────────────────────────────────────────────┐
│ Active missions                                                            │
│                                                                            │
│ CLAUDE   85%   3h    Create CLAUDE.md documenting CLI and policy engine    │
│ CODEX    85%   26m   PLEASE IMPLEMENT THIS PLAN                            │
│                                                                            │
│ Stale / ignored                                                             │
│ 4 stale sessions ignored                                                    │
└────────────────────────────────────────────────────────────────────────────┘
```

## Sessions Requirements

Show:
- agent name
- confidence/progress if available
- duration
- mission title
- active/stale/ignored status
- associated files count
- quick action to inspect mission

Keyboard:
```text
enter          inspect session
c              close/stale selected session
n              new mission
1              review
2              chat
3              dashboard
4              sessions
?              help
```

---

# Theme System

## Goal

Create a reusable semantic theme system.

No widget should hardcode color values directly. Widgets should request semantic theme tokens.

## Theme Struct

Implement a theme struct similar to this:

```rust
pub struct Theme {
    pub name: &'static str,

    pub bg: Color,
    pub panel: Color,
    pub border: Color,
    pub border_focused: Color,

    pub text: Color,
    pub text_muted: Color,
    pub text_subtle: Color,

    pub accent: Color,

    pub expected: Color,
    pub suspicious: Color,
    pub blocked: Color,
    pub ignored: Color,

    pub agent_claude: Color,
    pub agent_codex: Color,
    pub agent_scope: Color,
    pub agent_system: Color,

    pub selection_bg: Color,
    pub selection_fg: Color,

    pub diff_add: Color,
    pub diff_remove: Color,

    pub warning: Color,
    pub success: Color,
    pub danger: Color,
}
```

## Theme Registry

Support these themes:

```text
agentscope
codex
claude
openclaw
high-contrast
```

Commands:
```text
/theme
/theme agentscope
/theme codex
/theme claude
/theme openclaw
/theme high-contrast
```

Keyboard:
```text
t       cycle theme
T       open theme picker
```

Persist selected theme in config.

---

# Theme 1: AgentScope

## Mood

AI security cockpit. Dark, precise, calm, premium.

## Palette

```text
bg              #080B12
panel           #0D111A
border          #1E2633
border_focused  #7C5CFF

text            #E6EDF7
text_muted      #8B95A7
text_subtle     #5E6678

accent          #7C5CFF

expected        #4ADE80
suspicious      #FBBF24
blocked         #F87171
ignored         #64748B

agent_claude    #D8A7FF
agent_codex     #67E8F9
agent_scope     #7C5CFF
agent_system    #8B95A7

selection_bg    #151B2B
selection_fg    #FFFFFF

diff_add        #4ADE80
diff_remove     #F87171

warning         #FBBF24
success         #4ADE80
danger          #F87171
```

## Usage Rules

- Brand name uses accent.
- Focused border uses accent.
- Expected/Suspicious/Blocked only color the badge.
- Avoid using purple on every label.
- Keep file paths mostly neutral.

---

# Theme 2: Codex

## Mood

Minimal, dark, terminal-native, low-noise, cyan accent.

## Palette

```text
bg              #09090B
panel           #0F1117
border          #242833
border_focused  #67E8F9

text            #E5E7EB
text_muted      #8B949E
text_subtle     #5B6472

accent          #67E8F9

expected        #22C55E
suspicious      #EAB308
blocked         #EF4444
ignored         #6B7280

agent_claude    #A78BFA
agent_codex     #67E8F9
agent_scope     #67E8F9
agent_system    #8B949E

selection_bg    #161B22
selection_fg    #F8FAFC

diff_add        #22C55E
diff_remove     #EF4444

warning         #EAB308
success         #22C55E
danger          #EF4444
```

## Usage Rules

- This theme should feel restrained.
- Avoid bright purple except for Claude agent badge.
- Prefer muted gray for metadata.
- Cyan should be used for current focus and the AgentScope brand.

---

# Theme 3: Claude

## Mood

Warm, calm, parchment-dark, amber, less neon.

## Palette

```text
bg              #11100D
panel           #181612
border          #2A251D
border_focused  #D97706

text            #F4EFE7
text_muted      #A8A29E
text_subtle     #78716C

accent          #D97706

expected        #84CC16
suspicious      #F59E0B
blocked         #EF4444
ignored         #78716C

agent_claude    #D97706
agent_codex     #38BDF8
agent_scope     #D97706
agent_system    #A8A29E

selection_bg    #241F18
selection_fg    #FFF7ED

diff_add        #84CC16
diff_remove     #EF4444

warning         #F59E0B
success         #84CC16
danger          #EF4444
```

## Usage Rules

- Warm amber is the accent.
- Use soft borders.
- Avoid neon cyan except for Codex agent badge.
- This should feel like a calm review environment.

---

# Theme 4: OpenClaw

## Mood

Open-source hacker cockpit. Electric green, black, energetic.

## Palette

```text
bg              #050807
panel           #07110D
border          #123126
border_focused  #00FF99

text            #D8FFE9
text_muted      #7DAE95
text_subtle     #4B705F

accent          #00FF99

expected        #00FF99
suspicious      #FFD166
blocked         #FF4D6D
ignored         #5C677D

agent_claude    #C084FC
agent_codex     #00E5FF
agent_scope     #00FF99
agent_system    #7DAE95

selection_bg    #0B1F17
selection_fg    #FFFFFF

diff_add        #00FF99
diff_remove     #FF4D6D

warning         #FFD166
success         #00FF99
danger          #FF4D6D
```

## Usage Rules

- This can be bold and viral.
- Do not make all text green.
- Use electric green for focus and brand.
- File paths should still be readable neutral text.

---

# Theme 5: High Contrast

## Mood

Accessible, screen-recording friendly, maximum readability.

## Palette

```text
bg              #000000
panel           #0A0A0A
border          #666666
border_focused  #FFFFFF

text            #FFFFFF
text_muted      #CFCFCF
text_subtle     #A3A3A3

accent          #00D9FF

expected        #00FF66
suspicious      #FFD400
blocked         #FF3355
ignored         #B0B0B0

agent_claude    #FFB86C
agent_codex     #00D9FF
agent_scope     #00D9FF
agent_system    #CFCFCF

selection_bg    #222222
selection_fg    #FFFFFF

diff_add        #00FF66
diff_remove     #FF3355

warning         #FFD400
success         #00FF66
danger          #FF3355
```

## Usage Rules

- Avoid subtle contrast.
- Borders and selected rows should be obvious.
- This theme should be best for demos and recording.

---

# Footer / Command Bar

## Goal

The footer must be compact, consistent, and mode-aware.

## Review Footer

```text
1 review  2 chat  3 dash  4 sessions  ·  enter diff  a allow  r revert  b block  / search  ? help
```

## Chat Footer

```text
1 review  2 chat  3 dash  4 sessions  ·  i compose  enter send  esc cancel  @agent target  ? help
```

## Dashboard Footer

```text
1 review  2 chat  3 dash  4 sessions  ·  r run judge  R refresh  ? help
```

## Sessions Footer

```text
1 review  2 chat  3 dash  4 sessions  ·  enter inspect  n new  c close  ? help
```

Rules:
- Do not show every shortcut in every mode.
- Keep the footer to one line.
- Muted shortcuts, brighter active mode.

---

# Help Overlay

Pressing `?` should open a centered help overlay.

```text
┌ Help ───────────────────────────────┐
│ Modes                               │
│ 1 Review                            │
│ 2 Chat                              │
│ 3 Dashboard                         │
│ 4 Sessions                          │
│                                     │
│ Review actions                      │
│ enter Open diff                     │
│ a     Allow selected file           │
│ r     Revert selected file          │
│ b     Block selected file           │
│ /     Search                        │
│                                     │
│ Global                              │
│ t     Cycle theme                   │
│ q     Quit                          │
└─────────────────────────────────────┘
```

---

# Empty States

Create polished empty states instead of raw blank areas.

## No changes

```text
No changes yet

AgentScope is watching this repo.
Start Codex, Claude Code, or another agent to see scoped changes here.
```

## No chat messages

```text
No chat messages yet

Press i to ask AgentScope about the selected file.
Try: /explain selected
```

## No mission

```text
No active mission

Create a mission so AgentScope can decide which changes are expected.
Press n to create a mission.
```

## Judge unavailable

```text
Judge unavailable

Ollama is not responding or the configured model is missing.
AgentScope will continue using rule-based scope detection.
```

---

# Diff View

## Goal

Diff view should make review decisions fast.

Open with:
```text
enter
```

## Layout

```text
┌ diff: agentscope/src/git.rs ──────────────────────────────────────────────┐
│ Verdict: SUSPICIOUS · Agent: CODEX · +205 -5                              │
├────────────────────────────────────────────────────────────────────────────┤
│ @@ function detect_changes @@                                              │
│ + added line                                                               │
│ - removed line                                                             │
│   unchanged context                                                        │
├────────────────────────────────────────────────────────────────────────────┤
│ a allow   r revert   b block   esc back                                    │
└────────────────────────────────────────────────────────────────────────────┘
```

Requirements:
- show verdict in header
- show agent
- show diff stat
- use `diff_add` and `diff_remove`
- allow/revert/block from diff view
- `esc` returns to previous mode

---

# Policy / Mission Language

Use user-facing language in the UI.

Instead of:

```text
matched active mission
unmatched
```

Use:

```text
Matched mission rule
No mission rule matched
Denied by policy
Ignored by runtime rule
```

Example detail panel:

```text
Policy
No mission rule matched

Why
The current mission asks for documentation, but this file changes source logic.
```

---

# Agent Badges

Use compact agent badges.

```text
CLAUDE
CODEX
SCOPE
JUDGE
SYSTEM
```

Rules:
- agent badges appear at end of file rows
- do not color the whole row based on agent
- Claude badge uses `agent_claude`
- Codex badge uses `agent_codex`
- AgentScope internal messages use `agent_scope`

---

# Implementation Phases

## Phase 1: Theme Token Refactor

Goal:
- Extract all colors/styles into `Theme`.
- Remove hardcoded colors from widgets.
- Add theme registry.
- Implement `/theme <name>`.
- Implement `t` to cycle theme.
- Persist selected theme.

Tasks:
1. Create `src/theme.rs`.
2. Define `Theme` struct.
3. Define themes:
   - agentscope
   - codex
   - claude
   - openclaw
   - high-contrast
4. Replace direct color usage in TUI widgets with semantic theme tokens.
5. Add tests for theme lookup and fallback.
6. Add config field:
   ```yaml
   theme: agentscope
   ```

Acceptance criteria:
- App renders with all five themes.
- No widget hardcodes product colors directly.
- `t` cycles themes.
- `/theme codex` switches to codex theme.
- Unknown theme falls back to `agentscope`.

---

## Phase 2: App Modes

Goal:
- Create mode architecture: Review, Chat, Dashboard, Sessions.
- Review is default.
- Number keys switch modes.

Tasks:
1. Add enum:
   ```rust
   enum AppMode {
       Review,
       Chat,
       Dashboard,
       Sessions,
   }
   ```
2. Add mode switching:
   - `1` Review
   - `2` Chat
   - `3` Dashboard
   - `4` Sessions
3. Move current chart/stat UI into Dashboard mode.
4. Move current chat UI into Chat mode.
5. Keep file review in Review mode.

Acceptance criteria:
- Default launch opens Review.
- `2` opens Chat.
- `3` opens Dashboard.
- `4` opens Sessions.
- Dashboard charts no longer appear in Review mode.
- Chat panel no longer appears as a small bottom area in Review mode.

---

## Phase 3: Review Screen Redesign

Goal:
- Make Review screen clean and decision-focused.

Tasks:
1. Build top mission/status header.
2. Build left change list.
3. Build right decision panel.
4. Build mode-aware footer.
5. Rename visible statuses:
   - `IN SCOPE` -> `EXPECTED`
   - `UNASKED` -> `SUSPICIOUS`
   - keep `BLOCKED`
   - ignored/stale -> `IGNORED`
6. Sort files by severity:
   - BLOCKED
   - SUSPICIOUS
   - EXPECTED
   - IGNORED
7. Add selected-row styling.
8. Add status badge styling.

Acceptance criteria:
- User can understand the selected file verdict in under 3 seconds.
- File list is not visually noisy.
- Right panel explains why the file got its verdict.
- Footer matches Review mode.

---

## Phase 4: Chat Redesign

Goal:
- Make chat a proper mission-aware assistant mode.

Tasks:
1. Create full Chat screen.
2. Add message history.
3. Add input composer.
4. Support sender labels:
   - YOU
   - AGENTSCOPE
   - JUDGE
   - CLAUDE
   - CODEX
   - SYSTEM
5. Support commands:
   - `/explain selected`
   - `/explain <path>`
   - `/ask claude <question>`
   - `/ask codex <question>`
   - `/ask judge <question>`
   - `/report`
6. Make `i` enter compose mode.
7. Make `enter` send message when composer is focused.
8. Make `esc` leave composer.

Acceptance criteria:
- `2` opens chat mode.
- `i` reliably focuses composer.
- Chat displays message blocks with sender labels.
- `/explain selected` works.
- Chat history persists during the app session.
- Chat footer shows chat-specific controls.

---

## Phase 5: Dashboard Redesign

Goal:
- Make dashboard useful but secondary.

Tasks:
1. Move scope distribution into Dashboard.
2. Move verdict chart into Dashboard.
3. Add agent breakdown.
4. Add judge health summary.
5. Add mission timing.
6. Add refresh/run judge actions.

Acceptance criteria:
- Dashboard has no file decision controls.
- Dashboard summarizes the mission.
- `r` runs judge.
- `R` refreshes stats.
- Dashboard footer is mode-specific.

---

## Phase 6: Sessions Screen

Goal:
- Show active/stale/ignored agent sessions clearly.

Tasks:
1. Create Sessions mode.
2. Show active missions.
3. Show stale/ignored sessions.
4. Show agent, confidence/progress, duration, mission text.
5. Add inspect action.

Acceptance criteria:
- `4` opens sessions.
- Active Claude/Codex missions are visible.
- Stale/ignored sessions are separated from active sessions.
- `enter` can inspect selected session.

---

## Phase 7: Polish Pass

Goal:
- Make the UI feel premium.

Tasks:
1. Reduce border noise.
2. Ensure consistent padding.
3. Ensure focused panel is obvious.
4. Ensure empty states are helpful.
5. Ensure all modes have consistent header/footer.
6. Ensure all themes are readable.
7. Check narrow terminal behavior.
8. Check wide terminal behavior.
9. Check small laptop screen behavior.
10. Add screenshots/gifs for README.

Acceptance criteria:
- App looks polished at 120x40.
- App remains usable at 100x30.
- No panel looks accidentally empty.
- Colors are consistent across modes.
- Theme switch is visible immediately.
- README screenshots show:
  - Review mode
  - Chat mode
  - Dashboard mode
  - Theme examples

---

# Non-Goals

Do not change:
- core policy engine
- judge logic
- git detection logic
- agent adapter behavior
- underlying mission matching rules

Do not add:
- new cloud service
- new database
- paid dashboard
- complex animations
- new agent protocol

This redesign is about:
- UI structure
- themes
- status language
- chat usability
- keyboard consistency
- polish

---

# Testing Plan

## Manual Test Matrix

Test each theme:
- agentscope
- codex
- claude
- openclaw
- high-contrast

Test each mode:
- Review
- Chat
- Dashboard
- Sessions

Test terminal sizes:
- 100x30
- 120x40
- 160x50

Test file states:
- no changes
- expected only
- suspicious only
- blocked files
- ignored files
- mixed states

Test agents:
- Claude only
- Codex only
- Claude + Codex
- no active agent

Test judge:
- judge available
- judge unavailable
- judge degraded
- judge not configured

## Regression Checks

Commands to run:

```bash
cargo fmt -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo run -- watch
```

## UX Acceptance Test

A new user should be able to answer these in under 10 seconds:

1. What mission is active?
2. How many files are suspicious?
3. Which file is selected?
4. Why is it suspicious or expected?
5. Which key opens the diff?
6. Which key opens chat?
7. Which key switches theme?

---

# Final Design Sentence

AgentScope should feel like:

> Claude Code polish + Codex speed + a security cockpit for AI code changes.

The default experience should be:

> AI agents can write code, but AgentScope decides whether the diff deserves to survive.
