---
created_at: "2026-06-16T18:12:06.287354920+00:00"
id: "atelier-hu3v"
evidence_type: "test"
captured_at: "2026-06-16T18:12:04.032948146+00:00"
command: "cargo test -p atelier-cli test_issue_list_ -- --nocapture"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-3s9y"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-3s9y"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Issue-list tests cover visible blocker order, readable state labels, ready filtering, blocked-list drill-down cues, and quiet ID order."
updated_at: "2026-06-16T18:12:09.952247204+00:00"
---

## Summary

Issue-list tests cover visible blocker order, readable state labels, ready filtering, blocked-list drill-down cues, and quiet ID order.

## Command

```console
cargo test -p atelier-cli test_issue_list_ -- --nocapture
```

Exit status: 0

## Stdout

Bytes: 992
Truncated: no

```text

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 171 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 7 tests
test issues::test_issue_list_ready_rejects_closed_status ... ok
test issues::test_issue_list_orders_visible_blockers_before_blocked_rows ... ok
test issues::test_issue_list_blocked_replaces_blocked_helper ... ok
test issues::test_issue_list_ready_treats_internal_epic_blockers_as_ready ... ok
test issues::test_issue_list_marks_external_epic_blockers_by_id ... ok
test issues::test_issue_list_ready_marks_blocked_parent_headers_as_context ... ok
test issues::test_issue_list_ready_excludes_blocked_and_quiet_matches_human_order ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 328 filtered out; finished in 0.47s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s
```

## Stderr

Bytes: 473
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.71s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)
```
