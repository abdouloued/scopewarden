# AgentScope TUI — Complete Visual Specification
# Agent-readable: all colors as Rgb(r,g,b), all layout as exact constraints.
# Source of truth for tui.rs. Every value here is extracted from the live screenshots.

# ═══════════════════════════════════════════════════════════════════════════════
# SECTION 1 — COLOUR PALETTE (copy these constants verbatim into tui.rs)
# ═══════════════════════════════════════════════════════════════════════════════

# Background layers
CLR_BG          = Color::Rgb(13,  15,  20)   # #0d0f14  — outermost terminal bg (near black)
CLR_SURFACE     = Color::Rgb(20,  23,  30)   # #14171e  — panel / card surface
CLR_SURFACE2    = Color::Rgb(26,  29,  38)   # #1a1d26  — slightly lighter surface (selection bg)
CLR_BORDER      = Color::Rgb(38,  42,  55)   # #262a37  — all panel borders

# Text hierarchy
CLR_TEXT_PRI    = Color::Rgb(220, 225, 235)  # #dce1eb  — primary readable text (mission, file paths)
CLR_TEXT_SEC    = Color::Rgb(140, 148, 164)  # #8c94a4  — secondary (labels: "file", "verdict")
CLR_TEXT_DIM    = Color::Rgb(72,  78,  95)   # #484e5f  — dim / decorative (dividers, timestamps)
CLR_TEXT_MUTED  = Color::Rgb(100, 108, 126)  # #646c7e  — hint text, inactive items

# Accent colours (verdicts)
CLR_GREEN       = Color::Rgb(80,  200, 120)  # #50c878  — IN SCOPE tag + bar fill
CLR_GREEN_DIM   = Color::Rgb(30,  80,  50)   # #1e5032  — IN SCOPE tag background
CLR_AMBER       = Color::Rgb(230, 160, 50)   # #e6a032  — UNASKED tag + bar fill
CLR_AMBER_DIM   = Color::Rgb(80,  50,  10)   # #50320a  — UNASKED tag background
CLR_RED         = Color::Rgb(220, 80,  80)   # #dc5050  — BLOCKED tag + BLOCK banner
CLR_RED_DIM     = Color::Rgb(80,  20,  20)   # #501414  — BLOCKED tag background

# UI accent colours
CLR_PURPLE      = Color::Rgb(160, 120, 255)  # #a078ff  — "agentscope" wordmark, headings
CLR_CYAN        = Color::Rgb(80,  210, 220)  # #50d2dc  — session IDs, interactive values
CLR_BLUE        = Color::Rgb(90,  160, 230)  # #5aa0e6  — IN SCOPE file paths
CLR_TEAL        = Color::Rgb(60,  180, 160)  # #3cb4a0  — agent names (CLAUDE, CODEX)

# Stats panel
CLR_STAT_LINES_POS = Color::Rgb(80, 200, 120)   # #50c878 — "+2354" additions
CLR_STAT_LINES_NEG = Color::Rgb(220, 80, 80)    # #dc5050 — "-222" deletions
CLR_HEALTH_BAR  = Color::Rgb(230, 160, 50)      # #e6a032 — health/confidence bar fill

# Separator / judge section labels
CLR_JUDGE_LABEL = Color::Rgb(160, 120, 255)  # #a078ff  — "— Judge —" label
CLR_MISSION_LBL = Color::Rgb(72,  78,  95)   # #484e5f  — "— Mission —" label

# Bottom status bar
CLR_STATUSBAR_BG    = Color::Rgb(13, 15, 20)    # same as CLR_BG
CLR_STATUSBAR_KEY   = Color::Rgb(160, 120, 255) # #a078ff — keybind keys (/theme, xjudge)
CLR_STATUSBAR_VAL   = Color::Rgb(140, 148, 164) # #8c94a4 — keybind descriptions
CLR_LIVE_DOT        = Color::Rgb(80,  200, 120) # #50c878 — ● live indicator dot


# ═══════════════════════════════════════════════════════════════════════════════
# SECTION 2 — TYPOGRAPHY & TEXT STYLES
# ═══════════════════════════════════════════════════════════════════════════════
# Ratatui has no font loading — all "font" choices are via Style modifiers.
# The terminal font is set by the user's terminal emulator.
# Recommended terminal fonts for best look: JetBrains Mono, Zed Mono, Iosevka.

