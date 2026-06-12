---
created_at: "2026-06-12T22:08:49.583496405+00:00"
id: "atelier-4hec"
data: "{\"captured_at\":\"2026-06-12T22:08:42.721512963+00:00\",\"command\":\"bash -lc 'set -euo pipefail\\ncargo test --lib test_inventory -- --nocapture\\ncargo test --test cli_integration ignored -- --nocapture\\ntarget/debug/atelier mission status atelier-tcmr | tee /tmp/atelier-tcmr-status-after-jqds.txt\\nrg \\\"Validator ignored_tests_reviewed: pass\\\" /tmp/atelier-tcmr-status-after-jqds.txt\\n! rg \\\"ignored product-behavior test is still blocking closeout\\\" /tmp/atelier-tcmr-status-after-jqds.txt\\ntarget/debug/atelier export --check\\ntarget/debug/atelier lint atelier-jqds\\ntarget/debug/atelier workflow validate issue atelier-jqds'\",\"exit_code\":0,\"exit_status\":\"0\",\"kind\":\"validation\",\"output\":{\"limit_bytes_per_stream\":4096,\"stderr\":{\"bytes\":11251,\"summary\":\"   Compiling atelier-tracker v0.2.0 (/root/atelier)\\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.08s\\n     Running unittests src/lib.rs (target/debug/deps/atelier-0015c38aaafea46d)\\n   Compiling atelier-tracker v0.2.0 (/root/atelier)\\nwarning: function `list_all_issue_activities` is never used\\n   --> src/activity.rs:125:8\\n    |\\n125 | pub fn list_all_issue_activities(state_dir: &Path) -> Result<Vec<IssueActivity>> {\\n    |        ^^^^^^^^^^^^^^^^^^^^^^^^^\\n    |\\n    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default\\n\\nwarning: struct `CreateInput` is never constructed\\n    --> src/commands/agent_factory.rs:1252:12\\n     |\\n1252 | pub struct CreateInput<'a> {\\n     |            ^^^^^^^^^^^\\n\\nwarning: function `create` is never used\\n    --> src/commands/agent_factory.rs:1380:8\\n     |\\n1380 | pub fn create(db: &Database, input: CreateInput<'_>) -> Result<()> {\\n     |        ^^^^^^\\n\\nwarning: function `update` is never used\\n    --> src/commands/agent_factory.rs:1430:8\\n     |\\n1430 | pub fn update(db: &Database, input: UpdateInput<'_>) -> Result<()> {\\n     |        ^^^^^^\\n\\nwarning: function `close` is never used\\n    --> src/commands/agent_factory.rs:1757:8\\n     |\\n1757 | pub fn close(db: &Database, issue_ref: &str, reason: Option<&str>) -> Result<()> {\\n     |        ^^^^^\\n\\nwarning: function `ensure_canonical_issue_sections_valid` is never used\\n    --> src/commands/agent_factory.rs:1846:4\\n     |\\n1846 | fn ensure_canonical_issue_sections_valid(issue_id: &str) -> Result<()> {\\n     |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\\n\\nwarning: function `reopen` is never used\\n    --> src/commands/agent_factory.rs:1854:8\\n     |\\n1854 | pub fn reopen(db: &Database, issue_ref: &str) -> Result<()> {\\n     |        ^^^^^^\\n\\nwarning: function `dep_add` is never used\\n    --> src/commands/agent_factory.rs:1982:8\\n     |\\n1982 | pub fn dep_add(db: &Database, blocked_ref: &str, blocker_ref: &str) -> Result<()> {\\n     |        ^^^^^^^\\n\\nwarning: function `dep_remove` is never used\\n    --> src/commands/agent_factory.rs:2001:8\\n     |\\n2001 | pub fn dep_remove(db: &Database, blocked_ref: &str, blocker_ref: &str) -> Result<()> {\\n     |        ^^^^^^^^^^\\n\\nwarning: function `run` is never used\\n  --> src/commands/comment.rs:21:8\\n   |\\n21 | pub fn run(db: &Database, issue_id: &str, content: &str, kind: &str) -> Result<()> {\\n   |        ^^^\\n\\nwarning: struct `CreateOpts` is never constructed\\n  --> src/commands/create.rs:79:12\\n   |\\n79 | pub struct CreateOpts<'a> {\\n   |            ^^^^^^^^^^\\n\\nwarning: function `run` is never used\\n  --> src/commands/create.rs:86:8\\n   |\\n86 | pub fn run(\\n   |        ^^^\\n\\nwarning: function `run_subissue` is never used\\n   --> src/commands/create.rs:175:8\\n    |\\n175 | pub fn run_subissue(\\n    |        ^^^^^^^^^^^^\\n\\nwarning: function `run` is never used\\n --> src/commands/delete.rs:9:8\\n  |\\n9 | pub fn run(db: &Database, id: &str, force: bool) -> Result<()> {\\n  |        ^^^\\n\\nwarning: function `add` is never used\\n --> src/commands/label.rs:7:8\\n  |\\n7 | pub fn add(db: &Database, issue_id: &str, label: &str) -> Result<()> {\\n  |        ^^^\\n\\nwarning: function `remove` is never used\\n  --> src/commands/label.rs:50:8\\n   |\\n50 | pub fn remove(db: &Database, issue_id: &str, label: &str) -> Result<()> {\\n   |        ^^^^^^\\n\\nwarning: function `refresh_open_database_after_canonical_write` is never used\\n  --> src/commands/projection.rs:26:8\\n   |\\n26 | pub fn refresh_open_database_after_canonical_write(\\n   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\\n\\nwarning: function `add_typed` is never used\\n --> src/commands/relate.rs:7:8\\n  |\\n7 | pub fn add_typed(\\n  |        ^^^^^^^^^\\n\\nwarning: function `remove_typed` is never used\\n  --> src/commands/relate.rs:66:8\\n   |\\n66 | pub fn remove_typed(\\n   |        ^^^^^^^^^^^^\\n\\nwarning: function `close_all` is never used\\n  --> src/commands/status.rs:92:8\\n   |\\n92 | pub fn close_all(\\n   |        ^^^^^^^^^\\n\\nwarning: method `remove_dependency` is never used\\n  --> src/db/dependencies.rs:53:12\\n   |\\n 7 | impl Database {\\n   | ------------- method in this implementation\\n...\\n53 |     pub fn remove_dependency(\\n \",\"truncated\":true},\"stdout\":{\"bytes\":4804,\"summary\":\"\\nrunning 4 tests\\ntest test_inventory::tests::ignored_test_without_reason_or_owner_is_reported ... ok\\ntest test_inventory::tests::obsolete_ignored_test_requires_follow_up_issue ... ok\\ntest test_inventory::tests::ignored_product_behavior_defaults_to_closeout_blocker ... ok\\ntest test_inventory::tests::extended_product_behavior_can_be_explicitly_non_blocking ... ok\\n\\ntest result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 159 filtered out; finished in 0.00s\\n\\n\\nrunning 3 tests\\ntest test_workflow_validate_reports_ignored_tests_without_owner ... ok\\ntest test_rebuild_temp_files_are_ignored_by_query_lint_export_and_doctor ... ok\\ntest test_mission_status_shows_ignored_product_behavior_closeout_blocker ... ok\\n\\ntest result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 238 filtered out; finished in 0.84s\\n\\nLint passed.\\nMission Status atelier-tcmr [ready] - Repair CLI workflow rework and validation gaps\\n====================================================================================\\nHealth:   blocked\\nTracker:  ok\\nCloseout: blocked\\n\\nWork\\n----\\nTotal: 11 ready, 16 blocked, 23 done\\n  [epic] atelier-gjaz [open] high - Replace escaped mission data JSON with readable mission records | 1 ready, 1 blocked, 3 done\\n  [epic] atelier-nzy1 [open] high - Harden mission closeout validators and evidence requirements | 3 blocked\\n  [epic] atelier-wpyb [open] high - Repair status start history prime and transition surfaces | 3 ready, 1 blocked, 1 done\\n  [epic] atelier-ymfl [open] high - Align docs tests and Agent Factory process with enforced proof | 2 done\\n  [epic] atelier-zue4 [open] high - Overhaul mission validation and reliability system | 7 ready, 6 blocked, 4 done\\n  [epic] atelier-40ou [closed] high - Parse issue Markdown sections as first-class structure | 7 done\\n  [epic] atelier-efpk [closed] high - Repair and consolidate CLI command surfaces | 4 done\\n\\nBlockers\\n--------\\nMission blockers: 0 open\\nBlocked work: 16 blocked\\n\\nEvidence\\n--------\\nLinked evidence: 2\\n\\nCloseout Gates\\n--------------\\nWork: open - atelier-0u2k, atelier-1p83, atelier-6aor, atelier-8o34, atelier-9pkx, atelier-a4sn, atelier-cany, atelier-diom, atelier-g18z, atelier-gjaz, atelier-hah9, atelier-jqds, atelier-k9m8, atelier-nzy1, atelier-pvuz, atelier-pyre, atelier-qb7m, atelier-sckq, atelier-trr2, atelier-u4nx, atelier-u6ax, atelier-v9id, atelier-w8e3, atelier-wpyb, atelier-ymfl, atelier-ys5p, atelier-zue4\\n  Next: atelier issue close <issue-id> --reason \\\"...\\\"\\nBlockers: open - atelier-1p83, atelier-6aor, atelier-8o34, atelier-9pkx, atelier-a4sn, atelier-cany, atelier-diom, atelier-g18z, atelier-pvuz, atelier-pyre, atelier-sckq, atelier-trr2, atelier-u4nx, atelier-u6ax, atelier-v9id, atelier-w8e3, atelier-ys5p\\n  Next: close or unblock the blocker issues.\\nEvidence: attached\\nValidator durable_state_current: pass\\nValidator issue_sections_parseable: pass\\nValidator evidence_attached: pass\\nValidator no_open_blockers: fail - open blockers: atelier-1p83, atelier-6aor, atelier-8o34, atelier-9pkx, atelier-a4sn, atelier-cany, atelier-diom, atelier-g18z, atelier-pvuz, atelier-pyre, atelier-sckq, atelier-trr2, atelier-u4nx, atelier-u6ax, atelier-v9id, atelier-w8e3, atelier-ys5p\\nValidator no_blocking_lints: pass\\nValidator ignored_tests_reviewed: pass\\nValidator git_worktree_clean: fail - git worktree has 4 dirty entries:  M tests/cli_integration.rs;  M tests/smoke/cli_data.rs;  M tests/smoke/lifecycle.rs; ?? .atelier/issues/atelier-jqds.activity/20260612T220832693451Z.md\\n\\nValidators\\n----------\\n2 closeout validator failure detected.\\n  fail  no_open_blockers - open blockers: atelier-1p83, atelier-6aor, atelier-8o34, atelier-9pkx, atelier-a4sn, atelier-cany, atelier-diom, atelier-g18z, atelier-pvuz, atelier-pyre, atelier-sckq, atelier-trr2, atelier-u4nx, atelier-u6ax, atelier-v9id, atelier-w8e3, atelier-ys5p\\n  fail  git_worktree_clean - git worktree has 4 dirty entries:  M tests/cli_integration.rs;  M tests/smoke/cli_data.rs;  M tests/smoke/lifecycle.rs; ?? .atelier/issues/atelier-jqds.activity/20260612T220832693451Z.md\\n\\nActive Work\\n-----------\\n  atelier-jqds [active] branch=codex/orchestrate-at\",\"truncated\":true}},\"path\":null,\"producer\":null,\"result\":\"pass\",\"spawn_error\":null,\"success\":true,\"target\":{\"id\":\"atelier-jqds\",\"kind\":\"issue\",\"role\":\"validates\"},\"uri\":null}"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-jqds"
    type: "validates"
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "atelier-jqds ignored legacy test inventory cleanup"
updated_at: "2026-06-12T22:08:51.059964560+00:00"
---

