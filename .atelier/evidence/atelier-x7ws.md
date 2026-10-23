---
created_at: "2026-10-23T14:26:55.774211320+00:00"
id: "atelier-x7ws"
evidence_type: "test"
commit: "259a5bf2c31fa0c584e607011bd578f2c9f4d8a1"
result: "pass"
captured_at: "2026-10-23T14:26:53.286261583+00:00"
command: "cargo nextest run -j 4 -p atelier-cli -p atelier-records -E 'test(evidence_capture_keeps_the_failure_tail_and_summarizes_a_pass) | test(evidence_transcript_keeps_capture_and_codec_whitespace_boundaries) | test(passing_evidence_streams_keep_size_digest_and_count_not_text) | test(failing_evidence_streams_keep_head_tail_and_digest) | test(evidence_receipts_in_both_old_shapes_still_parse_and_render)'"
exit_status: "0"
agent_identity: "claude-implementer-dg3h"
target:
  kind: "issue"
  id: "atelier-auca"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-auca"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "New evidence records end with exactly one newline and pass git diff --check"
updated_at: "2026-10-23T14:26:55.775011831+00:00"
---

## Summary

New evidence records end with exactly one newline and pass git diff --check

## Command

```console
cargo nextest run -j 4 -p atelier-cli -p atelier-records -E 'test(evidence_capture_keeps_the_failure_tail_and_summarizes_a_pass) | test(evidence_transcript_keeps_capture_and_codec_whitespace_boundaries) | test(passing_evidence_streams_keep_size_digest_and_count_not_text) | test(failing_evidence_streams_keep_head_tail_and_digest) | test(evidence_receipts_in_both_old_shapes_still_parse_and_render)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 1044
SHA-256: 7d20687e985aa42d3d8d81989d50a424bd05dfc6d9bf4f5f7f14705bc1f5a3fb
Test summary: Summary [   0.115s] 5 tests run: 5 passed, 742 skipped
