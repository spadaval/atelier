---
created_at: "2026-06-12T21:34:14.023229453+00:00"
id: "atelier-826i"
data: "{\"captured_at\":\"2026-06-12T21:33:59.268215097+00:00\",\"command\":\"bash -lc 'target/debug/atelier issue --help && target/debug/atelier --help && cargo test --test cli_integration test_issue_help_uses_reduced_lifecycle_surface -- --nocapture && cargo test --test cli_integration test_non_lifecycle_issue_flows_use_explicit_homes -- --nocapture && cargo test --test cli_integration test_hidden_issue_helpers_print_replacement_guidance -- --nocapture && cargo test --test cli_integration test_update_issue_remove_label_replaces_unlabel_helper -- --nocapture && cargo test --test cli_integration test_issue_list_blocked_replaces_blocked_helper -- --nocapture'\",\"exit_code\":0,\"exit_status\":\"0\",\"kind\":\"validation\",\"output\":{\"limit_bytes_per_stream\":4096,\"stderr\":{\"bytes\":51940,\"summary\":\"   Compiling atelier-tracker v0.2.0 (/root/atelier)\\nwarning: function `list_all_issue_activities` is never used\\n   --> src/activity.rs:125:8\\n    |\\n125 | pub fn list_all_issue_activities(state_dir: &Path) -> Result<Vec<IssueActivity>> {\\n    |        ^^^^^^^^^^^^^^^^^^^^^^^^^\\n    |\\n    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default\\n\\nwarning: struct `CreateInput` is never constructed\\n    --> src/commands/agent_factory.rs:1252:12\\n     |\\n1252 | pub struct CreateInput<'a> {\\n     |            ^^^^^^^^^^^\\n\\nwarning: function `create` is never used\\n    --> src/commands/agent_factory.rs:1380:8\\n     |\\n1380 | pub fn create(db: &Database, input: CreateInput<'_>) -> Result<()> {\\n     |        ^^^^^^\\n\\nwarning: function `update` is never used\\n    --> src/commands/agent_factory.rs:1430:8\\n     |\\n1430 | pub fn update(db: &Database, input: UpdateInput<'_>) -> Result<()> {\\n     |        ^^^^^^\\n\\nwarning: function `close` is never used\\n    --> src/commands/agent_factory.rs:1757:8\\n     |\\n1757 | pub fn close(db: &Database, issue_ref: &str, reason: Option<&str>) -> Result<()> {\\n     |        ^^^^^\\n\\nwarning: function `ensure_canonical_issue_sections_valid` is never used\\n    --> src/commands/agent_factory.rs:1846:4\\n     |\\n1846 | fn ensure_canonical_issue_sections_valid(issue_id: &str) -> Result<()> {\\n     |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\\n\\nwarning: function `reopen` is never used\\n    --> src/commands/agent_factory.rs:1854:8\\n     |\\n1854 | pub fn reopen(db: &Database, issue_ref: &str) -> Result<()> {\\n     |        ^^^^^^\\n\\nwarning: function `dep_add` is never used\\n    --> src/commands/agent_factory.rs:1982:8\\n     |\\n1982 | pub fn dep_add(db: &Database, blocked_ref: &str, blocker_ref: &str) -> Result<()> {\\n     |        ^^^^^^^\\n\\nwarning: function `dep_remove` is never used\\n    --> src/commands/agent_factory.rs:2001:8\\n     |\\n2001 | pub fn dep_remove(db: &Database, blocked_ref: &str, blocker_ref: &str) -> Result<()> {\\n     |        ^^^^^^^^^^\\n\\nwarning: function `run` is never used\\n  --> src/commands/comment.rs:21:8\\n   |\\n21 | pub fn run(db: &Database, issue_id: &str, content: &str, kind: &str) -> Result<()> {\\n   |        ^^^\\n\\nwarning: struct `CreateOpts` is never constructed\\n  --> src/commands/create.rs:79:12\\n   |\\n79 | pub struct CreateOpts<'a> {\\n   |            ^^^^^^^^^^\\n\\nwarning: function `run` is never used\\n  --> src/commands/create.rs:86:8\\n   |\\n86 | pub fn run(\\n   |        ^^^\\n\\nwarning: function `run_subissue` is never used\\n   --> src/commands/create.rs:175:8\\n    |\\n175 | pub fn run_subissue(\\n    |        ^^^^^^^^^^^^\\n\\nwarning: function `run` is never used\\n --> src/commands/delete.rs:9:8\\n  |\\n9 | pub fn run(db: &Database, id: &str, force: bool) -> Result<()> {\\n  |        ^^^\\n\\nwarning: function `add` is never used\\n --> src/commands/label.rs:7:8\\n  |\\n7 | pub fn add(db: &Database, issue_id: &str, label: &str) -> Result<()> {\\n  |        ^^^\\n\\nwarning: function `remove` is never used\\n  --> src/commands/label.rs:50:8\\n   |\\n50 | pub fn remove(db: &Database, issue_id: &str, label: &str) -> Result<()> {\\n   |        ^^^^^^\\n\\nwarning: function `refresh_open_database_after_canonical_write` is never used\\n  --> src/commands/projection.rs:26:8\\n   |\\n26 | pub fn refresh_open_database_after_canonical_write(\\n   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\\n\\nwarning: function `add_typed` is never used\\n --> src/commands/relate.rs:7:8\\n  |\\n7 | pub fn add_typed(\\n  |        ^^^^^^^^^\\n\\nwarning: function `remove_typed` is never used\\n  --> src/commands/relate.rs:66:8\\n   |\\n66 | pub fn remove_typed(\\n   |        ^^^^^^^^^^^^\\n\\nwarning: function `close_all` is never used\\n  --> src/commands/status.rs:92:8\\n   |\\n92 | pub fn close_all(\\n   |        ^^^^^^^^^\\n\\nwarning: method `remove_dependency` is never used\\n  --> src/db/dependencies.rs:53:12\\n   |\\n 7 | impl Database {\\n   | ------------- method in this implementation\\n...\\n53 |     pub fn remove_dependency(\\n   |            ^^^^^^^^^^^^^^^^^\\n\\nwarning: multiple methods are never used\\n   --> src/db/issues.rs:54:12\\n    |\\n 10 | impl Database {\\n    | ------------- methods in this implementation\\n...\\n 54 |     pub fn\",\"truncated\":true},\"stdout\":{\"bytes\":3799,\"summary\":\"Issue lifecycle commands (create, show, list, close, ...)\\n\\nUsage: atelier issue [OPTIONS] <COMMAND>\\n\\nCommands:\\n  create      Create a new issue\\n  list        List issues\\n  show        Show issue details\\n  transition  Show issue transition options and blockers\\n  update      Update an issue\\n  close       Close an issue\\n  help        Print this message or the help of the given subcommand(s)\\n\\nOptions:\\n  -q, --quiet                    Quiet mode: only output essential data (IDs, counts)\\n      --log-level <LOG_LEVEL>    Log level for diagnostic output (error, warn, info, debug, trace) [env: ATELIER_LOG=] [default: warn]\\n      --log-format <LOG_FORMAT>  Log format (text, json) [env: ATELIER_LOG_FORMAT=] [default: text]\\n  -h, --help                     Print help\\nA simple, lean issue tracker CLI\\n\\nUsage: atelier [OPTIONS] <COMMAND>\\n\\n\\n\\nSetup:\\n  init          Initialize Atelier in the current repository\\n\\nOrientation:\\n  status        Show checkout, mission, work, and tracker signposts\\n  start         Start tracked work on an issue\\n  finish        Finish tracked work, defaulting to active work\\n\\nIssues:\\n  issue         Create, list, show, update, and close issues\\n  dep           Manage issue blockers with add, remove, and list\\n  search        Search issue text\\n  link          Manage typed issue links\\n  graph         Inspect issue hierarchy and impact\\n  note          Add issue activity notes\\n\\nMissions and planning:\\n  mission       Create, list, show, status, and update durable missions\\n  plan          Create, apply, revise, list, and link durable plans\\n\\nRecords:\\n  evidence      Capture validation evidence\\n\\nAdvanced work:\\n  worktree      Create, inspect, merge, and remove issue worktrees\\n\\nState management:\\n  export        Write or check canonical tracker records\\n  rebuild       Rebuild local SQLite state from canonical tracker records\\n  import-beads  Import an external Beads JSONL backup\\n\\nIntegrations:\\n  integrations  Install optional integrations such as Claude hooks\\n\\nMaintenance:\\n  maintenance   Run explicit destructive maintenance commands\\n  diagnostics   Inspect local command diagnostics\\n  lint          Validate tracker records\\n  doctor        Check runtime and exported-state health\\n\\nCommon commands:\\n  atelier status\\n  atelier issue list\\n  atelier issue list --ready\\n  atelier issue show <id>\\n  atelier mission list\\n  atelier mission show <id>\\n  atelier mission status\\n  atelier start <issue-id>\\n  atelier finish [issue-id]\\n  atelier doctor\\n  atelier help <command>\\n\\nOptions:\\n  -q, --quiet                    Quiet mode: only output essential data (IDs, counts)\\n      --log-level <LOG_LEVEL>    Log level for diagnostic output (error, warn, info, debug, trace) [env: ATELIER_LOG=] [default: warn]\\n      --log-format <LOG_FORMAT>  Log format (text, json) [env: ATELIER_LOG_FORMAT=] [default: text]\\n  -h, --help                     Print help\\n  -V, --version                  Print version\\n\\nrunning 1 test\\ntest test_issue_help_uses_reduced_lifecycle_surface ... ok\\n\\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 237 filtered out; finished in 0.07s\\n\\n\\nrunning 1 test\\ntest test_non_lifecycle_issue_flows_use_explicit_homes ... ok\\n\\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 237 filtered out; finished in 0.70s\\n\\n\\nrunning 1 test\\ntest test_hidden_issue_helpers_print_replacement_guidance ... ok\\n\\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 237 filtered out; finished in 0.44s\\n\\n\\nrunning 1 test\\ntest test_update_issue_remove_label_replaces_unlabel_helper ... ok\\n\\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 237 filtered out; finished in 0.24s\\n\\n\\nrunning 1 test\\ntest test_issue_list_blocked_replaces_blocked_helper ... ok\\n\\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 237 filtered out; finished in 0.31s\\n\\n\",\"truncated\":false}},\"path\":null,\"producer\":null,\"result\":\"pass\",\"spawn_error\":null,\"success\":true,\"target\":{\"id\":\"atelier-auqt\",\"kind\":\"issue\",\"role\":\"validates\"},\"uri\":null}"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-auqt"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "atelier-auqt reduced issue command surface validation"
updated_at: "2026-06-12T21:34:15.197155190+00:00"
---

