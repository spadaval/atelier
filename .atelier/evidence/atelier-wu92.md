---
created_at: "2026-06-13T00:26:37.725952665+00:00"
id: "atelier-wu92"
evidence_type: "validation"
captured_at: "2026-06-13T00:26:31.397411680+00:00"
command: "bash -lc 'atelier issue show atelier-wpyb && atelier history --issue atelier-wpyb --include-descendants --limit 60 && atelier evidence show atelier-bxjl && atelier evidence show atelier-0c13 && atelier evidence show atelier-x23c && cargo nextest run test_spec_representative_commands_match_signpost_surfaces test_workflow_configuration_docs_describe_internal_diagnostics test_workflow_help_is_scoped_as_advanced_internal_diagnostic test_agent_factory_guidance_avoids_raw_workflow_validate_commands test_root_status_guides_active_work_to_finish_not_start test_root_status_reports_active_mission_contract_fields test_root_status_summarizes_checkout_orientation test_root_status_no_ready_work_suggests_valid_blocked_list test_root_start_finish_and_issue_transition_surface test_prime_guides_empty_checkout_without_repeating_status test_prime_names_active_mission test_prime_names_active_work test_history_repo_wide_supports_filters_bounded_output_and_drill_downs test_history_mission_scope_includes_linked_work_descendants_and_evidence test_history_issue_scope_defaults_single_issue_and_can_include_descendants test_history_empty_states_and_invalid_limit test_mission_status_cli_reports_control_state test_active_mission_focus_guides_status_and_work test_mission_help_uses_show_not_view && atelier lint atelier-wpyb && atelier export --check && git diff --check'"
exit_status: "0"
path: null
uri: null
proof_scope: null
agent_identity: null
independence_level: null
follow_up_ids: []
residual_risks: []
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-wpyb"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Signpost epic closeout pass: all wpyb children are closed; trr2 validation evidence verifies root status, start finish/current-work, transition, history, prime, docs/help/SPEC/Agent Factory parity, and raw validator hiding; focused signpost tests, lint, export check, and diff check pass."
updated_at: "2026-06-13T00:26:38.711503035+00:00"
---

Signpost epic closeout pass: all wpyb children are closed; trr2 validation evidence verifies root status, start finish/current-work, transition, history, prime, docs/help/SPEC/Agent Factory parity, and raw validator hiding; focused signpost tests, lint, export check, and diff check pass.

Command: bash -lc 'atelier issue show atelier-wpyb && atelier history --issue atelier-wpyb --include-descendants --limit 60 && atelier evidence show atelier-bxjl && atelier evidence show atelier-0c13 && atelier evidence show atelier-x23c && cargo nextest run test_spec_representative_commands_match_signpost_surfaces test_workflow_configuration_docs_describe_internal_diagnostics test_workflow_help_is_scoped_as_advanced_internal_diagnostic test_agent_factory_guidance_avoids_raw_workflow_validate_commands test_root_status_guides_active_work_to_finish_not_start test_root_status_reports_active_mission_contract_fields test_root_status_summarizes_checkout_orientation test_root_status_no_ready_work_suggests_valid_blocked_list test_root_start_finish_and_issue_transition_surface test_prime_guides_empty_checkout_without_repeating_status test_prime_names_active_mission test_prime_names_active_work test_history_repo_wide_supports_filters_bounded_output_and_drill_downs test_history_mission_scope_includes_linked_work_descendants_and_evidence test_history_issue_scope_defaults_single_issue_and_can_include_descendants test_history_empty_states_and_invalid_limit test_mission_status_cli_reports_control_state test_active_mission_focus_guides_status_and_work test_mission_help_uses_show_not_view && atelier lint atelier-wpyb && atelier export --check && git diff --check'
Exit status: 0

Stdout summary (truncated):
atelier-wpyb [epic] open - Repair status start history prime and transition surfaces
====================================================================================
Status:   open
Type:     epic
Priority: high
Created:  2026-06-12 00:58 -04:00
Updated:  2026-06-12 20:20 -04:00
Labels:   cli, rework, workflow
File:     /root/atelier/.atelier/issues/atelier-wpyb.md

Hierarchy
---------
Parent: (none)

Transition Readiness
--------------------
  start: ready - issue is open and required sections parse
    atelier start atelier-wpyb
  close: blocked - missing issue-level proof; capture passing evidence or attach existing evidence
  options: atelier issue transition atelier-wpyb --options

Description
-----------
Repair missing or underpowered workflow signpost surfaces from the previous
mission: root status, normal start flow, history, prime, and issue transition
options. Use the closed signpost decisions in `atelier-rqvv`, `atelier-v02t`,
`atelier-vr9g`, `atelier-hggl`, and `atelier-bzts`; this epic should implement
or explicitly supersede those decisions, not reopen them implicitly.

Outcome
-------
- Root `atelier status` shows checkout state, active work, active mission,
  tracker health, recent relevant activity, blockers, ready work, and
  active-mission-scoped next actions.
- `atelier start <issue-id>` exists as the normal work entrypoint.
- The `atelier work` command group is exploded or hidden from the normal
  workflow; start, finish, and current-work orientation move to domain/root
  surfaces.
- Issue transition/options output exists so users can ask what an issue can do
  next without knowing internal validator commands.
- History and prime surfaces follow their closed contracts or have new
  superseding tracker items.
- User-facing next actions stop pushing normal users toward raw workflow
  validator commands when a status/transition surface should own that guidance.
- Any remaining workflow-validator implementation is hidden behind domain
  status/transition/closeout surfaces instead of appearing in normal help.
- Help, docs, and tests agree on the implemented surfaces.

Evidence
--------
- Transcript tests cover empty and active root status, start behavior, finish or
  equivalent current-work completion behavior, issue transition/options output,
  and retained or removed history/prime behavior.

- Negative transcripts prove normal next actions do not route users to raw
  workflow-validator commands when a domain status, transition, start, or
  closeout surface owns the answer.

- Tests cover the absence or replacement behavior for intentionally removed
  surfaces.

- Docs/help parity checks show Agent Factory guidance and repository docs match
  the implemented status, start, finish/current-work, transition, history, and
  prime surfaces.

- Run focused CLI integration tests for the repaired workflow signposts plus
  `atelier lint`.

Notes
-----
The product decisions for status, mission status/show, transitions, history, and
prime are in closed decision records. This epic owns implementation and
validation against those decisions.

Blocked by
----------
  atelier-trr2 [closed] high - Validate workflow signpost surfaces end to end

Blocking
--------
(none)

Subissues
---------
8 total | status: closed=8 | priority: high=8
  atelier-a4rz [closed] high - Repair active-work root status guidance
  atelier-cany [closed] high - Implement issue transition options surface
  atelier-f3p6 [closed] high - Explode work command group into domain start finish and status surfaces
  atelier-sckq [closed] high - Repair root and mission status signpost surfaces
  atelier-sdmo [closed] high - Repair signpost validation gaps
  atelier-trr2 [closed] high - Validate workflow signpost surfaces end to end
  atelier-u4nx [closed] high - Repair history and prime signpost surfaces
  atelier-ywe6 [closed] high - Repair SPEC workflow command drift

Recent Activity
---------------
(none)

Next Commands
-------------
  Edit issue Markdown: /root/atelier/.atelier/issues/atelier-wpyb.md
  Validate this issue: atelier lint atelier-wpyb

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

