// This module is the shared formatter boundary for the output refresh.

use std::collections::BTreeSet;
use std::io::IsTerminal;

use atelier_app::issue_inventory::IssueInventoryView;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct RenderContext {
    style_policy: StylePolicy,
    width: usize,
}

impl RenderContext {
    /// Undecorated output, as hook context and redirected output receive.
    pub(crate) fn plain() -> Self {
        Self {
            style_policy: StylePolicy::plain(),
            width: usize::MAX,
        }
    }

    pub(crate) fn for_stdout() -> Self {
        Self {
            style_policy: StylePolicy::for_stdout(),
            width: stdout_width(),
        }
    }

    #[cfg(test)]
    pub(crate) fn from_parts(choice: ColorChoice, is_terminal: bool, no_color: bool) -> Self {
        Self {
            style_policy: StylePolicy::from_context(choice, is_terminal, no_color),
            width: usize::MAX,
        }
    }

    #[cfg(test)]
    pub(crate) fn from_parts_with_width(
        choice: ColorChoice,
        is_terminal: bool,
        no_color: bool,
        width: usize,
    ) -> Self {
        Self {
            style_policy: StylePolicy::from_context(choice, is_terminal, no_color),
            width,
        }
    }

    pub(crate) fn style_policy(self) -> StylePolicy {
        self.style_policy
    }

    pub(crate) fn paint(self, style: TextStyle, text: impl AsRef<str>) -> String {
        self.style_policy.paint(style, text)
    }

    pub(crate) fn width(self) -> usize {
        self.width
    }
}

fn stdout_width() -> usize {
    std::env::var("COLUMNS")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|width| *width > 0)
        .unwrap_or(usize::MAX)
}

pub(crate) fn render_page(
    title: &str,
    panels: impl IntoIterator<Item = Vec<String>>,
    context: RenderContext,
) -> String {
    let mut lines = vec![
        context.paint(TextStyle::Heading, title),
        "=".repeat(title.len()),
    ];
    for panel in panels {
        if !panel.is_empty() {
            lines.push(String::new());
            lines.extend(panel);
        }
    }
    lines.join("\n")
}

pub(crate) fn print_page(
    title: &str,
    panels: impl IntoIterator<Item = Vec<String>>,
    context: RenderContext,
) {
    println!("{}", render_page(title, panels, context));
}

pub(crate) fn render_metadata(title: Option<&str>, rows: &[(&str, String)]) -> Vec<String> {
    if rows.is_empty() {
        return Vec::new();
    }
    let width = rows.iter().map(|(label, _)| label.len()).max().unwrap_or(0);
    let mut lines = title.map(section_heading).into_iter().collect::<Vec<_>>();
    lines.extend(
        rows.iter()
            .map(|(label, value)| format!("{label:<width$}: {value}")),
    );
    lines
}

pub(crate) fn render_text_section(title: &str, body: Option<&str>) -> Vec<String> {
    match body.map(str::trim).filter(|body| !body.is_empty()) {
        Some(body) => vec![section_heading(title), body.to_string()],
        None => Vec::new(),
    }
}

pub(crate) fn render_lines_section(
    title: &str,
    lines: impl IntoIterator<Item = impl Into<String>>,
) -> Vec<String> {
    let mut lines = lines.into_iter().map(Into::into).collect::<Vec<_>>();
    if lines.is_empty() {
        lines.push("(none)".to_string());
    }
    if !title.is_empty() {
        lines.insert(0, section_heading(title));
    }
    lines
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

pub(crate) struct IssueHeadingRow<'a> {
    pub indent: &'a str,
    pub id: &'a str,
    pub issue_type: Option<&'a str>,
    /// The exact stored status; its category selects the colour.
    pub status: &'a str,
    pub status_category: &'a str,
    pub priority: &'a str,
    pub title: &'a str,
}

