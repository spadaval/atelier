---
created_at: "2026-06-13T01:14:37.311182883+00:00"
id: "atelier-99ug"
evidence_type: "validation"
captured_at: "2026-06-13T01:14:14.528826144+00:00"
command: "bash -lc 'cargo nextest run --cargo-quiet test_dependency_chain test_list_filter_by_status test_list_all_statuses test_tree_with_status_filter test_next_with_subissue_progress test_next_only_subissues_ready test_stress_rapid_operations test_issue_mutations_are_durable_without_manual_export test_issue_mutations_create_activity_sidecars test_issue_show_json_recovers_activity_fields_after_rebuild test_mission_list_human_overview_orders_and_summarizes test_mission_list_default_current_empty_state test_issue_tree_status_filter test_dependency_chain_and_ready && target/debug/atelier rebuild && target/debug/atelier lint atelier-jspo && target/debug/atelier export --check && git diff --check'"
exit_status: "0"
path: null
uri: null
proof_scope: null
agent_identity: null
independence_level: null
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-jspo"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Success-close fixtures attach proof before close across integration and smoke flows"
updated_at: "2026-06-13T01:14:38.959832159+00:00"
---

Success-close fixtures attach proof before close across integration and smoke flows

Command: bash -lc 'cargo nextest run --cargo-quiet test_dependency_chain test_list_filter_by_status test_list_all_statuses test_tree_with_status_filter test_next_with_subissue_progress test_next_only_subissues_ready test_stress_rapid_operations test_issue_mutations_are_durable_without_manual_export test_issue_mutations_create_activity_sidecars test_issue_show_json_recovers_activity_fields_after_rebuild test_mission_list_human_overview_orders_and_summarizes test_mission_list_default_current_empty_state test_issue_tree_status_filter test_dependency_chain_and_ready && target/debug/atelier rebuild && target/debug/atelier lint atelier-jspo && target/debug/atelier export --check && git diff --check'
Exit status: 0

Stdout summary:
Runtime state rebuilt
State:    /root/atelier/.atelier
Database: /root/atelier/.atelier/state.db

Next Commands
-------------
  atelier doctor
  atelier export --check
Lint passed.
Canonical export is current
State: /root/atelier/.atelier

Stderr summary (truncated):
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
...
53 |     pub fn remove_dependency(
   |            ^^^^^^^^^^^^^^^^^

warning: multiple methods are never used
   --> src/db/issues.rs:50:12
    |
 11 | impl Database {
    | ------------- methods in this implementation
...
 50 |     pub fn create_issue(
    |            ^^^^^^^^^^^^
...
 59 |     pub fn create_subissue(
    |            ^^^^^^^^^^^^^^^
...
 69 |     pub fn create_issue_with_type(
    |            ^^^^^^^^^^^^^^^^^^^^^^
...
 79 |     pub fn create_subissue_with_type(
    |            ^^^

