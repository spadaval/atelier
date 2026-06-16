---
created_at: "2026-06-16T20:05:32.999996377+00:00"
id: "atelier-u06l"
evidence_type: "validation"
captured_at: "2026-06-16T20:05:03.771356228+00:00"
command: "bash -lc 'cargo fmt -- --check && cargo test -p atelier-cli setup_guidance::test_mission_status_help_exposes_verbose_terminal_detail -- --nocapture && cargo test -p atelier-cli setup_guidance::test_mission_help_exposes_close_with_reason -- --nocapture && cargo test -p atelier-cli mission_projection_worktree::test_mission_closeout_enforces_gates_and_reopen_skips_close_validators -- --nocapture && cargo test -p atelier-cli mission_projection_worktree::test_dirty_worktree_blocks_mission_closeout -- --nocapture && cargo test -p atelier-cli mission_projection_worktree::test_mission_status_cli_reports_control_state -- --nocapture && cargo test -p atelier-cli mission_projection_worktree::test_mission_status_names_concrete_closeout_blockers -- --nocapture && cargo test -p atelier-cli records_evidence::test_mission_status_reports_terminal_checks_and_explicit_approval -- --nocapture && cargo test -p atelier-cli records_evidence::test_mission_closeout_blocks_undeferred_obsolete_command_test -- --nocapture && cargo test -p atelier-cli records_evidence::test_mission_closeout_accepts_shell_mission_without_direct_mission_evidence -- --nocapture && target/debug/atelier mission --help | tee /tmp/mission-help.txt && target/debug/atelier mission status --help | tee /tmp/mission-status-help.txt && ! rg -n -- \"--closeout|mission audit|Mission Closeout Audit|Closeout Gates|Closeout:|mission closeout blocked|Closeout blockers|closeout audit|closeout gate|closeout validator|closeout validators|mission closeout judgment|active mission closeout\" crates/atelier-cli/src && ! rg -n -- \"--closeout|mission audit|Closeout Gates|Closeout:|Mission closeout blocked|mission closeout blocked|All required closeout gates|advanced closeout|closeout gate|closeout audit\" crates/atelier-cli/tests/cli_integration | rg -v \"assert!\\\\(!\" && target/debug/atelier lint atelier-jeyr && target/debug/atelier export --check && git diff --check'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-jeyr"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-jeyr"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Mission closeout vocabulary removed from live surfaces"
updated_at: "2026-06-16T20:05:36.653083344+00:00"
---

## Summary

Mission closeout vocabulary removed from live surfaces

## Command

```console
bash -lc 'cargo fmt -- --check && cargo test -p atelier-cli setup_guidance::test_mission_status_help_exposes_verbose_terminal_detail -- --nocapture && cargo test -p atelier-cli setup_guidance::test_mission_help_exposes_close_with_reason -- --nocapture && cargo test -p atelier-cli mission_projection_worktree::test_mission_closeout_enforces_gates_and_reopen_skips_close_validators -- --nocapture && cargo test -p atelier-cli mission_projection_worktree::test_dirty_worktree_blocks_mission_closeout -- --nocapture && cargo test -p atelier-cli mission_projection_worktree::test_mission_status_cli_reports_control_state -- --nocapture && cargo test -p atelier-cli mission_projection_worktree::test_mission_status_names_concrete_closeout_blockers -- --nocapture && cargo test -p atelier-cli records_evidence::test_mission_status_reports_terminal_checks_and_explicit_approval -- --nocapture && cargo test -p atelier-cli records_evidence::test_mission_closeout_blocks_undeferred_obsolete_command_test -- --nocapture && cargo test -p atelier-cli records_evidence::test_mission_closeout_accepts_shell_mission_without_direct_mission_evidence -- --nocapture && target/debug/atelier mission --help | tee /tmp/mission-help.txt && target/debug/atelier mission status --help | tee /tmp/mission-status-help.txt && ! rg -n -- "--closeout|mission audit|Mission Closeout Audit|Closeout Gates|Closeout:|mission closeout blocked|Closeout blockers|closeout audit|closeout gate|closeout validator|closeout validators|mission closeout judgment|active mission closeout" crates/atelier-cli/src && ! rg -n -- "--closeout|mission audit|Closeout Gates|Closeout:|Mission closeout blocked|mission closeout blocked|All required closeout gates|advanced closeout|closeout gate|closeout audit" crates/atelier-cli/tests/cli_integration | rg -v "assert!\\(!" && target/debug/atelier lint atelier-jeyr && target/debug/atelier export --check && git diff --check'
```

Exit status: 0

## Stdout

Bytes: 6770
Truncated: yes

```text

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test setup_guidance::test_mission_status_help_exposes_verbose_terminal_detail ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 339 filtered out; finished in 0.01s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test setup_guidance::test_mission_help_exposes_close_with_reason ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 339 filtered out; finished in 0.01s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test mission_projection_worktree::test_mission_closeout_enforces_gates_and_reopen_skips_close_validators ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 339 filtered out; finished in 1.36s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test mission_projection_worktree::test_dirty_worktree_blocks_mission_closeout ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 339 filtered out; finished in 1.18s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test mission_projection_worktree::test_mission_status_cli_reports_control_state ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 339 filtered out; finished in 3.07s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test mission_projection_worktree::test_mission_status_names_concrete_closeout_blockers ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 339 filtered out; finished in 0.65s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test records_evidence::test_mission_status_reports_terminal_checks_and_explicit_approval ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 339 filtered out; finished in 2.35s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test records_evidence::test_m
```

## Stderr

Bytes: 4257
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.76s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.72s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.71s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.72s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.71s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.68s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.70s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.65s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.70s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)
     Running
```