impl IssueHeadingRow<'_> {
    pub(crate) fn render(&self, context: RenderContext) -> Vec<String> {
        let kind = self
            .issue_type
            .map(|issue_type| format!("  {issue_type}"))
            .unwrap_or_default();
        let wide_text = format!(
            "{}{}{kind}  {}  {}  {}",
            self.indent, self.id, self.status, self.priority, self.title
        );
        if visible_width(&wide_text) <= context.width() {
            return vec![format!(
                "{}{}{kind}  {}  {}  {}",
                self.indent,
                context.paint(TextStyle::Heading, self.id),
                context.paint(status_style(self.status_category), self.status),
                context.paint(priority_style(self.priority), self.priority),
                self.title
            )];
        }

        let detail_indent = format!("{}  ", self.indent);
        let mut lines = vec![format!(
            "{}{}{kind}",
            self.indent,
            context.paint(TextStyle::Heading, self.id)
        )];
        lines.extend(render_labeled_lines(
            context,
            &detail_indent,
            "Status",
            self.status,
            status_style(self.status_category),
        ));
        lines.extend(render_labeled_lines(
            context,
            &detail_indent,
            "Priority",
            self.priority,
            priority_style(self.priority),
        ));
        lines.extend(render_labeled_lines(
            context,
            &detail_indent,
            "Title",
            self.title,
            TextStyle::Secondary,
        ));
        lines
    }
}

pub(crate) fn render_labeled_lines(
    context: RenderContext,
    indent: &str,
    label: &str,
    value: &str,
    value_style: TextStyle,
) -> Vec<String> {
    let wide_text = format!("{indent}{label}: {value}");
    if visible_width(&wide_text) <= context.width() {
        return vec![format!(
            "{indent}{label}: {}",
            context.paint(value_style, value)
        )];
    }

    let value_indent = format!("{indent}  ");
    let mut lines = vec![format!("{indent}{label}:")];
    lines.extend(render_wrapped_lines(
        context,
        &value_indent,
        value,
        value_style,
    ));
    lines
}

pub(crate) fn render_wrapped_lines(
    context: RenderContext,
    indent: &str,
    text: &str,
    style: TextStyle,
) -> Vec<String> {
    let available = context.width().saturating_sub(visible_width(indent)).max(1);
    wrap_words(text, available)
        .into_iter()
        .map(|line| format!("{indent}{}", context.paint(style, line)))
        .collect()
}

fn wrap_words(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    for word in text.split_whitespace() {
        if visible_width(word) > width {
            if !current.is_empty() {
                lines.push(current);
                current = String::new();
            }
            let mut chunks = hard_wrap_token(word, width).into_iter().peekable();
            while let Some(chunk) = chunks.next() {
                if chunks.peek().is_some() {
                    lines.push(chunk);
                } else {
                    current = chunk;
                }
            }
            continue;
        }
        let separator = usize::from(!current.is_empty());
        if !current.is_empty() && visible_width(&current) + separator + visible_width(word) > width
        {
            lines.push(current);
            current = String::new();
        }
        if !current.is_empty() {
            current.push(' ');
        }
        current.push_str(word);
    }
    if !current.is_empty() {
        lines.push(current);
    }
    if lines.is_empty() {
        lines.push(String::new());
    }
    lines
}

fn hard_wrap_token(token: &str, width: usize) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut chunk = String::new();
    let mut chunk_width = 0;
    for character in token.chars() {
        if chunk_width == width {
            chunks.push(chunk);
            chunk = String::new();
            chunk_width = 0;
        }
        chunk.push(character);
        chunk_width += 1;
    }
    if !chunk.is_empty() {
        chunks.push(chunk);
    }
    chunks
}

fn visible_width(text: &str) -> usize {
    text.chars().count()
}

pub(crate) struct IssueListPanel {
    title: String,
    rows: Vec<IssueListRow>,
    limit: usize,
}

