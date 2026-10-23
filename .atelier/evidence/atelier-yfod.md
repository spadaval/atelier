---
created_at: "2026-10-23T14:34:29.826538871+00:00"
id: "atelier-yfod"
evidence_type: "test"
commit: "4cdaf39f9ed01ecdc89fabff4e53a320cc03a372"
result: "pass"
captured_at: "2026-10-23T14:33:57.220370360+00:00"
command: "cargo nextest run -j 4 -p atelier-cli -p atelier-records -E 'test(captured_headings_fields_and_fences_stay_inside_their_stream) | test(evidence_show_keeps_hostile_and_blank_failing_output_in_its_stream) | test(evidence_receipts_in_both_old_shapes_still_parse_and_render) | test(failing_evidence_streams_keep_head_tail_and_digest)'"
exit_status: "0"
agent_identity: "claude-implementer-dg3h"
target:
  kind: "issue"
  id: "atelier-bphy"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-bphy"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Review fixes yif8/7yla: captured text imitating a heading, field or fence stays in its stream; a blank failing stream shows (whitespace only); old receipts still parse"
updated_at: "2026-10-23T14:34:29.827352487+00:00"
---

## Summary

Review fixes yif8/7yla: captured text imitating a heading, field or fence stays in its stream; a blank failing stream shows (whitespace only); old receipts still parse

## Command

```console
cargo nextest run -j 4 -p atelier-cli -p atelier-records -E 'test(captured_headings_fields_and_fences_stay_inside_their_stream) | test(evidence_show_keeps_hostile_and_blank_failing_output_in_its_stream) | test(evidence_receipts_in_both_old_shapes_still_parse_and_render) | test(failing_evidence_streams_keep_head_tail_and_digest)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 989
SHA-256: a7b9bb8368a6068071cf89028d99473ee617a427dd32802f38fb440180a05432
Test summary: Summary [   0.116s] 4 tests run: 4 passed, 745 skipped
