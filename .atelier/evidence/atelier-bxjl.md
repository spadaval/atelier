---
created_at: "2026-06-13T00:25:39.958197445+00:00"
id: "atelier-bxjl"
evidence_type: "validation"
captured_at: "2026-06-13T00:25:32.743570810+00:00"
command: "bash -lc 'atelier --version && atelier status && atelier mission status atelier-tcmr && atelier mission show atelier-tcmr && atelier issue show atelier-trr2 && atelier issue show atelier-wpyb && atelier issue transition atelier-trr2 --options && atelier start --help && atelier finish --help && if atelier current-work --help; then exit 1; else true; fi && atelier work --help && atelier work status && atelier history --issue atelier-trr2 --limit 5 && atelier history --mission atelier-tcmr --limit 5 && atelier prime && atelier workflow --help && atelier workflow validate --help && ! rg -n \"atelier work start|atelier work finish|atelier workflow validate\" SPEC.md AGENTFACTORY.md && rg -n \"Root .*atelier start|hidden .*atelier work start/finish/status|atelier issue transition\" docs/product/cli-surface.md && rg -n \"Hidden advanced/internal workflow diagnostics|atelier lint|atelier doctor\" docs/product/workflow-configuration.md && rg -n \"atelier status|atelier mission status|atelier issue transition|atelier start|atelier finish\" AGENTFACTORY.md docs/product/cli-surface.md && cargo nextest run test_spec_representative_commands_match_signpost_surfaces test_workflow_configuration_docs_describe_internal_diagnostics test_workflow_help_is_scoped_as_advanced_internal_diagnostic test_agent_factory_guidance_avoids_raw_workflow_validate_commands test_root_status_guides_active_work_to_finish_not_start test_root_status_reports_active_mission_contract_fields test_root_status_summarizes_checkout_orientation test_root_status_no_ready_work_suggests_valid_blocked_list test_root_start_finish_and_issue_transition_surface test_prime_guides_empty_checkout_without_repeating_status test_prime_names_active_mission test_prime_names_active_work test_history_repo_wide_supports_filters_bounded_output_and_drill_downs test_history_mission_scope_includes_linked_work_descendants_and_evidence test_history_issue_scope_defaults_single_issue_and_can_include_descendants test_history_empty_states_and_invalid_limit test_mission_status_cli_reports_control_state test_active_mission_focus_guides_status_and_work test_mission_help_uses_show_not_view && atelier lint && atelier export --check && git diff --check'"
exit_status: "0"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-trr2"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Workflow signpost validation pass: root status, mission status/show, start finish/current-work, transition, history, prime, docs/help/SPEC/Agent Factory parity, raw validator absence from normal guidance, and historical decision satisfaction all verified after repairs a4rz and ywe6."
updated_at: "2026-06-13T00:25:40.914230614+00:00"
---

Workflow signpost validation pass: root status, mission status/show, start finish/current-work, transition, history, prime, docs/help/SPEC/Agent Factory parity, raw validator absence from normal guidance, and historical decision satisfaction all verified after repairs a4rz and ywe6.

Command: bash -lc 'atelier --version && atelier status && atelier mission status atelier-tcmr && atelier mission show atelier-tcmr && atelier issue show atelier-trr2 && atelier issue show atelier-wpyb && atelier issue transition atelier-trr2 --options && atelier start --help && atelier finish --help && if atelier current-work --help; then exit 1; else true; fi && atelier work --help && atelier work status && atelier history --issue atelier-trr2 --limit 5 && atelier history --mission atelier-tcmr --limit 5 && atelier prime && atelier workflow --help && atelier workflow validate --help && ! rg -n "atelier work start|atelier work finish|atelier workflow validate" SPEC.md AGENTFACTORY.md && rg -n "Root .*atelier start|hidden .*atelier work start/finish/status|atelier issue transition" docs/product/cli-surface.md && rg -n "Hidden advanced/internal workflow diagnostics|atelier lint|atelier doctor" docs/product/workflow-configuration.md && rg -n "atelier status|atelier mission status|atelier issue transition|atelier start|atelier finish" AGENTFACTORY.md docs/product/cli-surface.md && cargo nextest run test_spec_representative_commands_match_signpost_surfaces test_workflow_configuration_docs_describe_internal_diagnostics test_workflow_help_is_scoped_as_advanced_internal_diagnostic test_agent_factory_guidance_avoids_raw_workflow_validate_commands test_root_status_guides_active_work_to_finish_not_start test_root_status_reports_active_mission_contract_fields test_root_status_summarizes_checkout_orientation test_root_status_no_ready_work_suggests_valid_blocked_list test_root_start_finish_and_issue_transition_surface test_prime_guides_empty_checkout_without_repeating_status test_prime_names_active_mission test_prime_names_active_work test_history_repo_wide_supports_filters_bounded_output_and_drill_downs test_history_mission_scope_includes_linked_work_descendants_and_evidence test_history_issue_scope_defaults_single_issue_and_can_include_descendants test_history_empty_states_and_invalid_limit test_mission_status_cli_reports_control_state test_active_mission_focus_guides_status_and_work test_mission_help_uses_show_not_view && atelier lint && atelier export --check && git diff --check'
Exit status: 0

