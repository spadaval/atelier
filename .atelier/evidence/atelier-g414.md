---
created_at: "2026-06-13T01:37:05.810670038+00:00"
id: "atelier-g414"
data: "{\"captured_at\":\"2026-06-13T01:36:25.241824837+00:00\",\"command\":\"bash -lc 'cargo fmt -- --check && cargo nextest run --cargo-quiet test_validation_issue_closeout_requires_contract_audit_evidence test_issue_closeout_rejects_evidence_attached_to_another_issue test_workflow_validate_fails_without_required_evidence test_workflow_validate_defaults_are_target_and_transition_aware test_closed_children_alone_do_not_close_epic_parent test_epic_closeout_requires_closed_children_and_parent_evidence test_evidence_capture_records_command_metadata_and_attaches_targets test_root_start_finish_and_issue_transition_surface && cargo nextest run --status-level fail --final-status-level fail --failure-output final --max-fail 1:immediate --cargo-quiet && cargo build --quiet && target/debug/atelier rebuild && target/debug/atelier lint atelier-xala && target/debug/atelier export --check && git diff --check'\",\"exit_code\":0,\"exit_status\":\"0\",\"kind\":\"validation\",\"output\":{\"limit_bytes_per_stream\":4096,\"stderr\":{\"bytes\":35601,\"summary\":\"warning: struct `CreateInput` is never constructed\\n    --> src/commands/agent_factory.rs:1525:12\\n     |\\n1525 | pub struct CreateInput<'a> {\\n     |            ^^^^^^^^^^^\\n     |\\n     = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default\\n\\nwarning: function `create` is never used\\n    --> src/commands/agent_factory.rs:1653:8\\n     |\\n1653 | pub fn create(db: &Database, input: CreateInput<'_>) -> Result<()> {\\n     |        ^^^^^^\\n\\nwarning: function `update` is never used\\n    --> src/commands/agent_factory.rs:1703:8\\n     |\\n1703 | pub fn update(db: &Database, input: UpdateInput<'_>) -> Result<()> {\\n     |        ^^^^^^\\n\\nwarning: function `close` is never used\\n    --> src/commands/agent_factory.rs:2047:8\\n     |\\n2047 | pub fn close(db: &Database, issue_ref: &str, reason: Option<&str>) -> Result<()> {\\n     |        ^^^^^\\n\\nwarning: function `ensure_canonical_issue_sections_valid` is never used\\n    --> src/commands/agent_factory.rs:2119:4\\n     |\\n2119 | fn ensure_canonical_issue_sections_valid(issue_id: &str) -> Result<()> {\\n     |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\\n\\nwarning: function `reopen` is never used\\n    --> src/commands/agent_factory.rs:2127:8\\n     |\\n2127 | pub fn reopen(db: &Database, issue_ref: &str) -> Result<()> {\\n     |        ^^^^^^\\n\\nwarning: function `dep_add` is never used\\n    --> src/commands/agent_factory.rs:2255:8\\n     |\\n2255 | pub fn dep_add(db: &Database, blocked_ref: &str, blocker_ref: &str) -> Result<()> {\\n     |        ^^^^^^^\\n\\nwarning: function `dep_remove` is never used\\n    --> src/commands/agent_factory.rs:2274:8\\n     |\\n2274 | pub fn dep_remove(db: &Database, blocked_ref: &str, blocker_ref: &str) -> Result<()> {\\n     |        ^^^^^^^^^^\\n\\nwarning: function `run` is never used\\n  --> src/commands/comment.rs:21:8\\n   |\\n21 | pub fn run(db: &Database, issue_id: &str, content: &str, kind: &str) -> Result<()> {\\n   |        ^^^\\n\\nwarning: struct `CreateOpts` is never constructed\\n  --> src/commands/create.rs:79:12\\n   |\\n79 | pub struct CreateOpts<'a> {\\n   |            ^^^^^^^^^^\\n\\nwarning: function `run` is never used\\n  --> src/commands/create.rs:86:8\\n   |\\n86 | pub fn run(\\n   |        ^^^\\n\\nwarning: function `run_subissue` is never used\\n   --> src/commands/create.rs:175:8\\n    |\\n175 | pub fn run_subissue(\\n    |        ^^^^^^^^^^^^\\n\\nwarning: function `run` is never used\\n --> src/commands/delete.rs:9:8\\n  |\\n9 | pub fn run(db: &Database, id: &str, force: bool) -> Result<()> {\\n  |        ^^^\\n\\nwarning: function `add` is never used\\n --> src/commands/label.rs:7:8\\n  |\\n7 | pub fn add(db: &Database, issue_id: &str, label: &str) -> Result<()> {\\n  |        ^^^\\n\\nwarning: function `remove` is never used\\n  --> src/commands/label.rs:50:8\\n   |\\n50 | pub fn remove(db: &Database, issue_id: &str, label: &str) -> Result<()> {\\n   |        ^^^^^^\\n\\nwarning: function `refresh_open_database_after_canonical_write` is never used\\n  --> src/commands/projection.rs:26:8\\n   |\\n26 | pub fn refresh_open_database_after_canonical_write(\\n   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\\n\\nwarning: function `add_typed` is never used\\n --> src/commands/relate.rs:7:8\\n  |\\n7 | pub fn add_typed(\\n  |        ^^^^^^^^^\\n\\nwarning: function `remove_typed` is never used\\n  --> src/commands/relate.rs:66:8\\n   |\\n66 | pub fn remove_typed(\\n   |        ^^^^^^^^^^^^\\n\\nwarning: function `close_all` is never used\\n   --> src/commands/status.rs:473:8\\n    |\\n473 | pub fn close_all(\\n    |        ^^^^^^^^^\\n\\nwarning: method `remove_dependency` is never used\\n  --> src/db/dependencies.rs:53:12\\n   |\\n 7 | impl Database {\\n   | ------------- method in this implementation\\n...\\n53 |     pub fn remove_dependency(\\n   |            ^^^^^^^^^^^^^^^^^\\n\\nwarning: multiple methods are never used\\n   --> src/db/issues.rs:50:12\\n    |\\n 11 | impl Database {\\n    | ------------- methods in this implementation\\n...\\n 50 |     pub fn create_issue(\\n    |            ^^^^^^^^^^^^\\n...\\n 59 |     pub fn create_subissue(\\n    |            ^^^^^^^^^^^^^^^\\n...\\n 69 |     pub fn create_issue_with_type(\\n    |            ^^^^^^^^^^^^^^^^^^^^^^\\n...\\n 79 |     pub fn create_subissue_with_type(\\n    |            ^^^\",\"truncated\":true},\"stdout\":{\"bytes\":239,\"summary\":\"Runtime state rebuilt\\nState:    /root/atelier/.atelier\\nDatabase: /root/atelier/.atelier/state.db\\n\\nNext Commands\\n-------------\\n  atelier doctor\\n  atelier export --check\\nLint passed.\\nCanonical export is current\\nState: /root/atelier/.atelier\\n\",\"truncated\":false}},\"path\":null,\"producer\":null,\"result\":\"pass\",\"spawn_error\":null,\"success\":true,\"target\":{\"id\":\"atelier-xala\",\"kind\":\"issue\",\"role\":\"validates\"},\"uri\":null}"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-xala"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Validation closeout gate requires contract-audit proof for high-risk issues"
updated_at: "2026-06-13T01:37:07.568305784+00:00"
---

