---
created_at: "2026-06-15T07:50:05.358602044+00:00"
id: "atelier-agci"
evidence_type: "validation"
captured_at: "2026-06-15T07:49:57.612631726+00:00"
command: "sh -c 'cargo check && target/debug/atelier lint atelier-uz8g && target/debug/atelier export --check && git diff --check'"
exit_status: "0"
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-uz8g"
  role: "validates"
follow_up_ids: []
residual_risks: []
output:
  limit_bytes_per_stream: 4096
  stdout:
    bytes: 103
    summary: "Lint passed.\nCanonical export is current\nState: /root/atelier/.atelier-worktrees/atelier-v5nb/.atelier\n"
    truncated: false
  stderr:
    bytes: 5446
    summary: "    Blocking waiting for file lock on package cache\n    Blocking waiting for file lock on package cache\n    Blocking waiting for file lock on package cache\n    Blocking waiting for file lock on package cache\n    Blocking waiting for file lock on build directory\n   Compiling atelier-tracker v0.2.0 (/root/atelier/.atelier-worktrees/atelier-v5nb)\nwarning: function `find_atelier_dir` is never used\n --> src/command_storage.rs:9:8\n  |\n9 | pub fn find_atelier_dir() -> Result<PathBuf> {\n  |        ^^^^^^^^^^^^^^^^\n  |\n  = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default\n\nwarning: variants `Failed`, `Deferred`, and `NotApplicable` are never constructed\n   --> src/commands/agent_factory.rs:600:5\n    |\n597 | pub(crate) enum ProofCoverageStatus {\n    |                 ------------------- variants in this enum\n...\n600 |     Failed,\n    |     ^^^^^^\n601 |     Blocked,\n602 |     Deferred,\n    |     ^^^^^^^^\n603 |     NotApplicable,\n    |     ^^^^^^^^^^^^^\n    |\n    = note: `ProofCoverageStatus` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis\n\nwarning: constant `KNOWN_COMMENT_KINDS` is never used\n --> src/commands/comment.rs:6:7\n  |\n6 | const KNOWN_COMMENT_KINDS: &[&str] = &[\n  |       ^^^^^^^^^^^^^^^^^^^\n\nwarning: function `validate_comment_kind` is never used\n  --> src/commands/comment.rs:17:8\n   |\n17 | pub fn validate_comment_kind(kind: &str) -> bool {\n   |        ^^^^^^^^^^^^^^^^^^^^^\n\nwarning: function `run_canonical` is never used\n  --> src/commands/comment.rs:38:8\n   |\n38 | pub fn run_canonical(db: &Database, issue_id: &str, content: &str, kind: &str) -> Result<()> {\n   |        ^^^^^^^^^^^^^\n\nwarning: function `add_canonical` is never used\n  --> src/commands/label.rs:27:8\n   |\n27 | pub fn add_canonical(\n   |        ^^^^^^^^^^^^^\n\nwarning: function `remove_canonical` is never used\n  --> src/commands/label.rs:71:8\n   |\n71 | pub fn remove_canonical(\n   |        ^^^^^^^^^^^^^^^^\n\nwarning: type alias `Progress` is never used\n --> src/commands/next.rs:8:6\n  |\n8 | type Progress = Option<(i32, i32)>;\n  |      ^^^^^^^^\n\nwarning: type alias `ScoredIssue` is never used\n  --> src/commands/next.rs:11:6\n   |\n11 | type ScoredIssue = (Issue, i32, Progress);\n   |      ^^^^^^^^^^^\n\nwarning: function `priority_weight` is never used\n  --> src/commands/next.rs:14:4\n   |\n14 | fn priority_weight(priority: &str) -> i32 {\n   |    ^^^^^^^^^^^^^^^\n\nwarning: function `calculate_progress` is never used\n  --> src/commands/next.rs:25:4\n   |\n25 | fn calculate_progress(db: &Database, issue: &Issue) -> Result<Progress> {\n   |    ^^^^^^^^^^^^^^^^^^\n\nwarning: function `run` is never used\n  --> src/commands/next.rs:39:8\n   |\n39 | pub fn run(db: &Database) -> Result<()> {\n   |        ^^^\n\nwarning: function `add_typed_canonical` is never used\n  --> src/commands/relate.rs:38:8\n   |\n38 | pub fn add_typed_canonical(\n   |        ^^^^^^^^^^^^^^^^^^^\n\nwarning: function `remove_typed_canonical` is never used\n  --> src/commands/relate.rs:96:8\n   |\n96 | pub fn remove_typed_canonical(\n   |        ^^^^^^^^^^^^^^^^^^^^^^\n\nwarning: function `list` is never used\n   --> src/commands/relate.rs:126:8\n    |\n126 | pub fn list(db: &Database, issue_id: &str) -> Result<()> {\n    |        ^^^^\n\nwarning: function `close_all_lifecycle` is never used\n   --> src/commands/status.rs:428:8\n    |\n428 | pub fn close_all_lifecycle(\n    |        ^^^^^^^^^^^^^^^^^^^\n\nwarning: function `run` is never used\n --> src/commands/tested.rs:5:8\n  |\n5 | pub fn run(atelier_dir: &Path) -> Result<()> {\n  |        ^^^\n\nwarning: constant `WELL_KNOWN_RELATION_TYPES` is never used\n  --> src/db/mod.rs:23:11\n   |\n23 | pub const WELL_KNOWN_RELATION_TYPES: &[&str] = &[\n   |           ^^^^^^^^^^^^^^^^^^^^^^^^^\n\nwarning: function `validate_relation_type` is never used\n   --> src/db/mod.rs:128:8\n    |\n128 | pub fn validate_relation_type(relation_type: &str) -> Result<()> {\n    |        ^^^^^^^^^^^^^^^^^^^^^^\n\nwarning: function `validate_relationship_type` is never used\n   --> src/db/mod.rs:161:8\n    |\n161 | pub fn validate_relationship_ty"
    truncated: true
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-uz8g"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "sh -c 'cargo check && target/debug/atelier lint atelier-uz8g && target/debug/atelier export --check && git diff --check'"
updated_at: "2026-06-15T07:50:08.533662030+00:00"
---