Stdout summary (truncated):
atelier 0.2.0+5ebbccc
Atelier Status
==============
Tracker:       current
Ready work:    3
Active work:   none
Active mission: atelier-tcmr - Repair CLI workflow rework and validation gaps

Local State
-----------
Branch:   codex/orchestrate-atelier-fork...origin/codex/orchestrate-atelier-fork [ahead 59]
Worktree: clean
Tracker:  current

Active Mission
--------------
atelier-tcmr - Repair CLI workflow rework and validation gaps
Health:   blocked
Work:     ready 3, blocked 2, done 48, backlog 0

Ready In Active Mission
-----------------------
  atelier-trr2 - Validate workflow signpost surfaces end to end
  atelier-v9id - Adversarially validate repair mission outcomes
  atelier-ymfl - Align docs tests and Agent Factory process with enforced proof

Immediate Blockers
------------------
  atelier-trr2 - Validate workflow signpost surfaces end to end
  atelier-v9id - Adversarially validate repair mission outcomes

Recent Activity
---------------
  atelier-ywe6 close_reason: Recorded close reason
  atelier-ywe6 status_changed: Changed status from open to closed
  atelier-ywe6 evidence_attached: Attached evidence atelier-x23c

Next Actions
------------
  Inspect active mission health (atelier-tcmr): atelier mission status atelier-tcmr
  Open active mission record (atelier-tcmr): atelier mission show atelier-tcmr
  Start ready active-mission work (3 ready issue(s)): atelier start atelier-trr2
  Check runtime health (tracker export is current): atelier doctor
Lint passed.
Mission Status atelier-tcmr [active] - Repair CLI workflow rework and validation gaps
=====================================================================================
Health:   blocked
Tracker:  ok
Closeout: blocked

Work
----
Total: 3 ready, 2 blocked, 48 done
  [epic] atelier-wpyb [open] high - Repair status start history prime and transition surfaces | 1 ready, 7 done
  [epic] atelier-ymfl [open] high - Align docs tests and Agent Factory process with enforced proof | 2 done
  [epic] atelier-zue4 [open] high - Overhaul mission validation and reliability system | 1 ready, 16 done
  [epic] atelier-40ou [closed] high - Parse issue Markdown sections as first-class structure | 7 done
  [epic] atelier-efpk [closed] high - Repair and consolidate CLI command surfaces | 4 done
  [epic] atelier-gjaz [closed] high - Replace escaped mission data JSON with readable mission records | 5 done
  [epic] atelier-nzy1 [closed] high - Harden mission closeout validators and evidence requirements | 3 done

Blockers
--------
Mission blockers: 0 open
Blocked work: 2 blocked

Evidence
--------
Linked evidence: 2

Reliability
-----------
Projection Freshness: current
Malformed Work: none
Missing Outcome Sections: none
Missing Evidence Sections: none
Attached Proof: missing - issue proof gaps: atelier-trr2, atelier-v9id, atelier-wpyb, atelier-ymfl, atelier-zue4
  Next: atelier evidence add --kind validation --result pass "..."
  Next: atelier evidence attach <evidence-id> issue <issue-id>
Docs/Help Drift: clear
Ignored Test Review: current
Open Blockers: 2 open - atelier-trr2, atelier-v9id
  Next: close or unblock listed blockers
Drill-downs:
  atelier mission audit atelier-tcmr
  atelier lint
  atelier doctor

Closeout Gates
--------------
Work: open - atelier-trr2, atelier-v9id, atelier-wpyb, atelier-ymfl, atelier-zue4
  Next: atelier issue close <issue-id> --reason "..."
Blockers: open - atelier-trr2, atelier-v9id
  Next: close or unblock the blocker issues.
Mission Proof: attached
Contract Audit: fail - 39 unresolved item(s)
  Next: atelier mission audit atelier-tcmr
Tracker State: current
Linked Issue Records: parseable
Blocking Lints: clear
Docs/Help Drift: clear
Ignored Test Review: current
Worktree: clean

Advanced Validator Detail
-------------------------
2 advanced closeout validator failure detected.
  fail  no_open_work - open linked work: atelier-trr2, atelier-v9id, atelier-wpyb, atelier-ymfl, atelier-zue4
  fail  no_open_blockers - open blockers: atelier-trr2, atelier-v9id

Active Work
-----------
(none)

Next Commands
-------------
  Inspect mission record (durabl

Stderr summary (truncated):
error: unrecognized subcommand 'current-work'

Usage: atelier [OPTIONS] <COMMAND>

For more information, try '--help'.
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
    |

