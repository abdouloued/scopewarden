//! All terminal output for AgentScope.
//! Implements the exact visual design from the UI spec:
//! dark-terminal aesthetic, IN SCOPE / UNASKED / BLOCKED tags,
//! red BLOCK banner, LLM judge verdict, summary stats.

use console::{style, Term};
use serde_json::json;

use crate::judge::{JudgeResult, JudgeVerdict};
use crate::policy::{AnnotatedFile, FileVerdict, LimitWarning};
use crate::session::Session;

// ── Colour palette (matches mockup exactly) ───────────────────────────────────

#[allow(dead_code)]
pub mod theme {
    use console::Style;

    pub fn dim()     -> Style { Style::new().dim() }
    pub fn muted()   -> Style { Style::new().color256(245) }  // #6b7280
    pub fn white()   -> Style { Style::new().white().bold() }
    pub fn green()   -> Style { Style::new().green() }
    pub fn red()     -> Style { Style::new().red() }
    pub fn amber()   -> Style { Style::new().color256(214) }  // #fbbf24
    pub fn cyan()    -> Style { Style::new().cyan() }
    pub fn blue()    -> Style { Style::new().blue() }
    pub fn purple()  -> Style { Style::new().color256(135) }  // #c084fc

    // Tags
    pub fn tag_ok()    -> Style { Style::new().green().bold() }
    pub fn tag_block() -> Style { Style::new().red().bold() }
    pub fn tag_warn()  -> Style { Style::new().color256(214).bold() }
    pub fn tag_skip()  -> Style { Style::new().color256(245) }

    // Structural
    pub fn rule()    -> Style { Style::new().dim() }
    pub fn label()   -> Style { Style::new().dim() }
}

// ── Printer ───────────────────────────────────────────────────────────────────

pub struct Printer {
    #[allow(dead_code)]
    term: Term,
}

impl Printer {
    pub fn new() -> Self {
        Self {
            term: Term::stdout(),
        }
    }

    // ── Helpers ───────────────────────────────────────────────────────────────

    fn rule(&self) {
        println!("{}", theme::rule().apply_to("  ─────────────────────────────────────────────────────"));
    }

    fn blank(&self) { println!(); }

    pub fn success(&self, msg: &str) {
        println!("  {} {}", theme::green().apply_to("✓"), msg);
    }

    pub fn warn(&self, msg: &str) {
        println!("  {} {}", theme::amber().apply_to("⚠"), msg);
    }

    pub fn hint(&self, msg: &str) {
        println!("  {}", theme::muted().apply_to(msg));
    }

    #[allow(dead_code)]
    pub fn error(&self, msg: &str) {
        println!("  {} {}", theme::red().apply_to("✕"), msg);
    }

    // ── Session started ───────────────────────────────────────────────────────

    pub fn session_started(&self, session: &Session) {
        self.blank();
        println!(
            "  {} {}",
            theme::purple().apply_to("agentscope"),
            theme::dim().apply_to("v0.1.0"),
        );
        self.blank();
        println!(
            "  {} {}  {}  {}",
            theme::label().apply_to("session"),
            theme::cyan().apply_to(&session.id[..12]),
            theme::dim().apply_to("·"),
            theme::muted().apply_to(&session.agent),
        );
        println!(
            "  {} {}",
            theme::label().apply_to("mission"),
            style(&format!("\"{}\"", session.mission)).white(),
        );
        self.blank();
        println!(
            "  {} {}",
            theme::green().apply_to("✓"),
            theme::muted().apply_to("Session started — run agentscope check when done"),
        );
        self.blank();
    }

    // ── One-liner status ──────────────────────────────────────────────────────

    pub fn session_one_liner(&self, session: &Session) {
        println!(
            "  {} {} {} {}",
            theme::cyan().apply_to(&session.id[..8]),
            theme::dim().apply_to("·"),
            theme::muted().apply_to(&session.agent),
            style(&format!("\"{}\"", session.mission)).white(),
        );
    }

    // ── Full check report (the viral screenshot moment) ───────────────────────

