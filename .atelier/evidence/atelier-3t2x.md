---
created_at: "2026-11-09T15:34:19.081610812+00:00"
id: "atelier-3t2x"
evidence_type: "test"
commit: "e9b97752c7c263d9bcc2116a6d6947e4b412c89e"
result: "pass"
captured_at: "2026-11-09T15:34:18.561045007+00:00"
command: "cargo nextest run --build-jobs 4 -p atelier-app record_history"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-t7pb"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-t7pb"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Fixture of three items, one edited on a merged branch: done_when_edits makes one git log run (ReadContext::logs_run == 1), returns the expected edits, and each item's share of the walk equals its own git log --full-history. Seen failing (4 runs) with the walk done per item."
updated_at: "2026-11-09T15:34:19.210552895+00:00"
---

## Summary

Fixture of three items, one edited on a merged branch: done_when_edits makes one git log run (ReadContext::logs_run == 1), returns the expected edits, and each item's share of the walk equals its own git log --full-history. Seen failing (4 runs) with the walk done per item.

## Command

```console
cargo nextest run --build-jobs 4 -p atelier-app record_history
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 466
SHA-256: 24b0ae36a35952f2a306798474eb9ce9e2fff764c7792421b3a1d55ef09defe5
Test summary: Summary [   0.095s] 1 test run: 1 passed, 119 skipped
