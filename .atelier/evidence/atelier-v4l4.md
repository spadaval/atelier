---
created_at: "2026-06-12T22:10:06.786690540+00:00"
id: "atelier-v4l4"
data: "{\"captured_at\":\"2026-06-12T22:10:02.539300232+00:00\",\"command\":\"bash -lc 'set -euo pipefail\\ntarget/debug/atelier issue transition atelier-cany --options\\ntarget/debug/atelier issue show atelier-cany\\ntarget/debug/atelier issue --help\\ncargo test --test cli_integration test_root_start_finish_and_issue_transition_surface -- --nocapture\\ntarget/debug/atelier export --check\\ntarget/debug/atelier lint atelier-cany\\ntarget/debug/atelier workflow validate issue atelier-cany'\",\"exit_code\":0,\"exit_status\":\"0\",\"kind\":\"validation\",\"output\":{\"limit_bytes_per_stream\":4096,\"stderr\":{\"bytes\":11047,\"summary\":\"   Compiling atelier-tracker v0.2.0 (/root/atelier)\\nwarning: function `list_all_issue_activities` is never used\\n   --> src/activity.rs:125:8\\n    |\\n125 | pub fn list_all_issue_activities(state_dir: &Path) -> Result<Vec<IssueActivity>> {\\n    |        ^^^^^^^^^^^^^^^^^^^^^^^^^\\n    |\\n    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default\\n\\nwarning: struct `CreateInput` is never constructed\\n    --> src/commands/agent_factory.rs:1252:12\\n     |\\n1252 | pub struct CreateInput<'a> {\\n     |            ^^^^^^^^^^^\\n\\nwarning: function `create` is never used\\n    --> src/commands/agent_factory.rs:1380:8\\n     |\\n1380 | pub fn create(db: &Database, input: CreateInput<'_>) -> Result<()> {\\n     |        ^^^^^^\\n\\nwarning: function `update` is never used\\n    --> src/commands/agent_factory.rs:1430:8\\n     |\\n1430 | pub fn update(db: &Database, input: UpdateInput<'_>) -> Result<()> {\\n     |        ^^^^^^\\n\\nwarning: function `close` is never used\\n    --> src/commands/agent_factory.rs:1757:8\\n     |\\n1757 | pub fn close(db: &Database, issue_ref: &str, reason: Option<&str>) -> Result<()> {\\n     |        ^^^^^\\n\\nwarning: function `ensure_canonical_issue_sections_valid` is never used\\n    --> src/commands/agent_factory.rs:1846:4\\n     |\\n1846 | fn ensure_canonical_issue_sections_valid(issue_id: &str) -> Result<()> {\\n     |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\\n\\nwarning: function `reopen` is never used\\n    --> src/commands/agent_factory.rs:1854:8\\n     |\\n1854 | pub fn reopen(db: &Database, issue_ref: &str) -> Result<()> {\\n     |        ^^^^^^\\n\\nwarning: function `dep_add` is never used\\n    --> src/commands/agent_factory.rs:1982:8\\n     |\\n1982 | pub fn dep_add(db: &Database, blocked_ref: &str, blocker_ref: &str) -> Result<()> {\\n     |        ^^^^^^^\\n\\nwarning: function `dep_remove` is never used\\n    --> src/commands/agent_factory.rs:2001:8\\n     |\\n2001 | pub fn dep_remove(db: &Database, blocked_ref: &str, blocker_ref: &str) -> Result<()> {\\n     |        ^^^^^^^^^^\\n\\nwarning: function `run` is never used\\n  --> src/commands/comment.rs:21:8\\n   |\\n21 | pub fn run(db: &Database, issue_id: &str, content: &str, kind: &str) -> Result<()> {\\n   |        ^^^\\n\\nwarning: struct `CreateOpts` is never constructed\\n  --> src/commands/create.rs:79:12\\n   |\\n79 | pub struct CreateOpts<'a> {\\n   |            ^^^^^^^^^^\\n\\nwarning: function `run` is never used\\n  --> src/commands/create.rs:86:8\\n   |\\n86 | pub fn run(\\n   |        ^^^\\n\\nwarning: function `run_subissue` is never used\\n   --> src/commands/create.rs:175:8\\n    |\\n175 | pub fn run_subissue(\\n    |        ^^^^^^^^^^^^\\n\\nwarning: function `run` is never used\\n --> src/commands/delete.rs:9:8\\n  |\\n9 | pub fn run(db: &Database, id: &str, force: bool) -> Result<()> {\\n  |        ^^^\\n\\nwarning: function `add` is never used\\n --> src/commands/label.rs:7:8\\n  |\\n7 | pub fn add(db: &Database, issue_id: &str, label: &str) -> Result<()> {\\n  |        ^^^\\n\\nwarning: function `remove` is never used\\n  --> src/commands/label.rs:50:8\\n   |\\n50 | pub fn remove(db: &Database, issue_id: &str, label: &str) -> Result<()> {\\n   |        ^^^^^^\\n\\nwarning: function `refresh_open_database_after_canonical_write` is never used\\n  --> src/commands/projection.rs:26:8\\n   |\\n26 | pub fn refresh_open_database_after_canonical_write(\\n   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\\n\\nwarning: function `add_typed` is never used\\n --> src/commands/relate.rs:7:8\\n  |\\n7 | pub fn add_typed(\\n  |        ^^^^^^^^^\\n\\nwarning: function `remove_typed` is never used\\n  --> src/commands/relate.rs:66:8\\n   |\\n66 | pub fn remove_typed(\\n   |        ^^^^^^^^^^^^\\n\\nwarning: function `close_all` is never used\\n  --> src/commands/status.rs:92:8\\n   |\\n92 | pub fn close_all(\\n   |        ^^^^^^^^^\\n\\nwarning: method `remove_dependency` is never used\\n  --> src/db/dependencies.rs:53:12\\n   |\\n 7 | impl Database {\\n   | ------------- method in this implementation\\n...\\n53 |     pub fn remove_dependency(\\n   |            ^^^^^^^^^^^^^^^^^\\n\\nwarning: multiple methods are never used\\n   --> src/db/issues.rs:54:12\\n    |\\n 10 | impl Database {\\n    | ------------- methods in this implementation\\n...\\n 54 |     pub fn\",\"truncated\":true},\"stdout\":{\"bytes\":4820,\"summary\":\"Issue Transitions atelier-cany - Implement issue transition options surface\\n===========================================================================\\nState\\n-----\\nStatus:   open\\nWork:     none active\\n\\nAllowed Actions\\n---------------\\n  start\\n    Reason:  issue is open and required sections parse\\n    Command: atelier start atelier-cany\\n\\nBlocked Actions\\n---------------\\n  finish\\n    Reason:  no active work is associated with this issue\\n    Command: atelier finish atelier-cany\\n  close\\n    Reason:  no validating evidence linked\\n    Command: atelier issue close atelier-cany --reason \\\"...\\\"\\n  reopen\\n    Reason:  issue is already open\\n    Command: atelier issue update atelier-cany --status open\\n  repair sections\\n    Reason:  required sections parse\\n    Command: atelier lint atelier-cany\\natelier-cany [task] open - Implement issue transition options surface\\n=====================================================================\\nStatus:   open\\nType:     task\\nPriority: high\\nCreated:  2026-06-12 16:29 -04:00\\nUpdated:  2026-06-12 16:29 -04:00\\nLabels:   cli, implementation, workflow\\nFile:     /root/atelier/.atelier/issues/atelier-cany.md\\n\\nHierarchy\\n---------\\nParent: atelier-wpyb [open] high - Repair status start history prime and transition surfaces\\n\\nTransition Readiness\\n--------------------\\n  start: ready - issue is open and required sections parse\\n    atelier start atelier-cany\\n  close: blocked - no validating evidence linked\\n  options: atelier issue transition atelier-cany --options\\n\\nDescription\\n-----------\\nImplement the user-facing issue transition options surface specified in\\n`atelier-vr9g`. Operators should be able to ask what an issue can do next\\nwithout knowing internal workflow validator names.\\n\\nOutcome\\n-------\\n- A command such as `atelier issue transition <id> --options` or the approved\\n  equivalent lists allowed actions, blocked actions, fast gate reasons, and the\\n  command to perform each allowed action.\\n- Issue show includes compact transition readiness without dumping validator\\n  internals.\\n- Missing evidence, open blockers, malformed sections, active work, and closed\\n  issue states are shown as actionable transition blockers.\\n- Expensive proof remains in evidence records and validation transcripts, not\\n  synchronous transition calculation.\\n- Help text positions raw workflow validation as advanced/internal rather than\\n  normal next-action discovery.\\n\\nEvidence\\n--------\\n- CLI transcript tests for open ready issue, blocked issue, closed issue,\\n  active-work issue, close-ready issue, and missing-evidence close block.\\n- Consistency check between issue show, transition/options output, and closeout\\n  behavior.\\n- Help transcript proving the transition surface is discoverable.\\n\\nBlocked by\\n----------\\n(none)\\n\\nBlocking\\n--------\\n  atelier-a4sn [open] high - Remove workflow validate from the normal public workflow\\n  atelier-trr2 [open] high - Validate workflow signpost surfaces end to end\\n\\nSubissues\\n---------\\n(none)\\n\\nRecent Activity\\n---------------\\n(none)\\n\\nNext Commands\\n-------------\\n  Edit issue Markdown: /root/atelier/.atelier/issues/atelier-cany.md\\n  Validate this issue: atelier lint atelier-cany\\n  Add a note: atelier note add issue atelier-cany \\\"...\\\"\\n  Show transition options: atelier issue transition atelier-cany --options\\n  Start tracked work: atelier start atelier-cany\\n  Close when complete: atelier issue close atelier-cany --reason \\\"...\\\"\\nIssue lifecycle commands (create, show, list, close, ...)\\n\\nUsage: atelier issue [OPTIONS] <COMMAND>\\n\\nCommands:\\n  create      Create a new issue\\n  list        List issues\\n  show        Show issue details\\n  transition  Show issue transition options and blockers\\n  update      Update an issue\\n  close       Close an issue\\n  help        Print this message or the help of the given subcommand(s)\\n\\nOptions:\\n  -q, --quiet                    Quiet mode: only output essential data (IDs, counts)\\n      --log-level <LOG_LEVEL>    Log level for diagnostic output (error, warn, info, debug, trace) [env: ATELIER_LOG=] [default: warn]\\n      --log-format <LOG_FORMAT>  Log format (text, json) [env: ATELIER_LOG_FORMAT=] \",\"truncated\":true}},\"path\":null,\"producer\":null,\"result\":\"pass\",\"spawn_error\":null,\"success\":true,\"target\":{\"id\":\"atelier-cany\",\"kind\":\"issue\",\"role\":\"validates\"},\"uri\":null}"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-cany"
    type: "validates"
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "atelier-cany transition surface validation"
updated_at: "2026-06-12T22:10:08.150077165+00:00"
---