atelier-jqds ignored legacy test inventory cleanup

Command: bash -lc 'set -euo pipefail
cargo test --lib test_inventory -- --nocapture
cargo test --test cli_integration ignored -- --nocapture
target/debug/atelier mission status atelier-tcmr | tee /tmp/atelier-tcmr-status-after-jqds.txt
rg "Validator ignored_tests_reviewed: pass" /tmp/atelier-tcmr-status-after-jqds.txt
! rg "ignored product-behavior test is still blocking closeout" /tmp/atelier-tcmr-status-after-jqds.txt
target/debug/atelier export --check
target/debug/atelier lint atelier-jqds
target/debug/atelier workflow validate issue atelier-jqds'
Exit status: 0

Stdout summary (truncated):

running 4 tests
test test_inventory::tests::ignored_test_without_reason_or_owner_is_reported ... ok
test test_inventory::tests::obsolete_ignored_test_requires_follow_up_issue ... ok
test test_inventory::tests::ignored_product_behavior_defaults_to_closeout_blocker ... ok
test test_inventory::tests::extended_product_behavior_can_be_explicitly_non_blocking ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 159 filtered out; finished in 0.00s


running 3 tests
test test_workflow_validate_reports_ignored_tests_without_owner ... ok
test test_rebuild_temp_files_are_ignored_by_query_lint_export_and_doctor ... ok
test test_mission_status_shows_ignored_product_behavior_closeout_blocker ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 238 filtered out; finished in 0.84s

