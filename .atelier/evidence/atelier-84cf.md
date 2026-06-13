---
created_at: "2026-06-13T00:07:32.370384570+00:00"
id: "atelier-84cf"
data: "{\"captured_at\":\"2026-06-13T00:07:26.882543429+00:00\",\"command\":\"bash -lc 'target/debug/atelier status && target/debug/atelier lint && target/debug/atelier export --check && cargo nextest run test_root_status_summarizes_checkout_orientation test_root_status_reports_active_mission_contract_fields test_root_status_no_ready_work_suggests_valid_blocked_list test_workflow_configuration_docs_describe_internal_diagnostics test_workflow_help_is_scoped_as_advanced_internal_diagnostic test_agent_factory_guidance_avoids_raw_workflow_validate_commands test_mission_closeout_blocks_undeferred_obsolete_command_test'\",\"exit_code\":0,\"exit_status\":\"0\",\"kind\":\"validation\",\"output\":{\"limit_bytes_per_stream\":4096,\"stderr\":{\"bytes\":13200,\"summary\":\"   Compiling atelier-tracker v0.2.0 (/root/atelier)\\nwarning: struct `CreateInput` is never constructed\\n    --> src/commands/agent_factory.rs:1413:12\\n     |\\n1413 | pub struct CreateInput<'a> {\\n     |            ^^^^^^^^^^^\\n     |\\n     = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default\\n\\nwarning: function `create` is never used\\n    --> src/commands/agent_factory.rs:1541:8\\n     |\\n1541 | pub fn create(db: &Database, input: CreateInput<'_>) -> Result<()> {\\n     |        ^^^^^^\\n\\nwarning: function `update` is never used\\n    --> src/commands/agent_factory.rs:1591:8\\n     |\\n1591 | pub fn update(db: &Database, input: UpdateInput<'_>) -> Result<()> {\\n     |        ^^^^^^\\n\\nwarning: function `close` is never used\\n    --> src/commands/agent_factory.rs:1935:8\\n     |\\n1935 | pub fn close(db: &Database, issue_ref: &str, reason: Option<&str>) -> Result<()> {\\n     |        ^^^^^\\n\\nwarning: function `ensure_canonical_issue_sections_valid` is never used\\n    --> src/commands/agent_factory.rs:2007:4\\n     |\\n2007 | fn ensure_canonical_issue_sections_valid(issue_id: &str) -> Result<()> {\\n     |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\\n\\nwarning: function `reopen` is never used\\n    --> src/commands/agent_factory.rs:2015:8\\n     |\\n2015 | pub fn reopen(db: &Database, issue_ref: &str) -> Result<()> {\\n     |        ^^^^^^\\n\\nwarning: function `dep_add` is never used\\n    --> src/commands/agent_factory.rs:2143:8\\n     |\\n2143 | pub fn dep_add(db: &Database, blocked_ref: &str, blocker_ref: &str) -> Result<()> {\\n     |        ^^^^^^^\\n\\nwarning: function `dep_remove` is never used\\n    --> src/commands/agent_factory.rs:2162:8\\n     |\\n2162 | pub fn dep_remove(db: &Database, blocked_ref: &str, blocker_ref: &str) -> Result<()> {\\n     |        ^^^^^^^^^^\\n\\nwarning: function `run` is never used\\n  --> src/commands/comment.rs:21:8\\n   |\\n21 | pub fn run(db: &Database, issue_id: &str, content: &str, kind: &str) -> Result<()> {\\n   |        ^^^\\n\\nwarning: struct `CreateOpts` is never constructed\\n  --> src/commands/create.rs:79:12\\n   |\\n79 | pub struct CreateOpts<'a> {\\n   |            ^^^^^^^^^^\\n\\nwarning: function `run` is never used\\n  --> src/commands/create.rs:86:8\\n   |\\n86 | pub fn run(\\n   |        ^^^\\n\\nwarning: function `run_subissue` is never used\\n   --> src/commands/create.rs:175:8\\n    |\\n175 | pub fn run_subissue(\\n    |        ^^^^^^^^^^^^\\n\\nwarning: function `run` is never used\\n --> src/commands/delete.rs:9:8\\n  |\\n9 | pub fn run(db: &Database, id: &str, force: bool) -> Result<()> {\\n  |        ^^^\\n\\nwarning: function `add` is never used\\n --> src/commands/label.rs:7:8\\n  |\\n7 | pub fn add(db: &Database, issue_id: &str, label: &str) -> Result<()> {\\n  |        ^^^\\n\\nwarning: function `remove` is never used\\n  --> src/commands/label.rs:50:8\\n   |\\n50 | pub fn remove(db: &Database, issue_id: &str, label: &str) -> Result<()> {\\n   |        ^^^^^^\\n\\nwarning: function `refresh_open_database_after_canonical_write` is never used\\n  --> src/commands/projection.rs:26:8\\n   |\\n26 | pub fn refresh_open_database_after_canonical_write(\\n   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\\n\\nwarning: function `add_typed` is never used\\n --> src/commands/relate.rs:7:8\\n  |\\n7 | pub fn add_typed(\\n  |        ^^^^^^^^^\\n\\nwarning: function `remove_typed` is never used\\n  --> src/commands/relate.rs:66:8\\n   |\\n66 | pub fn remove_typed(\\n   |        ^^^^^^^^^^^^\\n\\nwarning: function `close_all` is never used\\n   --> src/commands/status.rs:436:8\\n    |\\n436 | pub fn close_all(\\n    |        ^^^^^^^^^\\n\\nwarning: method `remove_dependency` is never used\\n  --> src/db/dependencies.rs:53:12\\n   |\\n 7 | impl Database {\\n   | ------------- method in this implementation\\n...\\n53 |     pub fn remove_dependency(\\n   |            ^^^^^^^^^^^^^^^^^\\n\\nwarning: multiple methods are never used\\n   --> src/db/issues.rs:54:12\\n    |\\n 10 | impl Database {\\n    | ------------- methods in this implementation\\n...\\n 54 |     pub fn create_issue(\\n    |            ^^^^^^^^^^^^\\n...\\n 63 |     pub fn create_subissue(\\n    |            ^^^^^^^^^^^^^^^\\n...\\n 73 |     pub fn create_issue_with_type(\\n    |            ^^^^^^^^^^^^^^^^^^^^^^\\n...\\n 83 |     pu\",\"truncated\":true},\"stdout\":{\"bytes\":1637,\"summary\":\"Atelier Status\\n==============\\nTracker:       current\\nReady work:    3\\nActive work:   none\\nActive mission: atelier-tcmr - Repair CLI workflow rework and validation gaps\\n\\nLocal State\\n-----------\\nBranch:   codex/orchestrate-atelier-fork...origin/codex/orchestrate-atelier-fork [ahead 54]\\nWorktree: dirty (6 entries)\\n   M .atelier/issues/atelier-wpyb.md\\n   M docs/product/workflow-configuration.md\\n   M src/commands/status.rs\\nTracker:  current\\n\\nActive Mission\\n--------------\\natelier-tcmr - Repair CLI workflow rework and validation gaps\\nHealth:   blocked\\nWork:     ready 3, blocked 3, done 45, backlog 0\\n\\nReady In Active Mission\\n-----------------------\\n  atelier-sdmo - Repair signpost validation gaps\\n  atelier-v9id - Adversarially validate repair mission outcomes\\n  atelier-ymfl - Align docs tests and Agent Factory process with enforced proof\\n\\nImmediate Blockers\\n------------------\\n  atelier-sdmo - Repair signpost validation gaps\\n  atelier-trr2 - Validate workflow signpost surfaces end to end\\n  atelier-v9id - Adversarially validate repair mission outcomes\\n\\nRecent Activity\\n---------------\\n  atelier-sdmo work_started: Started work\\n  atelier-9pkx work_finished: Finished work\\n  atelier-9pkx close_reason: Recorded close reason\\n\\nNext Actions\\n------------\\n  Inspect active mission health (atelier-tcmr): atelier mission status atelier-tcmr\\n  Open active mission record (atelier-tcmr): atelier mission show atelier-tcmr\\n  Start ready active-mission work (3 ready issue(s)): atelier start atelier-sdmo\\n  Check runtime health (tracker export is current): atelier doctor\\nLint passed.\\nCanonical export is current\\nState: /root/atelier/.atelier\\n\",\"truncated\":false}},\"path\":null,\"producer\":null,\"result\":\"pass\",\"spawn_error\":null,\"success\":true,\"target\":{\"id\":\"atelier-sdmo\",\"kind\":\"issue\",\"role\":\"validates\"},\"uri\":null}"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-sdmo"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Signpost gap repair validation: target/debug/atelier status shows git and worktree state, active work, active mission health and progress, active-mission ready work, immediate blockers, recent activity, and domain next actions; workflow configuration docs describe hidden advanced/internal diagnostics instead of a normal JSON workflow validate surface; focused tests, lint, and export check pass."
updated_at: "2026-06-13T00:07:33.290326666+00:00"
---