atelier-auqt reduced issue command surface validation

Command: bash -lc 'target/debug/atelier issue --help && target/debug/atelier --help && cargo test --test cli_integration test_issue_help_uses_reduced_lifecycle_surface -- --nocapture && cargo test --test cli_integration test_non_lifecycle_issue_flows_use_explicit_homes -- --nocapture && cargo test --test cli_integration test_hidden_issue_helpers_print_replacement_guidance -- --nocapture && cargo test --test cli_integration test_update_issue_remove_label_replaces_unlabel_helper -- --nocapture && cargo test --test cli_integration test_issue_list_blocked_replaces_blocked_helper -- --nocapture'
Exit status: 0

Stdout summary:
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
      --log-format <LOG_FORMAT>  Log format (text, json) [env: ATELIER_LOG_FORMAT=] [default: text]
  -h, --help                     Print help
A simple, lean issue tracker CLI

Usage: atelier [OPTIONS] <COMMAND>



Setup:
  init          Initialize Atelier in the current repository

Orientation:
  status        Show checkout, mission, work, and tracker signposts
  start         Start tracked work on an issue
  finish        Finish tracked work, defaulting to active work

Issues:
  issue         Create, list, show, update, and close issues
  dep           Manage issue blockers with add, remove, and list
  search        Search issue text
  link          Manage typed issue links
  graph         Inspect issue hierarchy and impact
  note          Add issue activity notes

