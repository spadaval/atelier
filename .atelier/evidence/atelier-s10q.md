---
created_at: "2026-11-04T23:27:09.969074589+00:00"
id: "atelier-s10q"
evidence_type: "test"
commit: "60b995d1ec3dec7057308fff5ac613580da137d7"
result: "pass"
captured_at: "2026-11-04T23:26:56.109976337+00:00"
command: "sh -c 'cargo nextest run -p atelier-core && cargo nextest run -p atelier-app && cargo nextest run -p atelier-cli -E \"test(/decision|debrief|brief|status|note/)\"'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-w5hx"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-w5hx"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Focused tests: atelier-core all; atelier-app all; atelier-cli tests matching decision|debrief|brief|status|note (98), covering work decisions order, the brief cap at three records with the omitted-rows line, debrief without one-way section or sidebar, status brief"
updated_at: "2026-11-04T23:27:09.969368829+00:00"
---

## Summary

Focused tests: atelier-core all; atelier-app all; atelier-cli tests matching decision|debrief|brief|status|note (98), covering work decisions order, the brief cap at three records with the omitted-rows line, debrief without one-way section or sidebar, status brief

## Command

```console
sh -c 'cargo nextest run -p atelier-core && cargo nextest run -p atelier-app && cargo nextest run -p atelier-cli -E "test(/decision|debrief|brief|status|note/)"'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 28223
SHA-256: 440083ebc8797850e54d87c79609fd123cebd01bd5f35d8bf034e2f0196f560a
Test summary: Summary [   3.914s] 98 tests run: 98 passed, 643 skipped
