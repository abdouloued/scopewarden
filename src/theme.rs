use ratatui::style::Color;

/// Semantic theme tokens used by every TUI widget.
/// No widget should reference `Color::Rgb(...)` directly — always use a token.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Theme {
    pub name: &'static str,

    // ── Surface ──────────────────────────────────────────────────────────────
    pub bg: Color,
    pub panel: Color,
    pub user_bubble: Color,
    pub assistant_bubble: Color,
    pub border: Color,
    pub border_focused: Color,

    // ── Typography ───────────────────────────────────────────────────────────
    pub text: Color,
    pub text_muted: Color,
    pub text_subtle: Color,

    // ── Brand ────────────────────────────────────────────────────────────────
    pub accent: Color,

    // ── Verdict statuses ─────────────────────────────────────────────────────
    pub expected: Color,
    pub suspicious: Color,
    pub blocked: Color,
    pub ignored: Color,

    // ── Agent badges ─────────────────────────────────────────────────────────
    pub agent_claude: Color,
    pub agent_codex: Color,
    pub agent_scope: Color,
    pub agent_system: Color,

    // ── Selection ────────────────────────────────────────────────────────────
    pub selection_bg: Color,
    pub selection_fg: Color,

    // ── Diff ─────────────────────────────────────────────────────────────────
    pub diff_add: Color,
    pub diff_remove: Color,

    // ── Alerts ───────────────────────────────────────────────────────────────
    pub warning: Color,
    pub success: Color,
    pub danger: Color,
}

impl Theme {
    /// Look up a theme by name. Falls back to `scopewarden` on unknown names.
    pub fn by_name(name: &str) -> Self {
        match name {
            "codex" => Self::codex(),
            "claude" => Self::claude(),
            "openclaw" => Self::openclaw(),
            "high-contrast" => Self::high_contrast(),
            _ => Self::scopewarden(),
        }
    }

