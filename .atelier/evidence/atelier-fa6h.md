---
created_at: "2026-11-09T20:25:12.971221596+00:00"
id: "atelier-fa6h"
evidence_type: "test"
commit: "132006b728303cb90ff8815b499dd4388966856b"
result: "pass"
captured_at: "2026-11-09T20:24:00.208877092+00:00"
command: "cargo nextest run -j 4 -p atelier-cli -p atelier-app"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-ahpf"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ahpf"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "atelier-cli and atelier-app suites pass"
updated_at: "2026-11-09T20:25:13.106306003+00:00"
---

## Summary

atelier-cli and atelier-app suites pass

## Command

```console
cargo nextest run -j 4 -p atelier-cli -p atelier-app
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 129594
SHA-256: a9dfca45cb1b68e2a9aeab6642383f5c2a052855a52ad08c3377b1c89bd3ed1b
Test summary: Summary [  66.739s] 1012 tests run: 1012 passed, 1 skipped
