---
created_at: "2026-06-18T01:01:25.513235639+00:00"
id: "atelier-8ybt"
evidence_type: "validation"
captured_at: "2026-06-18T01:01:18.647688702+00:00"
command: "bash -lc 'set -euo pipefail\ncargo test -p atelier-cli persist_forge_pr_writes_owner_epic_field_and_child_inherits\ncargo test -p atelier-app effective_forge_pr_field_inherits_from_nearest_parent_epic\ncargo test -p atelier-app effective_forge_pr_field_rejects_child_duplicate\ncargo check -p atelier-cli\ncargo fmt -- --check\ntarget/debug/atelier lint atelier-vg25\ntarget/debug/atelier export --check\ngit diff --check'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-vg25"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-vg25"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "forge_pr owner-field persistence and child inheritance tests pass"
updated_at: "2026-06-18T01:01:29.614899514+00:00"
---

## Summary

forge_pr owner-field persistence and child inheritance tests pass

## Command

```console
bash -lc 'set -euo pipefail
cargo test -p atelier-cli persist_forge_pr_writes_owner_epic_field_and_child_inherits
cargo test -p atelier-app effective_forge_pr_field_inherits_from_nearest_parent_epic
cargo test -p atelier-app effective_forge_pr_field_rejects_child_duplicate
cargo check -p atelier-cli
cargo fmt -- --check
target/debug/atelier lint atelier-vg25
target/debug/atelier export --check
git diff --check'
```

Exit status: 0

## Stdout

Bytes: 1026
Truncated: no

```text

running 1 test
test commands::pr::tests::persist_forge_pr_writes_owner_epic_field_and_child_inherits ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 171 filtered out; finished in 0.14s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 354 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s


running 1 test
test workflow_policy::tests::effective_forge_pr_field_inherits_from_nearest_parent_epic ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 42 filtered out; finished in 0.07s


running 1 test
test workflow_policy::tests::effective_forge_pr_field_rejects_child_duplicate ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 42 filtered out; finished in 0.07s

Lint passed.
Canonical export is current
State: /root/atelier/.atelier
```

## Stderr

Bytes: 924
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.43s
     Running unittests src/lib.rs (target/debug/deps/atelier-50071cc57c116244)
     Running unittests src/main.rs (target/debug/deps/atelier-25cf0a98d3956199)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-1854c0ac2cd54c8d)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-d10014025669ce80)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running unittests src/lib.rs (target/debug/deps/atelier_app-91b312857bc4a702)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running unittests src/lib.rs (target/debug/deps/atelier_app-91b312857bc4a702)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.96s
```