# Text style constants (use these Style::default() chains in code):

STYLE_WORDMARK      = Style::default().fg(CLR_PURPLE).add_modifier(Modifier::BOLD)
                      # "agentscope" in top-left header

STYLE_HEADER_KEY    = Style::default().fg(CLR_TEXT_DIM)
                      # "watch:aggregate", "theme agentscope" — dim header items

STYLE_HEADER_ACTIVE = Style::default().fg(CLR_CYAN).add_modifier(Modifier::BOLD)
                      # "2 active" — highlighted count

STYLE_HEADER_WARN   = Style::default().fg(CLR_AMBER)
                      # "4 stale/ignored"

STYLE_SECTION_HDR   = Style::default().fg(CLR_TEXT_PRI).add_modifier(Modifier::BOLD)
                      # "Agent Missions", "Scope Distribution", "Stats & Judge", "Verdicts"

STYLE_AGENT_NAME    = Style::default().fg(CLR_TEAL).add_modifier(Modifier::BOLD)
                      # "CLAUDE", "CODEX" in missions panel

STYLE_AGENT_PCT_HI  = Style::default().fg(CLR_GREEN).add_modifier(Modifier::BOLD)
                      # "85%" when >= 70

STYLE_AGENT_PCT_MID = Style::default().fg(CLR_AMBER).add_modifier(Modifier::BOLD)
                      # percentage 40-69

STYLE_AGENT_PCT_LO  = Style::default().fg(CLR_RED).add_modifier(Modifier::BOLD)
                      # percentage < 40

STYLE_AGENT_TIME    = Style::default().fg(CLR_TEXT_DIM)
                      # "1h", "3m" — age of session

STYLE_MISSION_TEXT  = Style::default().fg(CLR_TEXT_PRI)
                      # mission description body text

# File list row styles
STYLE_TAG_INSCOPE   = Style::default().fg(CLR_GREEN).add_modifier(Modifier::BOLD)
                      # "IN SCOPE" prefix (8 chars, space-padded)

STYLE_TAG_UNASKED   = Style::default().fg(CLR_AMBER).add_modifier(Modifier::BOLD)
                      # "UNASKED " prefix

STYLE_TAG_BLOCKED   = Style::default().fg(CLR_RED).add_modifier(Modifier::BOLD)
                      # "BLOCKED " prefix

STYLE_FILE_INSCOPE  = Style::default().fg(CLR_BLUE)
                      # file path when IN SCOPE

STYLE_FILE_UNASKED  = Style::default().fg(CLR_AMBER)
                      # file path when UNASKED

STYLE_FILE_BLOCKED  = Style::default().fg(CLR_RED)
                      # file path when BLOCKED

STYLE_DIFF_ADD      = Style::default().fg(CLR_GREEN)
                      # "+205" addition count

STYLE_DIFF_DEL      = Style::default().fg(CLR_RED)
                      # "−5" deletion count (note: use − U+2212, not ASCII minus)

STYLE_AGENT_BADGE   = Style::default().fg(CLR_TEAL)
                      # "CLAUDE" / "UNMATCHED" agent badge at end of file row

STYLE_CURSOR_ROW    = Style::default().bg(CLR_SURFACE2)
                      # ▶ selected/cursor file row highlight

# Selection panel (right side, main view)
STYLE_SEL_LABEL     = Style::default().fg(CLR_TEXT_SEC)
                      # "file", "verdict", "agents", "policy" — left column

STYLE_SEL_VERDICT_U = Style::default().fg(CLR_AMBER).add_modifier(Modifier::BOLD)
                      # verdict value "UNASKED"

STYLE_SEL_VERDICT_B = Style::default().fg(CLR_RED).add_modifier(Modifier::BOLD)
                      # verdict value "BLOCKED"

STYLE_SEL_VERDICT_I = Style::default().fg(CLR_GREEN).add_modifier(Modifier::BOLD)
                      # verdict value "IN SCOPE"

STYLE_SEL_AGENTS_U  = Style::default().fg(CLR_AMBER).add_modifier(Modifier::BOLD)
                      # "UNMATCHED" agent verdict

