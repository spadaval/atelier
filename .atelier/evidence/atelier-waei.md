---
created_at: "2026-06-12T22:50:43.403423337+00:00"
id: "atelier-waei"
evidence_type: "validation"
captured_at: "2026-06-12T22:50:28.265736883+00:00"
command: "bash -lc 'set -euo pipefail\ncargo test --quiet workflow::tests\ncargo test --quiet --test cli_integration workflow_validate\ncargo test --quiet --test cli_integration mission_closeout\ncargo test --quiet --test cli_integration mission_status\ncargo fmt -- --check\ngit diff --check\ntarget/debug/atelier lint atelier-8o34\ntarget/debug/atelier lint\ntarget/debug/atelier export --check'"
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
    id: "atelier-8o34"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Target-aware validator defaults implemented"
updated_at: "2026-06-12T22:50:44.767209751+00:00"
---

Target-aware validator defaults implemented

Command: bash -lc 'set -euo pipefail
cargo test --quiet workflow::tests
cargo test --quiet --test cli_integration workflow_validate
cargo test --quiet --test cli_integration mission_closeout
cargo test --quiet --test cli_integration mission_status
cargo fmt -- --check
git diff --check
target/debug/atelier lint atelier-8o34
target/debug/atelier lint
target/debug/atelier export --check'
Exit status: 0

Stdout summary:

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 164 filtered out; finished in 0.00s


running 2 tests
..
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 319 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 251 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 51 filtered out; finished in 0.00s


running 5 tests
.....
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 246 filtered out; finished in 0.86s


running 2 tests
..
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 249 filtered out; finished in 0.84s


running 3 tests
...
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 248 filtered out; finished in 1.71s

Lint passed.
Lint passed.
Canonical export is current
State: /root/atelier/.atelier

Stderr summary (truncated):
warning: struct `CreateInput` is never constructed
    --> src/commands/agent_factory.rs:1256:12
     |
1256 | pub struct CreateInput<'a> {
     |            ^^^^^^^^^^^
     |
     = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `create` is never used
    --> src/commands/agent_factory.rs:1384:8
     |
1384 | pub fn create(db: &Database, input: CreateInput<'_>) -> Result<()> {
     |        ^^^^^^

warning: function `update` is never used
    --> src/commands/agent_factory.rs:1434:8
     |
1434 | pub fn update(db: &Database, input: UpdateInput<'_>) -> Result<()> {
     |        ^^^^^^

warning: function `close` is never used
    --> src/commands/agent_factory.rs:1761:8
     |
1761 | pub fn close(db: &Database, issue_ref: &str, reason: Option<&str>) -> Result<()> {
     |        ^^^^^

warning: function `ensure_canonical_issue_sections_valid` is never used
    --> src/commands/agent_factory.rs:1850:4
     |
1850 | fn ensure_canonical_issue_sections_valid(issue_id: &str) -> Result<()> {
     |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `reopen` is never used
    --> src/commands/agent_factory.rs:1858:8
     |
1858 | pub fn reopen(db: &Database, issue_ref: &str) -> Result<()> {
     |        ^^^^^^

warning: function `dep_add` is never used
    --> src/commands/agent_factory.rs:1986:8
     |
1986 | pub fn dep_add(db: &Database, blocked_ref: &str, blocker_ref: &str) -> Result<()> {
     |        ^^^^^^^

warning: function `dep_remove` is never used
    --> src/commands/agent_factory.rs:2005:8
     |
2005 | pub fn dep_remove(db: &Database, blocked_ref: &str, blocker_ref: &str) -> Result<()> {
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
 54 |     pub fn create_issue(
    |            ^^^^^^^^^^^^
...
 63 |     pub fn create_subissue(
    |            ^^^^^^^^^^^^^^^
...
 73 |     pub fn create_issue_with_type(
    |            ^^^^^^^^^^^^^^^^^^^^^^
...
 83 |     pub fn create_subissue_with_type(
    |            ^^^^^^^^
