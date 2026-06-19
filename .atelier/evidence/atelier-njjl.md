---
created_at: "2026-06-18T01:00:42.179733963+00:00"
id: "atelier-njjl"
evidence_type: "validation"
captured_at: "2026-06-18T01:00:35.303667669+00:00"
command: "bash -lc 'set -euo pipefail\ntarget/debug/atelier pr --help\ncargo test -p atelier-cli commands::pr\ncargo check -p atelier-cli\ncargo fmt -- --check\ntarget/debug/atelier lint atelier-yrwm\ntarget/debug/atelier export --check\ngit diff --check'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-yrwm"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-yrwm"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "atelier pr command surface and PR command tests pass"
updated_at: "2026-06-18T01:00:46.127780070+00:00"
---

## Summary

atelier pr command surface and PR command tests pass

## Command

```console
bash -lc 'set -euo pipefail
target/debug/atelier pr --help
cargo test -p atelier-cli commands::pr
cargo check -p atelier-cli
cargo fmt -- --check
target/debug/atelier lint atelier-yrwm
target/debug/atelier export --check
git diff --check'
```

Exit status: 0

## Stdout

Bytes: 1502
Truncated: no

```text
Forgejo pull request review artifacts

Usage: atelier pr [OPTIONS] <COMMAND>

Commands:
  open      Open or confirm the active Forgejo pull request for an issue owner
  status    Show concise linked PR status
  show      Show linked PR details
  comments  List live PR review comments
  comment   Add a Forgejo PR comment
  review    Submit a Forgejo PR review
  help      Print this message or the help of the given subcommand(s)

Options:
  -q, --quiet                    Quiet mode: only output essential data (IDs, counts)
      --log-level <LOG_LEVEL>    Log level for diagnostic output (error, warn, info, debug, trace) [env: ATELIER_LOG=] [default: warn]
      --log-format <LOG_FORMAT>  Log format (text, json) [env: ATELIER_LOG_FORMAT=] [default: text]
  -h, --help                     Print help

running 2 tests
test commands::pr::tests::parse_review_event_rejects_unknown_values ... ok
test commands::pr::tests::persist_forge_pr_writes_owner_epic_field_and_child_inherits ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.15s


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

Bytes: 612
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.65s
     Running unittests src/lib.rs (target/debug/deps/atelier-50071cc57c116244)
     Running unittests src/main.rs (target/debug/deps/atelier-25cf0a98d3956199)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-1854c0ac2cd54c8d)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-d10014025669ce80)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.94s
```

