---
created_at: "2026-06-12T22:42:27.779249185+00:00"
id: "atelier-evks"
data: "{\"captured_at\":\"2026-06-12T22:42:11.835302813+00:00\",\"command\":\"bash -lc 'set -euo pipefail\\ncargo test --quiet --test cli_integration test_lint_rejects_missing_evidence_section\\ncargo test --quiet --test cli_integration test_lint_rejects_empty_evidence_section\\ncargo test --quiet --test cli_integration test_lint_rejects_vague_evidence_even_when_notes_name_a_command\\ncargo test --quiet --test cli_integration test_lint_accepts_concrete_evidence_without_optional_notes\\ncargo test --quiet --test cli_integration lint\\ncargo fmt -- --check\\ngit diff --check\\ntarget/debug/atelier lint atelier-u6ax\\ntarget/debug/atelier lint\\ntarget/debug/atelier export --check'\",\"exit_code\":0,\"exit_status\":\"0\",\"kind\":\"validation\",\"output\":{\"limit_bytes_per_stream\":4096,\"stderr\":{\"bytes\":52005,\"summary\":\"warning: struct `CreateInput` is never constructed\\n    --> src/commands/agent_factory.rs:1256:12\\n     |\\n1256 | pub struct CreateInput<'a> {\\n     |            ^^^^^^^^^^^\\n     |\\n     = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default\\n\\nwarning: function `create` is never used\\n    --> src/commands/agent_factory.rs:1384:8\\n     |\\n1384 | pub fn create(db: &Database, input: CreateInput<'_>) -> Result<()> {\\n     |        ^^^^^^\\n\\nwarning: function `update` is never used\\n    --> src/commands/agent_factory.rs:1434:8\\n     |\\n1434 | pub fn update(db: &Database, input: UpdateInput<'_>) -> Result<()> {\\n     |        ^^^^^^\\n\\nwarning: function `close` is never used\\n    --> src/commands/agent_factory.rs:1761:8\\n     |\\n1761 | pub fn close(db: &Database, issue_ref: &str, reason: Option<&str>) -> Result<()> {\\n     |        ^^^^^\\n\\nwarning: function `ensure_canonical_issue_sections_valid` is never used\\n    --> src/commands/agent_factory.rs:1850:4\\n     |\\n1850 | fn ensure_canonical_issue_sections_valid(issue_id: &str) -> Result<()> {\\n     |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\\n\\nwarning: function `reopen` is never used\\n    --> src/commands/agent_factory.rs:1858:8\\n     |\\n1858 | pub fn reopen(db: &Database, issue_ref: &str) -> Result<()> {\\n     |        ^^^^^^\\n\\nwarning: function `dep_add` is never used\\n    --> src/commands/agent_factory.rs:1986:8\\n     |\\n1986 | pub fn dep_add(db: &Database, blocked_ref: &str, blocker_ref: &str) -> Result<()> {\\n     |        ^^^^^^^\\n\\nwarning: function `dep_remove` is never used\\n    --> src/commands/agent_factory.rs:2005:8\\n     |\\n2005 | pub fn dep_remove(db: &Database, blocked_ref: &str, blocker_ref: &str) -> Result<()> {\\n     |        ^^^^^^^^^^\\n\\nwarning: function `run` is never used\\n  --> src/commands/comment.rs:21:8\\n   |\\n21 | pub fn run(db: &Database, issue_id: &str, content: &str, kind: &str) -> Result<()> {\\n   |        ^^^\\n\\nwarning: struct `CreateOpts` is never constructed\\n  --> src/commands/create.rs:79:12\\n   |\\n79 | pub struct CreateOpts<'a> {\\n   |            ^^^^^^^^^^\\n\\nwarning: function `run` is never used\\n  --> src/commands/create.rs:86:8\\n   |\\n86 | pub fn run(\\n   |        ^^^\\n\\nwarning: function `run_subissue` is never used\\n   --> src/commands/create.rs:175:8\\n    |\\n175 | pub fn run_subissue(\\n    |        ^^^^^^^^^^^^\\n\\nwarning: function `run` is never used\\n --> src/commands/delete.rs:9:8\\n  |\\n9 | pub fn run(db: &Database, id: &str, force: bool) -> Result<()> {\\n  |        ^^^\\n\\nwarning: function `add` is never used\\n --> src/commands/label.rs:7:8\\n  |\\n7 | pub fn add(db: &Database, issue_id: &str, label: &str) -> Result<()> {\\n  |        ^^^\\n\\nwarning: function `remove` is never used\\n  --> src/commands/label.rs:50:8\\n   |\\n50 | pub fn remove(db: &Database, issue_id: &str, label: &str) -> Result<()> {\\n   |        ^^^^^^\\n\\nwarning: function `refresh_open_database_after_canonical_write` is never used\\n  --> src/commands/projection.rs:26:8\\n   |\\n26 | pub fn refresh_open_database_after_canonical_write(\\n   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\\n\\nwarning: function `add_typed` is never used\\n --> src/commands/relate.rs:7:8\\n  |\\n7 | pub fn add_typed(\\n  |        ^^^^^^^^^\\n\\nwarning: function `remove_typed` is never used\\n  --> src/commands/relate.rs:66:8\\n   |\\n66 | pub fn remove_typed(\\n   |        ^^^^^^^^^^^^\\n\\nwarning: function `close_all` is never used\\n  --> src/commands/status.rs:92:8\\n   |\\n92 | pub fn close_all(\\n   |        ^^^^^^^^^\\n\\nwarning: method `remove_dependency` is never used\\n  --> src/db/dependencies.rs:53:12\\n   |\\n 7 | impl Database {\\n   | ------------- method in this implementation\\n...\\n53 |     pub fn remove_dependency(\\n   |            ^^^^^^^^^^^^^^^^^\\n\\nwarning: multiple methods are never used\\n   --> src/db/issues.rs:54:12\\n    |\\n 10 | impl Database {\\n    | ------------- methods in this implementation\\n...\\n 54 |     pub fn create_issue(\\n    |            ^^^^^^^^^^^^\\n...\\n 63 |     pub fn create_subissue(\\n    |            ^^^^^^^^^^^^^^^\\n...\\n 73 |     pub fn create_issue_with_type(\\n    |            ^^^^^^^^^^^^^^^^^^^^^^\\n...\\n 83 |     pub fn create_subissue_with_type(\\n    |            ^^^^^^^^\",\"truncated\":true},\"stdout\":{\"bytes\":676,\"summary\":\"\\nrunning 1 test\\n.\\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 248 filtered out; finished in 0.15s\\n\\n\\nrunning 1 test\\n.\\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 248 filtered out; finished in 0.14s\\n\\n\\nrunning 1 test\\n.\\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 248 filtered out; finished in 0.16s\\n\\n\\nrunning 1 test\\n.\\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 248 filtered out; finished in 0.16s\\n\\n\\nrunning 15 tests\\n...............\\ntest result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 234 filtered out; finished in 1.71s\\n\\nLint passed.\\nLint passed.\\nCanonical export is current\\nState: /root/atelier/.atelier\\n\",\"truncated\":false}},\"path\":null,\"producer\":null,\"result\":\"pass\",\"spawn_error\":null,\"success\":true,\"target\":{\"id\":\"atelier-u6ax\",\"kind\":\"issue\",\"role\":\"validates\"},\"uri\":null}"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-u6ax"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Concrete issue Evidence lint enforced"
updated_at: "2026-06-12T22:42:29.080237340+00:00"
---

Concrete issue Evidence lint enforced

Command: bash -lc 'set -euo pipefail
cargo test --quiet --test cli_integration test_lint_rejects_missing_evidence_section
cargo test --quiet --test cli_integration test_lint_rejects_empty_evidence_section
cargo test --quiet --test cli_integration test_lint_rejects_vague_evidence_even_when_notes_name_a_command
cargo test --quiet --test cli_integration test_lint_accepts_concrete_evidence_without_optional_notes
cargo test --quiet --test cli_integration lint
cargo fmt -- --check
git diff --check
target/debug/atelier lint atelier-u6ax
target/debug/atelier lint
target/debug/atelier export --check'
Exit status: 0

Stdout summary:

running 1 test
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 248 filtered out; finished in 0.15s


running 1 test
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 248 filtered out; finished in 0.14s


running 1 test
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 248 filtered out; finished in 0.16s


running 1 test
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 248 filtered out; finished in 0.16s


running 15 tests
...............
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 234 filtered out; finished in 1.71s

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
