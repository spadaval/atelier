---
created_at: "2026-06-18T01:17:28.954440879+00:00"
id: "atelier-w95t"
evidence_type: "validation"
captured_at: "2026-06-18T01:17:22.046470754+00:00"
command: "bash -lc 'set -euo pipefail\ntarget/debug/atelier issue transition atelier-hw9t --options | rg \"pr_merged|atelier pr open --issue atelier-hw9t\"\ncargo test -p atelier-cli linked_pr_merged_validator_reports_required_states\ncargo fmt -- --check\ntarget/debug/atelier lint atelier-495r\ntarget/debug/atelier export --check\ngit diff --check'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-495r"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-495r"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "PR validator failure guidance is actionable in transition output and tests"
updated_at: "2026-06-18T01:17:33.233796488+00:00"
---

## Summary

PR validator failure guidance is actionable in transition output and tests

## Command

```console
bash -lc 'set -euo pipefail
target/debug/atelier issue transition atelier-hw9t --options | rg "pr_merged|atelier pr open --issue atelier-hw9t"
cargo test -p atelier-cli linked_pr_merged_validator_reports_required_states
cargo fmt -- --check
target/debug/atelier lint atelier-495r
target/debug/atelier export --check
git diff --check'
```

Exit status: 0

## Stdout

Bytes: 809
Truncated: no

```text
  fail  pr_merged
      no linked forge_pr field; run `atelier pr open --issue atelier-hw9t`
  validator pr_merged failed: no linked forge_pr field; run `atelier pr open --issue atelier-hw9t`

running 1 test
test commands::workflow::tests::linked_pr_merged_validator_reports_required_states ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 174 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 354 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s

Lint passed.
Canonical export is current
State: /root/atelier/.atelier
```

## Stderr

Bytes: 473
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.05s
     Running unittests src/lib.rs (target/debug/deps/atelier-50071cc57c116244)
     Running unittests src/main.rs (target/debug/deps/atelier-25cf0a98d3956199)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-1854c0ac2cd54c8d)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-d10014025669ce80)
```

