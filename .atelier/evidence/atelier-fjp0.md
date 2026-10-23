---
created_at: "2026-10-23T16:37:57.117082107+00:00"
id: "atelier-fjp0"
evidence_type: "test"
commit: "09880419b8f68268d56b466926c3d8155460350e"
result: "pass"
captured_at: "2026-10-23T16:37:50.709500714+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(evidence_views_judge_an_unmerged_claim_at_head_as_land_does) | test(evidence_views_judge_a_merged_claim_as_land_and_doctor_do) | test(retained) | test(evidence_list_verdict_agrees_with_the_close_gate) | test(evidence_list_by_item_marks_each_receipt_as_the_gates_judge_it)'"
exit_status: "0"
agent_identity: "claude-implementer-l7nz"
target:
  kind: "issue"
  id: "atelier-l7nz"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-3ekh"
    role: "validates"
  - kind: "issue"
    id: "atelier-b05y"
    role: "validates"
  - kind: "issue"
    id: "atelier-l7nz"
    role: "validates"
  - kind: "issue"
    id: "atelier-o8ni"
    role: "validates"
  - kind: "issue"
    id: "atelier-u0ig"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Review fix yr1n: an unmerged done claim seen from a sibling branch that merged it, or a detached checkout, is judged at HEAD as land does; merged claims still read held at close; land's retained-claim tests still pass"
updated_at: "2026-10-23T16:38:48.634711377+00:00"
---

## Summary

Review fix yr1n: an unmerged done claim seen from a sibling branch that merged it, or a detached checkout, is judged at HEAD as land does; merged claims still read held at close; land's retained-claim tests still pass

## Command

```console
cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(evidence_views_judge_an_unmerged_claim_at_head_as_land_does) | test(evidence_views_judge_a_merged_claim_as_land_and_doctor_do) | test(retained) | test(evidence_list_verdict_agrees_with_the_close_gate) | test(evidence_list_by_item_marks_each_receipt_as_the_gates_judge_it)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 1464
SHA-256: 3028d8ef92d9383d555edb7feed83f793509771ed8d00230d236f969d52e0308
Test summary: Summary [   2.453s] 8 tests run: 8 passed, 713 skipped
