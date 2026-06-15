use anyhow::Result;
use std::collections::BTreeSet;

/// Application-facing issue data needed to assemble status workflows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IssueSummary {
    pub id: String,
    pub title: String,
    pub status: String,
    pub issue_type: String,
    pub parent_id: Option<String>,
}

/// Application-facing mission record data needed by the status use case.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MissionSummary {
    pub id: String,
    pub title: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IssueStartReadiness {
    Ready,
    Blocked,
    NotReady,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitStatusView {
    pub branch: Option<String>,
    pub dirty_entries: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrackerState {
    Current,
    Stale { issue_count: usize },
}

impl TrackerState {
    pub fn label(self) -> &'static str {
        match self {
            TrackerState::Current => "current",
            TrackerState::Stale { .. } => "stale",
        }
    }

    pub fn stale_count(self) -> usize {
        match self {
            TrackerState::Current => 0,
            TrackerState::Stale { issue_count } => issue_count,
        }
    }
}

/// Storage and runtime dependencies for the status application use case.
///
/// Implementations may be backed by SQLite projection tables, canonical records,
/// runtime state, or test doubles. The app layer does not depend on Clap input
/// or concrete storage types.
pub trait StatusPorts {
    fn current_work_issues(&self) -> Result<Vec<IssueSummary>>;
    fn all_issues(&self) -> Result<Vec<IssueSummary>>;
    fn issue(&self, issue_id: &str) -> Result<Option<IssueSummary>>;
    fn blockers(&self, issue_id: &str) -> Result<Vec<String>>;
    fn subissues(&self, issue_id: &str) -> Result<Vec<IssueSummary>>;
    fn active_mission(&self) -> Result<Option<MissionSummary>>;
    fn current_missions(&self) -> Result<Vec<MissionSummary>>;
    fn mission_issue_ids(&self, mission_id: &str) -> Result<BTreeSet<String>>;
    fn mission_direct_blocker_ids(&self, mission_id: &str) -> Result<Vec<String>>;
    fn has_validating_evidence(&self, issue_id: &str) -> Result<bool>;
    fn issue_start_readiness(&self, issue: &IssueSummary) -> Result<IssueStartReadiness>;
    fn issue_status_category(&self, status: &str) -> Option<String>;
    fn tracker_state(&self) -> Result<TrackerState>;
    fn recent_mission_activity(&self, issue_ids: &BTreeSet<String>) -> Result<Vec<String>>;
    fn git_status(&self) -> Result<GitStatusView>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusViewModel {
    pub tracker_state: TrackerState,
    pub ready_work_count: usize,
    pub current_work: Vec<IssueSummary>,
    pub active_mission: Option<MissionSummary>,
    pub current_mission_count: usize,
    pub git: ResultView<GitStatusView>,
    pub active_mission_snapshot: Option<MissionSnapshotViewModel>,
}

impl StatusViewModel {
    pub fn quiet_line(&self) -> String {
        format!(
            "work={} active_mission={} current_missions={} ready={} tracker={}",
            match self.current_work.len() {
                0 => "none",
                1 => "current",
                _ => "multiple",
            },
            self.active_mission
                .as_ref()
                .map(|mission| mission.id.as_str())
                .unwrap_or("none"),
            self.current_mission_count,
            self.ready_work_count,
            self.tracker_state.label(),
        )
    }

    pub fn current_work_lines(&self) -> Vec<String> {
        self.current_work
            .iter()
            .map(format_current_work_line)
            .collect()
    }

    pub fn next_actions(&self) -> Vec<String> {
        next_actions(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MissionSnapshotViewModel {
    pub mission: MissionSummary,
    pub issue_ids: BTreeSet<String>,
    pub current_work: Vec<IssueSummary>,
    pub ready_issues: Vec<IssueSummary>,
    pub selectable_issues: Vec<SelectableIssueView>,
    pub open_blockers: Vec<BlockerView>,
    pub recent_activity: Vec<String>,
    pub active: usize,
    pub ready: usize,
    pub blocked: usize,
    pub done: usize,
    pub backlog: usize,
}

impl MissionSnapshotViewModel {
    pub fn health(&self) -> &'static str {
        if !self.open_blockers.is_empty() || self.blocked > 0 {
            "blocked"
        } else if self.active > 0 {
            "active"
        } else if self.ready > 0 {
            "ready"
        } else if self.done > 0 {
            "closeout"
        } else {
            "steady"
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectableIssueView {
    pub issue: IssueSummary,
    pub parent_context: String,
    pub proof_context: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockerView {
    pub id: String,
    pub title: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResultView<T> {
    Available(T),
    Unavailable(String),
}

pub fn load_status<P: StatusPorts>(ports: &P) -> Result<StatusViewModel> {
    let current_work = ports.current_work_issues()?;
    let current_work_ids = current_work
        .iter()
        .map(|issue| issue.id.as_str())
        .collect::<BTreeSet<_>>();
    let active_mission = ports.active_mission()?;
    let current_mission_count = ports.current_missions()?.len();
    let ready_work_count = ports
        .all_issues()?
        .into_iter()
        .filter(|issue| !current_work_ids.contains(issue.id.as_str()))
        .filter_map(|issue| match ports.issue_start_readiness(&issue) {
            Ok(IssueStartReadiness::Ready) => Some(Ok(())),
            Ok(_) => None,
            Err(error) => Some(Err(error)),
        })
        .collect::<Result<Vec<_>>>()?
        .len();
    let tracker_state = ports.tracker_state()?;
    let active_mission_snapshot = active_mission
        .clone()
        .map(|mission| mission_snapshot(ports, mission, &current_work_ids))
        .transpose()?;
    let git = match ports.git_status() {
        Ok(status) => ResultView::Available(status),
        Err(error) => ResultView::Unavailable(error.to_string()),
    };

    Ok(StatusViewModel {
        tracker_state,
        ready_work_count,
        current_work,
        active_mission,
        current_mission_count,
        git,
        active_mission_snapshot,
    })
}

fn mission_snapshot<P: StatusPorts>(
    ports: &P,
    mission: MissionSummary,
    current_work_ids: &BTreeSet<&str>,
) -> Result<MissionSnapshotViewModel> {
    let issue_ids = ports.mission_issue_ids(&mission.id)?;
    let mut blocker_ids = ports
        .mission_direct_blocker_ids(&mission.id)?
        .into_iter()
        .collect::<BTreeSet<_>>();
    for issue_id in &issue_ids {
        for blocker_id in ports.blockers(issue_id)? {
            blocker_ids.insert(blocker_id);
        }
    }
    let mut open_blockers = blocker_ids
        .into_iter()
        .map(|id| ports.issue(&id))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .filter(|issue| issue_blocks_work(ports, issue))
        .map(|issue| BlockerView {
            id: issue.id,
            title: issue.title,
        })
        .collect::<Vec<_>>();
    open_blockers.sort_by(|a, b| a.id.cmp(&b.id));

    let mut snapshot = MissionSnapshotViewModel {
        mission,
        issue_ids,
        current_work: Vec::new(),
        ready_issues: Vec::new(),
        selectable_issues: Vec::new(),
        open_blockers,
        recent_activity: Vec::new(),
        active: 0,
        ready: 0,
        blocked: 0,
        done: 0,
        backlog: 0,
    };

    for issue_id in &snapshot.issue_ids {
        let Some(issue) = ports.issue(issue_id)? else {
            continue;
        };
        match issue_bucket(ports, &issue, current_work_ids)? {
            IssueBucket::Active => {
                snapshot.active += 1;
                snapshot.current_work.push(issue);
            }
            IssueBucket::Ready => {
                snapshot.ready += 1;
                if is_selectable_work(ports, &issue)? {
                    snapshot.selectable_issues.push(SelectableIssueView {
                        parent_context: parent_context(&issue),
                        proof_context: proof_context(ports, &issue.id)?,
                        issue: issue.clone(),
                    });
                }
                snapshot.ready_issues.push(issue);
            }
            IssueBucket::Blocked => snapshot.blocked += 1,
            IssueBucket::Done => snapshot.done += 1,
            IssueBucket::Backlog => snapshot.backlog += 1,
        }
    }
    snapshot.ready_issues.sort_by(|a, b| a.id.cmp(&b.id));
    snapshot
        .selectable_issues
        .sort_by(|a, b| a.issue.id.cmp(&b.issue.id));
    snapshot.current_work.sort_by(|a, b| a.id.cmp(&b.id));
    snapshot.recent_activity = ports.recent_mission_activity(&snapshot.issue_ids)?;
    Ok(snapshot)
}

enum IssueBucket {
    Active,
    Ready,
    Blocked,
    Done,
    Backlog,
}

fn issue_bucket<P: StatusPorts>(
    ports: &P,
    issue: &IssueSummary,
    current_work_ids: &BTreeSet<&str>,
) -> Result<IssueBucket> {
    if current_work_ids.contains(issue.id.as_str()) {
        return Ok(IssueBucket::Active);
    }
    if issue_is_done(ports, issue) {
        return Ok(IssueBucket::Done);
    }
    if !open_issue_blockers(ports, &issue.id)?.is_empty() {
        return Ok(IssueBucket::Blocked);
    }
    match ports.issue_start_readiness(issue)? {
        IssueStartReadiness::Ready => Ok(IssueBucket::Ready),
        IssueStartReadiness::Blocked => Ok(IssueBucket::Blocked),
        IssueStartReadiness::NotReady => Ok(IssueBucket::Backlog),
    }
}

pub fn is_current_work_issue<P: StatusPorts>(ports: &P, issue: &IssueSummary) -> bool {
    ports.issue_status_category(&issue.status).as_deref() == Some("active")
        || issue.status == "in_progress"
}

fn issue_is_done<P: StatusPorts>(ports: &P, issue: &IssueSummary) -> bool {
    ports.issue_status_category(&issue.status).as_deref() == Some("done")
}

fn issue_blocks_work<P: StatusPorts>(ports: &P, issue: &IssueSummary) -> bool {
    !issue_is_done(ports, issue)
}

fn open_issue_blockers<P: StatusPorts>(ports: &P, issue_id: &str) -> Result<Vec<String>> {
    let mut blockers = Vec::new();
    for blocker_id in ports.blockers(issue_id)? {
        if let Some(issue) = ports.issue(&blocker_id)? {
            if issue_blocks_work(ports, &issue) {
                blockers.push(blocker_id);
            }
        }
    }
    blockers.sort();
    Ok(blockers)
}

fn is_selectable_work<P: StatusPorts>(ports: &P, issue: &IssueSummary) -> Result<bool> {
    Ok(issue.issue_type != "epic" || ports.subissues(&issue.id)?.is_empty())
}

fn parent_context(issue: &IssueSummary) -> String {
    match issue.parent_id.as_deref() {
        Some(parent_id) => format!("parent {parent_id}"),
        None => "mission-linked root".to_string(),
    }
}

fn proof_context<P: StatusPorts>(ports: &P, issue_id: &str) -> Result<&'static str> {
    if ports.has_validating_evidence(issue_id)? {
        Ok("proof attached")
    } else {
        Ok("proof missing")
    }
}

pub fn format_current_work_line(issue: &IssueSummary) -> String {
    format!("{} - {} [{}]", issue.id, issue.title, issue.status)
}

fn next_actions(view: &StatusViewModel) -> Vec<String> {
    let mut actions = Vec::new();
    match (&view.active_mission, &view.active_mission_snapshot) {
        (Some(mission), Some(snapshot)) => {
            actions.push(format!(
                "Inspect active mission health ({}): atelier mission status {}",
                mission.id, mission.id
            ));
            actions.push(format!(
                "Open active mission record ({}): atelier mission show {}",
                mission.id, mission.id
            ));
            if let Some(issue) = snapshot.current_work.first() {
                actions.push(format!(
                    "Inspect current work transitions ({} in progress; first {}): atelier issue transition {} --options",
                    snapshot.current_work.len(),
                    issue.id,
                    issue.id
                ));
            } else if let Some(issue) = snapshot.selectable_issues.first() {
                actions.push(format!(
                    "Start selectable active-mission work ({} selectable issue(s)): atelier start {}",
                    snapshot.selectable_issues.len(),
                    issue.issue.id
                ));
            } else if snapshot.blocked > 0 || !snapshot.open_blockers.is_empty() {
                actions.push(format!(
                    "Inspect blocked active-mission work (no ready work is available): atelier mission status {}",
                    mission.id
                ));
            } else {
                actions.push(format!(
                    "Review active mission closeout (no ready work is available): atelier mission status {}",
                    mission.id
                ));
            }
        }
        (Some(mission), None) => actions.push(format!(
            "Inspect active mission ({} is active): atelier mission status {}",
            mission.id, mission.id
        )),
        (None, _) if view.current_mission_count == 0 => actions.push(
            "Inspect mission readiness (no mission is active): atelier mission status".to_string(),
        ),
        (None, _) => actions.push(format!(
            "Inspect mission choices ({} current mission(s), none active): atelier mission status",
            view.current_mission_count
        )),
    }

    if view.active_mission.is_none() {
        if view.ready_work_count == 0 {
            actions.push(
                "Inspect blocked work (no ready work is available): atelier issue list --blocked"
                    .to_string(),
            );
        } else {
            actions.push(format!(
                "Choose ready work ({} ready issue(s) available): atelier issue list --ready",
                view.ready_work_count
            ));
            actions
                .push("Start selected work (ready work exists): atelier start <issue-id>".into());
        }
    }

    match view.tracker_state {
        TrackerState::Current => {
            actions.push("Check runtime health (tracker export is current): atelier doctor".into());
        }
        TrackerState::Stale { issue_count } => {
            actions.push(format!(
                "Refresh canonical export ({issue_count} stale record(s)): atelier export"
            ));
            actions.push("Check tracker records (export is stale): atelier lint".into());
        }
    }

    actions
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;
    use std::collections::{BTreeMap, BTreeSet};

    #[derive(Default)]
    struct FakePorts {
        issues: BTreeMap<String, IssueSummary>,
        blockers: BTreeMap<String, Vec<String>>,
        subissues: BTreeMap<String, Vec<String>>,
        validating_evidence: BTreeSet<String>,
        active_mission: Option<MissionSummary>,
        current_missions: Vec<MissionSummary>,
        mission_issue_ids: BTreeSet<String>,
        mission_direct_blockers: Vec<String>,
        recent_activity: Vec<String>,
        git_error: Option<String>,
    }

    impl FakePorts {
        fn issue(id: &str, title: &str, status: &str, issue_type: &str) -> IssueSummary {
            IssueSummary {
                id: id.to_string(),
                title: title.to_string(),
                status: status.to_string(),
                issue_type: issue_type.to_string(),
                parent_id: None,
            }
        }
    }

    impl StatusPorts for FakePorts {
        fn current_work_issues(&self) -> Result<Vec<IssueSummary>> {
            let mut issues = self
                .issues
                .values()
                .filter(|issue| is_current_work_issue(self, issue))
                .cloned()
                .collect::<Vec<_>>();
            issues.sort_by(|a, b| a.id.cmp(&b.id));
            Ok(issues)
        }

        fn all_issues(&self) -> Result<Vec<IssueSummary>> {
            Ok(self.issues.values().cloned().collect())
        }

        fn issue(&self, issue_id: &str) -> Result<Option<IssueSummary>> {
            Ok(self.issues.get(issue_id).cloned())
        }

        fn blockers(&self, issue_id: &str) -> Result<Vec<String>> {
            Ok(self.blockers.get(issue_id).cloned().unwrap_or_default())
        }

        fn subissues(&self, issue_id: &str) -> Result<Vec<IssueSummary>> {
            self.subissues
                .get(issue_id)
                .into_iter()
                .flatten()
                .map(|id| {
                    self.issue(id)?
                        .ok_or_else(|| anyhow!("missing fake subissue {id}"))
                })
                .collect()
        }

        fn active_mission(&self) -> Result<Option<MissionSummary>> {
            Ok(self.active_mission.clone())
        }

        fn current_missions(&self) -> Result<Vec<MissionSummary>> {
            Ok(self.current_missions.clone())
        }

        fn mission_issue_ids(&self, _mission_id: &str) -> Result<BTreeSet<String>> {
            Ok(self.mission_issue_ids.clone())
        }

        fn mission_direct_blocker_ids(&self, _mission_id: &str) -> Result<Vec<String>> {
            Ok(self.mission_direct_blockers.clone())
        }

        fn has_validating_evidence(&self, issue_id: &str) -> Result<bool> {
            Ok(self.validating_evidence.contains(issue_id))
        }

        fn issue_start_readiness(&self, issue: &IssueSummary) -> Result<IssueStartReadiness> {
            if !self.blockers(issue.id.as_str())?.is_empty() {
                return Ok(IssueStartReadiness::Blocked);
            }
            Ok(match issue.status.as_str() {
                "todo" => IssueStartReadiness::Ready,
                "blocked" => IssueStartReadiness::Blocked,
                _ => IssueStartReadiness::NotReady,
            })
        }

        fn issue_status_category(&self, status: &str) -> Option<String> {
            match status {
                "todo" => Some("todo".to_string()),
                "in_progress" => Some("active".to_string()),
                "blocked" => Some("blocked".to_string()),
                "done" => Some("done".to_string()),
                _ => None,
            }
        }

        fn tracker_state(&self) -> Result<TrackerState> {
            Ok(TrackerState::Current)
        }

        fn recent_mission_activity(&self, _issue_ids: &BTreeSet<String>) -> Result<Vec<String>> {
            Ok(self.recent_activity.clone())
        }

        fn git_status(&self) -> Result<GitStatusView> {
            if let Some(error) = &self.git_error {
                anyhow::bail!(error.clone());
            }
            Ok(GitStatusView {
                branch: Some("mission/test".to_string()),
                dirty_entries: Vec::new(),
            })
        }
    }

    #[test]
    fn status_use_case_builds_active_mission_view_without_clap() {
        let mut ports = FakePorts::default();
        ports.active_mission = Some(MissionSummary {
            id: "atelier-m1".to_string(),
            title: "Mission".to_string(),
        });
        ports.current_missions = vec![ports.active_mission.clone().unwrap()];
        ports.issues.insert(
            "atelier-a".into(),
            FakePorts::issue("atelier-a", "Active", "in_progress", "task"),
        );
        ports.issues.insert(
            "atelier-r".into(),
            FakePorts::issue("atelier-r", "Ready", "todo", "task"),
        );
        ports.issues.insert(
            "atelier-d".into(),
            FakePorts::issue("atelier-d", "Done", "done", "task"),
        );
        ports.mission_issue_ids = ["atelier-a", "atelier-r", "atelier-d"]
            .into_iter()
            .map(str::to_string)
            .collect();
        ports.validating_evidence.insert("atelier-r".to_string());
        ports.recent_activity = vec!["  atelier-a note: started".to_string()];

        let view = load_status(&ports).unwrap();
        let snapshot = view.active_mission_snapshot.as_ref().unwrap();

        assert_eq!(
            view.quiet_line(),
            "work=current active_mission=atelier-m1 current_missions=1 ready=1 tracker=current"
        );
        assert_eq!(snapshot.health(), "active");
        assert_eq!(snapshot.active, 1);
        assert_eq!(snapshot.ready, 1);
        assert_eq!(snapshot.done, 1);
        assert_eq!(
            snapshot.selectable_issues[0].proof_context,
            "proof attached"
        );
        assert_eq!(
            snapshot.recent_activity,
            vec!["  atelier-a note: started".to_string()]
        );
    }

    #[test]
    fn status_use_case_separates_runtime_error_from_query_view() {
        let mut ports = FakePorts {
            git_error: Some("git unavailable".to_string()),
            ..FakePorts::default()
        };
        ports.issues.insert(
            "atelier-r".into(),
            FakePorts::issue("atelier-r", "Ready", "todo", "task"),
        );

        let view = load_status(&ports).unwrap();

        assert!(matches!(view.git, ResultView::Unavailable(error) if error == "git unavailable"));
        assert_eq!(view.ready_work_count, 1);
    }
}
