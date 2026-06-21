---
created_at: "2026-06-12T22:34:41.916275887+00:00"
id: "atelier-ojjl"
evidence_type: "validation"
captured_at: "2026-06-12T22:34:28.620609048+00:00"
command: "bash -lc 'set -euo pipefail\ntarget/debug/atelier --help | rg \"prime|history|atelier prime|history --mission|history --issue\"\nrg -n \"atelier prime|atelier history|history --mission|history --issue\" docs/product/cli-surface.md\ntarget/debug/atelier issue show atelier-hggl | rg \"repo-wide and scoped history|Event sources|Ordering and filtering|Output shape|Empty states|Proof expectations|history --mission|history --issue\"\ntarget/debug/atelier issue show atelier-bzts | rg \"atelier prime|Context Recovery|Essential Commands|concrete reason|mostly static guidance|active mission\"\ntarget/debug/atelier prime | rg \"Atelier Prime|Context Recovery|Active mission|Active work|Essential Commands|history --issue|Check active work\"\ntarget/debug/atelier history --limit 3 | rg \"History|Scope:|Source:|Ordering:|Showing:|Next Commands\"\ntarget/debug/atelier history --issue atelier-u4nx --limit 5 | rg \"Scope:.*issue atelier-u4nx|Next Commands|atelier history --issue atelier-u4nx\"\ntarget/debug/atelier history --mission atelier-tcmr --event-kind evidence_attached --limit 2 | rg \"Scope:.*mission atelier-tcmr|evidence_attached|Next Commands\"\ntarget/debug/atelier issue show atelier-u4nx | rg \"Show full activity: atelier history --issue atelier-u4nx\"\ntarget/debug/atelier mission show atelier-tcmr | rg \"atelier history --mission atelier-tcmr\"\ncargo test --quiet --test cli_integration history\ncargo test --quiet --test cli_integration prime\ncargo test --quiet --test cli_integration test_top_level_help_only_shows_core_commands\ncargo test --quiet --test cli_integration test_show_issue_prefers_activity_sidecars_for_recent_activity\ncargo fmt -- --check\ngit diff --check\ntarget/debug/atelier export --check\ntarget/debug/atelier lint atelier-u4nx'"
exit_status: "0"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-u4nx"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "History and prime signpost surfaces implemented and validated"
updated_at: "2026-06-12T22:34:43.319682089+00:00"
---

History and prime signpost surfaces implemented and validated

Command: bash -lc 'set -euo pipefail
target/debug/atelier --help | rg "prime|history|atelier prime|history --mission|history --issue"
rg -n "atelier prime|atelier history|history --mission|history --issue" docs/product/cli-surface.md
target/debug/atelier issue show atelier-hggl | rg "repo-wide and scoped history|Event sources|Ordering and filtering|Output shape|Empty states|Proof expectations|history --mission|history --issue"
target/debug/atelier issue show atelier-bzts | rg "atelier prime|Context Recovery|Essential Commands|concrete reason|mostly static guidance|active mission"
target/debug/atelier prime | rg "Atelier Prime|Context Recovery|Active mission|Active work|Essential Commands|history --issue|Check active work"
target/debug/atelier history --limit 3 | rg "History|Scope:|Source:|Ordering:|Showing:|Next Commands"
target/debug/atelier history --issue atelier-u4nx --limit 5 | rg "Scope:.*issue atelier-u4nx|Next Commands|atelier history --issue atelier-u4nx"
target/debug/atelier history --mission atelier-tcmr --event-kind evidence_attached --limit 2 | rg "Scope:.*mission atelier-tcmr|evidence_attached|Next Commands"
target/debug/atelier issue show atelier-u4nx | rg "Show full activity: atelier history --issue atelier-u4nx"
target/debug/atelier mission show atelier-tcmr | rg "atelier history --mission atelier-tcmr"
cargo test --quiet --test cli_integration history
cargo test --quiet --test cli_integration prime
cargo test --quiet --test cli_integration test_top_level_help_only_shows_core_commands
cargo test --quiet --test cli_integration test_show_issue_prefers_activity_sidecars_for_recent_activity
cargo fmt -- --check
git diff --check
target/debug/atelier export --check
target/debug/atelier lint atelier-u4nx'
Exit status: 0

