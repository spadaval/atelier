---
created_at: "2026-11-04T23:20:10.346584304+00:00"
id: "atelier-omnw"
evidence_type: "test"
commit: "e223e881e7d063d2ce289a0c5693eaf14bcd5465"
result: "pass"
captured_at: "2026-11-04T23:20:07.042161052+00:00"
command: "cargo nextest run -p atelier-cli --test cli_integration decisions debrief"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-s5x4"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-s5x4"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "CLI test removes workflow.yaml after deciding two records; work decisions shows the newer current and the older superseded, -q lists the newer; decisions and debrief tests pass"
updated_at: "2026-11-04T23:20:10.346880497+00:00"
---

## Summary

CLI test removes workflow.yaml after deciding two records; work decisions shows the newer current and the older superseded, -q lists the newer; decisions and debrief tests pass

## Command

```console
cargo nextest run -p atelier-cli --test cli_integration decisions debrief
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 4165
SHA-256: f8000ea7ce3689942f840f6bd0198dc766548df7b308a2adb38a506bdc0b2e95
Test summary: Summary [   1.569s] 29 tests run: 29 passed, 363 skipped
