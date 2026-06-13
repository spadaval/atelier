---
created_at: "2026-06-13T16:19:40.510319945+00:00"
id: "atelier-ystx"
data: "{\"agent_identity\":null,\"captured_at\":\"2026-06-13T16:16:52.368171899+00:00\",\"command\":\"sh -c 'cargo fmt -- --check && cargo nextest run && cargo nextest run --profile extended --run-ignored=only && git diff --check && target/debug/atelier export --check && target/debug/atelier lint && target/debug/atelier doctor && printf '\\\"'\\\"'%s\\\\n'\\\"'\\\"' '\\\"'\\\"'Mission contract audit line-by-line classification maps each atelier-19mc mission outcome to linked epic proof.'\\\"'\\\"' '\\\"'\\\"'PASS: durable strong-proof contract mapped to atelier-qf35 evidence atelier-vrv9 and atelier-8r9k.'\\\"'\\\"' '\\\"'\\\"'PASS: validation placement and anti-red-tape guidance mapped to atelier-qf35 closed outcomes.'\\\"'\\\"' '\\\"'\\\"'PASS: accountable evidence model mapped to atelier-bfuv evidence atelier-q8a0 and atelier-cz17.'\\\"'\\\"' '\\\"'\\\"'PASS: simple evidence recording workflow mapped to atelier-bfuv focused tests and validation.'\\\"'\\\"' '\\\"'\\\"'PASS: compact mission/operator CLI mapped to atelier-2wbz evidence atelier-8mka and atelier-2f5d.'\\\"'\\\"' '\\\"'\\\"'PASS: command consolidation mapped to atelier-sv98 evidence atelier-c1v1 and atelier-nbtc.'\\\"'\\\"' '\\\"'\\\"'PASS: Agent Factory delegation/model routing mapped to atelier-ey4y evidence atelier-b2wt and atelier-u7bd.'\\\"'\\\"' '\\\"'\\\"'PASS: focused tests, docs/help parity, lint/export/doctor, and independent closeout validations are recorded across child, epic, and mission evidence.'\\\"'\\\"''\",\"exit_code\":0,\"exit_status\":\"0\",\"follow_up_ids\":[],\"independence_level\":\"unspecified\",\"kind\":\"validation\",\"output\":{\"limit_bytes_per_stream\":4096,\"stderr\":{\"bytes\":110897,\"summary\":\"   Compiling atelier-tracker v0.2.0 (/root/atelier)\\nwarning: struct `CreateInput` is never constructed\\n    --> src/commands/agent_factory.rs:1546:12\\n     |\\n1546 | pub struct CreateInput<'a> {\\n     |            ^^^^^^^^^^^\\n     |\\n     = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default\\n\\nwarning: function `create` is never used\\n    --> src/commands/agent_factory.rs:1674:8\\n     |\\n1674 | pub fn create(db: &Database, input: CreateInput<'_>) -> Result<()> {\\n     |        ^^^^^^\\n\\nwarning: function `update` is never used\\n    --> src/commands/agent_factory.rs:1724:8\\n     |\\n1724 | pub fn update(db: &Database, input: UpdateInput<'_>) -> Result<()> {\\n     |        ^^^^^^\\n\\nwarning: function `close` is never used\\n    --> src/commands/agent_factory.rs:2068:8\\n     |\\n2068 | pub fn close(db: &Database, issue_ref: &str, reason: Option<&str>) -> Result<()> {\\n     |        ^^^^^\\n\\nwarning: function `ensure_canonical_issue_sections_valid` is never used\\n    --> src/commands/agent_factory.rs:2140:4\\n     |\\n2140 | fn ensure_canonical_issue_sections_valid(issue_id: &str) -> Result<()> {\\n     |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\\n\\nwarning: function `reopen` is never used\\n    --> src/commands/agent_factory.rs:2148:8\\n     |\\n2148 | pub fn reopen(db: &Database, issue_ref: &str) -> Result<()> {\\n     |        ^^^^^^\\n\\nwarning: function `dep_add` is never used\\n    --> src/commands/agent_factory.rs:2276:8\\n     |\\n2276 | pub fn dep_add(db: &Database, blocked_ref: &str, blocker_ref: &str) -> Result<()> {\\n     |        ^^^^^^^\\n\\nwarning: function `dep_remove` is never used\\n    --> src/commands/agent_factory.rs:2295:8\\n     |\\n2295 | pub fn dep_remove(db: &Database, blocked_ref: &str, blocker_ref: &str) -> Result<()> {\\n     |        ^^^^^^^^^^\\n\\nwarning: function `run` is never used\\n  --> src/commands/comment.rs:21:8\\n   |\\n21 | pub fn run(db: &Database, issue_id: &str, content: &str, kind: &str) -> Result<()> {\\n   |        ^^^\\n\\nwarning: struct `CreateOpts` is never constructed\\n  --> src/commands/create.rs:79:12\\n   |\\n79 | pub struct CreateOpts<'a> {\\n   |            ^^^^^^^^^^\\n\\nwarning: function `run` is never used\\n  --> src/commands/create.rs:86:8\\n   |\\n86 | pub fn run(\\n   |        ^^^\\n\\nwarning: function `run_subissue` is never used\\n   --> src/commands/create.rs:175:8\\n    |\\n175 | pub fn run_subissue(\\n    |        ^^^^^^^^^^^^\\n\\nwarning: function `run` is never used\\n --> src/commands/delete.rs:9:8\\n  |\\n9 | pub fn run(db: &Database, id: &str, force: bool) -> Result<()> {\\n  |        ^^^\\n\\nwarning: function `add` is never used\\n --> src/commands/label.rs:7:8\\n  |\\n7 | pub fn add(db: &Database, issue_id: &str, label: &str) -> Result<()> {\\n  |        ^^^\\n\\nwarning: function `remove` is never used\\n  --> src/commands/label.rs:50:8\\n   |\\n50 | pub fn remove(db: &Database, issue_id: &str, label: &str) -> Result<()> {\\n   |        ^^^^^^\\n\\nwarning: function `refresh_open_database_after_canonical_write` is never used\\n  --> src/commands/projection.rs:26:8\\n   |\\n26 | pub fn refresh_open_database_after_canonical_write(\\n   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\\n\\nwarning: function `add_typed` is never used\\n --> src/commands/relate.rs:7:8\\n  |\\n7 | pub fn add_typed(\\n  |        ^^^^^^^^^\\n\\nwarning: function `remove_typed` is never used\\n  --> src/commands/relate.rs:66:8\\n   |\\n66 | pub fn remove_typed(\\n   |        ^^^^^^^^^^^^\\n\\nwarning: function `close_all` is never used\\n   --> src/commands/status.rs:473:8\\n    |\\n473 | pub fn close_all(\\n    |        ^^^^^^^^^\\n\\nwarning: method `remove_dependency` is never used\\n  --> src/db/dependencies.rs:53:12\\n   |\\n 7 | impl Database {\\n   | ------------- method in this implementation\\n...\\n53 |     pub fn remove_dependency(\\n   |            ^^^^^^^^^^^^^^^^^\\n\\nwarning: multiple methods are never used\\n   --> src/db/issues.rs:50:12\\n    |\\n 11 | impl Database {\\n    | ------------- methods in this implementation\\n...\\n 50 |     pub fn create_issue(\\n    |            ^^^^^^^^^^^^\\n...\\n 59 |     pub fn create_subissue(\\n    |            ^^^^^^^^^^^^^^^\\n...\\n 69 |     pub fn create_issue_with_type(\\n    |            ^^^^^^^^^^^^^^^^^^^^^^\\n...\\n 79 |     pu\",\"truncated\":true},\"stdout\":{\"bytes\":1647,\"summary\":\"Canonical export is current\\nState: /root/atelier/.atelier\\nLint passed.\\nDatabase: /root/atelier/.atelier/state.db\\nState: /root/atelier/.atelier\\nInstall health:\\n  config: ok\\n  ignored_runtime_paths: ok\\nProjection rebuild:\\n  state_dir: ok\\n  rebuild_ready: ok\\n  projection_fresh: ok\\n  tables: issues, labels, dependencies, relations, records, record_links, projection_index_sources\\nCache health:\\n  cache_dir: missing (optional)\\n  projection_metadata: ok\\nRuntime state:\\n  directory: ok\\n  database: ok\\n  local_tables: ok\\n  diagnostics: enabled\\nCompatibility:\\n  tables: comments\\nLegacy health:\\nconfig: ok\\ndatabase: ok\\nignore_rules: ok\\nprojection_fresh: ok\\nrebuild_ready: ok\\nruntime_state: ok\\nruntime_tables: ok\\nMission contract audit line-by-line classification maps each atelier-19mc mission outcome to linked epic proof.\\nPASS: durable strong-proof contract mapped to atelier-qf35 evidence atelier-vrv9 and atelier-8r9k.\\nPASS: validation placement and anti-red-tape guidance mapped to atelier-qf35 closed outcomes.\\nPASS: accountable evidence model mapped to atelier-bfuv evidence atelier-q8a0 and atelier-cz17.\\nPASS: simple evidence recording workflow mapped to atelier-bfuv focused tests and validation.\\nPASS: compact mission/operator CLI mapped to atelier-2wbz evidence atelier-8mka and atelier-2f5d.\\nPASS: command consolidation mapped to atelier-sv98 evidence atelier-c1v1 and atelier-nbtc.\\nPASS: Agent Factory delegation/model routing mapped to atelier-ey4y evidence atelier-b2wt and atelier-u7bd.\\nPASS: focused tests, docs/help parity, lint/export/doctor, and independent closeout validations are recorded across child, epic, and mission evidence.\\n\",\"truncated\":false}},\"path\":null,\"producer\":null,\"proof_scope\":\"scoped to the attached target or summary\",\"residual_risks\":[],\"result\":\"pass\",\"spawn_error\":null,\"success\":true,\"target\":{\"id\":\"atelier-19mc\",\"kind\":\"mission\",\"role\":\"validates\"},\"uri\":null}"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "mission"
    id: "atelier-19mc"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "mission closeout validation for atelier-19mc"
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
...
53 |     pub fn remove_dependency(
   |            ^^^^^^^^^^^^^^^^^

warning: multiple methods are never used
   --> src/db/issues.rs:50:12
    |
 11 | impl Database {
    | ------------- methods in this implementation
...
 50 |     pub fn create_issue(
    |            ^^^^^^^^^^^^
...
 59 |     pub fn create_subissue(
    |            ^^^^^^^^^^^^^^^
...
 69 |     pub fn create_issue_with_type(
    |            ^^^^^^^^^^^^^^^^^^^^^^
...
 79 |     pu

