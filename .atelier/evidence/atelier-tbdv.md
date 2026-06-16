---
created_at: "2026-06-12T22:27:28.015133241+00:00"
id: "atelier-tbdv"
evidence_type: "validation"
captured_at: "2026-06-12T22:27:23.343261485+00:00"
command: "bash -lc '\nset -euo pipefail\ncargo nextest run --status-level fail --final-status-level pass --success-output never --failure-output final \\\n  mission_record_renders_and_parses_deterministically_without_data_blob \\\n  mission_render_normalizes_legacy_evidence_attachments \\\n  legacy_mission_data_record_loads_into_typed_sections_and_relationships \\\n  rebuild_round_trips_canonical_domain_records \\\n  test_first_class_detail_views_read_payloads_from_record_store \\\n  test_first_class_records_export_rebuild_and_validate \\\n  test_mission_relationship_filtering_keeps_supporting_records_out_of_work\n'"
exit_status: "0"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-6aor"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Focused mission record parsing rendering migration rebuild relationship tests"
updated_at: "2026-06-12T22:27:29.420316608+00:00"
---

Focused mission record parsing rendering migration rebuild relationship tests

Command: bash -lc '
set -euo pipefail
cargo nextest run --status-level fail --final-status-level pass --success-output never --failure-output final \
  mission_record_renders_and_parses_deterministically_without_data_blob \
  mission_render_normalizes_legacy_evidence_attachments \
  legacy_mission_data_record_loads_into_typed_sections_and_relationships \
  rebuild_round_trips_canonical_domain_records \
  test_first_class_detail_views_read_payloads_from_record_store \
  test_first_class_records_export_rebuild_and_validate \
  test_mission_relationship_filtering_keeps_supporting_records_out_of_work
'
Exit status: 0

Stdout summary:
(none)

Stderr summary (truncated):
   Compiling atelier-tracker v0.2.0 (/root/atelier)
warning: function `list_all_issue_activities` is never used
   --> src/activity.rs:125:8
    |
125 | pub fn list_all_issue_activities(state_dir: &Path) -> Result<Vec<IssueActivity>> {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: struct `CreateInput` is never constructed
    --> src/commands/agent_factory.rs:1252:12
     |
1252 | pub struct CreateInput<'a> {
     |            ^^^^^^^^^^^

warning: function `create` is never used
    --> src/commands/agent_factory.rs:1380:8
     |
1380 | pub fn create(db: &Database, input: CreateInput<'_>) -> Result<()> {
     |        ^^^^^^

warning: function `update` is never used
    --> src/commands/agent_factory.rs:1430:8
     |
1430 | pub fn update(db: &Database, input: UpdateInput<'_>) -> Result<()> {
     |        ^^^^^^

warning: function `close` is never used
    --> src/commands/agent_factory.rs:1757:8
     |
1757 | pub fn close(db: &Database, issue_ref: &str, reason: Option<&str>) -> Result<()> {
     |        ^^^^^

warning: function `ensure_canonical_issue_sections_valid` is never used
    --> src/commands/agent_factory.rs:1846:4
     |
1846 | fn ensure_canonical_issue_sections_valid(issue_id: &str) -> Result<()> {
     |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `reopen` is never used
    --> src/commands/agent_factory.rs:1854:8
     |
1854 | pub fn reopen(db: &Database, issue_ref: &str) -> Result<()> {
     |        ^^^^^^

warning: function `dep_add` is never used
    --> src/commands/agent_factory.rs:1982:8
     |
1982 | pub fn dep_add(db: &Database, blocked_ref: &str, blocker_ref: &str) -> Result<()> {
     |        ^^^^^^^

warning: function `dep_remove` is never used
    --> src/commands/agent_factory.rs:2001:8
     |
2001 | pub fn dep_remove(db: &Database, blocked_ref: &str, blocker_ref: &str) -> Result<()> {
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
...
53 |     pub fn remove_dependency(
   |            ^^^^^^^^^^^^^^^^^

warning: multiple methods are never used
   --> src/db/issues.rs:54:12
    |
 10 | impl Database {
    | ------------- methods in this implementation
...
 54 |     pub fn
