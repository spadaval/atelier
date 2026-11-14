//! The JSON each route returns (docs/architecture/desk.md, Responses).
//! The app's TypeScript types are generated from these with `ts-rs` into
//! `desk/src/api/`, one file per type.

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// `GET /api/repos`.
#[derive(Debug, Clone, Serialize, TS)]
pub struct ReposResponse {
    pub repos: Vec<RepoSummary>,
    /// The commit the server was built from (`crate::BUILT_FROM`).
    pub built_from: String,
}

/// One repository and its counts, or why it cannot be read: then `error`
/// is set and the rest is empty.
#[derive(Debug, Clone, Default, Serialize, TS)]
pub struct RepoSummary {
    pub name: String,
    pub path: String,
    pub base: Option<String>,
    pub base_tip: Option<String>,
    /// Agreed missions with a started item.
    pub in_progress: usize,
    /// Missions proposed: open on a branch that has not merged.
    pub proposed: usize,
    /// Open items labelled needs-human, as `list --needs-human` lists them.
    pub waiting: usize,
    pub error: Option<String>,
    pub fetched: FetchMark,
}

/// When the Desk last fetched a repository's remote, so a page can say how
/// fresh what it read from remote-tracking branches is.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, TS)]
pub struct FetchMark {
    /// The remote fetched: the forge's, else `origin`; `null` when the
    /// repository has neither, and nothing is fetched.
    pub remote: Option<String>,
    /// When it was last fetched, by the Desk or anything else, from the
    /// clone's `FETCH_HEAD`; `null` when never.
    pub at: Option<String>,
    /// Why the Desk's last fetch failed.
    pub error: Option<String>,
    /// How often the Desk fetches, in seconds; `0` when it does not.
    pub every_seconds: u32,
}

/// `GET /api/repos/{repo}/missions`.
#[derive(Debug, Clone, Serialize, TS)]
pub struct MissionsResponse {
    /// The base's tip it was read at.
    pub commit: String,
    pub missions: Vec<MissionRow>,
    /// The open items in no mission (finished ones with `?done=true`).
    pub loose: usize,
    pub fetched: FetchMark,
}

#[derive(Debug, Clone, Serialize, TS)]
pub struct MissionRow {
    pub id: String,
    pub title: String,
    /// As its record that counts stores it.
    pub status: MissionStatus,
    /// Where it stands, as the human is shown it.
    pub state: MissionState,
    pub labels: Vec<String>,
    pub lead: Option<String>,
    pub created_at: String,
    pub in_progress: bool,
    /// Its items per status, in the order of the lifecycle; a status with
    /// none is left out.
    pub counts: Vec<StatusCount>,
    /// Its open items labelled needs-human.
    pub waiting: Vec<String>,
    /// The ref its record was read from: a branch, or `<remote>/<branch>`;
    /// `null` for the base.
    pub branch: Option<String>,
}

/// Where a mission stands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum MissionState {
    /// Open on a branch that has not merged: its plan is under review.
    Proposed,
    /// Agreed: on the base and open.
    Open,
    /// A stop is written and not published yet: a plan dropped on its
    /// branch whose pull request is still open, or an open mission
    /// dropped on a branch that has not merged.
    StopPending,
    /// Dropped, and the stop is published.
    Stopped,
    Done,
}

/// A pull request on the forge, as `gh pr list` gives it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
pub struct PrMark {
    pub number: u32,
    pub url: String,
    /// `OPEN`, `CLOSED` or `MERGED`.
    pub state: String,
}

/// The verdicts of a plan's current round of review.
#[derive(Debug, Clone, Serialize, TS)]
pub struct RoundMark {
    pub round: u32,
    pub reviews: Vec<RoundReview>,
}

#[derive(Debug, Clone, Serialize, TS)]
pub struct RoundReview {
    pub name: String,
    pub reviewer: Option<String>,
    pub model: Option<String>,
    /// `null` while the review is not finished.
    pub verdict: Option<Verdict>,
}