    pub fn print_check_report(&self, report: &CheckReport) {
        let session = &report.session;
        let files = &report.annotated;
        let judge = &report.judge_result;

        let in_scope: Vec<_> = files.iter().filter(|f| f.verdict == FileVerdict::InScope).collect();
        let unasked: Vec<_> = files.iter().filter(|f| f.verdict == FileVerdict::Unasked).collect();
        let blocked: Vec<_> = files.iter().filter(|f| f.verdict.is_blocked()).collect();
        let has_blocks = !blocked.is_empty();

        self.blank();

        // Header
        println!(
            "  {} {}  {}  {}",
            theme::label().apply_to("session"),
            theme::cyan().apply_to(&session.id[..12]),
            theme::dim().apply_to("·"),
            theme::muted().apply_to(&session.agent),
        );
        println!(
            "  {} {}",
            theme::label().apply_to("mission"),
            style(&format!("\"{}\"", session.mission)).white(),
        );

        self.rule();
        println!(
            "  {}",
            theme::muted().apply_to("  scanning working tree against git baseline..."),
        );
        self.blank();

        // File list
        let max_show = 20;
        let total = files.len();
        let shown = total.min(max_show);

        for file in files.iter().take(shown) {
            self.print_file_row(file);
        }

        if total > max_show {
            println!(
                "  {}  {}",
                fmt_tag_skip(),
                theme::dim().apply_to(&format!("… and {} more files", total - max_show)),
            );
        }

        // Limit warnings
        for w in &report.limit_warnings {
            self.blank();
            match w {
                LimitWarning::TooManyFiles { actual, limit } => {
                    self.print_warn_banner(&format!(
                        "{} files changed (limit: {}) — unusually broad for a single mission",
                        actual, limit
                    ));
                }
                LimitWarning::TooManyLines { actual, limit } => {
                    self.print_warn_banner(&format!(
                        "{} lines changed (limit: {}) — review carefully",
                        actual, limit
                    ));
                }
            }
        }

        self.rule();

        // BLOCK banner (the viral moment)
        if has_blocks {
            self.print_block_banner(&blocked);
        }

        // UNASKED warning
        if !unasked.is_empty() && !has_blocks {
            self.print_unasked_banner(&unasked);
        }

        // LLM judge
        if let Some(judge) = judge {
            self.print_judge_result(judge);
            self.rule();
        }

        // Summary stats
        self.print_summary(in_scope.len(), unasked.len(), blocked.len(), total);

        self.blank();

        // Audit hint
        println!(
            "  {}  {}",
            theme::dim().apply_to("→ full forensics:"),
            theme::muted().apply_to(&format!("agentscope audit {}", &session.id[..12])),
        );

        self.blank();
    }

    fn print_file_row(&self, file: &AnnotatedFile) {
        let path = file.diff.path.display().to_string();
        let stats = format!("+{} −{}", file.diff.additions, file.diff.deletions);

        match &file.verdict {
            FileVerdict::InScope => {
                println!(
                    "  {}  {}  {}",
                    fmt_tag_ok(),
                    style(&path).color256(111),   // soft blue
                    theme::dim().apply_to(&stats),
                );
            }
            FileVerdict::Unasked => {
                println!(
                    "  {}  {}  {}",
                    fmt_tag_warn(),
                    style(&path).color256(214),   // amber
                    theme::dim().apply_to(&stats),
                );
            }
            FileVerdict::Blocked { .. } => {
                println!(
                    "  {}  {}  {}",
                    fmt_tag_block(),
                    style(&path).red(),
                    theme::dim().apply_to(&stats),
                );
            }
            FileVerdict::Clean => {
                println!(
                    "  {}  {}",
                    fmt_tag_skip(),
                    theme::dim().apply_to(&path),
                );
            }
        }
    }

    fn print_block_banner(&self, blocked: &[&AnnotatedFile]) {
        self.blank();
        println!("  {}", style("  ╔═══════════════════════════════════════════╗").red());
        println!("  {}", style("  ║  BLOCK — session halted                   ║").red().bold());
        println!("  {}", style("  ║  violations of declared scope policy       ║").red());
        println!("  {}", style("  ╚═══════════════════════════════════════════╝").red());
        self.blank();

        for file in blocked {
            let policy = match &file.verdict {
                FileVerdict::Blocked { policy } => policy.clone(),
                _ => "policy violation".into(),
            };
            println!(
                "    {}  {}  {}",
                style("✕").red().bold(),
                style(file.diff.path.display().to_string()).white(),
                theme::muted().apply_to(format!("— {}", policy)),
            );
        }
        self.blank();
    }

    fn print_unasked_banner(&self, unasked: &[&AnnotatedFile]) {
        self.blank();
        let names: Vec<_> = unasked.iter().map(|f| {
            f.diff.path.file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default()
        }).collect();
        let preview = names[..names.len().min(3)].join(" · ");

        println!(
            "  {}  {} unasked file{} — review before committing",
            style("⚠").color256(214).bold(),
            unasked.len(),
            if unasked.len() == 1 { "" } else { "s" },
        );
        println!(
            "    {}  {}",
            theme::dim().apply_to("→"),
            theme::muted().apply_to(&preview),
        );
    }

    fn print_warn_banner(&self, msg: &str) {
        println!(
            "  {}  {}",
            style("⚠").color256(214).bold(),
            theme::amber().apply_to(msg),
        );
    }

