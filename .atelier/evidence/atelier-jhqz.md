---
created_at: "2026-10-23T14:26:52.906850154+00:00"
id: "atelier-jhqz"
evidence_type: "test"
commit: "259a5bf2c31fa0c584e607011bd578f2c9f4d8a1"
result: "pass"
captured_at: "2026-10-23T14:26:50.174113132+00:00"
command: "cargo nextest run -j 4 -p atelier-cli -p atelier-records -E 'test(passing_capture_keeps_digest_and_test_count_not_text) | test(evidence_capture_keeps_the_failure_tail_and_summarizes_a_pass) | test(passing_evidence_streams_keep_size_digest_and_count_not_text) | test(evidence_receipts_in_both_old_shapes_still_parse_and_render) | test(test_evidence_capture_records_command_metadata_and_attaches_targets)'"
exit_status: "0"
agent_identity: "claude-implementer-dg3h"
target:
  kind: "issue"
  id: "atelier-57wj"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-57wj"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Passing capture stores bytes, SHA-256 and test-count line per stream, no log; old receipts without output sections still parse"
updated_at: "2026-10-23T14:26:52.907639996+00:00"
---

## Summary

Passing capture stores bytes, SHA-256 and test-count line per stream, no log; old receipts without output sections still parse

## Command

```console
cargo nextest run -j 4 -p atelier-cli -p atelier-records -E 'test(passing_capture_keeps_digest_and_test_count_not_text) | test(evidence_capture_keeps_the_failure_tail_and_summarizes_a_pass) | test(passing_evidence_streams_keep_size_digest_and_count_not_text) | test(evidence_receipts_in_both_old_shapes_still_parse_and_render) | test(test_evidence_capture_records_command_metadata_and_attaches_targets)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 1088
SHA-256: 78423abe0590d3b40d6cb94ffec3362360d71bd3a85ce598e193e7440948d1ca
Test summary: Summary [   0.312s] 5 tests run: 5 passed, 742 skipped
