---
created_at: "2026-07-09T17:30:56.996579116+00:00"
id: "atelier-g14g"
evidence_type: "test"
captured_at: "2026-07-09T17:30:56.584812662+00:00"
command: "bash -lc 'cargo test -p atelier-records mission_plan_review -- --nocapture && cargo run --quiet --manifest-path /root/tmp/atelier-wyxn-review-probe/Cargo.toml'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-wyxn"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-wyxn"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "bash -lc 'cargo test -p atelier-records mission_plan_review -- --nocapture && cargo run --quiet --manifest-path /root/tmp/atelier-wyxn-review-probe/Cargo.toml'"
updated_at: "2026-07-09T17:31:01.299776560+00:00"
---

## Summary

bash -lc 'cargo test -p atelier-records mission_plan_review -- --nocapture && cargo run --quiet --manifest-path /root/tmp/atelier-wyxn-review-probe/Cargo.toml'

## Command

```console
bash -lc 'cargo test -p atelier-records mission_plan_review -- --nocapture && cargo run --quiet --manifest-path /root/tmp/atelier-wyxn-review-probe/Cargo.toml'
```

Exit status: 0

## Stdout

Bytes: 821
Truncated: no

```text

running 5 tests
test mission_plan_review::tests::rejects_provenance_loss_non_independent_approval_and_unresolved_findings ... ok
test mission_plan_review::tests::rejects_malformed_typed_activity_metadata ... ok
test mission_plan_review::tests::every_material_edit_class_changes_revision_but_notes_and_order_do_not ... ok
test mission_plan_review::tests::material_change_stales_approval_and_requires_attribution_for_new_revision ... ok
test mission_plan_review::tests::typed_events_round_trip_and_project_fresh_independent_approval ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 50 filtered out; finished in 0.02s

direct mission blocked_by changes digest: false
draft mission with bogus legacy status rebuild-valid and fresh-grandfathered: true
whitespace-variant self-approval accepted fresh: true
```

## Stderr

Bytes: 160
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running unittests src/lib.rs (target/debug/deps/atelier_records-7f050eb419aa17f6)
```

