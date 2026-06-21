---
created_at: "2026-06-13T00:19:52.031051841+00:00"
id: "atelier-0c13"
evidence_type: "validation"
captured_at: "2026-06-13T00:19:46.522696481+00:00"
command: "bash -lc 'target/debug/atelier status && target/debug/atelier lint atelier-a4rz && target/debug/atelier export --check && git diff --check && cargo nextest run test_root_status_guides_active_work_to_finish_not_start test_root_status_reports_active_mission_contract_fields test_root_status_summarizes_checkout_orientation test_root_status_no_ready_work_suggests_valid_blocked_list test_root_start_finish_and_issue_transition_surface test_workflow_help_is_scoped_as_advanced_internal_diagnostic test_agent_factory_guidance_avoids_raw_workflow_validate_commands'"
exit_status: "0"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-a4rz"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Active-work root status repair: target debug status shows active mission active bucket, ready work excludes active issue, finish next action replaces start; focused tests, lint, export check, and diff check pass."
updated_at: "2026-06-13T00:19:53.611267815+00:00"
---

Active-work root status repair: target debug status shows active mission active bucket, ready work excludes active issue, finish next action replaces start; focused tests, lint, export check, and diff check pass.

Command: bash -lc 'target/debug/atelier status && target/debug/atelier lint atelier-a4rz && target/debug/atelier export --check && git diff --check && cargo nextest run test_root_status_guides_active_work_to_finish_not_start test_root_status_reports_active_mission_contract_fields test_root_status_summarizes_checkout_orientation test_root_status_no_ready_work_suggests_valid_blocked_list test_root_start_finish_and_issue_transition_surface test_workflow_help_is_scoped_as_advanced_internal_diagnostic test_agent_factory_guidance_avoids_raw_workflow_validate_commands'
Exit status: 0

Stdout summary:
Atelier Status
==============
Tracker:       current
Ready work:    2
Active work:   atelier-a4rz - Repair active-work root status guidance
Work branch:   codex/orchestrate-atelier-fork
Worktree:      /root/atelier
Active mission: atelier-tcmr - Repair CLI workflow rework and validation gaps

Local State
-----------
Branch:   codex/orchestrate-atelier-fork...origin/codex/orchestrate-atelier-fork [ahead 55]
Worktree: dirty (5 entries)
   M .atelier/issues/atelier-wpyb.md
   M src/commands/status.rs
   M tests/cli_integration.rs
Tracker:  current

Active Mission
--------------
atelier-tcmr - Repair CLI workflow rework and validation gaps
Health:   blocked
Work:     ready 2, active 1, blocked 3, done 46, backlog 0

Ready In Active Mission
-----------------------
  atelier-v9id - Adversarially validate repair mission outcomes
  atelier-ymfl - Align docs tests and Agent Factory process with enforced proof

Immediate Blockers
------------------
  atelier-a4rz - Repair active-work root status guidance
  atelier-trr2 - Validate workflow signpost surfaces end to end
  atelier-v9id - Adversarially validate repair mission outcomes

Recent Activity
---------------
  atelier-a4rz work_started: Started work
  atelier-sdmo close_reason: Recorded close reason
  atelier-sdmo status_changed: Changed status from open to closed

Next Actions
------------
  Inspect active mission health (atelier-tcmr): atelier mission status atelier-tcmr
  Open active mission record (atelier-tcmr): atelier mission show atelier-tcmr
  Finish active work (atelier-a4rz): atelier finish atelier-a4rz
  Check runtime health (tracker export is current): atelier doctor
Lint passed.
Canonical export is current
State: /root/atelier/.atelier

Stderr summary (truncated):
   Compiling atelier-tracker v0.2.0 (/root/atelier)
warning: struct `CreateInput` is never constructed
    --> src/commands/agent_factory.rs:1413:12
     |
1413 | pub struct CreateInput<'a> {
     |            ^^^^^^^^^^^
     |
     = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `create` is never used
    --> src/commands/agent_factory.rs:1541:8
     |
1541 | pub fn create(db: &Database, input: CreateInput<'_>) -> Result<()> {
     |        ^^^^^^

warning: function `update` is never used
    --> src/commands/agent_factory.rs:1591:8
     |
1591 | pub fn update(db: &Database, input: UpdateInput<'_>) -> Result<()> {
     |        ^^^^^^

warning: function `close` is never used
    --> src/commands/agent_factory.rs:1935:8
     |
1935 | pub fn close(db: &Database, issue_ref: &str, reason: Option<&str>) -> Result<()> {
     |        ^^^^^

warning: function `ensure_canonical_issue_sections_valid` is never used
    --> src/commands/agent_factory.rs:2007:4
     |
2007 | fn ensure_canonical_issue_sections_valid(issue_id: &str) -> Result<()> {
     |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `reopen` is never used
    --> src/commands/agent_factory.rs:2015:8
     |
2015 | pub fn reopen(db: &Database, issue_ref: &str) -> Result<()> {
     |        ^^^^^^

warning: function `dep_add` is never used
    --> src/commands/agent_factory.rs:2143:8
     |
2143 | pub fn dep_add(db: &Database, blocked_ref: &str, blocker_ref: &str) -> Result<()> {
     |        ^^^^^^^

warning: function `dep_remove` is never used
    --> src/commands/agent_factory.rs:2162:8
     |
2162 | pub fn dep_remove(db: &Database, blocked_ref: &str, blocker_ref: &str) -> Result<()> {
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
   --> src/commands/status.rs:473:8
    |
473 | pub fn close_all(
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
 83 |     pu