STYLE_SEL_ACTION    = Style::default().fg(CLR_TEXT_DIM)
                      # "enter=diff  /allow  /block  /mission"

# Stats panel
STYLE_STAT_LABEL    = Style::default().fg(CLR_TEXT_SEC)
                      # "Files", "Lines", "Health", "Watch"

STYLE_STAT_VAL      = Style::default().fg(CLR_TEXT_PRI).add_modifier(Modifier::BOLD)
                      # "12", "3m 40s"

STYLE_STAT_ADD      = Style::default().fg(CLR_GREEN).add_modifier(Modifier::BOLD)
                      # "+2354"

STYLE_STAT_DEL      = Style::default().fg(CLR_RED).add_modifier(Modifier::BOLD)
                      # "−222"

STYLE_JUDGE_SECTION = Style::default().fg(CLR_JUDGE_LABEL)
                      # "— Judge —"

STYLE_JUDGE_DRIFT   = Style::default().fg(CLR_RED).add_modifier(Modifier::BOLD)
                      # "✗ DRIFT DETECTED"

STYLE_JUDGE_MATCHES = Style::default().fg(CLR_GREEN).add_modifier(Modifier::BOLD)
                      # "✓ MATCHES MISSION"

STYLE_CONF_BAR_FILL = Style::default().fg(CLR_GREEN).bg(CLR_GREEN)
                      # confidence bar filled segment (█ characters)

STYLE_CONF_BAR_EMPTY= Style::default().fg(CLR_BORDER).bg(CLR_BORDER)
                      # confidence bar empty segment

STYLE_JUDGE_QUOTE   = Style::default().fg(CLR_TEXT_SEC)
                      # judge reasoning text in quotes

STYLE_JUDGE_MODEL   = Style::default().fg(CLR_TEXT_DIM)
                      # "ollama / qwen3.5:2b"

# Verdicts bar chart
STYLE_BAR_INSCOPE   = Style::default().fg(CLR_GREEN).bg(CLR_GREEN)
                      # filled green bar segments (█)

STYLE_BAR_UNASKED   = Style::default().fg(CLR_AMBER).bg(CLR_AMBER)
                      # filled amber bar segments

STYLE_BAR_BLOCKED   = Style::default().fg(CLR_RED).bg(CLR_RED)
                      # filled red bar segments

STYLE_BAR_LABEL     = Style::default().fg(CLR_TEXT_DIM)
                      # "In Scope", "Unasked", "Blocked" bar labels

STYLE_BAR_COUNT     = Style::default().fg(CLR_TEXT_PRI).add_modifier(Modifier::BOLD)
                      # "6", "6" count below bars

# Scope distribution bar
STYLE_DIST_GREEN    = Style::default().fg(CLR_GREEN).bg(CLR_GREEN)
                      # green portion of horizontal distribution bar

STYLE_DIST_AMBER    = Style::default().fg(CLR_AMBER).bg(CLR_AMBER)
                      # amber portion

STYLE_DIST_RED      = Style::default().fg(CLR_RED).bg(CLR_RED)
                      # red portion

STYLE_DIST_PCT_G    = Style::default().fg(CLR_GREEN)
                      # "● 50%" green legend

STYLE_DIST_PCT_A    = Style::default().fg(CLR_AMBER)
                      # "● 50%" amber legend

STYLE_DIST_PCT_R    = Style::default().fg(CLR_RED)
                      # "● 0%" red legend

STYLE_DIST_SUBLABEL = Style::default().fg(CLR_TEXT_DIM)
                      # "scope  unasked  blocked" sub-labels

# Slash command panel
STYLE_SLASH_HDR     = Style::default().fg(CLR_TEXT_PRI).add_modifier(Modifier::BOLD)
                      # "Slash Commands  Tab=complete  Enter=run  Esc=cancel"

STYLE_SLASH_KEY     = Style::default().fg(CLR_PURPLE)
                      # "/status", "/diff", "/check" command names

STYLE_SLASH_DESC    = Style::default().fg(CLR_TEXT_SEC)
                      # "Refresh session and file summary"

STYLE_SLASH_ARG     = Style::default().fg(CLR_TEXT_DIM)
                      # "[file]", "[claude|codex|ollama]" arg hints

STYLE_SLASH_CURSOR  = Style::default().fg(CLR_CYAN)
                      # ▶ active row indicator in command list

