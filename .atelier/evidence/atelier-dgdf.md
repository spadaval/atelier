---
created_at: "2026-11-05T00:48:42.542526545+00:00"
id: "atelier-dgdf"
evidence_type: "test"
commit: "8d62bd82de53939fe81d88325b89f90d8424a995"
result: "pass"
captured_at: "2026-11-05T00:48:23.445936824+00:00"
command: "sh -c 'cargo nextest run -p atelier-core && cargo nextest run -p atelier-cli -E \"test(/decision|debrief|brief|status|note/)\"'"
exit_status: "0"
agent_identity: "atelier merge --sync"
target:
  kind: "issue"
  id: "atelier-juyb"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-juyb"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "After the atelier-nu4k fix and master sync: core unit tests refuse Rejectd/Reasons/reason/REJECTED/Raeson and accept Person/Region/Lesson/Season/Treason/Selected/Detected/Rejects; CLI misspelled-label test and decision/debrief/brief/status/note CLI tests pass (rerun after merging master)"
updated_at: "2026-11-05T00:48:42.543023561+00:00"
---

## Summary

After the atelier-nu4k fix and master sync: core unit tests refuse Rejectd/Reasons/reason/REJECTED/Raeson and accept Person/Region/Lesson/Season/Treason/Selected/Detected/Rejects; CLI misspelled-label test and decision/debrief/brief/status/note CLI tests pass (rerun after merging master)

## Command

```console
sh -c 'cargo nextest run -p atelier-core && cargo nextest run -p atelier-cli -E "test(/decision|debrief|brief|status|note/)"'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 15584
SHA-256: 0f0fd63e52c09b16b478f8d0f067b62985f4f3581902624fcee92e64b1b6026e
Test summary: Summary [   5.137s] 101 tests run: 101 passed, 643 skipped
