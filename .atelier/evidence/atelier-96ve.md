---
created_at: "2026-06-12T23:53:23.343111536+00:00"
id: "atelier-96ve"
evidence_type: "validation"
captured_at: "2026-06-12T23:53:18.943788050+00:00"
command: "cargo nextest run test_issue_closeout_refuses_structurally_invalid_issue test_mission_closeout_enforces_gates_and_reopen_skips_close_validators test_dirty_worktree_blocks_mission_closeout test_mission_status_names_concrete_closeout_blockers test_mission_status_names_stale_and_malformed_record_blockers test_orientation_commands_enter_degraded_mode_for_malformed_records test_workflow_validate_defaults_are_target_and_transition_aware test_mission_audit_reports_missing_partial_and_ready_proof test_mission_closeout_uses_contract_audit test_epic_closeout_requires_closed_children_and_parent_evidence test_lint_rejects_vague_evidence_even_when_notes_name_a_command test_lint_rejects_missing_evidence_section test_lint_accepts_concrete_evidence_without_optional_notes"
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
    id: "atelier-nzy1"
    role: "validates"
  - kind: "issue"
    id: "atelier-pyre"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Closeout proof gate scenario matrix: issue closeout rejects malformed/missing proof; mission closeout rejects open work, missing mission proof, contract-audit gaps, dirty worktree, stale/malformed records; attached evidence satisfies issue and mission proof; domain status and closeout surfaces name the same blocker classes and normal recovery commands without requiring raw workflow validation."
updated_at: "2026-06-12T23:54:56.406451975+00:00"
---

Closeout proof gate scenario matrix: issue closeout rejects malformed/missing proof; mission closeout rejects open work, missing mission proof, contract-audit gaps, dirty worktree, stale/malformed records; attached evidence satisfies issue and mission proof; domain status and closeout surfaces name the same blocker classes and normal recovery commands without requiring raw workflow validation.

Command: cargo nextest run test_issue_closeout_refuses_structurally_invalid_issue test_mission_closeout_enforces_gates_and_reopen_skips_close_validators test_dirty_worktree_blocks_mission_closeout test_mission_status_names_concrete_closeout_blockers test_mission_status_names_stale_and_malformed_record_blockers test_orientation_commands_enter_degraded_mode_for_malformed_records test_workflow_validate_defaults_are_target_and_transition_aware test_mission_audit_reports_missing_partial_and_ready_proof test_mission_closeout_uses_contract_audit test_epic_closeout_requires_closed_children_and_parent_evidence test_lint_rejects_vague_evidence_even_when_notes_name_a_command test_lint_rejects_missing_evidence_section test_lint_accepts_concrete_evidence_without_optional_notes
Exit status: 0

Stdout summary:
(none)

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

