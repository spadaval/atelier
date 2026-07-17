---
created_at: "2026-07-17T00:22:10.479738332+00:00"
id: "atelier-d6o6"
evidence_type: "validation"
captured_at: "2026-07-17T00:22:03.743347627+00:00"
command: "bash -lc 'git diff --check 65759498..HEAD && target/debug/atelier check atelier-t876 && target/debug/atelier check && test \"$(git rev-parse HEAD)\" = f205bb1f53214e4388c011cd5c97d8e14627e5db'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-t876"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-t876"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "bash -lc 'git diff --check 65759498..HEAD && target/debug/atelier check atelier-t876 && target/debug/atelier check && test \"$(git rev-parse HEAD)\" = f205bb1f53214e4388c011cd5c97d8e14627e5db'"
updated_at: "2026-07-17T00:22:10.481603642+00:00"
---

## Summary

bash -lc 'git diff --check 65759498..HEAD && target/debug/atelier check atelier-t876 && target/debug/atelier check && test "$(git rev-parse HEAD)" = f205bb1f53214e4388c011cd5c97d8e14627e5db'

## Command

```console
bash -lc 'git diff --check 65759498..HEAD && target/debug/atelier check atelier-t876 && target/debug/atelier check && test "$(git rev-parse HEAD)" = f205bb1f53214e4388c011cd5c97d8e14627e5db'
```

Exit status: 0

## Stdout

Bytes: 26
Truncated: no

```text
Lint passed.
Lint passed.
```

## Stderr

Bytes: 0
Truncated: no

```text
```
