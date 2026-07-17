---
created_at: "2026-07-09T17:23:34.644709570+00:00"
id: "atelier-45sk"
evidence_type: "test"
captured_at: "2026-07-09T17:23:34.011130840+00:00"
command: "bash -lc 'cargo test -p atelier-records mission_plan_review -- --nocapture && cargo test -p atelier-app rebuild::tests::rebuild_ -- --nocapture'"
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
title: "bash -lc 'cargo test -p atelier-records mission_plan_review -- --nocapture && cargo test -p atelier-app rebuild::tests::rebuild_ -- --nocapture'"
updated_at: "2026-07-09T17:23:39.101028135+00:00"
---

## Summary

bash -lc 'cargo test -p atelier-records mission_plan_review -- --nocapture && cargo test -p atelier-app rebuild::tests::rebuild_ -- --nocapture'

## Command

```console
bash -lc 'cargo test -p atelier-records mission_plan_review -- --nocapture && cargo test -p atelier-app rebuild::tests::rebuild_ -- --nocapture'
```
Exit status: 0

## Stdout

Bytes: 937
Truncated: no

```text

running 5 tests
test mission_plan_review::tests::rejects_provenance_loss_non_independent_approval_and_unresolved_findings ... ok
test mission_plan_review::tests::rejects_malformed_typed_activity_metadata ... ok
test mission_plan_review::tests::every_material_edit_class_changes_revision_but_notes_and_order_do_not ... ok
test mission_plan_review::tests::material_change_stales_approval_and_requires_attribution_for_new_revision ... ok
test mission_plan_review::tests::typed_events_round_trip_and_project_fresh_independent_approval ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 50 filtered out; finished in 0.02s


running 2 tests
test rebuild::tests::rebuild_rejects_plan_approval_when_provenance_was_lost ... ok
test rebuild::tests::rebuild_preserves_complete_mission_plan_review_projection_deterministically ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 94 filtered out; finished in 0.16s
```

## Stderr

Bytes: 316
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running unittests src/lib.rs (target/debug/deps/atelier_records-7f050eb419aa17f6)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running unittests src/lib.rs (target/debug/deps/atelier_app-08ce193fbeb04291)
```
