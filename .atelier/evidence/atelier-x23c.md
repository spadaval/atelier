---
created_at: "2026-06-13T00:22:46.708981661+00:00"
id: "atelier-x23c"
evidence_type: "validation"
captured_at: "2026-06-13T00:22:42.163924206+00:00"
command: "bash -lc '! rg -n \"atelier work start|atelier work finish|atelier workflow validate\" SPEC.md && rg -n \"atelier start atelier-z1p8|atelier finish atelier-z1p8|atelier status|atelier issue transition atelier-z1p8 --options|atelier evidence capture -- <command>\" SPEC.md && cargo nextest run test_spec_representative_commands_match_signpost_surfaces test_workflow_configuration_docs_describe_internal_diagnostics test_workflow_help_is_scoped_as_advanced_internal_diagnostic test_agent_factory_guidance_avoids_raw_workflow_validate_commands test_root_status_guides_active_work_to_finish_not_start test_root_start_finish_and_issue_transition_surface && target/debug/atelier lint atelier-ywe6 && target/debug/atelier export --check && git diff --check'"
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
    id: "atelier-ywe6"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "SPEC workflow command drift repair: stale normal work and workflow validator examples are absent, current status start finish transition and evidence capture examples are present; focused docs tests, lint, export check, and diff check pass."
updated_at: "2026-06-13T00:22:48.229190131+00:00"
---

SPEC workflow command drift repair: stale normal work and workflow validator examples are absent, current status start finish transition and evidence capture examples are present; focused docs tests, lint, export check, and diff check pass.

Command: bash -lc '! rg -n "atelier work start|atelier work finish|atelier workflow validate" SPEC.md && rg -n "atelier start atelier-z1p8|atelier finish atelier-z1p8|atelier status|atelier issue transition atelier-z1p8 --options|atelier evidence capture -- <command>" SPEC.md && cargo nextest run test_spec_representative_commands_match_signpost_surfaces test_workflow_configuration_docs_describe_internal_diagnostics test_workflow_help_is_scoped_as_advanced_internal_diagnostic test_agent_factory_guidance_avoids_raw_workflow_validate_commands test_root_status_guides_active_work_to_finish_not_start test_root_start_finish_and_issue_transition_surface && target/debug/atelier lint atelier-ywe6 && target/debug/atelier export --check && git diff --check'
Exit status: 0

Stdout summary:
334:atelier start atelier-z1p8
336:atelier finish atelier-z1p8
421:atelier status
425:atelier issue transition atelier-z1p8 --options
433:atelier evidence capture -- <command>
Lint passed.
Canonical export is current
State: /root/atelier/.atelier

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