    /// Return the name of the next theme in the cycle order.
    #[allow(dead_code)]
    pub fn next_name(current: &str) -> &'static str {
        match current {
            "scopewarden" => "codex",
            "codex" => "claude",
            "claude" => "openclaw",
            "openclaw" => "high-contrast",
            _ => "scopewarden",
        }
    }

    // ── Theme definitions ─────────────────────────────────────────────────────

    /// Dark purple AI security cockpit.
    pub fn scopewarden() -> Self {
        Self {
            name: "scopewarden",
            bg: Color::Rgb(8, 11, 18),
            panel: Color::Rgb(13, 17, 26),
            user_bubble: Color::Rgb(28, 24, 52),
            assistant_bubble: Color::Rgb(15, 23, 33),
            border: Color::Rgb(30, 38, 51),
            border_focused: Color::Rgb(124, 92, 255),
            text: Color::Rgb(230, 237, 247),
            text_muted: Color::Rgb(139, 149, 167),
            text_subtle: Color::Rgb(94, 102, 120),
            accent: Color::Rgb(124, 92, 255),
            expected: Color::Rgb(74, 222, 128),
            suspicious: Color::Rgb(251, 191, 36),
            blocked: Color::Rgb(248, 113, 113),
            ignored: Color::Rgb(100, 116, 139),
            agent_claude: Color::Rgb(216, 167, 255),
            agent_codex: Color::Rgb(103, 232, 249),
            agent_scope: Color::Rgb(124, 92, 255),
            agent_system: Color::Rgb(139, 149, 167),
            selection_bg: Color::Rgb(21, 27, 43),
            selection_fg: Color::Rgb(255, 255, 255),
            diff_add: Color::Rgb(74, 222, 128),
            diff_remove: Color::Rgb(248, 113, 113),
            warning: Color::Rgb(251, 191, 36),
            success: Color::Rgb(74, 222, 128),
            danger: Color::Rgb(248, 113, 113),
        }
    }

    /// Minimal dark terminal-native, cyan accent.
    pub fn codex() -> Self {
        Self {
            name: "codex",
            bg: Color::Rgb(9, 9, 11),
            panel: Color::Rgb(15, 17, 23),
            user_bubble: Color::Rgb(19, 32, 45),
            assistant_bubble: Color::Rgb(20, 24, 31),
            border: Color::Rgb(36, 40, 51),
            border_focused: Color::Rgb(103, 232, 249),
            text: Color::Rgb(229, 231, 235),
            text_muted: Color::Rgb(139, 148, 158),
            text_subtle: Color::Rgb(91, 100, 114),
            accent: Color::Rgb(103, 232, 249),
            expected: Color::Rgb(34, 197, 94),
            suspicious: Color::Rgb(234, 179, 8),
            blocked: Color::Rgb(239, 68, 68),
            ignored: Color::Rgb(107, 114, 128),
            agent_claude: Color::Rgb(167, 139, 250),
            agent_codex: Color::Rgb(103, 232, 249),
            agent_scope: Color::Rgb(103, 232, 249),
            agent_system: Color::Rgb(139, 148, 158),
            selection_bg: Color::Rgb(22, 27, 34),
            selection_fg: Color::Rgb(248, 250, 252),
            diff_add: Color::Rgb(34, 197, 94),
            diff_remove: Color::Rgb(239, 68, 68),
            warning: Color::Rgb(234, 179, 8),
            success: Color::Rgb(34, 197, 94),
            danger: Color::Rgb(239, 68, 68),
        }
    }

    /// Warm parchment-dark, amber accent.
    pub fn claude() -> Self {
        Self {
            name: "claude",
            bg: Color::Rgb(17, 16, 13),
            panel: Color::Rgb(24, 22, 18),
            user_bubble: Color::Rgb(42, 31, 20),
            assistant_bubble: Color::Rgb(29, 27, 23),
            border: Color::Rgb(42, 37, 29),
            border_focused: Color::Rgb(217, 119, 6),
            text: Color::Rgb(244, 239, 231),
            text_muted: Color::Rgb(168, 162, 158),
            text_subtle: Color::Rgb(120, 113, 108),
            accent: Color::Rgb(217, 119, 6),
            expected: Color::Rgb(132, 204, 22),
            suspicious: Color::Rgb(245, 158, 11),
            blocked: Color::Rgb(239, 68, 68),
            ignored: Color::Rgb(120, 113, 108),
            agent_claude: Color::Rgb(217, 119, 6),
            agent_codex: Color::Rgb(56, 189, 248),
            agent_scope: Color::Rgb(217, 119, 6),
            agent_system: Color::Rgb(168, 162, 158),
            selection_bg: Color::Rgb(36, 31, 24),
            selection_fg: Color::Rgb(255, 247, 237),
            diff_add: Color::Rgb(132, 204, 22),
            diff_remove: Color::Rgb(239, 68, 68),
            warning: Color::Rgb(245, 158, 11),
            success: Color::Rgb(132, 204, 22),
            danger: Color::Rgb(239, 68, 68),
        }
    }

    /// Open-source hacker cockpit. Electric green, black, energetic.
    pub fn openclaw() -> Self {
        Self {
            name: "openclaw",
            bg: Color::Rgb(5, 8, 7),
            panel: Color::Rgb(7, 17, 13),
            user_bubble: Color::Rgb(9, 36, 27),
            assistant_bubble: Color::Rgb(10, 18, 15),
            border: Color::Rgb(18, 49, 38),
            border_focused: Color::Rgb(0, 255, 153),
            text: Color::Rgb(216, 255, 233),
            text_muted: Color::Rgb(125, 174, 149),
            text_subtle: Color::Rgb(75, 112, 95),
            accent: Color::Rgb(0, 255, 153),
            expected: Color::Rgb(0, 255, 153),
            suspicious: Color::Rgb(255, 209, 102),
            blocked: Color::Rgb(255, 77, 109),
            ignored: Color::Rgb(92, 103, 125),
            agent_claude: Color::Rgb(192, 132, 252),
            agent_codex: Color::Rgb(0, 229, 255),
            agent_scope: Color::Rgb(0, 255, 153),
            agent_system: Color::Rgb(125, 174, 149),
            selection_bg: Color::Rgb(11, 31, 23),
            selection_fg: Color::Rgb(255, 255, 255),
            diff_add: Color::Rgb(0, 255, 153),
            diff_remove: Color::Rgb(255, 77, 109),
            warning: Color::Rgb(255, 209, 102),
            success: Color::Rgb(0, 255, 153),
            danger: Color::Rgb(255, 77, 109),
        }
    }

    /// Maximum readability. Demo and screen-recording friendly.
    pub fn high_contrast() -> Self {
        Self {
            name: "high-contrast",
            bg: Color::Rgb(0, 0, 0),
            panel: Color::Rgb(10, 10, 10),
            user_bubble: Color::Rgb(28, 28, 28),
            assistant_bubble: Color::Rgb(12, 12, 12),
            border: Color::Rgb(102, 102, 102),
            border_focused: Color::Rgb(255, 255, 255),
            text: Color::Rgb(255, 255, 255),
            text_muted: Color::Rgb(207, 207, 207),
            text_subtle: Color::Rgb(163, 163, 163),
            accent: Color::Rgb(0, 217, 255),
            expected: Color::Rgb(0, 255, 102),
            suspicious: Color::Rgb(255, 212, 0),
            blocked: Color::Rgb(255, 51, 85),
            ignored: Color::Rgb(176, 176, 176),
            agent_claude: Color::Rgb(255, 184, 108),
            agent_codex: Color::Rgb(0, 217, 255),
            agent_scope: Color::Rgb(0, 217, 255),
            agent_system: Color::Rgb(207, 207, 207),
            selection_bg: Color::Rgb(34, 34, 34),
            selection_fg: Color::Rgb(255, 255, 255),
            diff_add: Color::Rgb(0, 255, 102),
            diff_remove: Color::Rgb(255, 51, 85),
            warning: Color::Rgb(255, 212, 0),
            success: Color::Rgb(0, 255, 102),
            danger: Color::Rgb(255, 51, 85),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_by_name_known() {
        assert_eq!(Theme::by_name("codex").name, "codex");
        assert_eq!(Theme::by_name("claude").name, "claude");
        assert_eq!(Theme::by_name("openclaw").name, "openclaw");
        assert_eq!(Theme::by_name("high-contrast").name, "high-contrast");
        assert_eq!(Theme::by_name("scopewarden").name, "scopewarden");
    }

    #[test]
    fn theme_by_name_fallback() {
        assert_eq!(Theme::by_name("unknown").name, "scopewarden");
        assert_eq!(Theme::by_name("").name, "scopewarden");
    }

    #[test]
    fn theme_cycle_order() {
        assert_eq!(Theme::next_name("scopewarden"), "codex");
        assert_eq!(Theme::next_name("codex"), "claude");
        assert_eq!(Theme::next_name("claude"), "openclaw");
        assert_eq!(Theme::next_name("openclaw"), "high-contrast");
        assert_eq!(Theme::next_name("high-contrast"), "scopewarden");
        // unknown falls back to scopewarden
        assert_eq!(Theme::next_name("unknown"), "scopewarden");
    }
}
