---
created_at: "2026-10-23T14:26:49.795818123+00:00"
id: "atelier-4dwk"
evidence_type: "test"
commit: "259a5bf2c31fa0c584e607011bd578f2c9f4d8a1"
result: "pass"
captured_at: "2026-10-23T14:26:46.726084661+00:00"
command: "cargo nextest run -j 4 -p atelier-cli -p atelier-records -E 'test(failing_capture_keeps) | test(evidence_capture_keeps_the_failure_tail_and_summarizes_a_pass) | test(failing_evidence_streams_keep_head_tail_and_digest) | test(evidence_receipts_in_both_old_shapes_still_parse_and_render)'"
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
title: "Failing capture keeps head and tail with an omitted-bytes marker on UTF-8 boundaries; a final FAIL line survives; old Truncated: yes receipts still parse"
updated_at: "2026-10-23T14:26:49.796594530+00:00"
---

## Summary

Failing capture keeps head and tail with an omitted-bytes marker on UTF-8 boundaries; a final FAIL line survives; old Truncated: yes receipts still parse

## Command

```console
cargo nextest run -j 4 -p atelier-cli -p atelier-records -E 'test(failing_capture_keeps) | test(evidence_capture_keeps_the_failure_tail_and_summarizes_a_pass) | test(failing_evidence_streams_keep_head_tail_and_digest) | test(evidence_receipts_in_both_old_shapes_still_parse_and_render)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 1054
SHA-256: 0074fe65725e899e2380bc8d478eb4995884257e470e3cf702b1ae7569485bcb
Test summary: Summary [   0.114s] 5 tests run: 5 passed, 742 skipped