Lint passed.
Mission Status atelier-tcmr [ready] - Repair CLI workflow rework and validation gaps
====================================================================================
Health:   blocked
Tracker:  ok
Closeout: blocked

Work
----
Total: 11 ready, 16 blocked, 23 done
  [epic] atelier-gjaz [open] high - Replace escaped mission data JSON with readable mission records | 1 ready, 1 blocked, 3 done
  [epic] atelier-nzy1 [open] high - Harden mission closeout validators and evidence requirements | 3 blocked
  [epic] atelier-wpyb [open] high - Repair status start history prime and transition surfaces | 3 ready, 1 blocked, 1 done
  [epic] atelier-ymfl [open] high - Align docs tests and Agent Factory process with enforced proof | 2 done
  [epic] atelier-zue4 [open] high - Overhaul mission validation and reliability system | 7 ready, 6 blocked, 4 done
  [epic] atelier-40ou [closed] high - Parse issue Markdown sections as first-class structure | 7 done
  [epic] atelier-efpk [closed] high - Repair and consolidate CLI command surfaces | 4 done

Blockers
--------
Mission blockers: 0 open
Blocked work: 16 blocked

Evidence
--------
Linked evidence: 2

Closeout Gates
--------------
Work: open - atelier-0u2k, atelier-1p83, atelier-6aor, atelier-8o34, atelier-9pkx, atelier-a4sn, atelier-cany, atelier-diom, atelier-g18z, atelier-gjaz, atelier-hah9, atelier-jqds, atelier-k9m8, atelier-nzy1, atelier-pvuz, atelier-pyre, atelier-qb7m, atelier-sckq, atelier-trr2, atelier-u4nx, atelier-u6ax, atelier-v9id, atelier-w8e3, atelier-wpyb, atelier-ymfl, atelier-ys5p, atelier-zue4
  Next: atelier issue close <issue-id> --reason "..."
