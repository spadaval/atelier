---
created_at: "2026-11-09T16:44:01.828482378+00:00"
id: "atelier-t0c4"
evidence_type: "test"
commit: "b7c839c1b46c70f668c05c810bb0bdc3127944d8"
result: "pass"
captured_at: "2026-11-09T16:43:59.220028172+00:00"
command: "cargo nextest run -j 4 --build-jobs 4 -p atelier-cli --test serve read_only"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-zpmb"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-zpmb"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Read-only proof: with both fixture repositories' working trees and .git unwritable, every route answers the same status and body as when writable, and refs, the index and git status are unchanged"
updated_at: "2026-11-09T16:44:01.961500213+00:00"
---

## Summary

Read-only proof: with both fixture repositories' working trees and .git unwritable, every route answers the same status and body as when writable, and refs, the index and git status are unchanged

## Command

```console
cargo nextest run -j 4 --build-jobs 4 -p atelier-cli --test serve read_only
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 571
SHA-256: 25a0017b897027080682d31b240282ddcd0e34414b49fe5f5c4d7ed5c871030d
Test summary: Summary [   0.854s] 1 test run: 1 passed, 14 skipped