STYLE_INPUT_PROMPT  = Style::default().fg(CLR_TEXT_DIM)
                      # "> " prompt prefix

STYLE_INPUT_TEXT    = Style::default().fg(CLR_TEXT_PRI)
                      # user-typed text "/|"

# Status bar (bottom strip)
STYLE_SB_COUNT_G    = Style::default().fg(CLR_GREEN)
                      # "6 in scope"

STYLE_SB_COUNT_A    = Style::default().fg(CLR_AMBER)
                      # "6 unasked"

STYLE_SB_COUNT_R    = Style::default().fg(CLR_RED)
                      # "0 blocked"

STYLE_SB_LIVE       = Style::default().fg(CLR_GREEN)
                      # "● live"

STYLE_SB_KEY        = Style::default().fg(CLR_PURPLE).add_modifier(Modifier::BOLD)
                      # "/=cmd", "/theme", "xjudge" — hotkey labels

STYLE_SB_SEP        = Style::default().fg(CLR_TEXT_DIM)
                      # "·" separators

STYLE_SB_JUDGEINFO  = Style::default().fg(CLR_TEXT_DIM)
                      # "judge=ollama / qwen3.5:2b"


# ═══════════════════════════════════════════════════════════════════════════════
# SECTION 3 — LAYOUT (ratatui constraints, exact from screenshots)
# ═══════════════════════════════════════════════════════════════════════════════

# ── ROOT vertical split ───────────────────────────────────────────────────────
# Direction::Vertical
# [0] Header bar         Length(1)   — "agentscope  watch:aggregate  2 active …"
# [1] Main body          Min(0)      — everything else
# [2] Status bar         Length(1)   — "6 in scope · 6 unasked · 0 blocked …"

# ── MAIN VIEW (default, no 'd' pressed) ──────────────────────────────────────
# Direction::Vertical inside [1]
# [0] Missions panel     Length(6)   — "Agent Missions" + 2 agent rows + padding
# [1] Separator          Length(1)   — blank line
# [2] File+panels area   Min(0)
#     Direction::Horizontal
#     [0] File list      Percentage(62)
#     [1] Selection pane Percentage(38)

# ── D-VIEW (press 'd', stats + verdicts + charts) ────────────────────────────
# Direction::Horizontal inside [1]
# [0] Left side          Percentage(52)
#     Direction::Vertical
#     [0] File list      Min(0)
#     [1] Verdicts panel Length(20)  — bar chart
# [1] Right side         Percentage(48)
#     Direction::Vertical
#     [0] Scope dist     Length(6)   — horizontal bar + legend
#     [1] Stats & Judge  Min(0)      — files/lines/health/watch + judge section

# ── SLASH COMMAND PANEL (press / or =) ────────────────────────────────────────
# Overlays the bottom portion of Main body
# Direction::Vertical inside [1] bottom-aligned
# [0] (spacer)           Min(0)
# [1] Slash panel        Length(~14) — header + command list + input line

# ── SELECTION PANE (right, main view) ─────────────────────────────────────────
# No borders (borderless). Sections:
# Row 0: "Selection"              — section header, bold white
# Row 1: "file    <filename>"     — 2-col: label (8w) + value
# Row 2: "verdict <VERDICT>"      — colored verdict
# Row 3: "agents  <AGENT STATUS>" — teal or amber
# Row 4: "policy  <policy text>"  — dim text
# Row 5: blank
# Row 6: "enter=diff  /allow  /block  /mission"  — action hints, all dim


# ═══════════════════════════════════════════════════════════════════════════════
# SECTION 4 — SPECIFIC WIDGET SPECS
# ═══════════════════════════════════════════════════════════════════════════════

# ── HEADER BAR (1 row) ────────────────────────────────────────────────────────
# Format (space-separated spans, no border):
#   [purple bold] "agentscope"
#   [dim]         "  watch:aggregate"        ← mode label
#   [dim]         "  "
#   [cyan bold]   "2 active"
#   [dim]         "  ·  "
#   [amber]       "4 stale/ignored"
#   [dim]         "  ·  "
#   [dim]         "theme agentscope"          ← current theme name

