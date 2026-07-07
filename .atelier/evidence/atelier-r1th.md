---
created_at: "2026-07-06T21:38:43.071990979+00:00"
id: "atelier-r1th"
evidence_type: "test"
captured_at: "2026-07-06T21:38:42.392110596+00:00"
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
title: "Superseding exact-scenario proof for atelier-umyf and atelier-rm91: with a valid earlier issue source and invalid later review-room YAML, the <=32-candidate incremental batch rolls back every cache row and complete source-metadata record; failed full rebuild leaves degraded Orientation on the exact last-good cache while Decision access fails. Successful bounded repair, graph fallback, full/incremental equivalence, and the >32 full-rebuild threshold remain green."
updated_at: "2026-07-06T21:38:46.765988375+00:00"
---

## Summary

Superseding exact-scenario proof for atelier-umyf and atelier-rm91: with a valid earlier issue source and invalid later review-room YAML, the <=32-candidate incremental batch rolls back every cache row and complete source-metadata record; failed full rebuild leaves degraded Orientation on the exact last-good cache while Decision access fails. Successful bounded repair, graph fallback, full/incremental equivalence, and the >32 full-rebuild threshold remain green.

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
 Nextest run ID ba9717ff-5929-433f-96be-bb6a40fd8880 with nextest profile: default
    Starting 6 tests across 1 binary (87 tests skipped)
        PASS [   0.095s] (1/6) atelier-app rebuild::tests::later_parse_failure_rolls_back_every_earlier_incremental_repair
        PASS [   0.095s] (2/6) atelier-app rebuild::tests::more_than_32_candidates_fall_back_without_changing_cache
        PASS [   0.098s] (3/6) atelier-app cache_manager::tests::orientation_can_degrade_but_decision_queries_reject_known_stale_rows
        PASS [   0.105s] (4/6) atelier-app rebuild::tests::one_issue_repair_is_bounded_and_has_one_transaction_owner
        PASS [   0.164s] (5/6) atelier-app rebuild::tests::cross_domain_graph_change_requests_one_safe_full_rebuild
        PASS [   0.284s] (6/6) atelier-app rebuild::tests::full_and_incremental_add_change_delete_are_equivalent_for_all_domains
────────────
     Summary [   0.285s] 6 tests run: 6 passed, 87 skipped
```
