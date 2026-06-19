---
created_at: "2026-06-18T01:14:11.013426114+00:00"
id: "atelier-rnfw"
evidence_type: "validation"
captured_at: "2026-06-18T01:14:03.812513685+00:00"
command: "bash -lc 'set -euo pipefail\ncargo test -p atelier-cli linked_pr_merged\ncargo test -p atelier-workflow unknown_validator\ncargo check -p atelier-cli\ncargo fmt -- --check\ntarget/debug/atelier lint atelier-onie\ntarget/debug/atelier export --check\ngit diff --check'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-onie"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-onie"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "linked_pr_merged validator focused tests pass"
updated_at: "2026-06-18T01:14:15.075701647+00:00"
---

## Summary

linked_pr_merged validator focused tests pass

## Command

```console
bash -lc 'set -euo pipefail
cargo test -p atelier-cli linked_pr_merged
cargo test -p atelier-workflow unknown_validator
cargo check -p atelier-cli
cargo fmt -- --check
target/debug/atelier lint atelier-onie
target/debug/atelier export --check
git diff --check'
```

Exit status: 0

## Stdout

Bytes: 885
Truncated: no

```text

running 2 tests
test commands::workflow::tests::linked_pr_merged_validator_rejects_repo_and_branch_mismatch ... ok
test commands::workflow::tests::linked_pr_merged_validator_reports_required_states ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 173 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 354 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s


running 1 test
test tests::rejects_unknown_validator_reference ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

Lint passed.
Canonical export is current
State: /root/atelier/.atelier
```

## Stderr

Bytes: 1065
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.97s
     Running unittests src/lib.rs (target/debug/deps/atelier-50071cc57c116244)
     Running unittests src/main.rs (target/debug/deps/atelier-25cf0a98d3956199)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-1854c0ac2cd54c8d)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-d10014025669ce80)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running unittests src/lib.rs (target/debug/deps/atelier_workflow-1c1848f4dab0f01c)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Checking atelier-workflow v0.2.0 (/root/atelier/crates/atelier-workflow)
    Checking atelier-records v0.2.0 (/root/atelier/crates/atelier-records)
    Checking atelier-sqlite v0.2.0 (/root/atelier/crates/atelier-sqlite)
    Checking atelier-app v0.2.0 (/root/atelier/crates/atelier-app)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.28s
```

