// This module is the shared formatter boundary for the output refresh.

use std::collections::{BTreeMap, BTreeSet};
use std::io::IsTerminal;

use atelier_app::issue_inventory::IssueInventoryView;
use atelier_app::issue_read::ObjectiveBucketTotals;
use atelier_app::work_classification::{WorkClassification, WorkRow};

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
        "=".repeat(display_width(title)),
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
    pub id: String,
    pub status: String,
    pub status_category: Option<String>,
    pub classification: WorkClassification,
    pub priority: String,
    pub title: String,
    pub blockers: usize,
    pub depth: usize,
}

/// What leads an item line in a list that mixes kinds of rows.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Lead<'a> {
    /// The row's work state, in a list that mixes states.
    State(WorkClassification),
    /// The row's relationship to the record shown, such as `advances`.
    Role(&'a str),
}

/// One item as every view writes it:
/// `[<lead> ]<id>[ <type>] <status> <priority> <title>[ | <cue>]...`.
///
/// Each field comes from one fixed vocabulary, rendered in one fixed order,
/// so a view chooses which fields to fill rather than writing its own
/// format. The status is the exact configured status (or, where a view
/// shows it instead, the work state), marked ✓ or ⚠ as [`status_cue`] says.
/// The type is set only in views that mix types. Nothing is bracketed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ItemLine<'a> {
    lead: Option<Lead<'a>>,
    id: &'a str,
    issue_type: Option<&'a str>,
    /// Owned, so a view may pass a label it builds, such as one naming an
    /// unknown category.
    status: String,
    category: Option<&'a str>,
    classification: Option<WorkClassification>,
    priority: &'a str,
    title: &'a str,
    cues: Vec<String>,
}

impl<'a> ItemLine<'a> {
    pub(crate) fn new(id: &'a str, status: &str, priority: &'a str, title: &'a str) -> Self {
        Self {
            lead: None,
            id,
            issue_type: None,
            status: status.to_owned(),
            category: None,
            classification: None,
            priority,
            title,
            cues: Vec::new(),
        }
    }

    /// The status's category and the row's work state, which choose the
    /// status mark and colour.
    pub(crate) fn state(
        mut self,
        category: Option<&'a str>,
        classification: Option<WorkClassification>,
    ) -> Self {
        self.category = category;
        self.classification = classification;
        self
    }

    pub(crate) fn lead(mut self, lead: Lead<'a>) -> Self {
        self.lead = Some(lead);
        self
    }

    /// The type, for views that mix types.
    pub(crate) fn issue_type(mut self, issue_type: &'a str) -> Self {
        self.issue_type = Some(issue_type);
        self
    }

    /// A trailing `| <cue>`, such as a blocker count.
    pub(crate) fn cue(mut self, cue: impl Into<String>) -> Self {
        self.cues.push(cue.into());
        self
    }

    /// `| 1 blocker` or `| N blockers` when there are any.
    pub(crate) fn blockers(self, count: usize) -> Self {
        if count == 0 {
            self
        } else {
            self.cue(count_noun(count, "blocker"))
        }
    }

    /// The line without decoration, as redirected output and page titles
    /// receive it.
    pub(crate) fn text(&self) -> String {
        self.render(RenderContext::plain())
    }

