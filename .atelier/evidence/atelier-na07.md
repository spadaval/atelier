---
created_at: "2026-11-04T23:30:22.963348424+00:00"
id: "atelier-na07"
evidence_type: "test"
commit: "d2f6fd57de0f0b7b4f12d64aa15bb34d22435f9a"
result: "pass"
captured_at: "2026-11-04T23:29:24.942033279+00:00"
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
title: "CLI test edits a record with Rejectd: and reason: continuation lines; check refuses both naming the label, and leaves a Note: line alone; core unit tests cover near-miss labels and prose lines; decision/debrief/brief/status/note CLI tests pass"
updated_at: "2026-11-04T23:30:22.963660777+00:00"
---

## Summary

CLI test edits a record with Rejectd: and reason: continuation lines; check refuses both naming the label, and leaves a Note: line alone; core unit tests cover near-miss labels and prose lines; decision/debrief/brief/status/note CLI tests pass

## Command

```console
sh -c 'cargo nextest run -p atelier-core && cargo nextest run -p atelier-cli -E "test(/decision|debrief|brief|status|note/)"'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 14689
SHA-256: 1287798173d9765ab09110053f3ddb28b30e3e1b369104c6014b5af320881191
Test summary: Summary [   4.467s] 99 tests run: 99 passed, 643 skipped
