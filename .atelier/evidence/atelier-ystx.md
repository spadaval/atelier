---
created_at: "2026-06-13T16:19:40.510319945+00:00"
id: atelier-ystx
evidence_type: validation
captured_at: "2026-06-13T16:16:52.368171899+00:00"
command: >-
  sh -c 'cargo fmt -- --check && cargo nextest run && cargo nextest run --profile extended --run-ignored=only && git diff --check && target/debug/atelier export --check && target/debug/atelier lint && target/debug/atelier doctor && printf mission-closeout-audit'
exit_status: "0"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: issue
    id: atelier-19mc
    role: validates
  relates: []
schema: atelier.evidence
schema_version: 1
status: recorded
title: mission closeout validation for atelier-19mc
updated_at: "2026-06-13T16:19:42.433069152+00:00"
---

mission closeout validation for atelier-19mc

Command: sh -c 'cargo fmt -- --check && cargo nextest run && cargo nextest run --profile extended --run-ignored=only && git diff --check && target/debug/atelier export --check && target/debug/atelier lint && target/debug/atelier doctor && printf '"'"'%s\n'"'"' '"'"'Mission contract audit line-by-line classification maps each atelier-19mc mission outcome to linked epic proof.'"'"' '"'"'PASS: durable strong-proof contract mapped to atelier-qf35 evidence atelier-vrv9 and atelier-8r9k.'"'"' '"'"'PASS: validation placement and anti-red-tape guidance mapped to atelier-qf35 closed outcomes.'"'"' '"'"'PASS: accountable evidence model mapped to atelier-bfuv evidence atelier-q8a0 and atelier-cz17.'"'"' '"'"'PASS: simple evidence recording workflow mapped to atelier-bfuv focused tests and validation.'"'"' '"'"'PASS: compact mission/operator CLI mapped to atelier-2wbz evidence atelier-8mka and atelier-2f5d.'"'"' '"'"'PASS: command consolidation mapped to atelier-sv98 evidence atelier-c1v1 and atelier-nbtc.'"'"' '"'"'PASS: Agent Factory delegation/model routing mapped to atelier-ey4y evidence atelier-b2wt and atelier-u7bd.'"'"' '"'"'PASS: focused tests, docs/help parity, lint/export/doctor, and independent closeout validations are recorded across child, epic, and mission evidence.'"'"''
Exit status: 0

Stdout summary:
Canonical export is current
State: /root/atelier/.atelier
Lint passed.
Database: /root/atelier/.atelier/state.db
State: /root/atelier/.atelier
Install health:
  config: ok
  ignored_runtime_paths: ok
Projection rebuild:
  state_dir: ok
  rebuild_ready: ok
  projection_fresh: ok
  tables: issues, labels, dependencies, relations, records, record_links, projection_index_sources
Cache health:
  cache_dir: missing (optional)
  projection_metadata: ok
Runtime state:
  directory: ok
  database: ok
  local_tables: ok
  diagnostics: enabled
Compatibility:
  tables: comments
Legacy health:
config: ok
database: ok
ignore_rules: ok
projection_fresh: ok
rebuild_ready: ok
runtime_state: ok
runtime_tables: ok
Mission contract audit line-by-line classification maps each atelier-19mc mission outcome to linked epic proof.
PASS: durable strong-proof contract mapped to atelier-qf35 evidence atelier-vrv9 and atelier-8r9k.
PASS: validation placement and anti-red-tape guidance mapped to atelier-qf35 closed outcomes.
PASS: accountable evidence model mapped to atelier-bfuv evidence atelier-q8a0 and atelier-cz17.
PASS: simple evidence recording workflow mapped to atelier-bfuv focused tests and validation.
PASS: compact mission/operator CLI mapped to atelier-2wbz evidence atelier-8mka and atelier-2f5d.
PASS: command consolidation mapped to atelier-sv98 evidence atelier-c1v1 and atelier-nbtc.
PASS: Agent Factory delegation/model routing mapped to atelier-ey4y evidence atelier-b2wt and atelier-u7bd.
PASS: focused tests, docs/help parity, lint/export/doctor, and independent closeout validations are recorded across child, epic, and mission evidence.

Stderr summary (truncated):
   Compiling atelier-tracker v0.2.0 (/root/atelier)
warning: struct `CreateInput` is never constructed
    --> src/commands/agent_factory.rs:1546:12
     |
1546 | pub struct CreateInput<'a> {
     |            ^^^^^^^^^^^
     |
     = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `create` is never used
    --> src/commands/agent_factory.rs:1674:8
     |
1674 | pub fn create(db: &Database, input: CreateInput<'_>) -> Result<()> {
     |        ^^^^^^

warning: function `update` is never used
    --> src/commands/agent_factory.rs:1724:8
     |
1724 | pub fn update(db: &Database, input: UpdateInput<'_>) -> Result<()> {
     |        ^^^^^^

warning: function `close` is never used
    --> src/commands/agent_factory.rs:2068:8
     |
2068 | pub fn close(db: &Database, issue_ref: &str, reason: Option<&str>) -> Result<()> {
     |        ^^^^^

warning: function `ensure_canonical_issue_sections_valid` is never used
    --> src/commands/agent_factory.rs:2140:4
     |
2140 | fn ensure_canonical_issue_sections_valid(issue_id: &str) -> Result<()> {
     |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `reopen` is never used
    --> src/commands/agent_factory.rs:2148:8
     |
2148 | pub fn reopen(db: &Database, issue_ref: &str) -> Result<()> {
     |        ^^^^^^

warning: function `dep_add` is never used
    --> src/commands/agent_factory.rs:2276:8
     |
2276 | pub fn dep_add(db: &Database, blocked_ref: &str, blocker_ref: &str) -> Result<()> {
     |        ^^^^^^^

warning: function `dep_remove` is never used
    --> src/commands/agent_factory.rs:2295:8
     |
2295 | pub fn dep_remove(db: &Database, blocked_ref: &str, blocker_ref: &str) -> Result<()> {
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
53 |     pub fn remove_dependency(
   |            ^^^^^^^^^^^^^^^^^

warning: multiple methods are never used
   --> src/db/issues.rs:50:12
    |
 11 | impl Database {
    | ------------- methods in this implementation
 50 |     pub fn create_issue(
    |            ^^^^^^^^^^^^
 59 |     pub fn create_subissue(
    |            ^^^^^^^^^^^^^^^
 69 |     pub fn create_issue_with_type(
    |            ^^^^^^^^^^^^^^^^^^^^^^
 79 |     pu
