---
created_at: "2026-10-23T14:34:32.810051460+00:00"
id: "atelier-dpm6"
evidence_type: "test"
commit: "4cdaf39f9ed01ecdc89fabff4e53a320cc03a372"
result: "pass"
captured_at: "2026-10-23T14:34:30.203009118+00:00"
command: "cargo nextest run -j 4 -p atelier-cli -p atelier-records -E 'test(passing_capture_keeps_digest_and_test_count_not_text) | test(evidence_capture_keeps_the_failure_tail_and_summarizes_a_pass) | test(passing_evidence_streams_keep_size_digest_and_count_not_text) | test(evidence_show_keeps_hostile_and_blank_failing_output_in_its_stream)'"
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
title: "After review fixes, passing captures still store bytes, SHA-256 and test-count line with no log, and show says so only for passes"
updated_at: "2026-10-23T14:34:32.810839339+00:00"
---

## Summary

After review fixes, passing captures still store bytes, SHA-256 and test-count line with no log, and show says so only for passes

## Command

```console
cargo nextest run -j 4 -p atelier-cli -p atelier-records -E 'test(passing_capture_keeps_digest_and_test_count_not_text) | test(evidence_capture_keeps_the_failure_tail_and_summarizes_a_pass) | test(passing_evidence_streams_keep_size_digest_and_count_not_text) | test(evidence_show_keeps_hostile_and_blank_failing_output_in_its_stream)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 973
SHA-256: 4643f4cfaf6206e6ce7d98105b1abea984c62732a7f5c6eaef576eea98c68656
Test summary: Summary [   0.130s] 4 tests run: 4 passed, 745 skipped
