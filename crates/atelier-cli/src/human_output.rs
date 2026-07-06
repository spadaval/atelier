// This module is the shared formatter boundary for the output refresh.

use std::collections::BTreeSet;
use std::io::IsTerminal;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct RenderContext {
    style_policy: StylePolicy,
}

impl RenderContext {
    #[allow(dead_code)]
    pub(crate) fn plain() -> Self {
        Self {
            style_policy: StylePolicy::plain(),
        }
    }

    pub(crate) fn for_stdout() -> Self {
        Self {
            style_policy: StylePolicy::for_stdout(),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn from_parts(choice: ColorChoice, is_terminal: bool, no_color: bool) -> Self {
        Self {
            style_policy: StylePolicy::from_context(choice, is_terminal, no_color),
        }
    }

    pub(crate) fn style_policy(self) -> StylePolicy {
        self.style_policy
    }

    pub(crate) fn paint(self, style: TextStyle, text: impl AsRef<str>) -> String {
        self.style_policy.paint(style, text)
    }
}

pub(crate) trait Panel {
    fn render(&self, context: RenderContext) -> Vec<String>;
}

pub(crate) struct Page {
    title: String,
    panels: Vec<Box<dyn Panel>>,
}

impl Page {
    pub(crate) fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            panels: Vec::new(),
        }
    }

    pub(crate) fn panel(mut self, panel: impl Panel + 'static) -> Self {
        self.panels.push(Box::new(panel));
        self
    }

    pub(crate) fn render(&self, context: RenderContext) -> String {
        let mut lines = vec![context.paint(TextStyle::Heading, &self.title)];
        lines.push("=".repeat(self.title.len()));
        for panel in &self.panels {
            let rendered = panel.render(context);
            if rendered.is_empty() {
                continue;
            }
            lines.push(String::new());
            lines.extend(rendered);
        }
        lines.join("\n")
    }

    pub(crate) fn print(&self, context: RenderContext) {
        println!("{}", self.render(context));
    }
}

pub(crate) struct MetadataPanel {
    title: Option<String>,
    rows: Vec<(String, String)>,
}

impl MetadataPanel {
    pub(crate) fn new(title: impl Into<String>) -> Self {
        Self {
            title: Some(title.into()),
            rows: Vec::new(),
        }
    }

    pub(crate) fn untitled() -> Self {
        Self {
            title: None,
            rows: Vec::new(),
        }
    }

    pub(crate) fn row(mut self, label: impl Into<String>, value: impl Into<String>) -> Self {
        self.rows.push((label.into(), value.into()));
        self
    }

    #[allow(dead_code)]
    pub(crate) fn optional_row(
        mut self,
        label: impl Into<String>,
        value: Option<impl Into<String>>,
    ) -> Self {
        if let Some(value) = value {
            self.rows.push((label.into(), value.into()));
        }
        self
    }
}

impl Panel for MetadataPanel {
    fn render(&self, _context: RenderContext) -> Vec<String> {
        if self.rows.is_empty() {
            return Vec::new();
        }
        let width = self
            .rows
            .iter()
            .map(|(label, _)| label.len())
            .max()
            .unwrap_or(0);
        let mut lines = Vec::new();
        if let Some(title) = &self.title {
            lines.push(section_heading(title));
        }
        lines.extend(
            self.rows
                .iter()
                .map(|(label, value)| format!("{label:<width$}: {value}", width = width)),
        );
        lines
    }
}

pub(crate) struct TextPanel {
    title: String,
    body: Option<String>,
}

impl TextPanel {
    pub(crate) fn new(title: impl Into<String>, body: Option<impl Into<String>>) -> Self {
        Self {
            title: title.into(),
            body: body.map(Into::into),
        }
    }
}

impl Panel for TextPanel {
    fn render(&self, _context: RenderContext) -> Vec<String> {
        let Some(body) = self
            .body
            .as_deref()
            .map(str::trim)
            .filter(|body| !body.is_empty())
        else {
            return Vec::new();
        };
        vec![section_heading(&self.title), body.to_string()]
    }
}

pub(crate) struct LinesPanel {
    title: String,
    lines: Vec<String>,
    empty: String,
}

