---
created_at: "2026-06-12T21:38:05.476473358+00:00"
id: "atelier-v6pk"
evidence_type: "validation"
captured_at: "2026-06-12T21:38:01.241345102+00:00"
command: "bash -lc 'set -euo pipefail\ncargo test --test cli_integration section -- --nocapture\ntarget/debug/atelier export --check\ntarget/debug/atelier lint atelier-40ou\ntarget/debug/atelier workflow validate issue atelier-40ou\ntarget/debug/atelier issue show atelier-40ou'"
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
    id: "atelier-40ou"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "atelier-40ou sectioned issue closeout validation"
updated_at: "2026-06-12T21:38:06.815481972+00:00"
---

atelier-40ou sectioned issue closeout validation

Command: bash -lc 'set -euo pipefail
cargo test --test cli_integration section -- --nocapture
target/debug/atelier export --check
target/debug/atelier lint atelier-40ou
target/debug/atelier workflow validate issue atelier-40ou
target/debug/atelier issue show atelier-40ou'
Exit status: 0

Stdout summary (truncated):

running 4 tests
test test_lint_rejects_missing_required_issue_section ... ok
test test_lint_rejects_empty_required_issue_section ... ok
test test_start_refuses_shared_section_diagnostic ... ok
test test_workflow_validate_can_use_parsed_issue_sections ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 234 filtered out; finished in 0.49s

Canonical export is current
State: /root/atelier/.atelier
Lint passed.
Workflow Validation: issue atelier-40ou
======================================
Transition: close
Validators: 2
Results
-------
  pass  durable_state_current
      Reason: canonical export is current
      Warning: validator took 269ms; validators should stay under 100ms
  pass  issue_sections_parseable
      Reason: parsed required sections Description, Outcome, Evidence are present and non-empty for 1 issue(s)
atelier-40ou [epic] open - Parse issue Markdown sections as first-class structure
=================================================================================
Status:   open
Type:     epic
Priority: high
Created:  2026-06-12 00:43 -04:00
Updated:  2026-06-12 00:55 -04:00
Labels:   lint, markdown, tracker
File:     /root/atelier/.atelier/issues/atelier-40ou.md

Hierarchy
---------
Parent: (none)

Transition Readiness
--------------------
  start: ready - issue is open and required sections parse
    atelier start atelier-40ou
  close: blocked - no validating evidence linked
  options: atelier issue transition atelier-40ou --options

Description
-----------
Issue Markdown should not be treated as one opaque description blob. Atelier
should parse known top-level Markdown headings into named issue sections and use
those sections consistently in show, lint, start/close gates, and future
workflow validation.

The immediate target is a small, explicit issue-body contract with required
Description and Outcome sections, plus an Evidence section for commands,
transcripts, evidence records, or manual checks that demonstrate the outcome is
real. Notes remain optional context, not completion criteria.

The canonical Markdown body remains the durable authoring surface. Frontmatter
stays limited to compact metadata such as id, title, status, type, priority,
labels, and relationships. Remove issue-level YAML evidence fields such as
`evidence_required`; evidence requirements belong in the Markdown Evidence
section, and durable proof artifacts belong in attached evidence records.

Outcome
-------
- The issue parser exposes named issue body sections instead of only returning
  one description string.
- `atelier issue show <id>` renders recognized sections in stable order and
  clearly distinguishes missing optional sections from absent required sections.
- Validators and command surfaces consume issue structure through one shared
  parsed-section API rather than ad hoc string searches or display-only splits.
- `atelier lint` and `atelier lint <id>` fail when an issue is missing a
  non-empty Outcome section.
- Lint also fails for malformed issue body structure, including duplicate
  recognized headings or content before the first required issue section, unless
  the final parser contract explicitly allows that form.
- Starting work on an issue fails when required issue structure lint fails.
- Mission or issue closeout paths cannot pass while linked implementation work
  has missing or empty Outcome.
- Existing issue records are migrated or repaired to the new section format as
  part of the same workstream, with generated changes kept reviewable.
- Issue YAML frontmatter no longer contains `acceptance` or `evidence_required`
  arrays after the schema migration; the body sections are the authoring surface
  for those concepts.
- Documentation explains the section contract, required sections, optional
  sections, and why Outcome describes the desired finished world rather than a
  mutable completion checklist.

Evidence
--------
- Add parser unit tests for recognized sections, unknown sections, duplicate
  headings, empty required sections, and round-trip rendering.
- Add CLI integration tests proving `atelier

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