Signpost gap repair validation: target/debug/atelier status shows git and worktree state, active work, active mission health and progress, active-mission ready work, immediate blockers, recent activity, and domain next actions; workflow configuration docs describe hidden advanced/internal diagnostics instead of a normal JSON workflow validate surface; focused tests, lint, and export check pass.

Command: bash -lc 'target/debug/atelier status && target/debug/atelier lint && target/debug/atelier export --check && cargo nextest run test_root_status_summarizes_checkout_orientation test_root_status_reports_active_mission_contract_fields test_root_status_no_ready_work_suggests_valid_blocked_list test_workflow_configuration_docs_describe_internal_diagnostics test_workflow_help_is_scoped_as_advanced_internal_diagnostic test_agent_factory_guidance_avoids_raw_workflow_validate_commands test_mission_closeout_blocks_undeferred_obsolete_command_test'
Exit status: 0

Stdout summary:
Atelier Status
==============
Tracker:       current
Ready work:    3
Active work:   none
Active mission: atelier-tcmr - Repair CLI workflow rework and validation gaps

Local State
-----------
Branch:   codex/orchestrate-atelier-fork...origin/codex/orchestrate-atelier-fork [ahead 54]
Worktree: dirty (6 entries)
   M .atelier/issues/atelier-wpyb.md
   M docs/product/workflow-configuration.md
   M src/commands/status.rs
