---
created_at: "2026-06-13T00:27:18.757468831+00:00"
id: "atelier-8xy6"
evidence_type: "validation"
captured_at: "2026-06-13T00:27:14.686440840+00:00"
command: "bash -lc 'atelier issue show atelier-ymfl && atelier history --issue atelier-ymfl --include-descendants --limit 40 && atelier evidence show atelier-4itj && atelier evidence show atelier-8fc1 && rg -n \"ordinary work closes with proof|parent-level|independent validation|new repair issues|raw workflow validators\" AGENTFACTORY.md /root/.agents/skills/agent-factory/procedures /root/.agents/skills/agent-factory/standards && ! rg -n \"atelier workflow validate issue|atelier workflow validate mission\" AGENTFACTORY.md /root/.agents/skills/agent-factory/procedures /root/.agents/skills/agent-factory/standards && cargo nextest run test_agent_factory_guidance_avoids_raw_workflow_validate_commands test_spec_representative_commands_match_signpost_surfaces test_workflow_configuration_docs_describe_internal_diagnostics test_workflow_help_is_scoped_as_advanced_internal_diagnostic && atelier lint atelier-ymfl && atelier export --check && atelier doctor && git diff --check'"
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
    id: "atelier-ymfl"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Process and docs epic closeout pass: Agent Factory proof escalation guidance, authoring/validation/review/orchestration updates, repair-issue guidance, raw workflow-validator non-normal guidance, and ignored-test ownership are covered by child evidence; lint, export check, doctor, and diff check pass."
updated_at: "2026-06-13T00:27:19.763950452+00:00"
---

Process and docs epic closeout pass: Agent Factory proof escalation guidance, authoring/validation/review/orchestration updates, repair-issue guidance, raw workflow-validator non-normal guidance, and ignored-test ownership are covered by child evidence; lint, export check, doctor, and diff check pass.

Command: bash -lc 'atelier issue show atelier-ymfl && atelier history --issue atelier-ymfl --include-descendants --limit 40 && atelier evidence show atelier-4itj && atelier evidence show atelier-8fc1 && rg -n "ordinary work closes with proof|parent-level|independent validation|new repair issues|raw workflow validators" AGENTFACTORY.md /root/.agents/skills/agent-factory/procedures /root/.agents/skills/agent-factory/standards && ! rg -n "atelier workflow validate issue|atelier workflow validate mission" AGENTFACTORY.md /root/.agents/skills/agent-factory/procedures /root/.agents/skills/agent-factory/standards && cargo nextest run test_agent_factory_guidance_avoids_raw_workflow_validate_commands test_spec_representative_commands_match_signpost_surfaces test_workflow_configuration_docs_describe_internal_diagnostics test_workflow_help_is_scoped_as_advanced_internal_diagnostic && atelier lint atelier-ymfl && atelier export --check && atelier doctor && git diff --check'
Exit status: 0

Stdout summary (truncated):
atelier-ymfl [epic] open - Align docs tests and Agent Factory process with enforced proof
=========================================================================================
Status:   open
Type:     epic
Priority: high
Created:  2026-06-12 00:59 -04:00
Updated:  2026-06-12 15:39 -04:00
Labels:   docs, process, rework
File:     /root/atelier/.atelier/issues/atelier-ymfl.md

Hierarchy
---------
Parent: (none)

Transition Readiness
--------------------
  start: ready - issue is open and required sections parse
    atelier start atelier-ymfl
  close: blocked - missing issue-level proof; capture passing evidence or attach existing evidence
  options: atelier issue transition atelier-ymfl --options

Description
-----------
Align documentation, tests, and Agent Factory process with enforced proof so
agents cannot close implementation work based on notes, intent summaries, or
unrelated green tests.

Outcome
-------
- Agent guidance explains when to create new repair issues versus reopening
  misleading closed work.
- The global Agent Factory skill procedures are updated so planning,
  orchestration, implementation, validation, review, docs, and closeout all
  require Outcome/Evidence discipline and attached proof.
- Agent Factory guidance teaches a simple proof rule: ordinary work closes with
  proof on the issue, while risky, broad, or parent-level claims require an
  independent check.
- Planning and closeout guidance require Outcome and Evidence sections on new
  work items once the section parser lands.
- Tests that preserve obsolete behavior are removed, rewritten, or explicitly
  tied to a migration window.
- Ignored tests have named follow-up owners or are deleted if the behavior is no
  longer part of the product.
- Mission closeout process requires a contract audit that maps mission Outcome
  lines to command output, tests, docs, or attached evidence.
- Documentation no longer contradicts the implemented command surface or
  compatibility policy.
- Agent Factory guidance no longer lists `atelier workflow validate` as a
  normal planning, closeout, or drill-down command.

Evidence
--------
- Docs updates cover Agent Factory workflow, repository guidance, and CLI
  surface documentation.
- Skill updates cover Agent Factory procedure files, not only repository-local
  instructions.
- Process review demonstrates the proof rule on representative issue, epic, and
  mission work.
- Test updates remove stale assertions that old commands are normal behavior.
- A closeout checklist or validator transcript demonstrates each mission
  outcome has evidence.
- Run focused docs/tests, `atelier export --check`, `atelier lint`, and
  `atelier doctor`.

Notes
-----
This is process repair, not a substitute for command implementation. It should
block mission closeout if documentation and tests still allow the old failure
mode.

Blocked by
----------
  atelier-a4sn [closed] high - Remove workflow validate from the normal public workflow
  atelier-wws5 [closed] high - Update Agent Factory skill for evidence-backed mission reliability

Blocking
--------
(none)

Subissues
---------
2 total | status: closed=2 | priority: high=2
  atelier-v6nd [closed] high - Teach Agent Factory proof escalation rules
  atelier-wws5 [closed] high - Update Agent Factory skill for evidence-backed mission reliability

Recent Activity
---------------
(none)

Next Commands
-------------
  Edit issue Markdown: /root/atelier/.atelier/issues/atelier-ymfl.md
  Validate this issue: atelier lint atelier-ymfl
  Add a note: atelier note add issue atelier-ymfl "..."
  Show full activity: atelier history --issue atelier-ymfl
  Show transition options: atelier issue transition atelier-ymfl --options
  Start tracked work: atelier start atelier-ymfl
  Close when complete: atelier issue close atelier-ymfl --reason "..."
History
=======
Scope:          issue atelier-ymfl - Align docs tests and Agent Factory process with enforced proof
Source:         canonical .atelier issue activity, records, evidence, and record links; local runtime diagnostics excluded
Ordering:       newe

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