# ── MISSIONS PANEL ────────────────────────────────────────────────────────────
# Header row:
#   [white bold]  "Agent Missions"
# Each agent row (one line each):
#   [teal bold]   "CLAUDE"      (or "CODEX", "GEMINI", etc. — 6 chars, left-pad)
#   " "
#   [green bold]  "85%"         (coloured by pct threshold)
#   [dim]         " 1h  "       (right-padded to 5 chars)
#   [text-pri]    mission_text truncated to terminal_width - 25
# Second agent row identical format.
# No borders. Bottom blank line separator.

# ── FILE LIST ─────────────────────────────────────────────────────────────────
# Each row is ONE Line with spans:
#   col 0: cursor indicator — [cyan] "▶ " or "  " (2 chars)
#   col 1: verdict tag      — exactly 8 chars, space-right-padded
#             "IN SCOPE" [green bold], "UNASKED " [amber bold], "BLOCKED " [red bold]
#   col 2: " " (1 space)
#   col 3: filename         — colored by verdict (blue/amber/red), variable width
#   col 4: " "
#   col 5: diff stats       — "+NNN" [green] " −NNN" [red/dim], e.g. "+205 −5"
#             If zero: show just "+0" in dim, not in green
#   col 6: " "
#   col 7: agent badge      — [teal] agent name, or [dim] "UNMATCHED"
#             Truncate to 12 chars. Right-align to terminal edge.
# Selected row: Style::default().bg(CLR_SURFACE2) applied to entire row.
# Cursor (▶): only on selected row. Uses CLR_CYAN.

# ── SCOPE DISTRIBUTION BAR ────────────────────────────────────────────────────
# Section header: [white bold] "Scope Distribution"
# Bar row: full-width horizontal bar, rendered as █ characters repeated.
#   Proportions: (in_scope / total) * bar_width green █
#                (unasked  / total) * bar_width amber █
#                (blocked  / total) * bar_width red   █
#   bar_width = panel_inner_width (no padding)
# Legend row (blank line after bar):
#   [green]  "● 50%"   [dim] " scope  "
#   [amber]  "● 50%"   [dim] " unasked  "
#   [red]    "● 0%"    [dim] " blocked"
# IMPORTANT: percentages shown as integers (round, no decimal).

# ── STATS & JUDGE PANEL ───────────────────────────────────────────────────────
# Section header: [white bold] "Stats & Judge"
# Stats rows (label: 7 chars right-padded, then value):
#   "Files  " [bold white] "12"
#   "Lines  " [green bold] "+2354"  [dim] " "  [red bold] "−222"
#   "Health " [amber bar 8 chars] " [amber bold] "50%"
#             Health bar: (in_scope / total * 8) amber █ + remainder dim ░
#   "Watch  " [dim] "3m 40s" [dim] "  (1492 cycles)"
# Blank line
# Judge subsection:
#   [purple] "— Judge —"
#   [red bold if DRIFT, green bold if MATCHES]  "✗ DRIFT DETECTED"  or  "✓ MATCHES MISSION"
#   "Conf.  " + confidence bar (10 chars) + " " + [bold] "95%"
#             Conf bar: (conf * 10) green █, remainder dim ░
#   [dim italic] '"Files modified include unasked modules …"'
#             Truncate to panel_width - 2. Wrap to 2 lines max.
#   [dim] "ollama / qwen3.5:2b"
# Blank line
# Mission subsection:
#   [dim] "— Mission —"
#   [text-pri] mission text, truncated to panel_width - 2 with "…" at end.

# ── VERDICTS BAR CHART ────────────────────────────────────────────────────────
# Section header: [green bold] "Verdicts"  (in a Box border)
# The chart area shows 3 side-by-side columns of horizontal bars.
# Implementation approach (terminal-native):
#   Each "bar row" is one Line of text:
#     col 0 (green, width=bar_col_width): if row_index < inscope_count → [green bg] "█"*col_w else blank
#     col 1 (amber, width=bar_col_width): if row_index < unasked_count → [amber bg] "█"*col_w else blank
#     col 2 (red,   width=bar_col_width): if row_index < blocked_count → [red bg]   "█"*col_w else blank
#   bar_col_width = 8 (from screenshot — 8 chars wide per column, 2-char gap between)
#   Number of rows = max(inscope_count, unasked_count, blocked_count)
#     If count > panel_height-3, scale: bar_height = (count / max_count) * (panel_height-3)
#   Bottom count label row: [bold white] right-aligned count per column
#   Bottom label row:       [dim] "In Scope"  "Unasked"  "Blocked"

