---
created_at: "2026-06-12T23:41:30.699521374+00:00"
id: "atelier-nptn"
data: "{\"captured_at\":\"2026-06-12T23:41:25.840651705+00:00\",\"command\":\"cargo nextest run test_mission_status_cli_reports_control_state test_mission_status_names_stale_and_malformed_record_blockers test_mission_status_names_concrete_closeout_blockers test_mission_status_shows_ignored_product_behavior_closeout_blocker test_workflow_validate_reports_docs_help_root_surface_drift\",\"exit_code\":0,\"exit_status\":\"0\",\"kind\":\"validation\",\"output\":{\"limit_bytes_per_stream\":4096,\"stderr\":{\"bytes\":12945,\"summary\":\"   Compiling atelier-tracker v0.2.0 (/root/atelier)\\nwarning: struct `CreateInput` is never constructed\\n    --> src/commands/agent_factory.rs:1396:12\\n     |\\n1396 | pub struct CreateInput<'a> {\\n     |            ^^^^^^^^^^^\\n     |\\n     = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default\\n\\nwarning: function `create` is never used\\n    --> src/commands/agent_factory.rs:1524:8\\n     |\\n1524 | pub fn create(db: &Database, input: CreateInput<'_>) -> Result<()> {\\n     |        ^^^^^^\\n\\nwarning: function `update` is never used\\n    --> src/commands/agent_factory.rs:1574:8\\n     |\\n1574 | pub fn update(db: &Database, input: UpdateInput<'_>) -> Result<()> {\\n     |        ^^^^^^\\n\\nwarning: function `close` is never used\\n    --> src/commands/agent_factory.rs:1918:8\\n     |\\n1918 | pub fn close(db: &Database, issue_ref: &str, reason: Option<&str>) -> Result<()> {\\n     |        ^^^^^\\n\\nwarning: function `ensure_canonical_issue_sections_valid` is never used\\n    --> src/commands/agent_factory.rs:1990:4\\n     |\\n1990 | fn ensure_canonical_issue_sections_valid(issue_id: &str) -> Result<()> {\\n     |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\\n\\nwarning: function `reopen` is never used\\n    --> src/commands/agent_factory.rs:1998:8\\n     |\\n1998 | pub fn reopen(db: &Database, issue_ref: &str) -> Result<()> {\\n     |        ^^^^^^\\n\\nwarning: function `dep_add` is never used\\n    --> src/commands/agent_factory.rs:2126:8\\n     |\\n2126 | pub fn dep_add(db: &Database, blocked_ref: &str, blocker_ref: &str) -> Result<()> {\\n     |        ^^^^^^^\\n\\nwarning: function `dep_remove` is never used\\n    --> src/commands/agent_factory.rs:2145:8\\n     |\\n2145 | pub fn dep_remove(db: &Database, blocked_ref: &str, blocker_ref: &str) -> Result<()> {\\n     |        ^^^^^^^^^^\\n\\nwarning: function `run` is never used\\n  --> src/commands/comment.rs:21:8\\n   |\\n21 | pub fn run(db: &Database, issue_id: &str, content: &str, kind: &str) -> Result<()> {\\n   |        ^^^\\n\\nwarning: struct `CreateOpts` is never constructed\\n  --> src/commands/create.rs:79:12\\n   |\\n79 | pub struct CreateOpts<'a> {\\n   |            ^^^^^^^^^^\\n\\nwarning: function `run` is never used\\n  --> src/commands/create.rs:86:8\\n   |\\n86 | pub fn run(\\n   |        ^^^\\n\\nwarning: function `run_subissue` is never used\\n   --> src/commands/create.rs:175:8\\n    |\\n175 | pub fn run_subissue(\\n    |        ^^^^^^^^^^^^\\n\\nwarning: function `run` is never used\\n --> src/commands/delete.rs:9:8\\n  |\\n9 | pub fn run(db: &Database, id: &str, force: bool) -> Result<()> {\\n  |        ^^^\\n\\nwarning: function `add` is never used\\n --> src/commands/label.rs:7:8\\n  |\\n7 | pub fn add(db: &Database, issue_id: &str, label: &str) -> Result<()> {\\n  |        ^^^\\n\\nwarning: function `remove` is never used\\n  --> src/commands/label.rs:50:8\\n   |\\n50 | pub fn remove(db: &Database, issue_id: &str, label: &str) -> Result<()> {\\n   |        ^^^^^^\\n\\nwarning: function `refresh_open_database_after_canonical_write` is never used\\n  --> src/commands/projection.rs:26:8\\n   |\\n26 | pub fn refresh_open_database_after_canonical_write(\\n   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\\n\\nwarning: function `add_typed` is never used\\n --> src/commands/relate.rs:7:8\\n  |\\n7 | pub fn add_typed(\\n  |        ^^^^^^^^^\\n\\nwarning: function `remove_typed` is never used\\n  --> src/commands/relate.rs:66:8\\n   |\\n66 | pub fn remove_typed(\\n   |        ^^^^^^^^^^^^\\n\\nwarning: function `close_all` is never used\\n   --> src/commands/status.rs:112:8\\n    |\\n112 | pub fn close_all(\\n    |        ^^^^^^^^^\\n\\nwarning: method `remove_dependency` is never used\\n  --> src/db/dependencies.rs:53:12\\n   |\\n 7 | impl Database {\\n   | ------------- method in this implementation\\n...\\n53 |     pub fn remove_dependency(\\n   |            ^^^^^^^^^^^^^^^^^\\n\\nwarning: multiple methods are never used\\n   --> src/db/issues.rs:54:12\\n    |\\n 10 | impl Database {\\n    | ------------- methods in this implementation\\n...\\n 54 |     pub fn create_issue(\\n    |            ^^^^^^^^^^^^\\n...\\n 63 |     pub fn create_subissue(\\n    |            ^^^^^^^^^^^^^^^\\n...\\n 73 |     pub fn create_issue_with_type(\\n    |            ^^^^^^^^^^^^^^^^^^^^^^\\n...\\n 83 |     pu\",\"truncated\":true},\"stdout\":{\"bytes\":0,\"summary\":\"\",\"truncated\":false}},\"path\":null,\"producer\":null,\"result\":\"pass\",\"spawn_error\":null,\"success\":true,\"target\":{\"id\":\"atelier-k9m8\",\"kind\":\"issue\",\"role\":\"validates\"},\"uri\":null}"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-k9m8"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Mission reliability status summary transcript tests"
updated_at: "2026-06-12T23:41:31.598210519+00:00"
---

