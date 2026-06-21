---
created_at: "2026-06-12T23:59:29.516877132+00:00"
id: "atelier-q58o"
evidence_type: "validation"
captured_at: "2026-06-12T23:59:24.562290551+00:00"
command: "bash -lc 'cargo nextest run test_workflow_validate_fails_without_required_evidence test_lint_rejects_vague_evidence_even_when_notes_name_a_command test_issue_closeout_rejects_evidence_attached_to_another_issue test_closed_children_alone_do_not_close_epic_parent test_mission_audit_reports_missing_partial_and_ready_proof test_mission_closeout_uses_contract_audit test_workflow_validate_reports_docs_help_root_surface_drift test_workflow_validate_reports_agent_factory_command_drift test_mission_closeout_blocks_undeferred_obsolete_command_test test_workflow_validate_reports_ignored_tests_without_owner test_mission_status_shows_ignored_product_behavior_closeout_blocker test_evidence_capture_rejects_failed_commands_as_pass_proof test_epic_closeout_requires_closed_children_and_parent_evidence && atelier issue show atelier-v6nd && atelier issue show atelier-hah9 && atelier issue show atelier-v9id'"
exit_status: "0"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-9pkx"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Validation failure scenario matrix for atelier-9pkx: missing proof, vague Evidence text, unrelated attached evidence, closed children with unproven parent outcomes, broad green checks that miss contract/docs/stale-test failures, docs/help drift, and implementer-only high-risk validation are all covered or guarded by tests, Agent Factory proof protocol, and the still-blocking independent mission validation item atelier-v9id; no follow-up items are needed from this matrix."
updated_at: "2026-06-12T23:59:30.384236321+00:00"
---

Validation failure scenario matrix for atelier-9pkx: missing proof, vague Evidence text, unrelated attached evidence, closed children with unproven parent outcomes, broad green checks that miss contract/docs/stale-test failures, docs/help drift, and implementer-only high-risk validation are all covered or guarded by tests, Agent Factory proof protocol, and the still-blocking independent mission validation item atelier-v9id; no follow-up items are needed from this matrix.

Command: bash -lc 'cargo nextest run test_workflow_validate_fails_without_required_evidence test_lint_rejects_vague_evidence_even_when_notes_name_a_command test_issue_closeout_rejects_evidence_attached_to_another_issue test_closed_children_alone_do_not_close_epic_parent test_mission_audit_reports_missing_partial_and_ready_proof test_mission_closeout_uses_contract_audit test_workflow_validate_reports_docs_help_root_surface_drift test_workflow_validate_reports_agent_factory_command_drift test_mission_closeout_blocks_undeferred_obsolete_command_test test_workflow_validate_reports_ignored_tests_without_owner test_mission_status_shows_ignored_product_behavior_closeout_blocker test_evidence_capture_rejects_failed_commands_as_pass_proof test_epic_closeout_requires_closed_children_and_parent_evidence && atelier issue show atelier-v6nd && atelier issue show atelier-hah9 && atelier issue show atelier-v9id'
Exit status: 0

Stdout summary (truncated):
atelier-v6nd [task] closed - Teach Agent Factory proof escalation rules
=======================================================================
Status:   closed
Type:     task
Priority: high
Created:  2026-06-12 15:39 -04:00
Updated:  2026-06-12 17:25 -04:00
Closed:   2026-06-12 17:25 -04:00
Assignee: root
Labels:   agent-factory, assignee:root, process, validation
File:     /root/atelier/.atelier/issues/atelier-v6nd.md

Hierarchy
---------
Parent: atelier-ymfl [open] high - Align docs tests and Agent Factory process with enforced proof

Transition Readiness
--------------------
  start: blocked - issue is closed
  close: blocked - issue is already closed
  reopen: ready - closed issue can be reopened for follow-up work
    atelier issue update atelier-v6nd --status open
  options: atelier issue transition atelier-v6nd --options

Description
-----------
Update Agent Factory so agents get one clear rule: ordinary work closes with
proof on the issue; risky, broad, or parent-level claims need an independent
check.

Outcome
-------
- Agent Factory guidance explains default proof and escalation in plain
  operational language.
- Orchestrator prompts name the proof expected for each assigned worker and
  create or block on validation issues only when independence is required.
- Implementers record the proof for their slice but do not act as independent
  validators for high-risk or parent-level claims.
- Review, validate, docs, and readiness subskills use the same proof language
  and failure classifications without competing terminology.
- Repository `AGENTFACTORY.md` is updated only where the local Atelier binding
  needs command-specific examples.

Evidence
--------
- Patch the relevant Agent Factory skill procedures and standards files under
  `/root/.agents/skills/agent-factory/`.
- Process review confirms the skill keeps ordinary issue proof lightweight and
  escalates risky, broad, or parent-level claims.
- Demonstrate the proof rule on at least one current `atelier-tcmr` issue or
  epic.
- Run relevant docs/process checks plus `atelier lint` and `atelier export
  --check`.

Close Reason
------------
Agent Factory proof escalation guidance implemented and evidence atelier-8fc1 attached.

Blocked by
----------
  atelier-6w0u [closed] high - Define validation routing policy for work items

Blocking
--------
  atelier-9pkx [open] high - Validate validation failure scenarios end to end
  atelier-wws5 [closed] high - Update Agent Factory skill for evidence-backed mission reliability

Subissues
---------
(none)

Recent Activity
---------------
  [2026-06-12 17:26 -04:00] work_finished: Finished work
  finished: true
  [2026-06-12 17:25 -04:00] close_reason: Recorded close reason
  Agent Factory proof escalation guidance implemented and evidence atelier-8fc1 attached.
  [2026-06-12 17:25 -04:00] status_changed: Changed status from open to closed
  Changed status: open -> closed
  [2026-06-12 17:25 -04:00] note: Added note
  Agent Factory skill changes are committed in /root/.agents-worktrees/atelier-v6nd on branch codex/atelier-v6nd at 6fd9949. Evidence atelier-8fc1 records the proof-routing review and tcmr demonstration: v6nd is process-policy proof with first-class evidence; ymfl parent claims remain escalated to validation/closeout work.
  [2026-06-12 17:25 -04:00] evidence_attached: Attached evidence atelier-8fc1
  evidence_id: "atelier-8fc1"
  result: "pass"
  [2026-06-12 17:21 -04:00] work_started: Started work
  branch: "codex/atelier-v6nd"
  worktree_path: "/root/atelier/.atelier-worktrees/atelier-v6nd"
  [2026-06-12 17:21 -04:00] note: Added note
  Claimed by root
  [2026-06-12 17:21 -04:00] field_changed: Changed assignee
  field: "assignee"
  old: null
  new: "root"

Next Commands
-------------
  Edit issue Markdown: /root/atelier/.atelier/issues/atelier-v6nd.md
  Validate this issue: atelier lint atelier-v6nd
  Add a note: atelier note add issue atelier-v6nd "..."
  Show full activity: atelier history --issue atelier-v6nd
  Show transition options: atelier issue transition atelier-v6nd --options
  Reopen this issue

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