impl IssueListPanel {
    pub(crate) fn from_work_rows(
        title: impl Into<String>,
        rows: &[atelier_app::work_classification::WorkRow],
    ) -> Self {
        Self::new(
            title,
            rows.iter()
                .map(|row| IssueListRow {
                    role: DisplayRole::for_work(row.state_label()),
                    id: row.id.clone(),
                    status: Some(row.status.clone()),
                    priority: row.priority.clone(),
                    title: row.title.clone(),
                    blockers: row.open_blockers.len(),
                    depth: 1,
                })
                .collect(),
        )
    }
    pub(crate) fn new(title: impl Into<String>, rows: Vec<IssueListRow>) -> Self {
        Self {
            title: title.into(),
            rows,
            limit: 20,
        }
    }

    pub(crate) fn limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }
}

impl IssueListPanel {
    pub(crate) fn render(&self, context: RenderContext) -> Vec<String> {
        let mut lines = vec![section_heading(&self.title)];
        if self.rows.is_empty() {
            lines.push("(none)".to_string());
            return lines;
        }
        let limit = self.limit.min(self.rows.len());
        for row in self.rows.iter().take(limit) {
            let indent = "  ".repeat(row.depth.max(1));
            let role = row.role.render(context.style_policy());
            lines.push(format!(
                "{indent}{role} {}",
                issue_row_text(
                    &row.id,
                    row.status.as_deref(),
                    &row.priority,
                    &row.title,
                    row.blockers
                )
            ));
        }
        let omitted = self.rows.len().saturating_sub(limit);
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

/// `<id> [<status>] <priority> - <title>`, with a blocker count when there
/// are blockers: the issue row of issue lists and unassigned work.
fn issue_row_text(
    id: &str,
    status: Option<&str>,
    priority: &str,
    title: &str,
    blockers: usize,
) -> String {
    let status = status
        .map(|status| format!(" [{status}]"))
        .unwrap_or_default();
    let blockers = if blockers == 0 {
        String::new()
    } else {
        format!(" | {blockers} blocker{}", plural_suffix(blockers))
    };
    format!("{id}{status} {priority} - {title}{blockers}")
}

/// Items shown in each label group of unassigned work.
const UNASSIGNED_SHOWN: usize = 5;

/// `<id> [<status>] <close date> - <title>` for the most recently closed
/// mission, named by `status` and `work missions` between missions.
pub(crate) fn last_closed_mission(mission: &atelier_core::Issue) -> String {
    format!(
        "{} [{}] {} - {}",
        mission.id,
        mission.status,
        mission
            .closed_at
            .unwrap_or(mission.updated_at)
            .with_timezone(&chrono::Local)
            .format("%Y-%m-%d"),
        mission.title
    )
}

/// Unassigned open work by label, for both `status` and `work missions`: a
/// `<label> (<count>)` line per group, then up to [`UNASSIGNED_SHOWN`] issue
/// rows in the shared work order and a count of the rest. A row too wide for
/// the terminal continues on lines indented past the rows, so it never reads
/// as a new item. Groups past `group_limit` are counted, not shown.
pub(crate) fn render_unassigned_groups(
    groups: &[atelier_app::mission_overview::UnassignedGroup],
    indent: &str,
    group_limit: usize,
    context: RenderContext,
) -> Vec<String> {
    let row_indent = format!("{indent}  ");
    let continuation_indent = format!("{row_indent}    ");
    let mut lines = Vec::new();
    for group in groups.iter().take(group_limit) {
        lines.push(format!(
            "{indent}{} ({})",
            group.label.as_deref().unwrap_or("no label"),
            group.work.len()
        ));
        for index in crate::commands::work_order::ordered_work_indices(&group.work)
            .into_iter()
            .take(UNASSIGNED_SHOWN)
        {
            let (issue, work) = &group.work[index];
            let row = issue_row_text(
                &issue.id,
                Some(&issue.status),
                &issue.priority,
                &issue.title,
                work.open_blockers.len(),
            );
            let width = context
                .width()
                .saturating_sub(visible_width(&continuation_indent))
                .max(1);
            for (number, line) in wrap_words(&row, width).into_iter().enumerate() {
                let indent = if number == 0 {
                    &row_indent
                } else {
                    &continuation_indent
                };
                lines.push(format!("{indent}{line}"));
            }
        }
        if group.work.len() > UNASSIGNED_SHOWN {
            lines.push(format!(
                "{row_indent}... {} more",
                group.work.len() - UNASSIGNED_SHOWN
            ));
        }
    }
    let hidden = groups.len().saturating_sub(group_limit);
    if hidden > 0 {
        lines.push(format!(
            "{indent}... {hidden} more label{}: atelier work missions",
            plural_suffix(hidden)
        ));
    }
    lines
}

pub(crate) fn render_issue_inventory(
    view: &IssueInventoryView<'_>,
    context: RenderContext,
) -> String {
    let headings = ["ID", "Type", "Status", "Priority"];
    fn fields(record: &atelier_records::CanonicalIssueRecord) -> [&str; 4] {
        [
            record.issue.id.as_str(),
            record.issue.issue_type.as_str(),
            record.issue.status.as_str(),
            record.issue.priority.as_str(),
        ]
    }
    let mut widths = headings.map(str::len);
    for record in &view.rows {
        for (width, value) in widths.iter_mut().zip(fields(record)) {
            *width = (*width).max(value.len());
        }
    }
    let columns = |values: [&str; 4]| {
        values
            .into_iter()
            .zip(widths)
            .map(|(value, width)| format!("{value:<width$}"))
            .collect::<Vec<_>>()
    };
    let mut lines = if view.matching_count == 0 {
        vec!["No issues match the selected filters.".to_string()]
    } else {
        vec![context.paint(
            TextStyle::Heading,
            format!("{}  Title", columns(headings).join("  ")),
        )]
    };
    for record in &view.rows {
        let issue = &record.issue;
        let mut row = columns(fields(record));
        let category = view
            .policy
            .and_then(|policy| policy.status_category(&issue.status));
        row[2] = context.paint(status_style(category.unwrap_or_default()), &row[2]);
        row[3] = context.paint(priority_style(&issue.priority), &row[3]);
        lines.push(format!("{}  {}", row.join("  "), issue.title));
    }
    if view.matching_count > 0 {
        lines.push(String::new());
        lines.push(format!(
            "Showing {} of {} issues",
            view.rows.len(),
            view.matching_count
        ));
        if view.rows.len() < view.matching_count {
            lines.push("Narrow the metadata filters or raise --limit.".to_string());
        }
    }
    render_page(
        "Issue Inventory",
        [
            render_lines_section("", lines),
            render_footer_section(
                "Drill Down",
                [FooterAction::new(
                    "Inspect a record",
                    "atelier issue show <issue-id>",
                )],
            ),
        ],
        context,
    )
}

pub(crate) fn render_issue_inventory_quiet(view: &IssueInventoryView<'_>) -> String {
    view.rows
        .iter()
        .map(|record| record.issue.id.as_str())
        .collect::<Vec<_>>()
        .join("\n")
}

pub(crate) fn render_footer_section(
    title: &str,
    actions: impl IntoIterator<Item = FooterAction>,
) -> Vec<String> {
    render_footer(title, actions)
        .lines()
        .map(str::to_string)
        .collect()
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

pub(crate) fn status_style(category: &str) -> TextStyle {
    match category {
        "active" => TextStyle::Active,
        "blocked" => TextStyle::Danger,
        "todo" => TextStyle::Warning,
        "done" => TextStyle::Success,
        _ => TextStyle::Secondary,
    }
}

pub(crate) fn priority_style(priority: &str) -> TextStyle {
    match priority {
        "critical" | "high" => TextStyle::Danger,
        "medium" => TextStyle::Warning,
        _ => TextStyle::Secondary,
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
pub(crate) enum DisplayRole {
    Executable,
    Selectable,
    Blocked,
    ContextOnly,
}

impl DisplayRole {
    pub(crate) fn for_work(label: &str) -> Self {
        match label {
            "active" => Self::Executable,
            "ready" => Self::Selectable,
            "blocked" => Self::Blocked,
            _ => Self::ContextOnly,
        }
    }

    pub(crate) fn label(self) -> &'static str {
        match self {
            DisplayRole::Executable => "active",
            DisplayRole::Selectable => "ready",
            DisplayRole::Blocked => "blocked",
            DisplayRole::ContextOnly => "shown for context",
        }
    }

    pub(crate) fn style(self) -> TextStyle {
        match self {
            DisplayRole::Executable => TextStyle::Active,
            DisplayRole::Selectable => TextStyle::Success,
            DisplayRole::Blocked => TextStyle::Danger,
            DisplayRole::ContextOnly => TextStyle::Secondary,
        }
    }

    pub(crate) fn render(self, policy: StylePolicy) -> String {
        policy.paint(self.style(), self.label())
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
}

pub(crate) fn heading(title: &str) -> String {
    format!("{title}\n{}", "=".repeat(title.len()))
}

pub(crate) fn section_heading(title: &str) -> String {
    format!("{title}\n{}", "-".repeat(title.len()))
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
            lines.push(format!("  {}: {}", action.label, action.command));
        }
    }
    lines.join("\n")
}

pub(crate) fn path_summary(paths: &[String], sample_limit: usize) -> String {
    if paths.is_empty() {
        return "clean".to_string();
    }
    let shown = paths.len().min(sample_limit);
    let sample = &paths[..shown];
    let omitted = paths.len() - shown;
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

    struct InventoryFixture {
        records: Vec<atelier_records::CanonicalIssueRecord>,
        matching_count: usize,
        policy: atelier_app::workflow_policy::WorkflowPolicy,
    }

    impl InventoryFixture {
        fn view(&self) -> IssueInventoryView<'_> {
            IssueInventoryView {
                rows: self.records.iter().collect(),
                matching_count: self.matching_count,
                policy: Some(&self.policy),
            }
        }
    }

    fn inventory_view(
        records: Vec<atelier_records::CanonicalIssueRecord>,
        matching_count: usize,
    ) -> InventoryFixture {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir(dir.path().join(".atelier")).unwrap();
        std::fs::write(
            dir.path().join(".atelier/workflow.yaml"),
            atelier_app::workflow_policy::STARTER_POLICY_YAML,
        )
        .unwrap();
        InventoryFixture {
            records,
            matching_count,
            policy: atelier_app::workflow_policy::load(dir.path()).unwrap(),
        }
    }

    fn inventory_row(
        id: &str,
        issue_type: &str,
        status: &str,
        priority: &str,
        title: &str,
    ) -> atelier_records::CanonicalIssueRecord {
        let now = chrono::Utc::now();
        atelier_records::CanonicalIssueRecord {
            issue: atelier_core::Issue {
                id: id.to_string(),
                issue_type: issue_type.to_string(),
                status: status.to_string(),
                priority: priority.to_string(),
                title: title.to_string(),
                description: None,
                strategy: None,
                workflow_branch: None,
                parent_id: None,
                created_at: now,
                updated_at: now,
                closed_at: None,
            },
            labels: Vec::new(),
            sections: atelier_records::IssueSections::for_type(issue_type, None),
            relationships: Default::default(),
        }
    }

    fn strip_test_ansi(value: &str) -> String {
        [
            "\u{1b}[1m",
            "\u{1b}[2m",
            "\u{1b}[31m",
            "\u{1b}[32m",
            "\u{1b}[33m",
            "\u{1b}[36m",
        ]
        .into_iter()
        .fold(value.to_string(), |value, code| value.replace(code, ""))
        .replace("\u{1b}[0m", "")
    }

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
    fn decision_labels_are_colorless_by_default() {
        let policy = StylePolicy::plain();
        assert_eq!(DecisionState::Allowed.render(policy), "allowed");
        assert_eq!(DecisionState::Fail.render(policy), "fail");
    }

    #[test]
    fn colorless_display_roles_keep_text_meaning() {
        let policy = StylePolicy::plain();
        assert_eq!(DisplayRole::Selectable.render(policy), "ready");
        assert_eq!(DisplayRole::Blocked.render(policy), "blocked");
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
    fn path_summary_bounds_samples() {
        let paths = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        assert_eq!(path_summary(&paths, 2), "3 paths: a, b, 1 more omitted");
    }

    #[test]
    fn page_skips_empty_panels_and_renders_metadata() {
        assert_eq!(
            render_page(
                "Example",
                [
                    render_metadata(None, &[("Status", "ready".to_string())]),
                    render_text_section("Notes", None)
                ],
                RenderContext::plain(),
            ),
            "Example\n=======\n\nStatus: ready"
        );
    }

    #[test]
    fn sections_preserve_distinct_empty_and_multiline_contracts() {
        assert!(render_metadata(Some("Metadata"), &[]).is_empty());
        assert!(render_text_section("Text", Some(" \n\t")).is_empty());
        assert_eq!(
            render_lines_section("Lines", Vec::<String>::new()),
            ["Lines\n-----", "(none)"]
        );
        assert_eq!(render_footer_section("Footer", []), ["Footer", "------"]);
        assert_eq!(
            render_text_section("Text", Some(" \nfirst\nsecond\n ")),
            ["Text\n----", "first\nsecond"]
        );
        assert_eq!(
            render_metadata(
                Some("Metadata"),
                &[
                    ("É", "héllo\ncontinued".to_string()),
                    ("Long", "value".to_string())
                ]
            ),
            [
                "Metadata\n--------",
                "É   : héllo\ncontinued",
                "Long: value"
            ]
        );
        assert_eq!(
            render_footer_section(
                "Footer",
                [
                    FooterAction::new("Inspect", "first\nsecond"),
                    FooterAction::new("Inspect", "first\nsecond"),
                    FooterAction::new("Other", "first\nsecond")
                ]
            ),
            [
                "Footer",
                "------",
                "  Inspect: first",
                "second",
                "  Other: first",
                "second"
            ]
        );
        assert_eq!(
            render_page(
                "É",
                [render_text_section("Text", None)],
                RenderContext::from_parts(ColorChoice::Always, true, false)
            ),
            "\x1b[1mÉ\x1b[0m\n=="
        );
    }

    #[test]
    fn issue_heading_row_keeps_wide_shape_and_decomposes_at_narrow_width() {
        let row = IssueHeadingRow {
            indent: "  ",
            id: "atelier-4fip",
            issue_type: Some("epic"),
            status: "in_progress",
            status_category: "active",
            priority: "high",
            title: "Build the formatted Mission Overview",
        };

        assert_eq!(
            row.render(RenderContext::plain()),
            vec!["  atelier-4fip  epic  in_progress  high  Build the formatted Mission Overview"]
        );

        let narrow = row.render(RenderContext::from_parts_with_width(
            ColorChoice::Never,
            false,
            false,
            40,
        ));
        assert_eq!(
            narrow,
            vec![
                "  atelier-4fip  epic",
                "    Status: in_progress",
                "    Priority: high",
                "    Title:",
                "      Build the formatted Mission",
                "      Overview",
            ]
        );
        assert!(narrow.iter().all(|line| line.chars().count() <= 40));
    }

    #[test]
    fn wrapped_lines_split_a_token_longer_than_the_available_width_without_loss() {
        let token = "x".repeat(100);
        let lines = render_wrapped_lines(
            RenderContext::from_parts_with_width(ColorChoice::Never, false, false, 40),
            "    ",
            &token,
            TextStyle::Secondary,
        );

        assert_eq!(
            lines
                .iter()
                .map(|line| line.chars().count())
                .collect::<Vec<_>>(),
            vec![40, 40, 32]
        );
        assert_eq!(
            lines
                .iter()
                .map(|line| line.trim_start())
                .collect::<String>(),
            token
        );
    }

    #[test]
    fn issue_list_panel_bounds_rows_and_keeps_colorless_roles() {
        let mut rows = vec![
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
        let mut third = rows[1].clone();
        third.id = "atelier-c".into();
        rows.push(third);
        let panel = IssueListPanel::new("Ready Work", rows).limit(1);
        assert_eq!(
            panel.render(RenderContext::plain()).join("\n"),
            "Ready Work\n----------\n  ready atelier-a [todo] high - A\n  ... 2 more ready work items omitted"
        );
    }

    #[test]
    fn issue_inventory_renders_one_flat_metadata_row_per_issue() {
        let view = inventory_view(
            vec![
                inventory_row(
                    "atelier-a",
                    "mission",
                    "in_progress",
                    "high",
                    "Coordinate the mission",
                ),
                inventory_row(
                    "atelier-b",
                    "epic",
                    "todo",
                    "critical",
                    "Own a parent slice",
                ),
                inventory_row(
                    "atelier-c",
                    "task",
                    "blocked",
                    "medium",
                    "A linked child record",
                ),
                inventory_row(
                    "atelier-d",
                    "bug",
                    "done",
                    "low",
                    "A completed standalone record",
                ),
            ],
            4,
        );

        let output = render_issue_inventory(&view.view(), RenderContext::plain());

        assert!(output.starts_with("Issue Inventory\n===============\n\nID"));
        assert!(output.contains("atelier-a  mission  in_progress  high"));
        assert!(output.contains("atelier-b  epic     todo         critical"));
        assert!(output.contains("atelier-c  task     blocked      medium"));
        assert!(output.contains("atelier-d  bug      done         low"));
        assert!(output.contains("Showing 4 of 4 issues"));
        assert!(output
            .contains("Drill Down\n----------\n  Inspect a record: atelier issue show <issue-id>"));
        assert!(!output.contains("Standalone"));
        assert!(!output.contains("blocker"));
        assert!(!output.contains("ready"));
        assert!(!output.contains("Queue"));
        for id in ["atelier-a", "atelier-b", "atelier-c", "atelier-d"] {
            assert_eq!(
                output.lines().filter(|line| line.starts_with(id)).count(),
                1,
                "{id} should appear exactly once as an unindented row"
            );
        }
    }

    #[test]
    fn issue_inventory_reports_truncation_and_filter_guidance() {
        let view = inventory_view(
            vec![inventory_row(
                "atelier-a",
                "task",
                "todo",
                "medium",
                "Visible",
            )],
            3,
        );

        let output = render_issue_inventory(&view.view(), RenderContext::plain());

        assert!(output.contains("Showing 1 of 3 issues"));
        assert!(output.contains("Narrow the metadata filters or raise --limit."));
    }

    #[test]
    fn issue_inventory_empty_and_quiet_states_are_explicit() {
        let view = inventory_view(Vec::new(), 0);

        let human = render_issue_inventory(&view.view(), RenderContext::plain());
        assert!(human.contains("No issues match the selected filters."));
        assert!(!human.contains("Showing 0 of 0 issues"));
        assert_eq!(render_issue_inventory_quiet(&view.view()), "");
    }

    #[test]
    fn issue_inventory_quiet_uses_view_order_and_contains_only_ids() {
        let view = inventory_view(
            vec![
                inventory_row("atelier-a", "task", "todo", "high", "First"),
                inventory_row("atelier-b", "epic", "done", "low", "Second"),
            ],
            4,
        );

        assert_eq!(
            render_issue_inventory_quiet(&view.view()),
            "atelier-a\natelier-b"
        );
    }

    #[test]
    fn issue_inventory_color_changes_style_only() {
        let view = inventory_view(
            vec![inventory_row(
                "atelier-a",
                "task",
                "blocked",
                "critical",
                "Needs attention",
            )],
            1,
        );
        let plain = render_issue_inventory(&view.view(), RenderContext::plain());
        let colored = render_issue_inventory(
            &view.view(),
            RenderContext::from_parts(ColorChoice::Always, true, false),
        );
        let no_color = render_issue_inventory(
            &view.view(),
            RenderContext::from_parts(ColorChoice::Always, true, true),
        );

        assert!(colored.contains("\u{1b}[31mblocked"));
        assert_eq!(strip_test_ansi(&colored), plain);
        assert_eq!(no_color, plain);
    }
}
