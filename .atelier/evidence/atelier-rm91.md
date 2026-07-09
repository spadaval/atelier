---
created_at: "2026-07-06T21:36:34.622904189+00:00"
id: "atelier-rm91"
evidence_type: "test"
captured_at: "2026-07-06T21:36:33.947014420+00:00"
command: "cargo nextest run -p atelier-app -E 'test(later_parse_failure_rolls_back_every_earlier_incremental_repair) or test(orientation_can_degrade_but_decision_queries_reject_known_stale_rows) or test(one_issue_repair_is_bounded_and_has_one_transaction_owner) or test(more_than_32_candidates_fall_back_without_changing_cache) or test(full_and_incremental_add_change_delete_are_equivalent_for_all_domains) or test(cross_domain_graph_change_requests_one_safe_full_rebuild)'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-mska"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-mska"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Corrective proof for atelier-umyf: bounded incremental cache repair is all-or-nothing across up to 32 candidates; a valid earlier source plus invalid later source preserves every prior cache row and source-metadata field, degraded Orientation reopens true last-good state, Decision access still fails, successful bounded repair remains incremental, and more than 32 candidates still request full rebuild."
updated_at: "2026-07-06T21:36:38.909254526+00:00"
---

## Summary

Corrective proof for atelier-umyf: bounded incremental cache repair is all-or-nothing across up to 32 candidates; a valid earlier source plus invalid later source preserves every prior cache row and source-metadata field, degraded Orientation reopens true last-good state, Decision access still fails, successful bounded repair remains incremental, and more than 32 candidates still request full rebuild.

## Command

```console
cargo nextest run -p atelier-app -E 'test(later_parse_failure_rolls_back_every_earlier_incremental_repair) or test(orientation_can_degrade_but_decision_queries_reject_known_stale_rows) or test(one_issue_repair_is_bounded_and_has_one_transaction_owner) or test(more_than_32_candidates_fall_back_without_changing_cache) or test(full_and_incremental_add_change_delete_are_equivalent_for_all_domains) or test(cross_domain_graph_change_requests_one_safe_full_rebuild)'
```
Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 1080
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.10s
────────────
 Nextest run ID 8a92177c-a103-485e-9063-c8ee652503eb with nextest profile: default
    Starting 6 tests across 1 binary (87 tests skipped)
        PASS [   0.082s] (1/6) atelier-app rebuild::tests::more_than_32_candidates_fall_back_without_changing_cache
        PASS [   0.087s] (2/6) atelier-app rebuild::tests::one_issue_repair_is_bounded_and_has_one_transaction_owner
        PASS [   0.090s] (3/6) atelier-app cache_manager::tests::orientation_can_degrade_but_decision_queries_reject_known_stale_rows
        PASS [   0.092s] (4/6) atelier-app rebuild::tests::later_parse_failure_rolls_back_every_earlier_incremental_repair
        PASS [   0.199s] (5/6) atelier-app rebuild::tests::cross_domain_graph_change_requests_one_safe_full_rebuild
        PASS [   0.303s] (6/6) atelier-app rebuild::tests::full_and_incremental_add_change_delete_are_equivalent_for_all_domains
────────────
     Summary [   0.304s] 6 tests run: 6 passed, 87 skipped
```