    pub(crate) fn render(&self, context: RenderContext) -> String {
        self.fields()
            .iter()
            .map(|field| field.paint(context, &field.text))
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// The line at `indent`. A line too wide for the terminal continues on
    /// lines indented past the rows, so it never reads as a new item. It
    /// breaks only between fields or title words, so a marked status and
    /// each `| <cue>` stay whole unless one alone is wider than the
    /// terminal, and each piece keeps the colour [`ItemLine::render`] gives
    /// it.
    pub(crate) fn render_wrapped(&self, context: RenderContext, indent: &str) -> Vec<String> {
        let text = self.text();
        if display_width(indent) + display_width(&text) <= context.width() {
            return vec![format!("{indent}{}", self.render(context))];
        }
        let continuation = format!("{indent}    ");
        let width = context
            .width()
            .saturating_sub(display_width(&continuation))
            .max(1);
        let fields = self.fields();
        let mut pieces = Vec::new();
        for field in &fields {
            let tokens = if field.breakable {
                field.text.split_whitespace().map(str::to_owned).collect()
            } else {
                vec![field.text.clone()]
            };
            for token in tokens {
                if display_width(&token) > width {
                    pieces.extend(
                        wrap_words(&token, width)
                            .into_iter()
                            .map(|part| (field, part)),
                    );
                } else {
                    pieces.push((field, token));
                }
            }
        }
        let mut lines: Vec<(usize, String)> = Vec::new();
        for (field, piece) in pieces {
            let piece_width = display_width(&piece);
            let painted = field.paint(context, &piece);
            match lines.last_mut() {
                Some((used, line)) if *used + 1 + piece_width <= width => {
                    *used += 1 + piece_width;
                    line.push(' ');
                    line.push_str(&painted);
                }
                _ => lines.push((piece_width, painted)),
            }
        }
        lines
            .into_iter()
            .enumerate()
            .map(|(number, (_, line))| {
                let indent = if number == 0 { indent } else { &continuation };
                format!("{indent}{line}")
            })
            .collect()
    }

    /// The line's fields in order, each with the style it is painted in.
    fn fields(&self) -> Vec<ItemField> {
        let mut fields = Vec::new();
        match self.lead {
            Some(Lead::State(state)) => {
                fields.push(ItemField::new(
                    state.label(),
                    Some(classification_style(state)),
                ));
            }
            Some(Lead::Role(role)) => fields.push(ItemField::new(role, None)),
            None => {}
        }
        fields.push(ItemField::new(self.id, None));
        if let Some(issue_type) = self.issue_type {
            fields.push(ItemField::new(issue_type, None));
        }
        let status_style = match (self.category, self.classification) {
            (Some(category), _) => status_style(category),
            (None, Some(state)) => classification_style(state),
            (None, None) => TextStyle::Secondary,
        };
        fields.push(ItemField::new(
            status_cue(&self.status, self.category, self.classification),
            Some(status_style),
        ));
        fields.push(ItemField::new(
            self.priority,
            Some(priority_style(self.priority)),
        ));
        fields.push(ItemField {
            breakable: true,
            ..ItemField::new(self.title, None)
        });
        fields.extend(
            self.cues
                .iter()
                .map(|cue| ItemField::new(format!("| {cue}"), None)),
        );
        fields
    }
}

/// One field of an [`ItemLine`]: its text, the style it is painted in, and
/// whether a wrap may break it between words, as it may the title.
struct ItemField {
    text: String,
    style: Option<TextStyle>,
    breakable: bool,
}

impl ItemField {
    fn new(text: impl Into<String>, style: Option<TextStyle>) -> Self {
        Self {
            text: text.into(),
            style,
            breakable: false,
        }
    }

