---
created_at: "2026-11-05T00:32:46.805870283+00:00"
id: "atelier-joa0"
evidence_type: "test"
commit: "4d2be5eaf35ae2ed2a3280e379ace3f38bc7291f"
result: "pass"
captured_at: "2026-11-05T00:32:41.067183223+00:00"
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
updated_at: "2026-11-05T00:32:46.806147961+00:00"
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

Bytes: 14629
SHA-256: bce3205524238cb5c96d5cb5689733e982b1524f43ea324d3357d53261d05ac4
Test summary: Summary [   3.209s] 99 tests run: 99 passed, 643 skipped
