---
created_at: "2026-07-09T17:24:16.224609651+00:00"
id: "atelier-x1mc"
evidence_type: "validation"
captured_at: "2026-07-09T17:24:14.788744496+00:00"
command: "bash -lc 'cargo fmt -- --check && git diff --check && atelier check atelier-wyxn'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-wyxn"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-wyxn"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "bash -lc 'cargo fmt -- --check && git diff --check && atelier check atelier-wyxn'"
updated_at: "2026-07-09T17:24:20.507344709+00:00"
---

## Summary

bash -lc 'cargo fmt -- --check && git diff --check && atelier check atelier-wyxn'

## Command

```console
bash -lc 'cargo fmt -- --check && git diff --check && atelier check atelier-wyxn'
```

Exit status: 0

## Stdout

Bytes: 13
Truncated: no

```text
Lint passed.
```

## Stderr

Bytes: 0
Truncated: no

```text
```