    fn paint(&self, context: RenderContext, text: &str) -> String {
        match self.style {
            Some(style) => context.paint(style, text),
            None => text.to_owned(),
        }
    }
}

/// A commit as views show it: its first 8 characters. Output that must be
/// passed back to a command, such as `--resolution-commit`, keeps the full
/// hash.
pub(crate) fn short_commit(commit: &str) -> &str {
    commit.get(..8).unwrap_or(commit)
}

/// A timestamp as views show it: local time, `YYYY-MM-DD HH:MM ±hh:mm`.
/// Records keep RFC 3339.
pub(crate) fn display_timestamp(timestamp: chrono::DateTime<chrono::Utc>) -> String {
    display_timestamp_in(timestamp, &chrono::Local)
}

fn display_timestamp_in<Zone: chrono::TimeZone>(
    timestamp: chrono::DateTime<chrono::Utc>,
    zone: &Zone,
) -> String
where
    Zone::Offset: std::fmt::Display,
{
    timestamp
        .with_timezone(zone)
        .format("%Y-%m-%d %H:%M %:z")
        .to_string()
}

/// The local day of a timestamp, the day [`display_timestamp`] shows. Notes
/// and prune cutoffs are dated by it.
pub(crate) fn local_date(timestamp: chrono::DateTime<chrono::Utc>) -> chrono::NaiveDate {
    local_date_in(timestamp, &chrono::Local)
}

fn local_date_in<Zone: chrono::TimeZone>(
    timestamp: chrono::DateTime<chrono::Utc>,
    zone: &Zone,
) -> chrono::NaiveDate {
    timestamp.with_timezone(zone).date_naive()
}

pub(crate) fn render_labeled_lines(
    context: RenderContext,
    indent: &str,
    label: &str,
    value: &str,
    value_style: TextStyle,
) -> Vec<String> {
    let wide_text = format!("{indent}{label}: {value}");
    if display_width(&wide_text) <= context.width() {
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
    let available = context.width().saturating_sub(display_width(indent)).max(1);
    wrap_words(text, available)
        .into_iter()
        .map(|line| format!("{indent}{}", context.paint(style, line)))
        .collect()
}

fn wrap_words(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    for word in text.split_whitespace() {
        if display_width(word) > width {
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
        if !current.is_empty() && display_width(&current) + separator + display_width(word) > width
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

/// The columns `text` takes: one per character. Headings size their
/// underlines by it, not by bytes, so `é`, `✓` and `—` count once.
pub(crate) fn display_width(text: &str) -> usize {
    text.chars().count()
}

pub(crate) struct IssueListPanel {
    title: String,
    rows: Vec<IssueListRow>,
    limit: usize,
    /// The command that shows every row, named when rows are omitted; only
    /// [`Self::limit`] omits rows, and it sets this.
    rest: String,
    /// Rows lead with their work state, for panels whose title does not
    /// name one state.
    row_states: bool,
    /// A `↳` continuation under a row, keyed by issue ID.
    marks: BTreeMap<String, String>,
}

impl IssueListPanel {
    pub(crate) fn from_work_rows(title: impl Into<String>, rows: &[WorkRow]) -> Self {
        Self::new(
            title,
            rows.iter()
                .map(|row| IssueListRow {
                    id: row.id.clone(),
                    status: row.status.clone(),
                    status_category: row.status_category.clone(),
                    classification: row.classification,
                    priority: row.priority.clone(),
                    title: row.title.clone(),
                    blockers: row.open_blockers.len(),
                    depth: 1,
                })
                .collect(),
        )
    }

    /// A panel titled by one work state, such as Ready Work, so its rows do
    /// not repeat the state; see [`Self::row_states`] for mixed panels.
    pub(crate) fn new(title: impl Into<String>, rows: Vec<IssueListRow>) -> Self {
        Self {
            title: title.into(),
            rows,
            limit: usize::MAX,
            rest: String::new(),
            row_states: false,
            marks: BTreeMap::new(),
        }
    }

    /// Shows at most `limit` rows; the line counting the rest names `rest`,
    /// the command that shows them all.
    pub(crate) fn limit(mut self, limit: usize, rest: impl Into<String>) -> Self {
        self.limit = limit;
        self.rest = rest.into();
        self
    }

    /// Lead each row with its work state, unless its status word already
    /// says it: for panels that mix states.
    pub(crate) fn row_states(mut self) -> Self {
        self.row_states = true;
        self
    }

    /// Notes such as another branch's state, each shown under its row.
    pub(crate) fn marks(mut self, marks: &BTreeMap<String, String>) -> Self {
        self.marks = self
            .rows
            .iter()
            .filter_map(|row| Some((row.id.clone(), marks.get(&row.id)?.clone())))
            .collect();
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
            let mut line = ItemLine::new(&row.id, &row.status, &row.priority, &row.title)
                .state(row.status_category.as_deref(), Some(row.classification))
                .blockers(row.blockers);
            if self.row_states && row.classification.label() != row.status {
                line = line.lead(Lead::State(row.classification));
            }
            lines.extend(line.render_wrapped(context, &indent));
            if let Some(mark) = self.marks.get(&row.id) {
                lines.push(mark_line(&indent, mark));
            }
        }
        let omitted = self.rows.len().saturating_sub(limit);
        if omitted > 0 {
            lines.push(format!(
                "  ... {} omitted; see `{}`",
                count_noun(omitted, &format!("more {} item", self.title.to_lowercase())),
                self.rest
            ));
        }
        lines
    }
}

/// The mark before a status word that reads without a legend: ✓ when its
/// category is done, ⚠ when its category or work state is blocked.
fn state_symbol(
    category: Option<&str>,
    classification: Option<WorkClassification>,
) -> Option<char> {
    match (category, classification) {
        (Some("done"), _) | (_, Some(WorkClassification::Done)) => Some('✓'),
        (Some("blocked"), _) | (_, Some(WorkClassification::Blocked)) => Some('⚠'),
        _ => None,
    }
}

/// `word`, with its [`state_symbol`] before it when it has one.
pub(crate) fn status_cue(
    word: &str,
    category: Option<&str>,
    classification: Option<WorkClassification>,
) -> String {
    match state_symbol(category, classification) {
        Some(symbol) => format!("{symbol} {word}"),
        None => word.to_string(),
    }
}

fn classification_style(classification: WorkClassification) -> TextStyle {
    match classification {
        WorkClassification::Active => TextStyle::Active,
        WorkClassification::Ready => TextStyle::Success,
        WorkClassification::Blocked => TextStyle::Danger,
        _ => TextStyle::Secondary,
    }
}

/// A note under the row at `indent`, such as another branch's state.
fn mark_line(indent: &str, mark: &str) -> String {
    format!("{indent}  ↳ {mark}")
}

/// Items shown in each label group of unassigned work.
const UNASSIGNED_SHOWN: usize = 5;

/// The most recently closed mission, named by `status` and `work missions`
/// between missions: its item line, marked done, with a `| closed <date>`
/// cue.
pub(crate) fn last_closed_mission(mission: &atelier_core::Issue) -> String {
    ItemLine::new(
        &mission.id,
        &mission.status,
        &mission.priority,
        &mission.title,
    )
    .state(Some("done"), None)
    .cue(format!(
        "closed {}",
        local_date(mission.closed_at.unwrap_or(mission.updated_at))
    ))
    .text()
}

/// Unassigned open work by label, for both `status` and `work missions`: a
/// `<label> (<count>)` line per group, then up to [`UNASSIGNED_SHOWN`] issue
/// rows in the shared work order and a count of the rest. A row too wide for
/// the terminal continues on lines indented past the rows, so it never reads
/// as a new item. A row with an entry in `marks` (another branch's state)
/// gets the same `↳` line as issue lists. Groups past `group_limit` are
/// counted, not shown.
pub(crate) fn render_unassigned_groups(
    groups: &[atelier_app::mission_overview::UnassignedGroup],
    marks: &BTreeMap<String, String>,
    indent: &str,
    group_limit: usize,
    context: RenderContext,
) -> Vec<String> {
    let row_indent = format!("{indent}  ");
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
            lines.extend(
                ItemLine::new(&issue.id, &issue.status, &issue.priority, &issue.title)
                    .state(work.status_category.as_deref(), Some(work.classification))
                    .blockers(work.open_blockers.len())
                    .render_wrapped(context, &row_indent),
            );
            if let Some(mark) = marks.get(&issue.id) {
                lines.push(mark_line(&row_indent, mark));
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
            "{indent}... {}: atelier work missions",
            count_noun(hidden, "more label")
        ));
    }
    lines
}

pub(crate) fn render_issue_inventory(
    view: &IssueInventoryView<'_>,
    context: RenderContext,
) -> String {
    let mut lines = if view.matching_count == 0 {
        vec!["No issues match the selected filters.".to_string()]
    } else {
        Vec::new()
    };
    for record in &view.rows {
        let issue = &record.issue;
        let category = view
            .policy
            .and_then(|policy| policy.status_category(&issue.status));
        lines.push(
            ItemLine::new(&issue.id, &issue.status, &issue.priority, &issue.title)
                .issue_type(&issue.issue_type)
                .state(category, None)
                .render(context),
        );
    }
    if view.matching_count > 0 {
        lines.push(String::new());
        lines.push(format!(
            "Showing {} of {}",
            view.rows.len(),
            count_noun(view.matching_count, "issue")
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
                context,
                view.rows.first().map(|record| {
                    FooterAction::new(
                        "Inspect the first record",
                        format!("atelier issue show {}", record.issue.id),
                    )
                }),
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

/// The footer as page-panel lines; empty when there is no action to offer.
pub(crate) fn render_footer_section(
    context: RenderContext,
    actions: impl IntoIterator<Item = FooterAction>,
) -> Vec<String> {
    render_footer(context, actions)
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

/// One next step in a footer, rendered `  <label>: <command>`.
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

/// A page title: one per command output.
pub(crate) fn heading(title: &str) -> String {
    format!("{title}\n{}", "=".repeat(display_width(title)))
}

pub(crate) fn section_heading(title: &str) -> String {
    format!("{title}\n{}", "-".repeat(display_width(title)))
}

pub(crate) fn print_section_heading(title: &str) {
    println!("{}", section_heading(title));
}

/// The heading every next-step footer uses.
pub(crate) const FOOTER_HEADING: &str = "Next";

/// The next-step footer every view ends with: a `Next` section of
/// `  <label>: <command>` lines, or nothing when there is no action to offer.
/// A line too wide for the terminal continues indented under its label.
pub(crate) fn render_footer(
    context: RenderContext,
    actions: impl IntoIterator<Item = FooterAction>,
) -> String {
    let mut seen = BTreeSet::new();
    let mut lines = Vec::new();
    for action in actions {
        if seen.insert((action.label.clone(), action.command.clone())) {
            lines.extend(render_labeled_lines(
                context,
                "  ",
                &action.label,
                &action.command,
                TextStyle::Secondary,
            ));
        }
    }
    if lines.is_empty() {
        return String::new();
    }
    lines.insert(0, section_heading(FOOTER_HEADING));
    lines.join("\n")
}

/// Prints the footer, preceded by a blank line, when there is an action.
pub(crate) fn print_footer(actions: impl IntoIterator<Item = FooterAction>) {
    let footer = render_footer(RenderContext::for_stdout(), actions);
    if !footer.is_empty() {
        println!("\n{footer}");
    }
}

/// The checkout's changed paths, up to `sample_limit` of them; the rest are
/// counted and left to `git status`.
pub(crate) fn path_summary(paths: &[String], sample_limit: usize) -> String {
    if paths.is_empty() {
        return "clean".to_string();
    }
    let shown = paths.len().min(sample_limit);
    let sample = &paths[..shown];
    let omitted = paths.len() - shown;
    let mut summary = count_noun(paths.len(), "path");
    if !sample.is_empty() {
        summary.push_str(": ");
        summary.push_str(&sample.join(", "));
    }
    if omitted > 0 {
        summary.push_str(&format!(", {omitted} more omitted; see `git status`"));
    }
    summary
}

pub(crate) use atelier_core::count_noun;

/// Counts of things as one phrase, count first in the order given, leaving
/// out zeros: `2 open blockers, 1 other link`. Empty when every count is
/// zero.
pub(crate) fn noun_counts<'a>(counts: impl IntoIterator<Item = (usize, &'a str)>) -> String {
    counts
        .into_iter()
        .filter(|(count, _)| *count > 0)
        .map(|(count, noun)| count_noun(count, noun))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Work counts by work state, count first in the shared work order, leaving
/// out states with none: `1 active, 2 ready, 1 needs a human, 3 backlog`.
/// Empty when every count is zero.
pub(crate) fn work_state_counts(counts: &ObjectiveBucketTotals) -> String {
    let human = if counts.human == 1 {
        "needs a human"
    } else {
        "need a human"
    };
    [
        (counts.active, "active"),
        (counts.ready, "ready"),
        (counts.human, human),
        (counts.context, "context"),
        (counts.blocked, "blocked"),
        (counts.done, "done"),
        (counts.backlog, "backlog"),
    ]
    .into_iter()
    .filter(|(count, _)| *count > 0)
    .map(|(count, state)| format!("{count} {state}"))
    .collect::<Vec<_>>()
    .join(", ")
}

/// A scope's size and its [`work_state_counts`]: `3 issues: 1 ready,
/// 2 backlog`, or `0 issues` for an empty scope.
pub(crate) fn scope_counts(total: usize, counts: &ObjectiveBucketTotals) -> String {
    let states = work_state_counts(counts);
    if states.is_empty() {
        count_noun(total, "issue")
    } else {
        format!("{}: {states}", count_noun(total, "issue"))
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
    fn status_cues_mark_done_and_blocked_next_to_the_status_word() {
        assert_eq!(
            status_cue("done", Some("done"), Some(WorkClassification::Done)),
            "✓ done"
        );
        assert_eq!(status_cue("superseded", Some("done"), None), "✓ superseded");
        assert_eq!(
            status_cue(
                "blocked",
                Some("blocked"),
                Some(WorkClassification::Blocked)
            ),
            "⚠ blocked"
        );
        // A todo item held by blockers is marked by its work state.
        assert_eq!(
            status_cue("todo", Some("todo"), Some(WorkClassification::Blocked)),
            "⚠ todo"
        );
        // A blocked status keeps its mark under a parked ancestor.
        assert_eq!(
            status_cue(
                "blocked",
                Some("blocked"),
                Some(WorkClassification::Backlog)
            ),
            "⚠ blocked"
        );
        for classification in [
            WorkClassification::Active,
            WorkClassification::Ready,
            WorkClassification::NeedsHuman,
            WorkClassification::Context,
            WorkClassification::Backlog,
        ] {
            assert_eq!(
                status_cue("todo", Some("todo"), Some(classification)),
                "todo"
            );
        }
    }

    #[test]
    fn counts_read_count_first_in_one_order_without_zeros() {
        let counts = ObjectiveBucketTotals {
            active: 1,
            ready: 2,
            human: 1,
            context: 0,
            blocked: 0,
            done: 3,
            backlog: 1,
        };
        assert_eq!(
            work_state_counts(&counts),
            "1 active, 2 ready, 1 needs a human, 3 done, 1 backlog"
        );
        assert_eq!(
            scope_counts(8, &counts),
            "8 issues: 1 active, 2 ready, 1 needs a human, 3 done, 1 backlog"
        );
        let two_for_the_human = ObjectiveBucketTotals {
            human: 2,
            context: 1,
            ..Default::default()
        };
        assert_eq!(
            scope_counts(3, &two_for_the_human),
            "3 issues: 2 need a human, 1 context"
        );
        assert_eq!(
            scope_counts(0, &ObjectiveBucketTotals::default()),
            "0 issues"
        );
        assert_eq!(
            noun_counts([(0, "advances link"), (2, "open blocker"), (1, "other link")]),
            "2 open blockers, 1 other link"
        );
        assert_eq!(noun_counts([(0, "other link")]), "");
    }

    /// Every count gets a real plural from `count_noun`, so no source in the
    /// workspace hedges one with an `s` in parentheses.
    #[test]
    fn no_source_hedges_a_plural_in_parentheses() {
        fn rust_files(dir: &std::path::Path, files: &mut Vec<std::path::PathBuf>) {
            for entry in std::fs::read_dir(dir).unwrap() {
                let path = entry.unwrap().path();
                if path.is_dir() {
                    rust_files(&path, files);
                } else if path.extension().is_some_and(|ext| ext == "rs") {
                    files.push(path);
                }
            }
        }
        let crates = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        let mut files = Vec::new();
        for entry in std::fs::read_dir(&crates).unwrap() {
            let crate_dir = entry.unwrap().path();
            for part in ["src", "tests"] {
                if crate_dir.join(part).is_dir() {
                    rust_files(&crate_dir.join(part), &mut files);
                }
            }
        }
        assert!(files.len() > 20, "found too few sources: {files:?}");
        let mut hedged = Vec::new();
        for file in &files {
            let text = std::fs::read_to_string(file).unwrap();
            for (number, line) in text.lines().enumerate() {
                let hedges = line.match_indices("(s)").any(|(at, _)| {
                    line[..at]
                        .chars()
                        .next_back()
                        .is_some_and(char::is_alphabetic)
                });
                if hedges {
                    hedged.push(format!("{}:{}: {line}", file.display(), number + 1));
                }
            }
        }
        assert!(
            hedged.is_empty(),
            "plurals hedged as `(s)`:\n{}",
            hedged.join("\n")
        );
    }

    #[test]
    fn footer_actions_are_deduplicated() {
        let footer = render_footer(
            RenderContext::plain(),
            [
                FooterAction::new("Inspect", "atelier issue show atelier-1234"),
                FooterAction::new("Inspect", "atelier issue show atelier-1234"),
                FooterAction::new("Validate", "atelier check atelier-1234"),
            ],
        );
        assert_eq!(
            footer,
            "Next\n----\n  Inspect: atelier issue show atelier-1234\n  Validate: atelier check atelier-1234"
        );
    }

    #[test]
    fn path_summary_bounds_samples() {
        let paths = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        assert_eq!(
            path_summary(&paths, 2),
            "3 paths: a, b, 1 more omitted; see `git status`"
        );
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
        assert!(render_footer_section(RenderContext::plain(), []).is_empty());
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
                RenderContext::plain(),
                [
                    FooterAction::new("Inspect", "first\nsecond"),
                    FooterAction::new("Inspect", "first\nsecond"),
                    FooterAction::new("Other", "first\nsecond")
                ]
            ),
            [
                "Next",
                "----",
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
            "\x1b[1mÉ\x1b[0m\n="
        );
    }

    #[test]
    fn item_lines_share_one_field_order_without_brackets() {
        let plain = ItemLine::new("atelier-a", "todo", "high", "Write the contract");
        assert_eq!(plain.text(), "atelier-a todo high Write the contract");
        assert_eq!(
            ItemLine::new("atelier-a", "todo", "high", "Write the contract")
                .issue_type("task")
                .lead(Lead::Role("advances"))
                .state(Some("todo"), Some(WorkClassification::Blocked))
                .blockers(2)
                .cue("not merged into master")
                .text(),
            "advances atelier-a task ⚠ todo high Write the contract | 2 blockers | not merged into master"
        );
        assert_eq!(
            ItemLine::new("atelier-a", "in_progress", "low", "Title")
                .lead(Lead::State(WorkClassification::Active))
                .state(Some("active"), Some(WorkClassification::Active))
                .text(),
            "active atelier-a in_progress low Title"
        );
        assert_eq!(
            ItemLine::new("atelier-a", "done", "low", "Title")
                .state(Some("done"), None)
                .blockers(0)
                .text(),
            "atelier-a ✓ done low Title"
        );
        let line =
            ItemLine::new("atelier-a", "blocked", "critical", "Title").state(Some("blocked"), None);
        let colored = line.render(RenderContext::from_parts(ColorChoice::Always, true, false));
        assert!(colored.contains("\u{1b}[31m⚠ blocked\u{1b}[0m"));
        assert_eq!(strip_test_ansi(&colored), line.text());
    }

    #[test]
    fn item_lines_too_wide_continue_past_the_row_indent() {
        let line = ItemLine::new(
            "atelier-4fip",
            "in_progress",
            "high",
            "Build the formatted Mission Overview",
        );
        assert_eq!(
            line.render_wrapped(RenderContext::plain(), "  "),
            ["  atelier-4fip in_progress high Build the formatted Mission Overview"]
        );
        let narrow = line.render_wrapped(
            RenderContext::from_parts_with_width(ColorChoice::Never, false, false, 40),
            "  ",
        );
        assert_eq!(
            narrow,
            [
                "  atelier-4fip in_progress high",
                "      Build the formatted Mission",
                "      Overview",
            ]
        );
        assert!(narrow.iter().all(|line| line.chars().count() <= 40));
    }

    #[test]
    fn narrow_coloured_item_lines_keep_colour_and_whole_tokens() {
        let line = ItemLine::new("atelier-4fip", "todo", "high", "Build the Mission Overview")
            .state(Some("todo"), Some(WorkClassification::Blocked))
            .blockers(1)
            .cue("not merged into master");
        let colour =
            |width| RenderContext::from_parts_with_width(ColorChoice::Always, true, false, width);

        // Each field keeps its colour, and `⚠ todo` and each `| <cue>` stay
        // whole, wherever the line breaks.
        for width in 12..=80 {
            let lines = line.render_wrapped(colour(width), "  ");
            let plain = lines
                .iter()
                .map(|line| strip_test_ansi(line))
                .collect::<Vec<_>>();
            assert!(
                lines
                    .iter()
                    .any(|line| line.contains("\u{1b}[33m⚠ todo\u{1b}[0m")),
                "width {width}: {lines:?}"
            );
            assert!(
                lines
                    .iter()
                    .any(|line| line.contains("\u{1b}[31mhigh\u{1b}[0m")),
                "width {width}: {lines:?}"
            );
            if width >= 30 {
                for token in ["| 1 blocker", "| not merged into master"] {
                    assert!(
                        plain.iter().any(|line| line.contains(token)),
                        "width {width}: {plain:?}"
                    );
                }
            }
            assert!(
                plain.iter().all(|line| line.chars().count() <= width),
                "width {width}: {plain:?}"
            );
            // Nothing is lost: a hard-wrapped token only gains a break.
            let characters = |text: &str| {
                text.chars()
                    .filter(|c| !c.is_whitespace())
                    .collect::<String>()
            };
            assert_eq!(characters(&plain.concat()), characters(&line.text()));
        }

        assert_eq!(
            line.render_wrapped(colour(40), "  "),
            [
                "  atelier-4fip \u{1b}[33m⚠ todo\u{1b}[0m \u{1b}[31mhigh\u{1b}[0m Build the",
                "      Mission Overview | 1 blocker",
                "      | not merged into master",
            ]
        );

        // A cue wider than the terminal alone breaks between its words.
        let narrow = line.render_wrapped(
            RenderContext::from_parts_with_width(ColorChoice::Never, false, false, 24),
            "",
        );
        assert!(
            narrow.contains(&"    | not merged into".to_string()),
            "{narrow:?}"
        );
    }

    #[test]
    fn commits_and_timestamps_have_one_display_form() {
        assert_eq!(
            short_commit("5ec6c235d4b1a0e9f8c7b6a5d4c3b2a1f0e9d8c7"),
            "5ec6c235"
        );
        assert_eq!(short_commit("abc"), "abc");
        let timestamp = chrono::DateTime::parse_from_rfc3339("2026-10-17T00:31:24.389518170Z")
            .unwrap()
            .with_timezone(&chrono::Utc);
        let eastern = chrono::FixedOffset::west_opt(4 * 3600).unwrap();
        assert_eq!(
            display_timestamp_in(timestamp, &eastern),
            "2026-10-16 20:31 -04:00"
        );
        assert_eq!(
            display_timestamp_in(timestamp, &chrono::Utc),
            "2026-10-17 00:31 +00:00"
        );
    }

    #[test]
    fn notes_are_dated_by_the_local_day_just_after_utc_midnight() {
        let now = chrono::DateTime::parse_from_rfc3339("2026-10-17T00:31:24Z")
            .unwrap()
            .with_timezone(&chrono::Utc);
        let eastern = chrono::FixedOffset::west_opt(4 * 3600).unwrap();
        let date = local_date_in(now, &eastern);
        assert_eq!(date.to_string(), "2026-10-16");
        assert_eq!(
            display_timestamp_in(now, &eastern).get(..10),
            Some("2026-10-16")
        );
        let mut sections = atelier_core::IssueSections::for_type("bug", None);
        sections.append_note(date, "note", "written at 20:31 local");
        assert_eq!(
            sections.notes.as_deref(),
            Some("- 2026-10-16 [note] written at 20:31 local")
        );
    }

    #[test]
    fn headings_are_underlined_to_their_display_width() {
        assert_eq!(heading("é ✓ — x"), "é ✓ — x\n=======");
        assert_eq!(section_heading("Café"), "Café\n----");
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

    fn list_row(
        id: &str,
        status: &str,
        category: &str,
        classification: WorkClassification,
        blockers: usize,
    ) -> IssueListRow {
        IssueListRow {
            id: id.to_string(),
            status: status.to_string(),
            status_category: Some(category.to_string()),
            classification,
            priority: "medium".to_string(),
            title: id.to_uppercase(),
            blockers,
            depth: 1,
        }
    }

    #[test]
    fn issue_list_panel_bounds_rows_and_leaves_the_state_to_its_title() {
        let rows = vec![
            list_row("atelier-a", "todo", "todo", WorkClassification::Ready, 0),
            list_row("atelier-b", "todo", "todo", WorkClassification::Ready, 0),
            list_row("atelier-c", "todo", "todo", WorkClassification::Ready, 0),
        ];
        let panel = IssueListPanel::new("Ready Work", rows).limit(1, "atelier work ready");
        assert_eq!(
            panel.render(RenderContext::plain()).join("\n"),
            "Ready Work\n----------\n  atelier-a todo medium ATELIER-A\n  ... 2 more ready work items omitted; see `atelier work ready`"
        );
    }

    #[test]
    fn issue_list_panel_marks_done_and_blocked_rows_by_their_status_word() {
        let rows = vec![
            list_row("atelier-a", "done", "done", WorkClassification::Done, 0),
            list_row("atelier-b", "todo", "todo", WorkClassification::Blocked, 2),
            list_row(
                "atelier-c",
                "blocked",
                "blocked",
                WorkClassification::Blocked,
                1,
            ),
        ];
        assert_eq!(
            IssueListPanel::new("Work", rows).render(RenderContext::plain()),
            [
                "Work\n----",
                "  atelier-a ✓ done medium ATELIER-A",
                "  atelier-b ⚠ todo medium ATELIER-B | 2 blockers",
                "  atelier-c ⚠ blocked medium ATELIER-C | 1 blocker",
            ]
        );
    }

    #[test]
    fn mixed_issue_list_panels_lead_with_a_state_the_status_word_does_not_say() {
        let rows = vec![
            list_row(
                "atelier-a",
                "in_progress",
                "active",
                WorkClassification::Active,
                0,
            ),
            list_row("atelier-b", "todo", "todo", WorkClassification::Ready, 0),
            list_row(
                "atelier-c",
                "todo",
                "todo",
                WorkClassification::NeedsHuman,
                0,
            ),
            list_row(
                "atelier-d",
                "backlog",
                "todo",
                WorkClassification::Backlog,
                0,
            ),
            list_row("atelier-e", "done", "done", WorkClassification::Done, 0),
            list_row(
                "atelier-f",
                "blocked",
                "blocked",
                WorkClassification::Blocked,
                1,
            ),
        ];
        let panel = IssueListPanel::new("Scoped Work", rows).row_states();
        assert_eq!(
            panel.render(RenderContext::plain()),
            [
                "Scoped Work\n-----------",
                "  active atelier-a in_progress medium ATELIER-A",
                "  ready atelier-b todo medium ATELIER-B",
                "  needs human atelier-c todo medium ATELIER-C",
                "  atelier-d backlog medium ATELIER-D",
                "  atelier-e ✓ done medium ATELIER-E",
                "  atelier-f ⚠ blocked medium ATELIER-F | 1 blocker",
            ]
        );
        let colored = panel.render(RenderContext::from_parts(ColorChoice::Always, true, false));
        assert!(colored[1].contains("\u{1b}[36mactive\u{1b}[0m"));
        assert!(colored[5].contains(" \u{1b}[32m✓ done\u{1b}[0m "));
        assert_eq!(
            colored
                .iter()
                .map(|line| strip_test_ansi(line))
                .collect::<Vec<_>>(),
            panel.render(RenderContext::plain())
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

        assert!(output.starts_with(
            "Issue Inventory\n===============\n\natelier-a mission in_progress high Coordinate the mission\n"
        ));
        assert!(output.contains("\natelier-b epic todo critical Own a parent slice\n"));
        assert!(output.contains("\natelier-c task ⚠ blocked medium A linked child record\n"));
        assert!(output.contains("\natelier-d bug ✓ done low A completed standalone record\n"));
        assert!(output.contains("Showing 4 of 4 issues"));
        assert!(
            output.contains("Next\n----\n  Inspect the first record: atelier issue show atelier-a")
        );
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
        assert!(!human.contains("Next"), "{human}");
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

        assert!(colored.contains("\u{1b}[31m⚠ blocked"));
        assert_eq!(strip_test_ansi(&colored), plain);
        assert_eq!(no_color, plain);
    }
}
