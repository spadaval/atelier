---
created_at: "2026-07-06T18:18:27.126760700+00:00"
id: "atelier-919w"
evidence_type: "test"
captured_at: "2026-07-06T18:18:25.083482589+00:00"
command: "cargo nextest run -p atelier-app -E 'test(concrete_evidence_and_review_services_round_trip_domain_types) | test(room_merge_requires_current_approval_and_resolved_blocking_findings) | test(baseline_default_checks_do_not_report_projection_freshness)'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-sdqy"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-sdqy"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo nextest run -p atelier-app -E 'test(concrete_evidence_and_review_services_round_trip_domain_types) | test(room_merge_requires_current_approval_and_resolved_blocking_findings) | test(baseline_default_checks_do_not_report_projection_freshness)'"
updated_at: "2026-07-06T18:18:31.445983904+00:00"
---

## Summary

cargo nextest run -p atelier-app -E 'test(concrete_evidence_and_review_services_round_trip_domain_types) | test(room_merge_requires_current_approval_and_resolved_blocking_findings) | test(baseline_default_checks_do_not_report_projection_freshness)'

## Command

```console
cargo nextest run -p atelier-app -E 'test(concrete_evidence_and_review_services_round_trip_domain_types) | test(room_merge_requires_current_approval_and_resolved_blocking_findings) | test(baseline_default_checks_do_not_report_projection_freshness)'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 731
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.13s
────────────
 Nextest run ID 885c5b25-3964-46c1-abb6-a7facac52525 with nextest profile: default
    Starting 3 tests across 1 binary (103 tests skipped)
        PASS [   0.012s] (1/3) atelier-app use_cases::tests::concrete_evidence_and_review_services_round_trip_domain_types
        PASS [   0.814s] (2/3) atelier-app review_room::tests::room_merge_requires_current_approval_and_resolved_blocking_findings
        PASS [   1.633s] (3/3) atelier-app workflow_validation::tests::baseline_default_checks_do_not_report_projection_freshness
────────────
     Summary [   1.634s] 3 tests run: 3 passed, 103 skipped
```