Validation closeout gate requires contract-audit proof for high-risk issues

Command: bash -lc 'cargo fmt -- --check && cargo nextest run --cargo-quiet test_validation_issue_closeout_requires_contract_audit_evidence test_issue_closeout_rejects_evidence_attached_to_another_issue test_workflow_validate_fails_without_required_evidence test_workflow_validate_defaults_are_target_and_transition_aware test_closed_children_alone_do_not_close_epic_parent test_epic_closeout_requires_closed_children_and_parent_evidence test_evidence_capture_records_command_metadata_and_attaches_targets test_root_start_finish_and_issue_transition_surface && cargo nextest run --status-level fail --final-status-level fail --failure-output final --max-fail 1:immediate --cargo-quiet && cargo build --quiet && target/debug/atelier rebuild && target/debug/atelier lint atelier-xala && target/debug/atelier export --check && git diff --check'
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
    --> src/commands/agent_factory.rs:1525:12
     |
1525 | pub struct CreateInput<'a> {
     |            ^^^^^^^^^^^
     |
     = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `create` is never used
    --> src/commands/agent_factory.rs:1653:8
     |
1653 | pub fn create(db: &Database, input: CreateInput<'_>) -> Result<()> {
     |        ^^^^^^

warning: function `update` is never used
    --> src/commands/agent_factory.rs:1703:8
     |
1703 | pub fn update(db: &Database, input: UpdateInput<'_>) -> Result<()> {
     |        ^^^^^^

warning: function `close` is never used
    --> src/commands/agent_factory.rs:2047:8
     |
2047 | pub fn close(db: &Database, issue_ref: &str, reason: Option<&str>) -> Result<()> {
     |        ^^^^^

warning: function `ensure_canonical_issue_sections_valid` is never used
    --> src/commands/agent_factory.rs:2119:4
     |
2119 | fn ensure_canonical_issue_sections_valid(issue_id: &str) -> Result<()> {
     |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `reopen` is never used
    --> src/commands/agent_factory.rs:2127:8
     |
2127 | pub fn reopen(db: &Database, issue_ref: &str) -> Result<()> {
     |        ^^^^^^

warning: function `dep_add` is never used
    --> src/commands/agent_factory.rs:2255:8
     |
2255 | pub fn dep_add(db: &Database, blocked_ref: &str, blocker_ref: &str) -> Result<()> {
     |        ^^^^^^^

warning: function `dep_remove` is never used
    --> src/commands/agent_factory.rs:2274:8
     |
2274 | pub fn dep_remove(db: &Database, blocked_ref: &str, blocker_ref: &str) -> Result<()> {
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