Stdout summary (truncated):
  prime         Show repository operating guidance for recovery and onboarding
  history       Inspect canonical repo, mission, issue, or epic activity
  atelier prime
  atelier history --mission <id>
  atelier history --issue <id>
18:- `atelier prime`
30:- `atelier history`
43:`atelier prime` is the recovery and onboarding signpost. It explains how an
95:`atelier history` is the canonical project-history view. Repo-wide history and
96:scoped forms such as `atelier history --mission <id>`, `atelier history --issue
97:<id>`, and `atelier history --epic <id>` read canonical activity sidecars,
atelier-hggl [task] closed - Specify repo-wide and scoped history views
  Planning contract: repo-wide and scoped history views.
  Command shape: provide a repo-wide history view and scoped forms for mission, issue, and epic context. Preferred syntax should be explicit and discoverable, for example `atelier history`, `atelier history --mission <id>`, `atelier history --issue <id>`, and `atelier history --since <duration-or-date>`, with aliases only if they fit the final command hierarchy. Issue/mission show commands may include a short Recent Activity section and point to the scoped history command for full detail.
  Event sources: canonical issue activity sidecars, mission activity when available, evidence creation/attachment records, workflow validation evidence or durable validation events, dependency/parent/link changes, status/assignee/priority/title/body changes, comments/notes, work start/finish events where durable, and close/reopen events. Local diagnostics and ignored runtime/cache state may be summarized only when deliberately surfaced as local history; they must not be mixed into canonical project history without a label.
  Ordering and filtering: default order newest first, stable tie-break by timestamp then record ID/path. Filters should support record scope, mission graph scope including child issues/epics, event kind, actor when available, and bounded limit. Mission-scoped history includes the mission record plus linked work/evidence activity. Epic/issue-scoped history includes the record and descendants when explicitly requested; default issue scope is the single issue.
  Output shape: header with scope, source boundary, count/limit, and ordering; compact rows with timestamp, event kind, actor when known, target ID, short title/context, and concise summary; omitted-row message when bounded; next commands for show/drill-down of target records and for widening/narrowing filters.
  Empty states: no activity in scope prints an explicit empty with the source boundary and a drill-down or widening suggestion; no matching filter says the scope has activity but none matched; unknown target IDs fail with actionable text.
  Proof expectations: transcript tests cover repo-wide history, mission-scoped history including child work/evidence, issue-scoped history, empty scope, no filter matches, bounded output, and drill-down commands. Acceptance requires canonical history to be separated from local runtime diagnostics.
  Show full activity: atelier history --issue atelier-hggl
Consider an `atelier prime` command that prints the most important workflow
- Decide whether `atelier prime` belongs in the product and how it differs from
  concrete reason an agent would use it.
  Planning contract: `atelier prime` workflow command.
  Decision: `atelier prime` belongs in the product as a recovery and onboarding command for agents, but it must not compete with `atelier status`. `status` answers the current checkout state. `prime` answers how an agent should operate in this repository and which command families matter. It is mostly static guidance blended with a small dynamic header that names the repo, active mission if one is discoverable, active work if one exists, and tracker health if cheap to compute.
  Output sections: Context Recovery, including durable state location, ignored runtime/cache boundaries, and active mission/work summary; Core Rules, including use installed `atelier`, no JSON as the primary agent contract, canonical Markdo

Stderr summary (truncated):
warning: struct `CreateInput` is never constructed
    --> src/commands/agent_factory.rs:1256:12
     |
1256 | pub struct CreateInput<'a> {
     |            ^^^^^^^^^^^
     |
     = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `create` is never used
    --> src/commands/agent_factory.rs:1384:8
     |