#[derive(Debug, Clone, Serialize, TS)]
pub struct ReadyMark {
    pub by: String,
    pub at: String,
}

#[derive(Debug, Clone, Serialize, TS)]
pub struct StatusCount {
    pub status: Status,
    pub count: usize,
}

/// `GET /api/repos/{repo}/missions/{id}`.
#[derive(Debug, Clone, Serialize, TS)]
pub struct MissionPage {
    pub commit: String,
    pub mission: MissionRow,
    /// Why, Outcome, Success criteria, Scope, Authority and Result, those
    /// it has.
    pub sections: Vec<Section>,
    pub dropped: Option<String>,
    /// The commit its record and documents were read at: the base's tip,
    /// or the tip of the branch it was read from.
    pub read_at: String,
    /// For a proposed plan, or one stopped on its branch, the pull request
    /// of its branch, where there is one.
    pub pull_request: Option<PrMark>,
    /// The verdicts of its plan's current round, where it has reviews.
    pub round: Option<RoundMark>,
    /// For a stop not yet published, the branch that holds it.
    pub stop_on: Option<String>,
    /// Whether the Desk can stop it from this page: a proposed plan on the
    /// forge's remote.
    pub can_stop: bool,
    /// For an open mission, the commands that stop it, run in a checkout
    /// of the repository (atelier-yclc moves this into the Desk).
    pub stop_commands: Vec<String>,
    pub documents: Vec<Document>,
    pub items: Vec<ItemRow>,
    pub plan_reviews: Vec<Review>,
    /// Its lead, the reviewers of its plan reviews, and every agent of
    /// each of its items, naming the item.
    pub threads: Vec<Agent>,
    pub t3: T3State,
    pub fetched: FetchMark,
}

#[derive(Debug, Clone, Serialize, TS)]
pub struct Section {
    pub heading: String,
    pub markdown: String,
}

#[derive(Debug, Clone, Serialize, TS)]
pub struct Document {
    /// From the repository's root, for `/doc?path=`.
    pub path: String,
    pub title: String,
}

/// An item in a mission, in the inbox, or at the top of its own page.
#[derive(Debug, Clone, Serialize, TS)]
pub struct ItemRow {
    pub id: String,
    pub title: String,
    pub status: Status,
    pub priority: Priority,
    pub labels: Vec<String>,
    pub needs_human: bool,
    pub startable: bool,
    pub blockers: Vec<Blocker>,
    /// Its blockers that are not done.
    pub waiting_on: Vec<String>,
    pub holder: Option<String>,
    /// Who holds this round's review, while it is in review.
    pub reviewer: Option<String>,
    /// For a ready item, why it cannot start now.
    pub cannot_start: Vec<String>,
    /// For a started item, what its holder's next act still lacks.
    pub needs: Vec<String>,
    pub branch: Option<BranchMark>,
    pub report_written: bool,
    pub dropped: Option<String>,
    pub approved_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, TS)]
pub struct Blocker {
    pub id: String,
    /// `null` when no such item is found.
    pub status: Option<Status>,
}

#[derive(Debug, Clone, Serialize, TS)]
pub struct BranchMark {
    pub merged: bool,
    /// The ref read: the branch, or `<remote>/<branch>`.
    pub name: String,
}

/// A question asked on an item before `escalate` was removed, and its
/// answer once given: shown, never asked or answered now.
#[derive(Debug, Clone, Serialize, TS)]
pub struct Question {
    pub item: String,
    pub question: String,
    pub finding: Option<String>,
    pub answer: Option<AnswerMark>,
    pub outcome: Option<Outcome>,
}

#[derive(Debug, Clone, Serialize, TS)]
pub struct AnswerMark {
    pub by: String,
    pub text: String,
}

