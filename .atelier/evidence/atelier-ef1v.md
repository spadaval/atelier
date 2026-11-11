---
created_at: "2026-11-11T07:03:07.286074101+00:00"
id: "atelier-ef1v"
evidence_type: "test"
commit: "6a1a37512e371e0ebcff9281d5a0046ec74354a5"
result: "pass"
captured_at: "2026-11-11T07:02:52.881438542+00:00"
command: "bash -c 'cd next && cargo nextest run -j 4 2>&1 | tail -3'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-h3hl"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-h3hl"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Every test of the next/ workspace, including atelier-desk's acts, sessions, desk, threads and types tests"
updated_at: "2026-11-11T07:03:07.451759867+00:00"
---

## Summary

Every test of the next/ workspace, including atelier-desk's acts, sessions, desk, threads and types tests

## Command

```console
bash -c 'cd next && cargo nextest run -j 4 2>&1 | tail -3'
```

Exit status: 0

## Stdout

Bytes: 214
SHA-256: e162bd7ab5ab68ca7d9978030a3a0561f30007efa5f9e97b0e817ba3eaaa4e2d
Test summary: Summary [  13.941s] 282 tests run: 282 passed, 0 skipped

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
