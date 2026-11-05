---
created_at: "2026-11-05T00:10:16.416996685+00:00"
id: "atelier-p80h"
evidence_type: "test"
commit: "4708c70de41401397f3963cdf500794e6dc76574"
result: "pass"
captured_at: "2026-11-05T00:10:06.892695984+00:00"
command: "sh -c 'cargo nextest run -p atelier-core && cargo nextest run -p atelier-cli -E \"test(/decision|debrief|brief|status|note/)\"'"
exit_status: "0"
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
title: "After the atelier-nu4k fix and master sync: core unit tests refuse Rejectd/Reasons/reason/REJECTED/Raeson and accept Person/Region/Lesson/Season/Treason/Selected/Detected/Rejects; CLI misspelled-label test and decision/debrief/brief/status/note CLI tests pass"
updated_at: "2026-11-05T00:10:16.417346868+00:00"
---

## Summary

After the atelier-nu4k fix and master sync: core unit tests refuse Rejectd/Reasons/reason/REJECTED/Raeson and accept Person/Region/Lesson/Season/Treason/Selected/Detected/Rejects; CLI misspelled-label test and decision/debrief/brief/status/note CLI tests pass

## Command

```console
sh -c 'cargo nextest run -p atelier-core && cargo nextest run -p atelier-cli -E "test(/decision|debrief|brief|status|note/)"'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 15029
SHA-256: 6a483e12ab35a17072985d9d3a12548416b284a80cb45ccbd07e69580f2235ff
Test summary: Summary [   3.102s] 99 tests run: 99 passed, 643 skipped