# ── SLASH COMMAND OVERLAY ─────────────────────────────────────────────────────
# Triggered by "/" or "=" key. Covers bottom ~14 rows of the main area.
# Header:  [bold white] "Slash Commands"
#          [dim]        "  Tab=complete  Enter=run  Esc=cancel"
# Separator: dim "─" * panel_width
# Command rows (max 9 visible, scrollable):
#   [cyan ▶] [purple] "/status"              [dim col2 at x=40] "Refresh session …"
#            [purple] "/diff" [dim] " [file]"                   "Open colored diff …"
#   etc.
# Selected command row: bg CLR_SURFACE2
# Input row at very bottom:
#   [dim] "> "  [white] typed_text  [cyan] "█"  (block cursor)

# ── STATUS BAR (1 row, bottom) ────────────────────────────────────────────────
# Left section:
#   [green]  "6 in scope"
#   [dim]    " · "
#   [amber]  "6 unasked"
#   [dim]    " · "
#   [red]    "0 blocked"
#   [dim]    "  "
#   [green]  "●"  [dim] " live"
#   [dim]    "  "
#   [dim]    "/=cmd"
# Center/right section (right-aligned):
#   [dim]    "enter=diff"
#   [dim]    "  d=hide"
#   [dim]    "  ?=help"
#   [dim]    "  "
#   [purple bold] "xjudge"
#   [dim]    "  "
#   [purple bold] "/theme"
#   [dim]    "  judge="
#   [dim]    "ollama / qwen3.5:2b"


# ═══════════════════════════════════════════════════════════════════════════════
# SECTION 5 — POLISH IMPROVEMENTS (delta from current screenshots)
# ═══════════════════════════════════════════════════════════════════════════════

# The current UI scores 9/10. These are the remaining 10%:

# FIX 1 — MISSION PANEL text overflow
# Current: long mission text ("PLEASE IMPLEMENT THIS PLAN:") is cut abruptly.
# Fix: truncate at terminal_width - 26, then append "…" if truncated.
# Code pattern:
#   let max_w = area.width as usize - 26;
#   let truncated = if mission.len() > max_w {
#       format!("{}…", &mission[..max_w])
#   } else {
#       mission.to_string()
#   };

# FIX 2 — VERDICTS bar chart: add gap between columns
# Current: green/amber bars may merge visually.
# Fix: render 3 columns with a 2-char gap:
#   [green bar col=8] [gap=2 spaces] [amber bar col=8] [gap=2] [red bar col=8]

# FIX 3 — SELECTION PANE: "no active mission matched" should be dim italic
# Currently reads as same style as other policy text. Differentiate:
#   STYLE_SEL_POLICY_NONE = Style::default().fg(CLR_TEXT_DIM).add_modifier(Modifier::ITALIC)

# FIX 4 — DIFF STATS: use U+2212 MINUS SIGN (−) not ASCII hyphen-minus (-)
# Current: "−167" — verify this is using the correct Unicode minus character.
# In Rust: let del_str = format!("\u{2212}{}", deletions);

# FIX 5 — HEALTH BAR in Stats panel: use distinct bar chars
# Use "█" for fill and "░" for empty (not "─" or spaces).
# Width: 8 chars. Example for 50%: "████░░░░"
# fn health_bar(pct: f32, width: usize) -> String {
#     let filled = (pct * width as f32).round() as usize;
#     "█".repeat(filled) + &"░".repeat(width - filled)
# }

# FIX 6 — SCOPE DISTRIBUTION bar: add 1-row gap after the bar before legend
# Current screenshots show the legend directly under the bar with no gap.
# Add a blank Line::default() between bar row and legend row.

# FIX 7 — STATUS BAR: "● live" dot should pulse (if terminal supports it)
# Use a tick counter: on even ticks show "●", on odd ticks show "○" (dim).
# Tick period: every 2 refresh cycles (i.e., ~1 second).

# FIX 8 — BLOCKED files: show the policy name that was triggered in file row
# Current file rows don't show the policy name inline.
# Append [dim] "  (policy: no-env-writes)" after the diff stats for BLOCKED rows.
# Truncate to fit if row too wide.

