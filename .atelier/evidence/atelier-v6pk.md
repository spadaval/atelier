---
created_at: "2026-06-12T21:38:05.476473358+00:00"
id: "atelier-v6pk"
data: "{\"captured_at\":\"2026-06-12T21:38:01.241345102+00:00\",\"command\":\"bash -lc 'set -euo pipefail\\ncargo test --test cli_integration section -- --nocapture\\ntarget/debug/atelier export --check\\ntarget/debug/atelier lint atelier-40ou\\ntarget/debug/atelier workflow validate issue atelier-40ou\\ntarget/debug/atelier issue show atelier-40ou'\",\"exit_code\":0,\"exit_status\":\"0\",\"kind\":\"validation\",\"output\":{\"limit_bytes_per_stream\":4096,\"stderr\":{\"bytes\":10388,\"summary\":\"   Compiling atelier-tracker v0.2.0 (/root/atelier)\\nwarning: function `list_all_issue_activities` is never used\\n   --> src/activity.rs:125:8\\n    |\\n125 | pub fn list_all_issue_activities(state_dir: &Path) -> Result<Vec<IssueActivity>> {\\n    |        ^^^^^^^^^^^^^^^^^^^^^^^^^\\n    |\\n    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default\\n\\nwarning: struct `CreateInput` is never constructed\\n    --> src/commands/agent_factory.rs:1252:12\\n     |\\n1252 | pub struct CreateInput<'a> {\\n     |            ^^^^^^^^^^^\\n\\nwarning: function `create` is never used\\n    --> src/commands/agent_factory.rs:1380:8\\n     |\\n1380 | pub fn create(db: &Database, input: CreateInput<'_>) -> Result<()> {\\n     |        ^^^^^^\\n\\nwarning: function `update` is never used\\n    --> src/commands/agent_factory.rs:1430:8\\n     |\\n1430 | pub fn update(db: &Database, input: UpdateInput<'_>) -> Result<()> {\\n     |        ^^^^^^\\n\\nwarning: function `close` is never used\\n    --> src/commands/agent_factory.rs:1757:8\\n     |\\n1757 | pub fn close(db: &Database, issue_ref: &str, reason: Option<&str>) -> Result<()> {\\n     |        ^^^^^\\n\\nwarning: function `ensure_canonical_issue_sections_valid` is never used\\n    --> src/commands/agent_factory.rs:1846:4\\n     |\\n1846 | fn ensure_canonical_issue_sections_valid(issue_id: &str) -> Result<()> {\\n     |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\\n\\nwarning: function `reopen` is never used\\n    --> src/commands/agent_factory.rs:1854:8\\n     |\\n1854 | pub fn reopen(db: &Database, issue_ref: &str) -> Result<()> {\\n     |        ^^^^^^\\n\\nwarning: function `dep_add` is never used\\n    --> src/commands/agent_factory.rs:1982:8\\n     |\\n1982 | pub fn dep_add(db: &Database, blocked_ref: &str, blocker_ref: &str) -> Result<()> {\\n     |        ^^^^^^^\\n\\nwarning: function `dep_remove` is never used\\n    --> src/commands/agent_factory.rs:2001:8\\n     |\\n2001 | pub fn dep_remove(db: &Database, blocked_ref: &str, blocker_ref: &str) -> Result<()> {\\n     |        ^^^^^^^^^^\\n\\nwarning: function `run` is never used\\n  --> src/commands/comment.rs:21:8\\n   |\\n21 | pub fn run(db: &Database, issue_id: &str, content: &str, kind: &str) -> Result<()> {\\n   |        ^^^\\n\\nwarning: struct `CreateOpts` is never constructed\\n  --> src/commands/create.rs:79:12\\n   |\\n79 | pub struct CreateOpts<'a> {\\n   |            ^^^^^^^^^^\\n\\nwarning: function `run` is never used\\n  --> src/commands/create.rs:86:8\\n   |\\n86 | pub fn run(\\n   |        ^^^\\n\\nwarning: function `run_subissue` is never used\\n   --> src/commands/create.rs:175:8\\n    |\\n175 | pub fn run_subissue(\\n    |        ^^^^^^^^^^^^\\n\\nwarning: function `run` is never used\\n --> src/commands/delete.rs:9:8\\n  |\\n9 | pub fn run(db: &Database, id: &str, force: bool) -> Result<()> {\\n  |        ^^^\\n\\nwarning: function `add` is never used\\n --> src/commands/label.rs:7:8\\n  |\\n7 | pub fn add(db: &Database, issue_id: &str, label: &str) -> Result<()> {\\n  |        ^^^\\n\\nwarning: function `remove` is never used\\n  --> src/commands/label.rs:50:8\\n   |\\n50 | pub fn remove(db: &Database, issue_id: &str, label: &str) -> Result<()> {\\n   |        ^^^^^^\\n\\nwarning: function `refresh_open_database_after_canonical_write` is never used\\n  --> src/commands/projection.rs:26:8\\n   |\\n26 | pub fn refresh_open_database_after_canonical_write(\\n   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\\n\\nwarning: function `add_typed` is never used\\n --> src/commands/relate.rs:7:8\\n  |\\n7 | pub fn add_typed(\\n  |        ^^^^^^^^^\\n\\nwarning: function `remove_typed` is never used\\n  --> src/commands/relate.rs:66:8\\n   |\\n66 | pub fn remove_typed(\\n   |        ^^^^^^^^^^^^\\n\\nwarning: function `close_all` is never used\\n  --> src/commands/status.rs:92:8\\n   |\\n92 | pub fn close_all(\\n   |        ^^^^^^^^^\\n\\nwarning: method `remove_dependency` is never used\\n  --> src/db/dependencies.rs:53:12\\n   |\\n 7 | impl Database {\\n   | ------------- method in this implementation\\n...\\n53 |     pub fn remove_dependency(\\n   |            ^^^^^^^^^^^^^^^^^\\n\\nwarning: multiple methods are never used\\n   --> src/db/issues.rs:54:12\\n    |\\n 10 | impl Database {\\n    | ------------- methods in this implementation\\n...\\n 54 |     pub fn\",\"truncated\":true},\"stdout\":{\"bytes\":6292,\"summary\":\"\\nrunning 4 tests\\ntest test_lint_rejects_missing_required_issue_section ... ok\\ntest test_lint_rejects_empty_required_issue_section ... ok\\ntest test_start_refuses_shared_section_diagnostic ... ok\\ntest test_workflow_validate_can_use_parsed_issue_sections ... ok\\n\\ntest result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 234 filtered out; finished in 0.49s\\n\\nCanonical export is current\\nState: /root/atelier/.atelier\\nLint passed.\\nWorkflow Validation: issue atelier-40ou\\n======================================\\nTransition: close\\nValidators: 2\\nResults\\n-------\\n  pass  durable_state_current\\n      Reason: canonical export is current\\n      Warning: validator took 269ms; validators should stay under 100ms\\n  pass  issue_sections_parseable\\n      Reason: parsed required sections Description, Outcome, Evidence are present and non-empty for 1 issue(s)\\natelier-40ou [epic] open - Parse issue Markdown sections as first-class structure\\n=================================================================================\\nStatus:   open\\nType:     epic\\nPriority: high\\nCreated:  2026-06-12 00:43 -04:00\\nUpdated:  2026-06-12 00:55 -04:00\\nLabels:   lint, markdown, tracker\\nFile:     /root/atelier/.atelier/issues/atelier-40ou.md\\n\\nHierarchy\\n---------\\nParent: (none)\\n\\nTransition Readiness\\n--------------------\\n  start: ready - issue is open and required sections parse\\n    atelier start atelier-40ou\\n  close: blocked - no validating evidence linked\\n  options: atelier issue transition atelier-40ou --options\\n\\nDescription\\n-----------\\nIssue Markdown should not be treated as one opaque description blob. Atelier\\nshould parse known top-level Markdown headings into named issue sections and use\\nthose sections consistently in show, lint, start/close gates, and future\\nworkflow validation.\\n\\nThe immediate target is a small, explicit issue-body contract with required\\nDescription and Outcome sections, plus an Evidence section for commands,\\ntranscripts, evidence records, or manual checks that demonstrate the outcome is\\nreal. Notes remain optional context, not completion criteria.\\n\\nThe canonical Markdown body remains the durable authoring surface. Frontmatter\\nstays limited to compact metadata such as id, title, status, type, priority,\\nlabels, and relationships. Remove issue-level YAML evidence fields such as\\n`evidence_required`; evidence requirements belong in the Markdown Evidence\\nsection, and durable proof artifacts belong in attached evidence records.\\n\\nOutcome\\n-------\\n- The issue parser exposes named issue body sections instead of only returning\\n  one description string.\\n- `atelier issue show <id>` renders recognized sections in stable order and\\n  clearly distinguishes missing optional sections from absent required sections.\\n- Validators and command surfaces consume issue structure through one shared\\n  parsed-section API rather than ad hoc string searches or display-only splits.\\n- `atelier lint` and `atelier lint <id>` fail when an issue is missing a\\n  non-empty Outcome section.\\n- Lint also fails for malformed issue body structure, including duplicate\\n  recognized headings or content before the first required issue section, unless\\n  the final parser contract explicitly allows that form.\\n- Starting work on an issue fails when required issue structure lint fails.\\n- Mission or issue closeout paths cannot pass while linked implementation work\\n  has missing or empty Outcome.\\n- Existing issue records are migrated or repaired to the new section format as\\n  part of the same workstream, with generated changes kept reviewable.\\n- Issue YAML frontmatter no longer contains `acceptance` or `evidence_required`\\n  arrays after the schema migration; the body sections are the authoring surface\\n  for those concepts.\\n- Documentation explains the section contract, required sections, optional\\n  sections, and why Outcome describes the desired finished world rather than a\\n  mutable completion checklist.\\n\\nEvidence\\n--------\\n- Add parser unit tests for recognized sections, unknown sections, duplicate\\n  headings, empty required sections, and round-trip rendering.\\n- Add CLI integration tests proving `atelier\",\"truncated\":true}},\"path\":null,\"producer\":null,\"result\":\"pass\",\"spawn_error\":null,\"success\":true,\"target\":{\"id\":\"atelier-40ou\",\"kind\":\"issue\",\"role\":\"validates\"},\"uri\":null}"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-40ou"
    type: "validates"
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