Missions and planning:
  mission       Create, list, show, status, and update durable missions
  plan          Create, apply, revise, list, and link durable plans

Records:
  evidence      Capture validation evidence

Advanced work:
  worktree      Create, inspect, merge, and remove issue worktrees

State management:
  export        Write or check canonical tracker records
  rebuild       Rebuild local SQLite state from canonical tracker records
  import-beads  Import an external Beads JSONL backup

Integrations:
  integrations  Install optional integrations such as Claude hooks

Maintenance:
  maintenance   Run explicit destructive maintenance commands
  diagnostics   Inspect local command diagnostics
  lint          Validate tracker records
  doctor        Check runtime and exported-state health

Common commands:
  atelier status
  atelier issue list
  atelier issue list --ready
  atelier issue show <id>
  atelier mission list
  atelier mission show <id>
  atelier mission status
  atelier start <issue-id>
  atelier finish [issue-id]
  atelier doctor
  atelier help <command>

Options:
  -q, --quiet                    Quiet mode: only output essential data (IDs, counts)
      --log-level <LOG_LEVEL>    Log level for diagnostic output (error, warn, info, debug, trace) [env: ATELIER_LOG=] [default: warn]
      --log-format <LOG_FORMAT>  Log format (text, json) [env: ATELIER_LOG_FORMAT=] [default: text]
  -h, --help                     Print help
  -V, --version                  Print version

running 1 test
test test_issue_help_uses_reduced_lifecycle_surface ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 237 filtered out; finished in 0.07s


running 1 test
test test_non_lifecycle_issue_flows_use_explicit_homes ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 237 filtered out; finished in 0.70s


running 1 test
test test_hidden_issue_helpers_print_replacement_guidance ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 237 filtered out; finished in 0.44s


running 1 test
test test_update_issue_remove_label_replaces_unlabel_helper ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 237 filtered out; finished in 0.24s


running 1 test
test test_issue_list_blocked_replaces_blocked_helper ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 237 filtered out; finished in 0.31s

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