/// A review of an item, or of a mission's plan.
#[derive(Debug, Clone, Serialize, TS)]
pub struct Review {
    pub name: String,
    pub round: u32,
    pub commit: String,
    pub reviewers: Vec<String>,
    pub model: Option<String>,
    pub verdict: Option<Verdict>,
    pub created_at: String,
    pub findings: Vec<FindingRow>,
    /// On a plan review, the plan's paths that differ between the commit
    /// reviewed and the base's tip; `null` on an item's review, and when
    /// this clone lacks the commit reviewed.
    pub changed_since: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, TS)]
pub struct FindingRow {
    pub name: String,
    pub title: String,
    pub recommendation: Recommendation,
    pub was: Option<String>,
    pub now: Option<Now>,
    pub decided: Option<Outcome>,
    pub settlement: Option<Settlement>,
    pub text: Option<String>,
    /// A fix-now finding nothing has closed.
    pub open: bool,
    pub deferred: bool,
}

#[derive(Debug, Clone, Serialize, TS)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Settlement {
    Taken { item: String, by: String },
    Dropped { by: String, reason: String },
}

/// `GET /api/repos/{repo}/doc?path=&at=`.
#[derive(Debug, Clone, Serialize, TS)]
pub struct DocResponse {
    pub commit: String,
    pub path: String,
    pub markdown: String,
}

/// `GET /api/repos/{repo}/items/{id}`.
#[derive(Debug, Clone, Serialize, TS)]
pub struct ItemPage {
    pub commit: String,
    pub row: ItemRow,
    pub mission: Option<String>,
    pub blocked_by: Vec<String>,
    pub implementers: Vec<String>,
    pub ready: Option<ReadyMark>,
    pub trivial: Option<String>,
    pub caused_by: Option<String>,
    pub created_at: String,
    pub read_from: ReadFrom,
    pub body: String,
    pub goal: Option<String>,
    pub scope: Option<String>,
    pub report: Option<String>,
    pub done_when: Vec<DoneWhen>,
    /// Its Decisions section, from before `escalate` was removed.
    pub decisions: Vec<Question>,
    pub responses: Vec<ResponseLine>,
    pub reviews: Vec<Review>,
    /// For a done item, its deferred findings nobody has settled, by name.
    pub unsettled: Vec<String>,
    /// Its implementers, the reviewers of its reviews and the deciders of
    /// its answers.
    pub threads: Vec<Agent>,
    pub t3: T3State,
    pub fetched: FetchMark,
}

#[derive(Debug, Clone, Serialize, TS)]
pub struct DoneWhen {
    pub ticked: bool,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, TS)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ResponseLine {
    Fixed { finding: String, commit: String },
    Rebutted { finding: String, fact: String },
}

/// `GET /api/inbox`.
#[derive(Debug, Clone, Serialize, TS)]
pub struct InboxResponse {
    pub repos: Vec<InboxRepo>,
    pub t3: T3State,
    /// Plans proposed and merged lately, newest first, in every
    /// repository.
    pub news: Vec<NewsEvent>,
}

/// A plan proposed, or merged, as the Desk saw it on its fetch timer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct NewsEvent {
    pub repo: String,
    pub mission: String,
    pub title: String,
    pub kind: NewsKind,
    /// When the Desk saw it.
    pub at: String,
    /// The mission's page.
    pub link: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum NewsKind {
    Proposed,
    Merged,
}

#[derive(Debug, Clone, Default, Serialize, TS)]
pub struct InboxRepo {
    pub repo: String,
    pub commit: Option<String>,
    pub error: Option<String>,
    /// As `list --needs-human` groups them: by mission, then those in none.
    pub needs_human: Vec<NeedsHuman>,
    /// Plans under review, which he may stop.
    pub proposed: Vec<MissionRow>,
    /// T3 Code's live threads waiting on a person that hold something in
    /// this repository's records, with only those holds.
    pub threads: Vec<LiveThread>,
}

#[derive(Debug, Clone, Serialize, TS)]
pub struct NeedsHuman {
    pub mission: Option<String>,
    pub mission_title: Option<String>,
    pub items: Vec<ItemRow>,
}