impl LinesPanel {
    pub(crate) fn new(
        title: impl Into<String>,
        lines: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            title: title.into(),
            lines: lines.into_iter().map(Into::into).collect(),
            empty: "(none)".to_string(),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn empty(mut self, empty: impl Into<String>) -> Self {
        self.empty = empty.into();
        self
    }
}

impl Panel for LinesPanel {
    fn render(&self, _context: RenderContext) -> Vec<String> {
        let mut lines = if self.title.is_empty() {
            Vec::new()
        } else {
            vec![section_heading(&self.title)]
        };
        if self.lines.is_empty() {
            lines.push(self.empty.clone());
        } else {
            lines.extend(self.lines.iter().cloned());
        }
        lines
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct IssueListRow {
    pub role: DisplayRole,
    pub id: String,
    pub status: Option<String>,
    pub priority: String,
    pub title: String,
    pub blockers: usize,
    pub depth: usize,
}

pub(crate) struct IssueListPanel {
    title: String,
    rows: Vec<IssueListRow>,
    total_count: usize,
    limit: usize,
}

impl IssueListPanel {
    pub(crate) fn new(title: impl Into<String>, rows: Vec<IssueListRow>) -> Self {
        let total_count = rows.len();
        Self {
            title: title.into(),
            rows,
            total_count,
            limit: 20,
        }
    }

    pub(crate) fn total_count(mut self, total_count: usize) -> Self {
        self.total_count = total_count;
        self
    }

    pub(crate) fn limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }
}

impl Panel for IssueListPanel {
    fn render(&self, context: RenderContext) -> Vec<String> {
        let mut lines = vec![section_heading(&self.title)];
        if self.total_count == 0 {
            lines.push("(none)".to_string());
            return lines;
        }
        let limit = self.limit.min(self.rows.len());
        for row in self.rows.iter().take(limit) {
            let indent = "  ".repeat(row.depth.max(1));
            let role = row.role.render(context.style_policy());
            let status = row
                .status
                .as_deref()
                .map(|status| format!(" [{status}]"))
                .unwrap_or_default();
            let blockers = if row.blockers == 0 {
                String::new()
            } else {
                format!(" | {} blocker{}", row.blockers, plural_suffix(row.blockers))
            };
            lines.push(format!(
                "{indent}{role} {}{} {} - {}{}",
                row.id, status, row.priority, row.title, blockers
            ));
        }
        let omitted = self.total_count.saturating_sub(limit);
        if omitted > 0 {
            lines.push(format!(
                "  ... {omitted} more {} item{} omitted",
                self.title.to_lowercase(),
                plural_suffix(omitted)
            ));
        }
        lines
    }
}

pub(crate) struct FooterPanel {
    title: String,
    actions: Vec<FooterAction>,
}

impl FooterPanel {
    pub(crate) fn new(
        title: impl Into<String>,
        actions: impl IntoIterator<Item = FooterAction>,
    ) -> Self {
        Self {
            title: title.into(),
            actions: actions.into_iter().collect(),
        }
    }
}

impl Panel for FooterPanel {
    fn render(&self, _context: RenderContext) -> Vec<String> {
        render_footer(&self.title, self.actions.clone())
            .lines()
            .map(str::to_string)
            .collect()
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ColorChoice {
    Auto,
    Always,
    Never,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct StylePolicy {
    color: bool,
}

impl StylePolicy {
    #[allow(dead_code)]
    pub(crate) fn plain() -> Self {
        Self { color: false }
    }

    pub(crate) fn from_context(choice: ColorChoice, is_terminal: bool, no_color: bool) -> Self {
        let color = match choice {
            ColorChoice::Always => !no_color,
            ColorChoice::Auto => is_terminal && !no_color,
            ColorChoice::Never => false,
        };
        Self { color }
    }

    pub(crate) fn for_stdout() -> Self {
        Self::from_context(
            ColorChoice::Auto,
            std::io::stdout().is_terminal(),
            std::env::var_os("NO_COLOR").is_some(),
        )
    }

    pub(crate) fn paint(self, style: TextStyle, text: impl AsRef<str>) -> String {
        let text = text.as_ref();
        if !self.color {
            return text.to_string();
        }
        let code = match style {
            TextStyle::Heading => "1",
            TextStyle::Secondary => "2",
            TextStyle::Success => "32",
            TextStyle::Warning => "33",
            TextStyle::Danger => "31",
            TextStyle::Active => "36",
        };
        format!("\x1b[{code}m{text}\x1b[0m")
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TextStyle {
    Heading,
    Secondary,
    Success,
    Warning,
    Danger,
    Active,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum DecisionState {
    Allowed,
    Blocked,
    Pass,
    Fail,
}

impl DecisionState {
    pub(crate) fn label(self) -> &'static str {
        match self {
            DecisionState::Allowed => "allowed",
            DecisionState::Blocked => "blocked",
            DecisionState::Pass => "pass",
            DecisionState::Fail => "fail",
        }
    }

    pub(crate) fn style(self) -> TextStyle {
        match self {
            DecisionState::Allowed | DecisionState::Pass => TextStyle::Success,
            DecisionState::Blocked | DecisionState::Fail => TextStyle::Danger,
        }
    }

    pub(crate) fn render(self, policy: StylePolicy) -> String {
        policy.paint(self.style(), self.label())
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum BlockerState {
    Clear,
    Direct,
    ThroughParent,
}

#[allow(dead_code)]
impl BlockerState {
    pub(crate) fn label(self) -> &'static str {
        match self {
            BlockerState::Clear => "not blocked",
            BlockerState::Direct => "blocked",
            BlockerState::ThroughParent => "blocked through parent",
        }
    }

    pub(crate) fn role(self) -> DisplayRole {
        match self {
            BlockerState::Clear => DisplayRole::Selectable,
            BlockerState::Direct => DisplayRole::Blocked,
            BlockerState::ThroughParent => DisplayRole::BlockedThroughParent,
        }
    }

    pub(crate) fn render(self, policy: StylePolicy) -> String {
        policy.paint(self.role().style(), self.label())
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum DisplayRole {
    Executable,
    Selectable,
    Blocked,
    BlockedThroughParent,
    ContextOnly,
    Omitted,
}

impl DisplayRole {
    pub(crate) fn label(self) -> &'static str {
        match self {
            DisplayRole::Executable => "active",
            DisplayRole::Selectable => "ready",
            DisplayRole::Blocked => "blocked",
            DisplayRole::BlockedThroughParent => "blocked through parent",
            DisplayRole::ContextOnly => "shown for context",
            DisplayRole::Omitted => "omitted",
        }
    }

    pub(crate) fn style(self) -> TextStyle {
        match self {
            DisplayRole::Executable => TextStyle::Active,
            DisplayRole::Selectable => TextStyle::Success,
            DisplayRole::Blocked | DisplayRole::BlockedThroughParent => TextStyle::Danger,
            DisplayRole::ContextOnly | DisplayRole::Omitted => TextStyle::Secondary,
        }
    }

    pub(crate) fn render(self, policy: StylePolicy) -> String {
        policy.paint(self.style(), self.label())
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct RecoveryCallout {
    title: String,
    steps: Vec<String>,
}

#[allow(dead_code)]
impl RecoveryCallout {
    pub(crate) fn new(
        title: impl Into<String>,
        steps: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            title: title.into(),
            steps: steps.into_iter().map(Into::into).collect(),
        }
    }

    pub(crate) fn render(&self) -> String {
        let mut lines = vec![section_heading(&self.title)];
        if self.steps.is_empty() {
            lines.push("(none)".to_string());
        } else {
            for (index, step) in self.steps.iter().enumerate() {
                lines.push(format!("  {}. {step}", index + 1));
            }
        }
        lines.join("\n")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct FooterAction {
    label: String,
    command: String,
}

impl FooterAction {
    pub(crate) fn new(label: impl Into<String>, command: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            command: command.into(),
        }
    }

    fn render(&self) -> String {
        format!("  {}: {}", self.label, self.command)
    }
}

pub(crate) fn heading(title: &str) -> String {
    format!("{title}\n{}", "=".repeat(title.len()))
}

pub(crate) fn section_heading(title: &str) -> String {
    format!("{title}\n{}", "-".repeat(title.len()))
}

pub(crate) fn print_heading(title: &str) {
    println!("{}", heading(title));
}

pub(crate) fn print_section_heading(title: &str) {
    println!("{}", section_heading(title));
}

pub(crate) fn render_footer(
    title: &str,
    actions: impl IntoIterator<Item = FooterAction>,
) -> String {
    let mut seen = BTreeSet::new();
    let mut lines = vec![section_heading(title)];
    for action in actions {
        if seen.insert((action.label.clone(), action.command.clone())) {
            lines.push(action.render());
        }
    }
    lines.join("\n")
}

pub(crate) fn bounded_items<T: Clone>(items: &[T], limit: usize) -> (Vec<T>, usize) {
    if items.len() <= limit {
        (items.to_vec(), 0)
    } else {
        (items[..limit].to_vec(), items.len() - limit)
    }
}

pub(crate) fn path_summary(paths: &[String], sample_limit: usize) -> String {
    if paths.is_empty() {
        return "clean".to_string();
    }
    let (sample, omitted) = bounded_items(paths, sample_limit);
    let mut summary = format!("{} path{}", paths.len(), plural_suffix(paths.len()));
    if !sample.is_empty() {
        summary.push_str(": ");
        summary.push_str(&sample.join(", "));
    }
    if omitted > 0 {
        summary.push_str(&format!(", {omitted} more omitted"));
    }
    summary
}

pub(crate) fn plural_suffix(count: usize) -> &'static str {
    if count == 1 {
        ""
    } else {
        "s"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_policy_auto_requires_terminal_and_no_color_absent() {
        assert!(StylePolicy::from_context(ColorChoice::Auto, true, false).color);
        assert!(!StylePolicy::from_context(ColorChoice::Auto, false, false).color);
        assert!(!StylePolicy::from_context(ColorChoice::Auto, true, true).color);
        assert!(!StylePolicy::from_context(ColorChoice::Always, true, true).color);
        assert!(StylePolicy::from_context(ColorChoice::Always, false, false).color);
        assert!(!StylePolicy::from_context(ColorChoice::Never, true, false).color);
    }

    #[test]
    fn decision_and_blocker_labels_are_colorless_by_default() {
        let policy = StylePolicy::plain();
        assert_eq!(DecisionState::Allowed.render(policy), "allowed");
        assert_eq!(DecisionState::Fail.render(policy), "fail");
        assert_eq!(BlockerState::Clear.render(policy), "not blocked");
        assert_eq!(
            BlockerState::ThroughParent.render(policy),
            "blocked through parent"
        );
    }

    #[test]
    fn colorless_display_roles_keep_text_meaning() {
        let policy = StylePolicy::plain();
        assert_eq!(DisplayRole::Selectable.render(policy), "ready");
        assert_eq!(
            DisplayRole::BlockedThroughParent.render(policy),
            "blocked through parent"
        );
        assert_eq!(DisplayRole::ContextOnly.render(policy), "shown for context");
    }

    #[test]
    fn colored_display_roles_wrap_text_with_ansi() {
        let policy = StylePolicy::from_context(ColorChoice::Always, false, false);
        assert_eq!(
            DisplayRole::Blocked.render(policy),
            "\u{1b}[31mblocked\u{1b}[0m"
        );
    }

    #[test]
    fn footer_actions_are_deduplicated() {
        let footer = render_footer(
            "Next Commands",
            [
                FooterAction::new("Inspect", "atelier issue show atelier-1234"),
                FooterAction::new("Inspect", "atelier issue show atelier-1234"),
                FooterAction::new("Validate", "atelier check atelier-1234"),
            ],
        );
        assert_eq!(
            footer,
            "Next Commands\n-------------\n  Inspect: atelier issue show atelier-1234\n  Validate: atelier check atelier-1234"
        );
    }

    #[test]
    fn recovery_callout_renders_ordered_public_steps() {
        let callout = RecoveryCallout::new(
            "Recovery",
            [
                "run `atelier check`",
                "fix the named canonical record",
                "rerun the blocked command",
            ],
        );
        assert_eq!(
            callout.render(),
            "Recovery\n--------\n  1. run `atelier check`\n  2. fix the named canonical record\n  3. rerun the blocked command"
        );
    }

    #[test]
    fn path_summary_bounds_samples() {
        let paths = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        assert_eq!(path_summary(&paths, 2), "3 paths: a, b, 1 more omitted");
    }

    #[test]
    fn page_skips_empty_panels_and_renders_metadata() {
        let page = Page::new("Example")
            .panel(MetadataPanel::untitled().row("Status", "ready"))
            .panel(TextPanel::new("Notes", None::<String>));
        assert_eq!(
            page.render(RenderContext::plain()),
            "Example\n=======\n\nStatus: ready"
        );
    }

    #[test]
    fn issue_list_panel_bounds_rows_and_keeps_colorless_roles() {
        let rows = vec![
            IssueListRow {
                role: DisplayRole::Selectable,
                id: "atelier-a".to_string(),
                status: Some("todo".to_string()),
                priority: "high".to_string(),
                title: "A".to_string(),
                blockers: 0,
                depth: 1,
            },
            IssueListRow {
                role: DisplayRole::Blocked,
                id: "atelier-b".to_string(),
                status: Some("blocked".to_string()),
                priority: "medium".to_string(),
                title: "B".to_string(),
                blockers: 2,
                depth: 1,
            },
        ];
        let panel = IssueListPanel::new("Ready Work", rows)
            .total_count(3)
            .limit(1);
        assert_eq!(
            panel.render(RenderContext::plain()).join("\n"),
            "Ready Work\n----------\n  ready atelier-a [todo] high - A\n  ... 2 more ready work items omitted"
        );
    }
}
