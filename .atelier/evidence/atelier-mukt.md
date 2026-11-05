---
created_at: "2026-11-05T01:00:34.737677669+00:00"
id: "atelier-mukt"
evidence_type: "test"
commit: "1c148fccd01185cd55dd9ce5fbe706e44bcca8bb"
result: "pass"
captured_at: "2026-11-05T01:00:28.026497179+00:00"
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
updated_at: "2026-11-05T01:00:34.738078738+00:00"
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

Bytes: 15086
SHA-256: 5307bf08237aaf36c029fb92bc24b82acce99444d872c762b58d7dec344c26a7
Test summary: Summary [   3.734s] 101 tests run: 101 passed, 643 skipped