atelier-cany transition surface validation

Command: bash -lc 'set -euo pipefail
target/debug/atelier issue transition atelier-cany --options
target/debug/atelier issue show atelier-cany
target/debug/atelier issue --help
cargo test --test cli_integration test_root_start_finish_and_issue_transition_surface -- --nocapture
target/debug/atelier export --check
target/debug/atelier lint atelier-cany
target/debug/atelier workflow validate issue atelier-cany'
Exit status: 0

Stdout summary (truncated):
Issue Transitions atelier-cany - Implement issue transition options surface
===========================================================================
State
-----
Status:   open
Work:     none active

Allowed Actions
---------------
  start
    Reason:  issue is open and required sections parse
    Command: atelier start atelier-cany

Blocked Actions
---------------
  finish
    Reason:  no active work is associated with this issue
    Command: atelier finish atelier-cany
  close
    Reason:  no validating evidence linked
    Command: atelier issue close atelier-cany --reason "..."
  reopen
    Reason:  issue is already open
    Command: atelier issue update atelier-cany --status open
  repair sections
    Reason:  required sections parse
    Command: atelier lint atelier-cany
atelier-cany [task] open - Implement issue transition options surface
=====================================================================
Status:   open
Type:     task
Priority: high
Created:  2026-06-12 16:29 -04:00
Updated:  2026-06-12 16:29 -04:00
Labels:   cli, implementation, workflow
File:     /root/atelier/.atelier/issues/atelier-cany.md

