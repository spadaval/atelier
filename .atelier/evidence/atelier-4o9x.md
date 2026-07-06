---
created_at: "2026-06-25T01:08:39.394970927+00:00"
id: "atelier-4o9x"
evidence_type: "test"
captured_at: "2026-06-25T01:08:39.122986744+00:00"
command: "cargo test -p atelier-app issue_read -- --nocapture"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-fasv"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-fasv"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo test -p atelier-app issue_read -- --nocapture"
updated_at: "2026-06-25T01:08:42.580864123+00:00"
---

## Summary

cargo test -p atelier-app issue_read -- --nocapture

## Command

```console
cargo test -p atelier-app issue_read -- --nocapture
```

Exit status: 0

## Stdout

Bytes: 381
Truncated: no

```text

running 3 tests
test issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer ... ok
test issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants ... ok
test issue_read::tests::objective_scope_buckets_ready_blocked_and_done_children ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 84 filtered out; finished in 0.12s
```

## Stderr

Bytes: 156
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running unittests src/lib.rs (target/debug/deps/atelier_app-91b312857bc4a702)
```