/// An identity a record names, in one role, and the live thread it is
/// when T3 Code lists it as one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
pub struct Agent {
    /// Whole, as the record holds it.
    pub identity: String,
    pub role: Role,
    /// On a mission's page, the item it is on; `null` for the mission's
    /// own lead and plan reviewers, and on an item's page.
    pub item: Option<String>,
    /// `null` when the identity is no live thread of T3 Code's, or T3
    /// Code has not been reached: then the page shows the identity as
    /// text.
    pub thread: Option<ThreadMark>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
pub struct ThreadMark {
    pub title: String,
    pub status: String,
    pub provider: String,
    pub model: String,
    pub link: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Planner,
    Implementer,
    Reviewer,
    Lead,
    Decider,
}

/// Whether T3 Code's threads could be read. Threads come from the last
/// list read, which is `age_seconds` old.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, TS)]
pub struct T3State {
    /// Whether the last poll read T3 Code's thread list.
    pub reachable: bool,
    /// The age of the list served; `null` when none has been read.
    pub age_seconds: Option<u32>,
    /// Why the last poll failed, or that T3 Code is not configured.
    pub error: Option<String>,
}

/// `GET /api/threads`.
#[derive(Debug, Clone, Serialize, TS)]
pub struct ThreadsResponse {
    pub t3: T3State,
    pub threads: Vec<LiveThread>,
}

/// One of T3 Code's live, unsettled threads.
#[derive(Debug, Clone, Serialize, TS)]
pub struct LiveThread {
    /// T3 Code's `threadId`, the identity records hold.
    pub id: String,
    pub title: String,
    pub status: String,
    pub provider: String,
    pub model: String,
    pub parent: Option<String>,
    pub pull_requests: Vec<PullRequest>,
    pub link: Option<String>,
    pub updated_at: String,
    /// What its identity holds in the records of every repository served.
    pub holds: Vec<Hold>,
}

#[derive(Debug, Clone, Serialize, TS)]
pub struct PullRequest {
    pub repository: String,
    pub number: u32,
    pub url: String,
    pub state: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
pub struct Hold {
    pub repo: String,
    pub kind: HoldKind,
    pub id: String,
    pub role: Role,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum HoldKind {
    Item,
    Mission,
}

/// `POST /api/act`, sent with the header `x-atelier-desk: 1`.
#[derive(Debug, Clone, Deserialize, TS)]
#[serde(tag = "act", rename_all = "snake_case")]
pub enum ActRequest {
    /// Stops a proposed plan: `atelier drop <id> --reason <reason>` on its
    /// branch, committed, pushed, and its pull request closed.
    Stop(StopAct),
}

#[derive(Debug, Clone, Deserialize, TS)]
#[serde(deny_unknown_fields)]
pub struct StopAct {
    pub repo: String,
    pub id: String,
    /// Why, on one line.
    pub reason: String,
}

/// What an act did, or why it was refused, as the core's function gave it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
#[serde(tag = "result", rename_all = "snake_case")]
pub enum ActResponse {
    Done {
        text: String,
        paths: Vec<String>,
        /// The commit the act made.
        commit: Option<String>,
    },
    Refused {
        /// The rules it broke, one line each; empty when no rule refused
        /// it.
        rules: Vec<String>,
        text: String,
        /// The way out, where the function names one.
        next: Option<String>,
    },
}

/// `POST /api/sessions`, sent with the header `x-atelier-desk: 1`: start a
/// planning session for `mission` as a new T3 Code thread.
#[derive(Debug, Clone, Deserialize, TS)]
#[serde(deny_unknown_fields)]
pub struct SessionRequest {
    pub repo: String,
    pub mission: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
#[serde(tag = "result", rename_all = "snake_case")]
pub enum SessionResponse {
    /// The thread T3 Code started.
    Started {
        thread_id: String,
        /// Its address in T3 Code's web app, when T3 Code gives one.
        link: Option<String>,
        /// The T3 project it runs in.
        project_id: String,
    },
    /// Nothing was started, and why.
    Refused { text: String },
}

/// `GET /api/proposed`: what is proposed in every repository.
#[derive(Debug, Clone, Serialize, TS)]
pub struct ProposedResponse {
    pub repos: Vec<ProposedRepo>,
}

#[derive(Debug, Clone, Default, Serialize, TS)]
pub struct ProposedRepo {
    pub repo: String,
    pub commit: Option<String>,
    pub error: Option<String>,
    pub fetched: FetchMark,
    /// Missions proposed, each read from its branch.
    pub plans: Vec<ProposedPlan>,
    /// Record changes on branches not merged, by branch, other than a
    /// proposed plan's own branch and a started item's.
    pub changes: Vec<RecordChanges>,
}

#[derive(Debug, Clone, Serialize, TS)]
pub struct ProposedPlan {
    pub mission: MissionRow,
    pub pull_request: Option<PrMark>,
    pub round: Option<RoundMark>,
}

#[derive(Debug, Clone, Serialize, TS)]
pub struct RecordChanges {
    /// The ref read: the branch, or `<remote>/<branch>`.
    pub branch: String,
    pub pull_request: Option<PrMark>,
    pub records: Vec<ChangedRecord>,
}

#[derive(Debug, Clone, Serialize, TS)]
pub struct ChangedRecord {
    pub kind: HoldKind,
    pub id: String,
    pub title: String,
    pub change: Change,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum Change {
    /// The base has no such record.
    Added,
    Changed,
    /// Changed to dropped.
    Dropped,
    Removed,
}

/// `GET /api/motion`: work in motion in every repository.
#[derive(Debug, Clone, Serialize, TS)]
pub struct MotionResponse {
    pub repos: Vec<MotionRepo>,
    pub t3: T3State,
}

#[derive(Debug, Clone, Default, Serialize, TS)]
pub struct MotionRepo {
    pub repo: String,
    pub commit: Option<String>,
    pub error: Option<String>,
    pub fetched: FetchMark,
    /// Each started item, its record read from its branch.
    pub items: Vec<MotionItem>,
    /// Each agreed mission with a started item.
    pub missions: Vec<MotionMission>,
}

#[derive(Debug, Clone, Serialize, TS)]
pub struct MotionItem {
    pub id: String,
    pub title: String,
    pub status: Status,
    pub mission: Option<String>,
    /// The ref its record was read from.
    pub branch: Option<String>,
    /// The thread that holds it, with its live thread where T3 Code lists
    /// one.
    pub holder: Option<Agent>,
    /// Who holds its review, while it is in review.
    pub reviewer: Option<Agent>,
}

#[derive(Debug, Clone, Serialize, TS)]
pub struct MotionMission {
    pub mission: MissionRow,
    pub lead: Option<Agent>,
}

/// `GET /api/disputed`: what `atelier list --disputed` gives, in every
/// repository.
#[derive(Debug, Clone, Serialize, TS)]
pub struct DisputedResponse {
    pub repos: Vec<DisputedRepo>,
    pub t3: T3State,
}

#[derive(Debug, Clone, Default, Serialize, TS)]
pub struct DisputedRepo {
    pub repo: String,
    pub commit: Option<String>,
    pub error: Option<String>,
    pub rebuttals: Vec<RebuttalRow>,
    pub plans: Vec<SplitRoundRow>,
}

/// A finding its implementer rebutted, and what the next round made of it.
#[derive(Debug, Clone, Serialize, TS)]
pub struct RebuttalRow {
    pub item: String,
    pub mission: Option<String>,
    pub finding: String,
    pub title: String,
    pub fact: String,
    /// The implementer who rebutted it.
    pub implementer: Option<Side>,
    /// The reviewer who raised it.
    pub raised_by: Option<Side>,
    /// Each later finding that carries it, judged; empty while none has.
    pub judged: Vec<JudgedRow>,
}

#[derive(Debug, Clone, Serialize, TS)]
pub struct JudgedRow {
    pub finding: String,
    /// `kept`, `withdrawn`, `fixed`, `lowered to defer`, `lowered to no
    /// action`, or `decided: defer` or `decided: no action`.
    pub judged: String,
    pub reviewer: Option<Side>,
}

/// A round of a plan's review whose reviewers gave different verdicts.
#[derive(Debug, Clone, Serialize, TS)]
pub struct SplitRoundRow {
    pub mission: String,
    pub round: u32,
    pub reviews: Vec<SplitReview>,
}

#[derive(Debug, Clone, Serialize, TS)]
pub struct SplitReview {
    pub name: String,
    pub reviewer: Option<Side>,
    pub verdict: Verdict,
}

/// One side of a disagreement: a thread, with the model its review
/// records or T3 Code gives for it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
pub struct Side {
    pub identity: String,
    /// From the review's `model`, else from T3 Code where it knows the
    /// thread; `null` when neither does.
    pub model: Option<String>,
    /// The provider T3 Code gives for the thread, where it knows it.
    pub provider: Option<String>,
    pub link: Option<String>,
}

/// The body of every response that is not a 200.
#[derive(Debug, Clone, Serialize, TS)]
pub struct ApiError {
    pub error: String,
}

/// A copy of a core enum, named as its records spell it, with the
/// conversion from it, so the app gets a union of its names.
macro_rules! named {
    ($name:ident, $core:path, $($variant:ident),+) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
        #[serde(rename_all = "snake_case")]
        pub enum $name {
            $($variant),+
        }

        impl From<$name> for $core {
            fn from(value: $name) -> Self {
                use $core as Core;
                match value {
                    $($name::$variant => Core::$variant),+
                }
            }
        }

        impl From<$core> for $name {
            fn from(value: $core) -> Self {
                use $core as Core;
                match value {
                    $(Core::$variant => $name::$variant),+
                }
            }
        }
    };
}

named!(
    Status,
    atelier_core::Status,
    Draft,
    Ready,
    Building,
    InReview,
    ChangesRequested,
    Approved,
    Done,
    Dropped
);
named!(
    MissionStatus,
    atelier_core::MissionStatus,
    Open,
    Done,
    Dropped,
    Draft
);
named!(
    Priority,
    atelier_core::Priority,
    Critical,
    High,
    Medium,
    Low
);
named!(Verdict, atelier_core::Verdict, Approve, Changes);
named!(
    Recommendation,
    atelier_core::Recommendation,
    FixNow,
    Defer,
    NoAction
);
named!(Now, atelier_core::Now, Fixed, Withdrawn, Kept);
named!(Outcome, atelier_core::Outcome, Defer, NoAction);
named!(ReadFrom, atelier_app::ReadFrom, Base, Branch);

/// Writes the TypeScript type of every response, and of everything they
/// hold, into `dir`, one file per type.
pub fn export_types(dir: &std::path::Path) -> Result<(), ts_rs::ExportError> {
    let config = ts_rs::Config::new().with_out_dir(dir);
    ReposResponse::export_all(&config)?;
    MissionsResponse::export_all(&config)?;
    MissionPage::export_all(&config)?;
    DocResponse::export_all(&config)?;
    ItemPage::export_all(&config)?;
    InboxResponse::export_all(&config)?;
    ThreadsResponse::export_all(&config)?;
    ProposedResponse::export_all(&config)?;
    MotionResponse::export_all(&config)?;
    DisputedResponse::export_all(&config)?;
    ActRequest::export_all(&config)?;
    ActResponse::export_all(&config)?;
    SessionRequest::export_all(&config)?;
    SessionResponse::export_all(&config)?;
    ApiError::export_all(&config)?;
    // ts-rs ends some lines with a space; strip them so the committed files
    // pass `git diff --check`.
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        let text = std::fs::read_to_string(&path)?;
        let lines: Vec<&str> = text.lines().map(str::trim_end).collect();
        std::fs::write(&path, format!("{}\n", lines.join("\n")))?;
    }
    Ok(())
}
