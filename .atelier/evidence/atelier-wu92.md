---
created_at: "2026-06-13T00:26:37.725952665+00:00"
id: "atelier-wu92"
data: "{\"captured_at\":\"2026-06-13T00:26:31.397411680+00:00\",\"command\":\"bash -lc 'atelier issue show atelier-wpyb && atelier history --issue atelier-wpyb --include-descendants --limit 60 && atelier evidence show atelier-bxjl && atelier evidence show atelier-0c13 && atelier evidence show atelier-x23c && cargo nextest run test_spec_representative_commands_match_signpost_surfaces test_workflow_configuration_docs_describe_internal_diagnostics test_workflow_help_is_scoped_as_advanced_internal_diagnostic test_agent_factory_guidance_avoids_raw_workflow_validate_commands test_root_status_guides_active_work_to_finish_not_start test_root_status_reports_active_mission_contract_fields test_root_status_summarizes_checkout_orientation test_root_status_no_ready_work_suggests_valid_blocked_list test_root_start_finish_and_issue_transition_surface test_prime_guides_empty_checkout_without_repeating_status test_prime_names_active_mission test_prime_names_active_work test_history_repo_wide_supports_filters_bounded_output_and_drill_downs test_history_mission_scope_includes_linked_work_descendants_and_evidence test_history_issue_scope_defaults_single_issue_and_can_include_descendants test_history_empty_states_and_invalid_limit test_mission_status_cli_reports_control_state test_active_mission_focus_guides_status_and_work test_mission_help_uses_show_not_view && atelier lint atelier-wpyb && atelier export --check && git diff --check'\",\"exit_code\":0,\"exit_status\":\"0\",\"kind\":\"validation\",\"output\":{\"limit_bytes_per_stream\":4096,\"stderr\":{\"bytes\":14627,\"summary\":\"   Compiling atelier-tracker v0.2.0 (/root/atelier)\\nwarning: struct `CreateInput` is never constructed\\n    --> src/commands/agent_factory.rs:1413:12\\n     |\\n1413 | pub struct CreateInput<'a> {\\n     |            ^^^^^^^^^^^\\n     |\\n     = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default\\n\\nwarning: function `create` is never used\\n    --> src/commands/agent_factory.rs:1541:8\\n     |\\n1541 | pub fn create(db: &Database, input: CreateInput<'_>) -> Result<()> {\\n     |        ^^^^^^\\n\\nwarning: function `update` is never used\\n    --> src/commands/agent_factory.rs:1591:8\\n     |\\n1591 | pub fn update(db: &Database, input: UpdateInput<'_>) -> Result<()> {\\n     |        ^^^^^^\\n\\nwarning: function `close` is never used\\n    --> src/commands/agent_factory.rs:1935:8\\n     |\\n1935 | pub fn close(db: &Database, issue_ref: &str, reason: Option<&str>) -> Result<()> {\\n     |        ^^^^^\\n\\nwarning: function `ensure_canonical_issue_sections_valid` is never used\\n    --> src/commands/agent_factory.rs:2007:4\\n     |\\n2007 | fn ensure_canonical_issue_sections_valid(issue_id: &str) -> Result<()> {\\n     |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\\n\\nwarning: function `reopen` is never used\\n    --> src/commands/agent_factory.rs:2015:8\\n     |\\n2015 | pub fn reopen(db: &Database, issue_ref: &str) -> Result<()> {\\n     |        ^^^^^^\\n\\nwarning: function `dep_add` is never used\\n    --> src/commands/agent_factory.rs:2143:8\\n     |\\n2143 | pub fn dep_add(db: &Database, blocked_ref: &str, blocker_ref: &str) -> Result<()> {\\n     |        ^^^^^^^\\n\\nwarning: function `dep_remove` is never used\\n    --> src/commands/agent_factory.rs:2162:8\\n     |\\n2162 | pub fn dep_remove(db: &Database, blocked_ref: &str, blocker_ref: &str) -> Result<()> {\\n     |        ^^^^^^^^^^\\n\\nwarning: function `run` is never used\\n  --> src/commands/comment.rs:21:8\\n   |\\n21 | pub fn run(db: &Database, issue_id: &str, content: &str, kind: &str) -> Result<()> {\\n   |        ^^^\\n\\nwarning: struct `CreateOpts` is never constructed\\n  --> src/commands/create.rs:79:12\\n   |\\n79 | pub struct CreateOpts<'a> {\\n   |            ^^^^^^^^^^\\n\\nwarning: function `run` is never used\\n  --> src/commands/create.rs:86:8\\n   |\\n86 | pub fn run(\\n   |        ^^^\\n\\nwarning: function `run_subissue` is never used\\n   --> src/commands/create.rs:175:8\\n    |\\n175 | pub fn run_subissue(\\n    |        ^^^^^^^^^^^^\\n\\nwarning: function `run` is never used\\n --> src/commands/delete.rs:9:8\\n  |\\n9 | pub fn run(db: &Database, id: &str, force: bool) -> Result<()> {\\n  |        ^^^\\n\\nwarning: function `add` is never used\\n --> src/commands/label.rs:7:8\\n  |\\n7 | pub fn add(db: &Database, issue_id: &str, label: &str) -> Result<()> {\\n  |        ^^^\\n\\nwarning: function `remove` is never used\\n  --> src/commands/label.rs:50:8\\n   |\\n50 | pub fn remove(db: &Database, issue_id: &str, label: &str) -> Result<()> {\\n   |        ^^^^^^\\n\\nwarning: function `refresh_open_database_after_canonical_write` is never used\\n  --> src/commands/projection.rs:26:8\\n   |\\n26 | pub fn refresh_open_database_after_canonical_write(\\n   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\\n\\nwarning: function `add_typed` is never used\\n --> src/commands/relate.rs:7:8\\n  |\\n7 | pub fn add_typed(\\n  |        ^^^^^^^^^\\n\\nwarning: function `remove_typed` is never used\\n  --> src/commands/relate.rs:66:8\\n   |\\n66 | pub fn remove_typed(\\n   |        ^^^^^^^^^^^^\\n\\nwarning: function `close_all` is never used\\n   --> src/commands/status.rs:473:8\\n    |\\n473 | pub fn close_all(\\n    |        ^^^^^^^^^\\n\\nwarning: method `remove_dependency` is never used\\n  --> src/db/dependencies.rs:53:12\\n   |\\n 7 | impl Database {\\n   | ------------- method in this implementation\\n...\\n53 |     pub fn remove_dependency(\\n   |            ^^^^^^^^^^^^^^^^^\\n\\nwarning: multiple methods are never used\\n   --> src/db/issues.rs:54:12\\n    |\\n 10 | impl Database {\\n    | ------------- methods in this implementation\\n...\\n 54 |     pub fn create_issue(\\n    |            ^^^^^^^^^^^^\\n...\\n 63 |     pub fn create_subissue(\\n    |            ^^^^^^^^^^^^^^^\\n...\\n 73 |     pub fn create_issue_with_type(\\n    |            ^^^^^^^^^^^^^^^^^^^^^^\\n...\\n 83 |     pu\",\"truncated\":true},\"stdout\":{\"bytes\":62490,\"summary\":\"atelier-wpyb [epic] open - Repair status start history prime and transition surfaces\\n====================================================================================\\nStatus:   open\\nType:     epic\\nPriority: high\\nCreated:  2026-06-12 00:58 -04:00\\nUpdated:  2026-06-12 20:20 -04:00\\nLabels:   cli, rework, workflow\\nFile:     /root/atelier/.atelier/issues/atelier-wpyb.md\\n\\nHierarchy\\n---------\\nParent: (none)\\n\\nTransition Readiness\\n--------------------\\n  start: ready - issue is open and required sections parse\\n    atelier start atelier-wpyb\\n  close: blocked - missing issue-level proof; capture passing evidence or attach existing evidence\\n  options: atelier issue transition atelier-wpyb --options\\n\\nDescription\\n-----------\\nRepair missing or underpowered workflow signpost surfaces from the previous\\nmission: root status, normal start flow, history, prime, and issue transition\\noptions. Use the closed signpost decisions in `atelier-rqvv`, `atelier-v02t`,\\n`atelier-vr9g`, `atelier-hggl`, and `atelier-bzts`; this epic should implement\\nor explicitly supersede those decisions, not reopen them implicitly.\\n\\nOutcome\\n-------\\n- Root `atelier status` shows checkout state, active work, active mission,\\n  tracker health, recent relevant activity, blockers, ready work, and\\n  active-mission-scoped next actions.\\n- `atelier start <issue-id>` exists as the normal work entrypoint.\\n- The `atelier work` command group is exploded or hidden from the normal\\n  workflow; start, finish, and current-work orientation move to domain/root\\n  surfaces.\\n- Issue transition/options output exists so users can ask what an issue can do\\n  next without knowing internal validator commands.\\n- History and prime surfaces follow their closed contracts or have new\\n  superseding tracker items.\\n- User-facing next actions stop pushing normal users toward raw workflow\\n  validator commands when a status/transition surface should own that guidance.\\n- Any remaining workflow-validator implementation is hidden behind domain\\n  status/transition/closeout surfaces instead of appearing in normal help.\\n- Help, docs, and tests agree on the implemented surfaces.\\n\\nEvidence\\n--------\\n- Transcript tests cover empty and active root status, start behavior, finish or\\n  equivalent current-work completion behavior, issue transition/options output,\\n  and retained or removed history/prime behavior.\\n\\n- Negative transcripts prove normal next actions do not route users to raw\\n  workflow-validator commands when a domain status, transition, start, or\\n  closeout surface owns the answer.\\n\\n- Tests cover the absence or replacement behavior for intentionally removed\\n  surfaces.\\n\\n- Docs/help parity checks show Agent Factory guidance and repository docs match\\n  the implemented status, start, finish/current-work, transition, history, and\\n  prime surfaces.\\n\\n- Run focused CLI integration tests for the repaired workflow signposts plus\\n  `atelier lint`.\\n\\nNotes\\n-----\\nThe product decisions for status, mission status/show, transitions, history, and\\nprime are in closed decision records. This epic owns implementation and\\nvalidation against those decisions.\\n\\nBlocked by\\n----------\\n  atelier-trr2 [closed] high - Validate workflow signpost surfaces end to end\\n\\nBlocking\\n--------\\n(none)\\n\\nSubissues\\n---------\\n8 total | status: closed=8 | priority: high=8\\n  atelier-a4rz [closed] high - Repair active-work root status guidance\\n  atelier-cany [closed] high - Implement issue transition options surface\\n  atelier-f3p6 [closed] high - Explode work command group into domain start finish and status surfaces\\n  atelier-sckq [closed] high - Repair root and mission status signpost surfaces\\n  atelier-sdmo [closed] high - Repair signpost validation gaps\\n  atelier-trr2 [closed] high - Validate workflow signpost surfaces end to end\\n  atelier-u4nx [closed] high - Repair history and prime signpost surfaces\\n  atelier-ywe6 [closed] high - Repair SPEC workflow command drift\\n\\nRecent Activity\\n---------------\\n(none)\\n\\nNext Commands\\n-------------\\n  Edit issue Markdown: /root/atelier/.atelier/issues/atelier-wpyb.md\\n  Validate this issue: atelier lint atelier-wpyb\\n  \",\"truncated\":true}},\"path\":null,\"producer\":null,\"result\":\"pass\",\"spawn_error\":null,\"success\":true,\"target\":{\"id\":\"atelier-wpyb\",\"kind\":\"issue\",\"role\":\"validates\"},\"uri\":null}"
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