Mission reliability status summary transcript tests

Command: cargo nextest run test_mission_status_cli_reports_control_state test_mission_status_names_stale_and_malformed_record_blockers test_mission_status_names_concrete_closeout_blockers test_mission_status_shows_ignored_product_behavior_closeout_blocker test_workflow_validate_reports_docs_help_root_surface_drift
Exit status: 0

Stdout summary:
(none)

Stderr summary (truncated):
   Compiling atelier-tracker v0.2.0 (/root/atelier)
warning: struct `CreateInput` is never constructed
    --> src/commands/agent_factory.rs:1396:12
     |
1396 | pub struct CreateInput<'a> {
     |            ^^^^^^^^^^^
     |
     = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `create` is never used
    --> src/commands/agent_factory.rs:1524:8
     |
1524 | pub fn create(db: &Database, input: CreateInput<'_>) -> Result<()> {
     |        ^^^^^^

warning: function `update` is never used
    --> src/commands/agent_factory.rs:1574:8
     |
1574 | pub fn update(db: &Database, input: UpdateInput<'_>) -> Result<()> {
     |        ^^^^^^

warning: function `close` is never used
    --> src/commands/agent_factory.rs:1918:8
     |
1918 | pub fn close(db: &Database, issue_ref: &str, reason: Option<&str>) -> Result<()> {
     |        ^^^^^

warning: function `ensure_canonical_issue_sections_valid` is never used
    --> src/commands/agent_factory.rs:1990:4
     |
1990 | fn ensure_canonical_issue_sections_valid(issue_id: &str) -> Result<()> {
     |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `reopen` is never used
    --> src/commands/agent_factory.rs:1998:8
     |
1998 | pub fn reopen(db: &Database, issue_ref: &str) -> Result<()> {
     |        ^^^^^^

warning: function `dep_add` is never used
    --> src/commands/agent_factory.rs:2126:8
     |
2126 | pub fn dep_add(db: &Database, blocked_ref: &str, blocker_ref: &str) -> Result<()> {
     |        ^^^^^^^

warning: function `dep_remove` is never used
    --> src/commands/agent_factory.rs:2145:8
     |
2145 | pub fn dep_remove(db: &Database, blocked_ref: &str, blocker_ref: &str) -> Result<()> {
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
   --> src/commands/status.rs:112:8
    |
112 | pub fn close_all(
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

