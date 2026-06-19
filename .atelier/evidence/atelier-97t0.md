---
created_at: "2026-06-18T01:02:50.615991677+00:00"
id: "atelier-97t0"
evidence_type: "validation"
captured_at: "2026-06-18T01:02:44.687278088+00:00"
command: "bash -lc 'set -euo pipefail\ncargo test -p atelier-cli render_comment_lines_filters_resolved_comments\ncargo test -p atelier-app lists_review_comments_and_surfaces_api_failures\ncargo check -p atelier-cli\ncargo fmt -- --check\ntarget/debug/atelier lint atelier-udny\ntarget/debug/atelier export --check\ngit diff --check'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-udny"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-udny"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Unresolved PR comment rendering and filtering tests pass"
updated_at: "2026-06-18T01:02:54.612320533+00:00"
---

## Summary

Unresolved PR comment rendering and filtering tests pass

## Command

```console
bash -lc 'set -euo pipefail
cargo test -p atelier-cli render_comment_lines_filters_resolved_comments
cargo test -p atelier-app lists_review_comments_and_surfaces_api_failures
cargo check -p atelier-cli
cargo fmt -- --check
target/debug/atelier lint atelier-udny
target/debug/atelier export --check
git diff --check'
```

Exit status: 0

## Stdout

Bytes: 796
Truncated: no

```text

running 1 test
test commands::pr::tests::render_comment_lines_filters_resolved_comments ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 172 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 354 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s


running 1 test
test forgejo::tests::lists_review_comments_and_surfaces_api_failures ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 42 filtered out; finished in 0.00s

Lint passed.
Canonical export is current
State: /root/atelier/.atelier
```

## Stderr

Bytes: 768
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.99s
     Running unittests src/lib.rs (target/debug/deps/atelier-50071cc57c116244)
     Running unittests src/main.rs (target/debug/deps/atelier-25cf0a98d3956199)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-1854c0ac2cd54c8d)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-d10014025669ce80)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running unittests src/lib.rs (target/debug/deps/atelier_app-91b312857bc4a702)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.88s
```

