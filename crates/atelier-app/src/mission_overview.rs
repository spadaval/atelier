//! Persistence-neutral Mission Overview projection.
//!
//! The acquisition adapter supplies directed mission membership and canonical
//! issue facts. This module deliberately does not infer `advances` direction,
//! query SQLite, or render terminal output.

use std::collections::{BTreeMap, BTreeSet};

use atelier_core::IssuePriority;

pub const MISSION_OVERVIEW_MISSION_LIMIT: usize = 20;
pub const MISSION_OVERVIEW_EPIC_LIMIT: usize = 10;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MissionOverviewRequest {
    pub include_done: bool,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MissionOverviewInput {
    pub issues: Vec<MissionOverviewIssue>,
    pub advances: Vec<MissionAdvancesRoot>,
    pub children: Vec<IssueParentChild>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionOverviewIssue {
    pub id: String,
    pub title: String,
    pub issue_type: String,
    pub status: String,
    pub status_category: String,
    pub priority: String,
    pub open_blocker_count: usize,
}

/// A directed membership fact: `mission_id` advances `root_id`.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct MissionAdvancesRoot {
    pub mission_id: String,
    pub root_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct IssueParentChild {
    pub parent_id: String,
    pub child_id: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MissionOverview {
    pub missions: Vec<MissionOverviewMission>,
    pub matching_mission_count: usize,
    pub omitted_mission_count: usize,
    pub outside_visible_missions: OutsideVisibleMissions,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionOverviewMission {
    pub id: String,
    pub title: String,
    pub status: String,
    pub status_category: String,
    pub priority: String,
    pub open_blocker_count: usize,
    pub progress: MissionOverviewCounts,
    pub epics: Vec<MissionOverviewEpic>,
    pub matching_epic_count: usize,
    pub omitted_epic_count: usize,
    pub direct_work: DirectWorkSummary,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionOverviewEpic {
    pub id: String,
    pub title: String,
    pub status: String,
    pub status_category: String,
    pub priority: String,
    pub open_blocker_count: usize,
    pub descendant_count: usize,
    pub descendant_progress: MissionOverviewCounts,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DirectWorkSummary {
    pub root_count: usize,
    pub root_progress: MissionOverviewCounts,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MissionOverviewCounts {
    pub total: usize,
    pub active: usize,
    pub blocked: usize,
    pub todo: usize,
    pub done: usize,
    pub unknown: usize,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OutsideVisibleMissions {
    pub unassigned_nonterminal: usize,
    pub linked_only_to_hidden_done_missions: usize,
}

/// Projects the bounded Mission Overview from a complete in-memory snapshot.
///
/// Selection and repository-wide exceptional-work facts are computed before
/// display budgets are applied. Relationships that reference missing records
/// are ignored, allowing an acquisition adapter to pass a concurrently read
/// snapshot without manufacturing placeholder rows.
pub fn project_mission_overview(
    request: MissionOverviewRequest,
    input: &MissionOverviewInput,
) -> MissionOverview {
    let issues = input
        .issues
        .iter()
        .map(|issue| (issue.id.as_str(), issue))
        .collect::<BTreeMap<_, _>>();
    let children = child_index(&input.children);
    let roots_by_mission = directed_roots(&input.advances, &issues);

    let mut all_mission_ids = issues
        .values()
        .filter(|issue| is_mission(issue))
        .map(|issue| issue.id.as_str())
        .collect::<Vec<_>>();
    all_mission_ids.sort_by(|left, right| compare_issues(issues[left], issues[right]));

    let visible_mission_ids = all_mission_ids
        .iter()
        .copied()
        .filter(|id| request.include_done || !is_done(issues[id]))
        .collect::<Vec<_>>();
    let visible_before_budget = visible_mission_ids.iter().copied().collect::<BTreeSet<_>>();

    let outside_visible_missions = outside_visible_missions(
        &issues,
        &children,
        &roots_by_mission,
        &all_mission_ids,
        &visible_before_budget,
    );
    let matching_mission_count = visible_mission_ids.len();
    let missions = visible_mission_ids
        .into_iter()
        .take(MISSION_OVERVIEW_MISSION_LIMIT)
        .map(|mission_id| {
            project_mission(
                issues[mission_id],
                &issues,
                &children,
                roots_by_mission.get(mission_id),
            )
        })
        .collect::<Vec<_>>();

    MissionOverview {
        omitted_mission_count: matching_mission_count.saturating_sub(missions.len()),
        matching_mission_count,
        missions,
        outside_visible_missions,
    }
}

fn project_mission(
    mission: &MissionOverviewIssue,
    issues: &BTreeMap<&str, &MissionOverviewIssue>,
    children: &BTreeMap<&str, BTreeSet<&str>>,
    root_ids: Option<&BTreeSet<&str>>,
) -> MissionOverviewMission {
    let root_ids = root_ids.cloned().unwrap_or_default();
    let mut scope_ids = BTreeSet::new();
    for root_id in &root_ids {
        collect_reachable(root_id, children, issues, &mut scope_ids);
    }
    let progress = counts_for_ids(scope_ids.iter().copied(), issues);

    let direct_root_ids = root_ids
        .iter()
        .copied()
        .filter(|id| issues.get(id).is_some_and(|issue| !is_mission(issue)))
        .collect::<Vec<_>>();
    let direct_work_ids = direct_root_ids
        .iter()
        .copied()
        .filter(|id| {
            issues
                .get(id)
                .is_some_and(|issue| issue.issue_type != "epic")
        })
        .collect::<Vec<_>>();
    let direct_work = DirectWorkSummary {
        root_count: direct_work_ids.len(),
        root_progress: counts_for_ids(direct_work_ids, issues),
    };

    let mut epic_ids = direct_root_ids
        .into_iter()
        .filter(|id| {
            issues
                .get(id)
                .is_some_and(|issue| issue.issue_type == "epic")
        })
        .collect::<Vec<_>>();
    epic_ids.sort_by(|left, right| compare_issues(issues[left], issues[right]));
    let matching_epic_count = epic_ids.len();
    let epics = epic_ids
        .into_iter()
        .take(MISSION_OVERVIEW_EPIC_LIMIT)
        .map(|epic_id| project_epic(issues[epic_id], issues, children))
        .collect::<Vec<_>>();

    MissionOverviewMission {
        id: mission.id.clone(),
        title: mission.title.clone(),
        status: mission.status.clone(),
        status_category: mission.status_category.clone(),
        priority: mission.priority.clone(),
        open_blocker_count: mission.open_blocker_count,
        progress,
        omitted_epic_count: matching_epic_count.saturating_sub(epics.len()),
        matching_epic_count,
        epics,
        direct_work,
    }
}

fn project_epic(
    epic: &MissionOverviewIssue,
    issues: &BTreeMap<&str, &MissionOverviewIssue>,
    children: &BTreeMap<&str, BTreeSet<&str>>,
) -> MissionOverviewEpic {
    let mut descendant_ids = BTreeSet::new();
    if let Some(child_ids) = children.get(epic.id.as_str()) {
        for child_id in child_ids {
            collect_reachable(child_id, children, issues, &mut descendant_ids);
        }
    }
    descendant_ids.remove(epic.id.as_str());
    let descendant_progress = counts_for_ids(descendant_ids.iter().copied(), issues);

    MissionOverviewEpic {
        id: epic.id.clone(),
        title: epic.title.clone(),
        status: epic.status.clone(),
        status_category: epic.status_category.clone(),
        priority: epic.priority.clone(),
        open_blocker_count: epic.open_blocker_count,
        descendant_count: descendant_progress.total,
        descendant_progress,
    }
}

fn outside_visible_missions(
    issues: &BTreeMap<&str, &MissionOverviewIssue>,
    children: &BTreeMap<&str, BTreeSet<&str>>,
    roots_by_mission: &BTreeMap<&str, BTreeSet<&str>>,
    all_mission_ids: &[&str],
    visible_mission_ids: &BTreeSet<&str>,
) -> OutsideVisibleMissions {
    let mut owners_by_issue = BTreeMap::<&str, BTreeSet<&str>>::new();
    for mission_id in all_mission_ids {
        let mut scope_ids = BTreeSet::new();
        if let Some(root_ids) = roots_by_mission.get(mission_id) {
            for root_id in root_ids {
                collect_reachable(root_id, children, issues, &mut scope_ids);
            }
        }
        for issue_id in scope_ids {
            if issues.get(issue_id).is_some_and(|issue| !is_mission(issue)) {
                owners_by_issue
                    .entry(issue_id)
                    .or_default()
                    .insert(mission_id);
            }
        }
    }

    let mut outside = OutsideVisibleMissions::default();
    for issue in issues.values() {
        if is_mission(issue) || is_done(issue) {
            continue;
        }
        let Some(owner_ids) = owners_by_issue.get(issue.id.as_str()) else {
            outside.unassigned_nonterminal += 1;
            continue;
        };
        if owner_ids.is_disjoint(visible_mission_ids) {
            outside.linked_only_to_hidden_done_missions += 1;
        }
    }
    outside
}

fn directed_roots<'a>(
    advances: &'a [MissionAdvancesRoot],
    issues: &BTreeMap<&'a str, &'a MissionOverviewIssue>,
) -> BTreeMap<&'a str, BTreeSet<&'a str>> {
    let mut roots = BTreeMap::<&str, BTreeSet<&str>>::new();
    for link in advances {
        if issues
            .get(link.mission_id.as_str())
            .is_some_and(|issue| is_mission(issue))
            && issues.contains_key(link.root_id.as_str())
        {
            roots
                .entry(link.mission_id.as_str())
                .or_default()
                .insert(link.root_id.as_str());
        }
    }
    roots
}

fn child_index(children: &[IssueParentChild]) -> BTreeMap<&str, BTreeSet<&str>> {
    let mut index = BTreeMap::<&str, BTreeSet<&str>>::new();
    for edge in children {
        index
            .entry(edge.parent_id.as_str())
            .or_default()
            .insert(edge.child_id.as_str());
    }
    index
}

fn collect_reachable<'a>(
    issue_id: &'a str,
    children: &BTreeMap<&'a str, BTreeSet<&'a str>>,
    issues: &BTreeMap<&'a str, &'a MissionOverviewIssue>,
    found: &mut BTreeSet<&'a str>,
) {
    if !issues.contains_key(issue_id) || !found.insert(issue_id) {
        return;
    }
    if let Some(child_ids) = children.get(issue_id) {
        for child_id in child_ids {
            collect_reachable(child_id, children, issues, found);
        }
    }
}

fn counts_for_ids<'a>(
    issue_ids: impl IntoIterator<Item = &'a str>,
    issues: &BTreeMap<&'a str, &'a MissionOverviewIssue>,
) -> MissionOverviewCounts {
    let mut counts = MissionOverviewCounts::default();
    for issue_id in issue_ids {
        let Some(issue) = issues.get(issue_id) else {
            continue;
        };
        if is_mission(issue) {
            continue;
        }
        counts.total += 1;
        match work_state(issue) {
            WorkState::Active => counts.active += 1,
            WorkState::Blocked => counts.blocked += 1,
            WorkState::Todo => counts.todo += 1,
            WorkState::Done => counts.done += 1,
            WorkState::Unknown => counts.unknown += 1,
        }
    }
    counts
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WorkState {
    Active,
    Blocked,
    Todo,
    Done,
    Unknown,
}

fn work_state(issue: &MissionOverviewIssue) -> WorkState {
    match issue.status_category.as_str() {
        "active" => WorkState::Active,
        "done" => WorkState::Done,
        "blocked" => WorkState::Blocked,
        _ if issue.open_blocker_count > 0 => WorkState::Blocked,
        "todo" => WorkState::Todo,
        _ => WorkState::Unknown,
    }
}

fn is_mission(issue: &MissionOverviewIssue) -> bool {
    issue.issue_type == "mission"
}

fn is_done(issue: &MissionOverviewIssue) -> bool {
    issue.status_category == "done"
}

fn compare_issues(left: &MissionOverviewIssue, right: &MissionOverviewIssue) -> std::cmp::Ordering {
    category_rank(&left.status_category)
        .cmp(&category_rank(&right.status_category))
        .then_with(|| priority_rank(&left.priority).cmp(&priority_rank(&right.priority)))
        .then_with(|| left.id.cmp(&right.id))
}

fn category_rank(category: &str) -> u8 {
    match category {
        "active" => 0,
        "blocked" => 1,
        "todo" => 2,
        "done" => 3,
        _ => 4,
    }
}

fn priority_rank(priority: &str) -> u8 {
    IssuePriority::from_label(priority)
        .or_else(|_| IssuePriority::from_canonical_token(priority))
        .map(IssuePriority::sort_rank)
        .unwrap_or(4)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn issue(
        id: impl Into<String>,
        issue_type: &str,
        category: &str,
        priority: &str,
    ) -> MissionOverviewIssue {
        let id = id.into();
        MissionOverviewIssue {
            title: format!("Title for {id}"),
            id,
            issue_type: issue_type.to_string(),
            status: category.to_string(),
            status_category: category.to_string(),
            priority: priority.to_string(),
            open_blocker_count: 0,
        }
    }

    fn link(mission_id: &str, root_id: &str) -> MissionAdvancesRoot {
        MissionAdvancesRoot {
            mission_id: mission_id.to_string(),
            root_id: root_id.to_string(),
        }
    }

    fn child(parent_id: &str, child_id: &str) -> IssueParentChild {
        IssueParentChild {
            parent_id: parent_id.to_string(),
            child_id: child_id.to_string(),
        }
    }

    #[test]
    fn projects_directed_epics_direct_work_descendants_and_blocker_state() {
        let mut mission = issue("mission-a", "mission", "active", "high");
        mission.open_blocker_count = 2;
        let mut epic = issue("epic-a", "epic", "todo", "high");
        epic.open_blocker_count = 1;
        let mut blocked = issue("task-blocked", "task", "todo", "low");
        blocked.open_blocker_count = 1;
        let input = MissionOverviewInput {
            issues: vec![
                mission,
                epic,
                issue("task-active", "task", "active", "medium"),
                blocked,
                issue("task-done", "task", "done", "low"),
                issue("direct-a", "task", "todo", "critical"),
                issue("parent-only", "epic", "todo", "medium"),
            ],
            advances: vec![link("mission-a", "epic-a"), link("mission-a", "direct-a")],
            children: vec![
                child("epic-a", "task-active"),
                child("epic-a", "task-blocked"),
                child("task-blocked", "task-done"),
                // Parentage alone must not create mission membership or an epic row.
                child("mission-a", "parent-only"),
            ],
        };

        let overview = project_mission_overview(MissionOverviewRequest::default(), &input);

        assert_eq!(overview.matching_mission_count, 1);
        let mission = &overview.missions[0];
        assert_eq!(mission.progress.total, 5);
        assert_eq!(mission.progress.active, 1);
        assert_eq!(mission.progress.blocked, 2);
        assert_eq!(mission.progress.todo, 1);
        assert_eq!(mission.progress.done, 1);
        assert_eq!(mission.matching_epic_count, 1);
        assert_eq!(mission.epics[0].id, "epic-a");
        assert_eq!(mission.open_blocker_count, 2);
        assert_eq!(mission.epics[0].open_blocker_count, 1);
        assert_eq!(mission.epics[0].descendant_count, 3);
        assert_eq!(mission.epics[0].descendant_progress.blocked, 1);
        assert_eq!(mission.epics[0].descendant_progress.done, 1);
        assert_eq!(mission.direct_work.root_count, 1);
        assert_eq!(mission.direct_work.root_progress.todo, 1);
        assert_eq!(overview.outside_visible_missions.unassigned_nonterminal, 1);
    }

    #[test]
    fn deduplicates_overlapping_roots_per_mission_but_preserves_shared_membership() {
        let input = MissionOverviewInput {
            issues: vec![
                issue("mission-a", "mission", "active", "high"),
                issue("mission-b", "mission", "todo", "medium"),
                issue("epic-a", "epic", "todo", "high"),
                issue("task-a", "task", "todo", "medium"),
            ],
            advances: vec![
                link("mission-a", "epic-a"),
                link("mission-a", "task-a"),
                link("mission-b", "epic-a"),
                link("mission-b", "epic-a"),
            ],
            children: vec![child("epic-a", "task-a")],
        };

        let overview = project_mission_overview(MissionOverviewRequest::default(), &input);

        assert_eq!(overview.missions.len(), 2);
        assert_eq!(overview.missions[0].id, "mission-a");
        assert_eq!(overview.missions[0].progress.total, 2);
        assert_eq!(overview.missions[0].direct_work.root_count, 1);
        assert_eq!(overview.missions[1].id, "mission-b");
        assert_eq!(overview.missions[1].progress.total, 2);
        assert_eq!(overview.missions[1].epics[0].id, "epic-a");
        assert_eq!(
            overview.outside_visible_missions,
            OutsideVisibleMissions::default()
        );
    }

    #[test]
    fn excludes_done_missions_and_reports_exceptional_work_until_all_is_requested() {
        let input = MissionOverviewInput {
            issues: vec![
                issue("mission-active", "mission", "active", "high"),
                issue("mission-done", "mission", "done", "high"),
                issue("shared", "task", "todo", "medium"),
                issue("hidden", "task", "active", "medium"),
                issue("hidden-done", "task", "done", "medium"),
                issue("unassigned", "task", "todo", "medium"),
            ],
            advances: vec![
                link("mission-active", "shared"),
                link("mission-done", "shared"),
                link("mission-done", "hidden"),
                link("mission-done", "hidden-done"),
            ],
            children: vec![],
        };

        let default = project_mission_overview(MissionOverviewRequest::default(), &input);
        assert_eq!(
            default
                .missions
                .iter()
                .map(|mission| mission.id.as_str())
                .collect::<Vec<_>>(),
            vec!["mission-active"]
        );
        assert_eq!(
            default.outside_visible_missions,
            OutsideVisibleMissions {
                unassigned_nonterminal: 1,
                linked_only_to_hidden_done_missions: 1,
            }
        );

        let all = project_mission_overview(MissionOverviewRequest { include_done: true }, &input);
        assert_eq!(
            all.missions
                .iter()
                .map(|mission| mission.id.as_str())
                .collect::<Vec<_>>(),
            vec!["mission-active", "mission-done"]
        );
        assert_eq!(all.outside_visible_missions.unassigned_nonterminal, 1);
        assert_eq!(
            all.outside_visible_missions
                .linked_only_to_hidden_done_missions,
            0
        );
    }

    #[test]
    fn orders_rows_deterministically_and_applies_budgets_after_counting() {
        let ordered_issues = vec![
            issue("mission-active-low", "mission", "active", "low"),
            issue("mission-active-high-z", "mission", "active", "high"),
            issue("mission-active-high-a", "mission", "active", "high"),
            issue("mission-blocked", "mission", "blocked", "critical"),
            issue("mission-todo", "mission", "todo", "critical"),
            issue("mission-done", "mission", "done", "critical"),
            issue("mission-unknown", "mission", "paused", "unknown"),
        ];
        let ordered = project_mission_overview(
            MissionOverviewRequest { include_done: true },
            &MissionOverviewInput {
                issues: ordered_issues.clone(),
                advances: vec![],
                children: vec![],
            },
        );
        assert_eq!(
            ordered
                .missions
                .iter()
                .map(|mission| mission.id.as_str())
                .collect::<Vec<_>>(),
            vec![
                "mission-active-high-a",
                "mission-active-high-z",
                "mission-active-low",
                "mission-blocked",
                "mission-todo",
                "mission-done",
                "mission-unknown",
            ]
        );

        let mut issues = ordered_issues;
        for index in 0..16 {
            issues.push(issue(
                format!("mission-extra-{index:02}"),
                "mission",
                "todo",
                "medium",
            ));
        }
        issues.push(issue("budget-owned", "task", "todo", "medium"));
        for index in 0..11 {
            issues.push(issue(format!("epic-{index:02}"), "epic", "todo", "medium"));
        }
        let mut advances = (0..11)
            .map(|index| link("mission-active-high-a", &format!("epic-{index:02}")))
            .collect::<Vec<_>>();
        advances.push(link("mission-extra-15", "budget-owned"));
        let input = MissionOverviewInput {
            issues,
            advances,
            children: vec![],
        };

        let default = project_mission_overview(MissionOverviewRequest::default(), &input);
        assert_eq!(default.matching_mission_count, 22);
        assert_eq!(default.omitted_mission_count, 2);
        assert_eq!(
            default.outside_visible_missions,
            OutsideVisibleMissions::default()
        );

        let overview =
            project_mission_overview(MissionOverviewRequest { include_done: true }, &input);

        assert_eq!(overview.matching_mission_count, 23);
        assert_eq!(overview.missions.len(), MISSION_OVERVIEW_MISSION_LIMIT);
        assert_eq!(overview.omitted_mission_count, 3);
        assert_eq!(
            overview.outside_visible_missions,
            OutsideVisibleMissions::default()
        );
        let first = &overview.missions[0];
        assert_eq!(first.matching_epic_count, 11);
        assert_eq!(first.epics.len(), MISSION_OVERVIEW_EPIC_LIMIT);
        assert_eq!(first.omitted_epic_count, 1);
        assert_eq!(first.epics[0].id, "epic-00");
        assert_eq!(first.epics[9].id, "epic-09");
    }

    #[test]
    fn cyclic_hierarchy_is_bounded_and_missing_or_reversed_links_do_not_assign_work() {
        let input = MissionOverviewInput {
            issues: vec![
                issue("mission-a", "mission", "active", "high"),
                issue("task-a", "task", "todo", "medium"),
                issue("task-b", "task", "todo", "medium"),
            ],
            advances: vec![
                link("mission-a", "task-a"),
                // Explicit direction matters: a non-mission source is not ownership.
                link("task-b", "mission-a"),
                link("missing-mission", "task-b"),
            ],
            children: vec![child("task-a", "task-b"), child("task-b", "task-a")],
        };

        let overview = project_mission_overview(MissionOverviewRequest::default(), &input);

        assert_eq!(overview.missions[0].progress.total, 2);
        assert_eq!(overview.missions[0].direct_work.root_count, 1);
        assert_eq!(
            overview.outside_visible_missions,
            OutsideVisibleMissions::default()
        );
    }
}