Hierarchy
---------
Parent: atelier-wpyb [open] high - Repair status start history prime and transition surfaces

Transition Readiness
--------------------
  start: ready - issue is open and required sections parse
    atelier start atelier-cany
  close: blocked - no validating evidence linked
  options: atelier issue transition atelier-cany --options

Description
-----------
Implement the user-facing issue transition options surface specified in
`atelier-vr9g`. Operators should be able to ask what an issue can do next
without knowing internal workflow validator names.

Outcome
-------
- A command such as `atelier issue transition <id> --options` or the approved
  equivalent lists allowed actions, blocked actions, fast gate reasons, and the
  command to perform each allowed action.
- Issue show includes compact transition readiness without dumping validator
  internals.
- Missing evidence, open blockers, malformed sections, active work, and closed
  issue states are shown as actionable transition blockers.
- Expensive proof remains in evidence records and validation transcripts, not
  synchronous transition calculation.
- Help text positions raw workflow validation as advanced/internal rather than
  normal next-action discovery.

Evidence
--------
- CLI transcript tests for open ready issue, blocked issue, closed issue,
  active-work issue, close-ready issue, and missing-evidence close block.
- Consistency check between issue show, transition/options output, and closeout
  behavior.
- Help transcript proving the transition surface is discoverable.

Blocked by
----------
(none)

Blocking
--------
  atelier-a4sn [open] high - Remove workflow validate from the normal public workflow
  atelier-trr2 [open] high - Validate workflow signpost surfaces end to end

Subissues
---------
(none)

Recent Activity
---------------
(none)

Next Commands
-------------
  Edit issue Markdown: /root/atelier/.atelier/issues/atelier-cany.md
  Validate this issue: atelier lint atelier-cany
  Add a note: atelier note add issue atelier-cany "..."
  Show transition options: atelier issue transition atelier-cany --options
  Start tracked work: atelier start atelier-cany
  Close when complete: atelier issue close atelier-cany --reason "..."
Issue lifecycle commands (create, show, list, close, ...)

Usage: atelier issue [OPTIONS] <COMMAND>

Commands:
  create      Create a new issue
  list        List issues
  show        Show issue details
  transition  Show issue transition options and blockers
  update      Update an issue
  close       Close an issue
  help        Print this message or the help of the given subcommand(s)

Options:
  -q, --quiet                    Quiet mode: only output essential data (IDs, counts)
      --log-level <LOG_LEVEL>    Log level for diagnostic output (error, warn, info, debug, trace) [env: ATELIER_LOG=] [default: warn]
      --log-format <LOG_FORMAT>  Log format (text, json) [env: ATELIER_LOG_FORMAT=]

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

