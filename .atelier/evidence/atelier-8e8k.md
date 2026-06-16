---
created_at: "2026-06-16T20:38:33.149316568+00:00"
id: "atelier-8e8k"
evidence_type: "validation"
captured_at: "2026-06-16T20:38:21.251660627+00:00"
command: "bash -lc 'set -euo pipefail\ncargo fmt -- --check\ncargo check -p atelier-cli\ncargo test -p atelier-records mission -- --nocapture\ncargo test -p atelier-cli mission_projection_worktree::test_mission_closeout_enforces_gates_and_reopen_skips_close_validators -- --nocapture\ncargo test -p atelier-cli mission_projection_worktree::test_mission_close_sees_issue_closeout_bookkeeping_committed_by_issue_close -- --nocapture\ncargo build -p atelier-cli\nif rg -n \"## Closeout Notes|closeout_notes|CloseoutNotes\" crates/atelier-records/src .atelier/missions; then\n  echo \"old mission terminal notes section remains\" >&2\n  exit 1\nfi\ntarget/debug/atelier lint atelier-rdyl\ntarget/debug/atelier export --check\ngit diff --check'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-rdyl"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-rdyl"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Mission terminal notes replace Closeout Notes"
updated_at: "2026-06-16T20:38:36.622521364+00:00"
---

## Summary

Mission terminal notes replace Closeout Notes

## Command

```console
bash -lc 'set -euo pipefail
cargo fmt -- --check
cargo check -p atelier-cli
cargo test -p atelier-records mission -- --nocapture
cargo test -p atelier-cli mission_projection_worktree::test_mission_closeout_enforces_gates_and_reopen_skips_close_validators -- --nocapture
cargo test -p atelier-cli mission_projection_worktree::test_mission_close_sees_issue_closeout_bookkeeping_committed_by_issue_close -- --nocapture
cargo build -p atelier-cli
if rg -n "## Closeout Notes|closeout_notes|CloseoutNotes" crates/atelier-records/src .atelier/missions; then
  echo "old mission terminal notes section remains" >&2
  exit 1
fi
target/debug/atelier lint atelier-rdyl
target/debug/atelier export --check
git diff --check'
```

Exit status: 0

## Stdout

Bytes: 1578
Truncated: no

```text

running 3 tests
test tests::mission_render_normalizes_legacy_evidence_attachments ... ok
test tests::mission_record_renders_and_parses_deterministically_without_data_blob ... ok
test tests::legacy_mission_data_record_loads_into_typed_sections_and_relationships ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 40 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test mission_projection_worktree::test_mission_closeout_enforces_gates_and_reopen_skips_close_validators ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 339 filtered out; finished in 1.52s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test mission_projection_worktree::test_mission_close_sees_issue_closeout_bookkeeping_committed_by_issue_close ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 339 filtered out; finished in 1.41s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s

Lint passed.
Canonical export is current
State: /root/atelier/.atelier
```

## Stderr

Bytes: 1384
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.84s
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running unittests src/lib.rs (target/debug/deps/atelier_records-6e5299045d686aef)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.74s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.49s
```

