---
created_at: "2026-06-13T01:37:05.810670038+00:00"
id: "atelier-g414"
evidence_type: "validation"
captured_at: "2026-06-13T01:36:25.241824837+00:00"
command: "bash -lc 'cargo fmt -- --check && cargo nextest run --cargo-quiet test_validation_issue_closeout_requires_contract_audit_evidence test_issue_closeout_rejects_evidence_attached_to_another_issue test_workflow_validate_fails_without_required_evidence test_workflow_validate_defaults_are_target_and_transition_aware test_closed_children_alone_do_not_close_epic_parent test_epic_closeout_requires_closed_children_and_parent_evidence test_evidence_capture_records_command_metadata_and_attaches_targets test_root_start_finish_and_issue_transition_surface && cargo nextest run --status-level fail --final-status-level fail --failure-output final --max-fail 1:immediate --cargo-quiet && cargo build --quiet && target/debug/atelier rebuild && target/debug/atelier lint atelier-xala && target/debug/atelier export --check && git diff --check'"
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

