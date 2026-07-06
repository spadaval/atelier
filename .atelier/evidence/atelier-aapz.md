---
created_at: "2026-07-06T18:18:45.210634782+00:00"
id: "atelier-aapz"
evidence_type: "test"
captured_at: "2026-07-06T18:18:44.075057700+00:00"
command: "cargo nextest run -p atelier-core -p atelier-records -p atelier-app -E 'test(issue_review_has_typed_room_and_provider_forms) | test(issue_record_round_trips_review_link) | test(issue_record_rejects_invalid_typed_review_field) | test(room_merge_requires_current_approval_and_resolved_blocking_findings)'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-muzq"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-muzq"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo nextest run -p atelier-core -p atelier-records -p atelier-app -E 'test(issue_review_has_typed_room_and_provider_forms) | test(issue_record_round_trips_review_link) | test(issue_record_rejects_invalid_typed_review_field) | test(room_merge_requires_current_approval_and_resolved_blocking_findings)'"
updated_at: "2026-07-06T18:18:49.697053394+00:00"
---

## Summary

cargo nextest run -p atelier-core -p atelier-records -p atelier-app -E 'test(issue_review_has_typed_room_and_provider_forms) | test(issue_record_round_trips_review_link) | test(issue_record_rejects_invalid_typed_review_field) | test(room_merge_requires_current_approval_and_resolved_blocking_findings)'

## Command

```console
cargo nextest run -p atelier-core -p atelier-records -p atelier-app -E 'test(issue_review_has_typed_room_and_provider_forms) | test(issue_record_round_trips_review_link) | test(issue_record_rejects_invalid_typed_review_field) | test(room_merge_requires_current_approval_and_resolved_blocking_findings)'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 771
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.11s
────────────
 Nextest run ID 2bfcec7a-968e-4500-9d94-eb3a98e1dbc6 with nextest profile: default
    Starting 4 tests across 3 binaries (159 tests skipped)
        PASS [   0.009s] (1/4) atelier-records tests::issue_record_round_trips_review_link
        PASS [   0.009s] (2/4) atelier-core tests::issue_review_has_typed_room_and_provider_forms
        PASS [   0.010s] (3/4) atelier-records tests::issue_record_rejects_invalid_typed_review_field
        PASS [   0.727s] (4/4) atelier-app review_room::tests::room_merge_requires_current_approval_and_resolved_blocking_findings
────────────
     Summary [   0.727s] 4 tests run: 4 passed, 159 skipped
```