sh -c 'cargo check && target/debug/atelier lint atelier-uz8g && target/debug/atelier export --check && git diff --check'

Command: sh -c 'cargo check && target/debug/atelier lint atelier-uz8g && target/debug/atelier export --check && git diff --check'
Exit status: 0

Stdout summary:
Lint passed.
Canonical export is current
State: /root/atelier/.atelier-worktrees/atelier-v5nb/.atelier

Stderr summary (truncated):
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on build directory
   Compiling atelier-tracker v0.2.0 (/root/atelier/.atelier-worktrees/atelier-v5nb)
warning: function `find_atelier_dir` is never used
 --> src/command_storage.rs:9:8
  |
9 | pub fn find_atelier_dir() -> Result<PathBuf> {
  |        ^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: variants `Failed`, `Deferred`, and `NotApplicable` are never constructed
   --> src/commands/agent_factory.rs:600:5
    |
597 | pub(crate) enum ProofCoverageStatus {
    |                 ------------------- variants in this enum
...
600 |     Failed,
    |     ^^^^^^
601 |     Blocked,
602 |     Deferred,
    |     ^^^^^^^^
603 |     NotApplicable,
    |     ^^^^^^^^^^^^^
    |
    = note: `ProofCoverageStatus` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis

warning: constant `KNOWN_COMMENT_KINDS` is never used
 --> src/commands/comment.rs:6:7
  |
6 | const KNOWN_COMMENT_KINDS: &[&str] = &[
  |       ^^^^^^^^^^^^^^^^^^^

warning: function `validate_comment_kind` is never used
  --> src/commands/comment.rs:17:8
   |
17 | pub fn validate_comment_kind(kind: &str) -> bool {
   |        ^^^^^^^^^^^^^^^^^^^^^

warning: function `run_canonical` is never used
  --> src/commands/comment.rs:38:8
   |
38 | pub fn run_canonical(db: &Database, issue_id: &str, content: &str, kind: &str) -> Result<()> {
   |        ^^^^^^^^^^^^^

warning: function `add_canonical` is never used
  --> src/commands/label.rs:27:8
   |
27 | pub fn add_canonical(
   |        ^^^^^^^^^^^^^

warning: function `remove_canonical` is never used
  --> src/commands/label.rs:71:8
   |
71 | pub fn remove_canonical(
   |        ^^^^^^^^^^^^^^^^

warning: type alias `Progress` is never used
 --> src/commands/next.rs:8:6
  |
8 | type Progress = Option<(i32, i32)>;
  |      ^^^^^^^^

warning: type alias `ScoredIssue` is never used
  --> src/commands/next.rs:11:6
   |
11 | type ScoredIssue = (Issue, i32, Progress);
   |      ^^^^^^^^^^^

warning: function `priority_weight` is never used
  --> src/commands/next.rs:14:4
   |
14 | fn priority_weight(priority: &str) -> i32 {
   |    ^^^^^^^^^^^^^^^

warning: function `calculate_progress` is never used
  --> src/commands/next.rs:25:4
   |
25 | fn calculate_progress(db: &Database, issue: &Issue) -> Result<Progress> {
   |    ^^^^^^^^^^^^^^^^^^

warning: function `run` is never used
  --> src/commands/next.rs:39:8
   |
39 | pub fn run(db: &Database) -> Result<()> {
   |        ^^^

warning: function `add_typed_canonical` is never used
  --> src/commands/relate.rs:38:8
   |
38 | pub fn add_typed_canonical(
   |        ^^^^^^^^^^^^^^^^^^^

warning: function `remove_typed_canonical` is never used
  --> src/commands/relate.rs:96:8
   |
96 | pub fn remove_typed_canonical(
   |        ^^^^^^^^^^^^^^^^^^^^^^

warning: function `list` is never used
   --> src/commands/relate.rs:126:8
    |
126 | pub fn list(db: &Database, issue_id: &str) -> Result<()> {
    |        ^^^^

warning: function `close_all_lifecycle` is never used
   --> src/commands/status.rs:428:8
    |
428 | pub fn close_all_lifecycle(
    |        ^^^^^^^^^^^^^^^^^^^

warning: function `run` is never used
 --> src/commands/tested.rs:5:8
  |
5 | pub fn run(atelier_dir: &Path) -> Result<()> {
  |        ^^^

warning: constant `WELL_KNOWN_RELATION_TYPES` is never used
  --> src/db/mod.rs:23:11
   |
23 | pub const WELL_KNOWN_RELATION_TYPES: &[&str] = &[
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `validate_relation_type` is never used
   --> src/db/mod.rs:128:8
    |
128 | pub fn validate_relation_type(relation_type: &str) -> Result<()> {
    |        ^^^^^^^^^^^^^^^^^^^^^^

warning: function `validate_relationship_type` is never used
   --> src/db/mod.rs:161:8
    |
161 | pub fn validate_relationship_ty