1384 | pub fn create(db: &Database, input: CreateInput<'_>) -> Result<()> {
     |        ^^^^^^

warning: function `update` is never used
    --> src/commands/agent_factory.rs:1434:8
     |
1434 | pub fn update(db: &Database, input: UpdateInput<'_>) -> Result<()> {
     |        ^^^^^^

warning: function `close` is never used
    --> src/commands/agent_factory.rs:1761:8
     |
1761 | pub fn close(db: &Database, issue_ref: &str, reason: Option<&str>) -> Result<()> {
     |        ^^^^^

warning: function `ensure_canonical_issue_sections_valid` is never used
    --> src/commands/agent_factory.rs:1850:4
     |
1850 | fn ensure_canonical_issue_sections_valid(issue_id: &str) -> Result<()> {
     |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `reopen` is never used
    --> src/commands/agent_factory.rs:1858:8
     |
1858 | pub fn reopen(db: &Database, issue_ref: &str) -> Result<()> {
     |        ^^^^^^

warning: function `dep_add` is never used
    --> src/commands/agent_factory.rs:1986:8
     |
1986 | pub fn dep_add(db: &Database, blocked_ref: &str, blocker_ref: &str) -> Result<()> {
     |        ^^^^^^^

warning: function `dep_remove` is never used
    --> src/commands/agent_factory.rs:2005:8
     |
2005 | pub fn dep_remove(db: &Database, blocked_ref: &str, blocker_ref: &str) -> Result<()> {
     |        ^^^^^^^^^^

warning: function `run` is never used
  --> src/commands/comment.rs:21:8
   |
21 | pub fn run(db: &Database, issue_id: &str, content: &str, kind: &str) -> Result<()> {
   |        ^^^

warning: struct `CreateOpts` is never constructed
  --> src/commands/create.rs:79:12
   |
79 | pub struct CreateOpts<'a> {
   |            ^^^^^^^^^^

warning: function `run` is never used
  --> src/commands/create.rs:86:8
   |
86 | pub fn run(
   |        ^^^

warning: function `run_subissue` is never used
   --> src/commands/create.rs:175:8
    |
175 | pub fn run_subissue(
    |        ^^^^^^^^^^^^

warning: function `run` is never used
 --> src/commands/delete.rs:9:8
  |
9 | pub fn run(db: &Database, id: &str, force: bool) -> Result<()> {
  |        ^^^

warning: function `add` is never used
 --> src/commands/label.rs:7:8
  |
7 | pub fn add(db: &Database, issue_id: &str, label: &str) -> Result<()> {
  |        ^^^

warning: function `remove` is never used
  --> src/commands/label.rs:50:8
   |
50 | pub fn remove(db: &Database, issue_id: &str, label: &str) -> Result<()> {
   |        ^^^^^^

warning: function `refresh_open_database_after_canonical_write` is never used
  --> src/commands/projection.rs:26:8
   |
26 | pub fn refresh_open_database_after_canonical_write(
   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `add_typed` is never used
 --> src/commands/relate.rs:7:8
  |
7 | pub fn add_typed(
  |        ^^^^^^^^^

warning: function `remove_typed` is never used
  --> src/commands/relate.rs:66:8
   |
66 | pub fn remove_typed(
   |        ^^^^^^^^^^^^

warning: function `close_all` is never used
  --> src/commands/status.rs:92:8
   |
92 | pub fn close_all(
   |        ^^^^^^^^^

warning: method `remove_dependency` is never used
  --> src/db/dependencies.rs:53:12
   |
 7 | impl Database {
   | ------------- method in this implementation
53 |     pub fn remove_dependency(
   |            ^^^^^^^^^^^^^^^^^

warning: multiple methods are never used
   --> src/db/issues.rs:54:12
    |
 10 | impl Database {
    | ------------- methods in this implementation
 54 |     pub fn create_issue(
    |            ^^^^^^^^^^^^
 63 |     pub fn create_subissue(
    |            ^^^^^^^^^^^^^^^
 73 |     pub fn create_issue_with_type(
    |            ^^^^^^^^^^^^^^^^^^^^^^
 83 |     pub fn create_subissue_with_type(
    |            ^^^^^^^^