    fn print_judge_result(&self, judge: &JudgeResult) {
        self.blank();
        println!(
            "  {}  {}",
            theme::label().apply_to("LLM judge"),
            theme::dim().apply_to(format!("({} / {})", judge.provider, judge.model)),
        );
        self.blank();

        let confidence_pct = (judge.confidence * 100.0) as u8;
        let verdict_str = judge.verdict.label();

        let verdict_styled = match judge.verdict {
            JudgeVerdict::Matches => style(verdict_str).green().bold(),
            JudgeVerdict::Drift => style(verdict_str).red().bold(),
            JudgeVerdict::Unknown => style(verdict_str).color256(245).bold(),
        };

        println!(
            "    {}  {} {}",
            verdict_styled,
            theme::dim().apply_to("—"),
            theme::muted().apply_to(format!("{}% confidence changes match mission", confidence_pct)),
        );

        self.blank();
        // Wrap reasoning at ~60 chars
        println!(
            "    {}",
            theme::muted().apply_to(format!("\"{}\"", judge.reasoning)),
        );
    }

    fn print_summary(&self, in_scope: usize, unasked: usize, blocked: usize, total: usize) {
        self.blank();
        let clean = total.saturating_sub(in_scope + unasked + blocked);

        println!(
            "  {}  {}  {}  {}  {}",
            style(format!("{} in scope", in_scope)).green(),
            theme::dim().apply_to("·"),
            style(format!("{} unasked", unasked)).color256(214),
            theme::dim().apply_to("·"),
            style(format!("{} blocked", blocked)).red(),
        );

        if clean > 0 {
            println!(
                "  {}",
                theme::dim().apply_to(format!("{} other files unchanged", clean)),
            );
        }
    }
}

// ── Tag formatters ─────────────────────────────────────────────────────────────

fn fmt_tag_ok() -> console::StyledObject<&'static str> {
    style("  IN SCOPE ").green().bold()
}

fn fmt_tag_warn() -> console::StyledObject<&'static str> {
    style("  UNASKED  ").color256(214).bold()
}

fn fmt_tag_block() -> console::StyledObject<&'static str> {
    style("  BLOCKED  ").red().bold()
}

fn fmt_tag_skip() -> console::StyledObject<&'static str> {
    style("  CLEAN    ").color256(245)
}

// ── CheckReport (shared data structure) ─────────────────────────────────────

pub struct CheckReport {
    pub session: Session,
    pub annotated: Vec<AnnotatedFile>,
    pub limit_warnings: Vec<LimitWarning>,
    pub judge_result: Option<JudgeResult>,
}

impl CheckReport {
    pub fn to_json(&self) -> serde_json::Value {
        json!({
            "session": {
                "id": self.session.id,
                "mission": self.session.mission,
                "agent": self.session.agent,
                "started_at": self.session.started_at,
            },
            "files": self.annotated.iter().map(|f| json!({
                "path": f.diff.path,
                "verdict": f.verdict.label(),
                "additions": f.diff.additions,
                "deletions": f.diff.deletions,
            })).collect::<Vec<_>>(),
            "blocked": self.annotated.iter().filter(|f| f.verdict.is_blocked()).count(),
            "unasked": self.annotated.iter().filter(|f| f.verdict == FileVerdict::Unasked).count(),
            "in_scope": self.annotated.iter().filter(|f| f.verdict == FileVerdict::InScope).count(),
            "judge": self.judge_result.as_ref().map(|j| json!({
                "confidence": j.confidence,
                "verdict": j.verdict.label(),
                "reasoning": j.reasoning,
            })),
        })
    }

    pub fn to_markdown(&self) -> String {
        let blocked = self.annotated.iter().filter(|f| f.verdict.is_blocked()).count();
        let unasked = self.annotated.iter().filter(|f| f.verdict == FileVerdict::Unasked).count();
        let in_scope = self.annotated.iter().filter(|f| f.verdict == FileVerdict::InScope).count();

        let status = if blocked > 0 { "🔴 BLOCKED" } else if unasked > 0 { "🟡 UNASKED FILES" } else { "🟢 IN SCOPE" };

        let mut md = format!(
            "## AgentScope — {status}\n\n\
            **Mission:** {mission}\n\
            **Agent:** {agent} · **Session:** `{id}`\n\n\
            | Verdict | Count |\n\
            |---------|-------|\n\
            | ✅ In scope | {in_scope} |\n\
            | ⚠️ Unasked | {unasked} |\n\
            | 🚫 Blocked | {blocked} |\n\n",
            status = status,
            mission = self.session.mission,
            agent = self.session.agent,
            id = &self.session.id[..12],
            in_scope = in_scope,
            unasked = unasked,
            blocked = blocked,
        );

        if blocked > 0 {
            md.push_str("### Blocked files\n");
            for f in self.annotated.iter().filter(|f| f.verdict.is_blocked()) {
                md.push_str(&format!("- `{}`\n", f.diff.path.display()));
            }
            md.push('\n');
        }

        if let Some(judge) = &self.judge_result {
            md.push_str(&format!(
                "### Judge verdict\n> {}\n\nConfidence: {}%\n",
                judge.reasoning,
                (judge.confidence * 100.0) as u8,
            ));
        }

        md
    }
}
