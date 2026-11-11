---
created_at: "2026-11-11T06:40:44.773165271+00:00"
id: "atelier-24fe"
evidence_type: "test"
commit: "be7ae9a77caf74c53f4c8af2c18caa337b415281"
result: "pass"
captured_at: "2026-11-11T06:40:30.406808865+00:00"
command: "bash -c 'cd next && cargo nextest run -j 4 2>&1 | tail -3'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-e05n"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-e05n"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full next/ suite at f4f7ee50, including atelier-desk's thread tests against a stand-in MCP server"
updated_at: "2026-11-11T06:40:44.946939410+00:00"
---

## Summary

Full next/ suite at f4f7ee50, including atelier-desk's thread tests against a stand-in MCP server

## Command

```console
bash -c 'cd next && cargo nextest run -j 4 2>&1 | tail -3'
```

Exit status: 0

## Stdout

Bytes: 204
SHA-256: 4bc369859700c12742f90cf962b50086038b201f5070b10d6ac61a803c3cc875
Test summary: Summary [  13.922s] 273 tests run: 273 passed, 0 skipped

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