# NO-CHANGE (things to preserve exactly as-is):
# - The purple "agentscope" wordmark style — perfect.
# - The "▶" cursor indicator placement — correct.
# - The amber for UNASKED — right hue, distinguishable from green.
# - The slash command overlay transparency (no bg box border) — clean.
# - "2 active · 4 stale/ignored" in header — clear and useful.
# - The mission text appearing twice (missions panel + stats mission section) — intentional, fine.


# ═══════════════════════════════════════════════════════════════════════════════
# SECTION 6 — KEYBOARD BINDINGS (implement in event handler)
# ═══════════════════════════════════════════════════════════════════════════════

# Global (always active):
#   q / Ctrl+C   → quit
#   ?            → toggle help overlay
#   /  or  =     → open slash command input
#   d            → toggle D-view (stats + charts mode)
#   Esc          → close slash panel / cancel

# File list navigation:
#   j / ↓        → move cursor down
#   k / ↑        → move cursor up
#   g            → jump to top
#   G            → jump to bottom
#   Enter        → open diff for selected file (call: agentscope diff <file>)
#   f            → filter: cycle through ALL / PROBLEMS (blocked+unasked) / BLOCKED only

# Judge:
#   x            → run judge (xjudge keybind in statusbar)
#   r            → re-run judge with current files

# Theme:
#   /theme       → cycle to next theme (currently only "agentscope" dark theme)

# Slash commands (while input active):
#   Tab          → autocomplete command name
#   ↑ / ↓       → navigate command list
#   Enter        → run selected/typed command
#   Esc          → cancel slash mode


# ═══════════════════════════════════════════════════════════════════════════════
# SECTION 7 — SLASH COMMANDS REFERENCE (full list from screenshot)
# ═══════════════════════════════════════════════════════════════════════════════

# /status                          Refresh session and file summary
# /diff [file]                     Open colored diff for selected or named file
# /check                           Summarize policy status in the activity log
# /judge                           Run the configured LLM judge
# /judge-provider [claude|codex|ollama]    List or switch judge provider
# /judge-model [model]             List or set judge model
# /judge-models [model]            Alias for /judge-model
# /ollama-models                   List installed Ollama models
# /ollama-model [model]            Set installed Ollama model
# /problems                        Toggle blocked/unasked filter


# ═══════════════════════════════════════════════════════════════════════════════
# SECTION 8 — RUST COLOUR CONSTANTS (paste directly into tui.rs)
# ═══════════════════════════════════════════════════════════════════════════════

# const CLR_BG:           Color = Color::Rgb(13,  15,  20);
# const CLR_SURFACE:      Color = Color::Rgb(20,  23,  30);
# const CLR_SURFACE2:     Color = Color::Rgb(26,  29,  38);
# const CLR_BORDER:       Color = Color::Rgb(38,  42,  55);
# const CLR_TEXT_PRI:     Color = Color::Rgb(220, 225, 235);
# const CLR_TEXT_SEC:     Color = Color::Rgb(140, 148, 164);
# const CLR_TEXT_DIM:     Color = Color::Rgb(72,  78,  95);
# const CLR_TEXT_MUTED:   Color = Color::Rgb(100, 108, 126);
# const CLR_GREEN:        Color = Color::Rgb(80,  200, 120);
# const CLR_GREEN_DIM:    Color = Color::Rgb(30,  80,  50);
# const CLR_AMBER:        Color = Color::Rgb(230, 160, 50);
# const CLR_AMBER_DIM:    Color = Color::Rgb(80,  50,  10);
# const CLR_RED:          Color = Color::Rgb(220, 80,  80);
# const CLR_RED_DIM:      Color = Color::Rgb(80,  20,  20);
# const CLR_PURPLE:       Color = Color::Rgb(160, 120, 255);
# const CLR_CYAN:         Color = Color::Rgb(80,  210, 220);
# const CLR_BLUE:         Color = Color::Rgb(90,  160, 230);
# const CLR_TEAL:         Color = Color::Rgb(60,  180, 160);
# const CLR_JUDGE_LABEL:  Color = Color::Rgb(160, 120, 255);
# const CLR_LIVE_DOT:     Color = Color::Rgb(80,  200, 120);