Blockers: open - atelier-1p83, atelier-6aor, atelier-8o34, atelier-9pkx, atelier-a4sn, atelier-cany, atelier-diom, atelier-g18z, atelier-pvuz, atelier-pyre, atelier-sckq, atelier-trr2, atelier-u4nx, atelier-u6ax, atelier-v9id, atelier-w8e3, atelier-ys5p
  Next: close or unblock the blocker issues.
Evidence: attached
Validator durable_state_current: pass
Validator issue_sections_parseable: pass
Validator evidence_attached: pass
Validator no_open_blockers: fail - open blockers: atelier-1p83, atelier-6aor, atelier-8o34, atelier-9pkx, atelier-a4sn, atelier-cany, atelier-diom, atelier-g18z, atelier-pvuz, atelier-pyre, atelier-sckq, atelier-trr2, atelier-u4nx, atelier-u6ax, atelier-v9id, atelier-w8e3, atelier-ys5p
Validator no_blocking_lints: pass
Validator ignored_tests_reviewed: pass
Validator git_worktree_clean: fail - git worktree has 4 dirty entries:  M tests/cli_integration.rs;  M tests/smoke/cli_data.rs;  M tests/smoke/lifecycle.rs; ?? .atelier/issues/atelier-jqds.activity/20260612T220832693451Z.md

Validators
----------
2 closeout validator failure detected.
  fail  no_open_blockers - open blockers: atelier-1p83, atelier-6aor, atelier-8o34, atelier-9pkx, atelier-a4sn, atelier-cany, atelier-diom, atelier-g18z, atelier-pvuz, atelier-pyre, atelier-sckq, atelier-trr2, atelier-u4nx, atelier-u6ax, atelier-v9id, atelier-w8e3, atelier-ys5p
  fail  git_worktree_clean - git worktree has 4 dirty entries:  M tests/cli_integration.rs;  M tests/smoke/cli_data.rs;  M tests/smoke/lifecycle.rs; ?? .atelier/issues/atelier-jqds.activity/20260612T220832693451Z.md

Active Work
-----------
  atelier-jqds [active] branch=codex/orchestrate-at

Stderr summary (truncated):
   Compiling atelier-tracker v0.2.0 (/root/atelier)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.08s
     Running unittests src/lib.rs (target/debug/deps/atelier-0015c38aaafea46d)
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

