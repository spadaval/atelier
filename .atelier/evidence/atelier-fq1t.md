---
created_at: "2026-06-16T18:37:17.571349392+00:00"
id: "atelier-fq1t"
evidence_type: "test"
captured_at: "2026-06-16T18:37:14.542861744+00:00"
command: "cargo test -p atelier-cli work_order --lib -- --nocapture"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-qh52"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-qh52"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Shared work-order unit tests still pass"
updated_at: "2026-06-16T18:37:24.853838509+00:00"
---

## Summary

Shared work-order unit tests still pass

## Command

```console
cargo test -p atelier-cli work_order --lib -- --nocapture
```

Exit status: 0

## Stdout

Bytes: 697
Truncated: no

```text

running 7 tests
test commands::work_order::tests::done_blockers_are_ignored_by_open_blocker_input ... ok
test commands::work_order::tests::hidden_blockers_do_not_insert_phantom_rows ... ok
test commands::work_order::tests::orders_diamond_dependencies_before_join ... ok
test commands::work_order::tests::cycles_are_deterministic_and_preserve_all_rows ... ok
test commands::work_order::tests::row_state_labels_cover_work_view_vocabulary ... ok
test commands::work_order::tests::ties_use_priority_update_time_and_id ... ok
test commands::work_order::tests::orders_simple_visible_blocker_chain ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 163 filtered out; finished in 0.00s
```

## Stderr

Bytes: 432
Truncated: no

```text
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on artifact directory
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.97s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
```
