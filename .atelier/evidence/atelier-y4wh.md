---
created_at: "2026-10-23T16:08:51.897583593+00:00"
id: "atelier-y4wh"
evidence_type: "test"
commit: "4c448459e684ba27e9133b4912bc96a360da8ce2"
result: "pass"
captured_at: "2026-10-23T16:08:49.345702519+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(evidence_list_by_item_marks_each_receipt_as_the_gates_judge_it) | test(test_evidence_list_bounds_default_output) | test(test_evidence_list_bounds_no_summary_command_capture) | test(test_evidence_list_elides_command_transcripts)'"
exit_status: "0"
agent_identity: "claude-implementer-l7nz"
target:
  kind: "issue"
  id: "atelier-u0ig"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-u0ig"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "evidence list <item> filters by item with short IDs, marks fresh/stale/superseded/blocks as the gates judge, ends with Proof verdict, honours --limit/--all and keeps filters in Next"
updated_at: "2026-10-23T16:08:51.898302104+00:00"
---

## Summary

evidence list <item> filters by item with short IDs, marks fresh/stale/superseded/blocks as the gates judge, ends with Proof verdict, honours --limit/--all and keeps filters in Next

## Command

```console
cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(evidence_list_by_item_marks_each_receipt_as_the_gates_judge_it) | test(test_evidence_list_bounds_default_output) | test(test_evidence_list_bounds_no_summary_command_capture) | test(test_evidence_list_elides_command_transcripts)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 965
SHA-256: e29041bf6847e37fcbb8a253f6ed80eb0b5388f6fe98abf95915d3544747c4ca
Test summary: Summary [   0.581s] 4 tests run: 4 passed, 704 skipped