Tracker:  current

Active Mission
--------------
atelier-tcmr - Repair CLI workflow rework and validation gaps
Health:   blocked
Work:     ready 3, blocked 3, done 45, backlog 0

Ready In Active Mission
-----------------------
  atelier-sdmo - Repair signpost validation gaps
  atelier-v9id - Adversarially validate repair mission outcomes
  atelier-ymfl - Align docs tests and Agent Factory process with enforced proof

Immediate Blockers
------------------
  atelier-sdmo - Repair signpost validation gaps
  atelier-trr2 - Validate workflow signpost surfaces end to end
  atelier-v9id - Adversarially validate repair mission outcomes

Recent Activity
---------------
  atelier-sdmo work_started: Started work
  atelier-9pkx work_finished: Finished work
  atelier-9pkx close_reason: Recorded close reason

Next Actions
------------
  Inspect active mission health (atelier-tcmr): atelier mission status atelier-tcmr
  Open active mission record (atelier-tcmr): atelier mission show atelier-tcmr
  Start ready active-mission work (3 ready issue(s)): atelier start atelier-sdmo
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
   --> src/commands/status.rs:436:8
    |
436 | pub fn close_all(
    |        ^^^^^^^^^

warning: method `remove_dependency` is never used
  --> src/db/dependencies.rs:53:12
   |
 7 | impl Database {
   | ------------- method in this implementation
...
53 |     pub fn remove_dependency(
   |            ^^^^^^^^^^^^^^^^^

warning: multiple methods are never used
   --> src/db/issues.rs:54:12
    |
 10 | impl Database {
    | ------------- methods in this implementation
...
 54 |     pub fn create_issue(
    |            ^^^^^^^^^^^^
...
 63 |     pub fn create_subissue(
    |            ^^^^^^^^^^^^^^^
...
 73 |     pub fn create_issue_with_type(
    |            ^^^^^^^^^^^^^^^^^^^^^^
...
 83 |     pu

