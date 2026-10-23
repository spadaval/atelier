---
created_at: "2026-10-23T16:26:30.930369694+00:00"
id: "atelier-4ab7"
evidence_type: "test"
commit: "2148b5a5b39468acecb1a3eae1cb00d4eb053fbe"
result: "pass"
captured_at: "2026-10-23T16:26:28.192213598+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(evidence_views_judge_a_merged_claim_as_land_and_doctor_do) | test(evidence_list_verdict_agrees_with_the_close_gate) | test(evidence_list_by_item_marks_each_receipt_as_the_gates_judge_it) | test(evidence_show_is_brief_and_full_prints_the_stored_record) | test(evidence_record_confirms_in_one_line_and_ends_in_its_verdict)'"
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
title: "Review fixes 2isj/g2pl: a merged done claim's pass reads held at close after later base code, agreeing with land and doctor; a pass over uncommitted code says it does not prove its item; list verdict agrees with the close gate"
updated_at: "2026-10-23T16:27:19.437519096+00:00"
---

## Summary

Review fixes 2isj/g2pl: a merged done claim's pass reads held at close after later base code, agreeing with land and doctor; a pass over uncommitted code says it does not prove its item; list verdict agrees with the close gate

## Command

```console
cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(evidence_views_judge_a_merged_claim_as_land_and_doctor_do) | test(evidence_list_verdict_agrees_with_the_close_gate) | test(evidence_list_by_item_marks_each_receipt_as_the_gates_judge_it) | test(evidence_show_is_brief_and_full_prints_the_stored_record) | test(evidence_record_confirms_in_one_line_and_ends_in_its_verdict)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 1102
SHA-256: 4c2e56660dd8e75ceec1e50c27bb15183bf825f25f024325d76f8593d0700ac7
Test summary: Summary [   0.647s] 5 tests run: 5 passed, 715 skipped
