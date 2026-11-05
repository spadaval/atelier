---
created_at: "2026-11-05T01:23:20.208618905+00:00"
id: "atelier-3bsh"
evidence_type: "test"
commit: "14293fddb0032f0c5d1158908881cb570404572b"
result: "pass"
captured_at: "2026-11-05T01:23:18.084320318+00:00"
command: "bash -c 'cargo build -j 4 2>&1 | tee /dev/stderr | grep -q \"^warning\" && exit 1; cargo nextest run -j 4 -p atelier-app criteria lint'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-pth2"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-pth2"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Workspace builds with no warnings after removing criteria_mission; criteria and lint unit tests pass"
updated_at: "2026-11-05T01:23:20.209291571+00:00"
---

## Summary

Workspace builds with no warnings after removing criteria_mission; criteria and lint unit tests pass

## Command

```console
bash -c 'cargo build -j 4 2>&1 | tee /dev/stderr | grep -q "^warning" && exit 1; cargo nextest run -j 4 -p atelier-app criteria lint'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 623
SHA-256: 3730fee14dfb1d0df5fb059fc1fd716423494439913893dcd023b7b403085970
Test summary: Summary [   0.005s] 1 test run: 1 passed, 106 skipped
