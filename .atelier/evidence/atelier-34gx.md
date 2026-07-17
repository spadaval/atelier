---
created_at: "2026-07-17T00:21:55.655553026+00:00"
id: "atelier-34gx"
evidence_type: "validation"
captured_at: "2026-07-17T00:21:48.697539728+00:00"
command: "bash -lc 'git diff --check 65759498..HEAD && target/debug/atelier check atelier-t876 && target/debug/atelier check && test \"$(git rev-parse HEAD)\" = f205bb1fd35f25c3687173c3ad16f630bf6bdbb4'"
exit_status: "1"
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
title: "bash -lc 'git diff --check 65759498..HEAD && target/debug/atelier check atelier-t876 && target/debug/atelier check && test \"$(git rev-parse HEAD)\" = f205bb1fd35f25c3687173c3ad16f630bf6bdbb4'"
updated_at: "2026-07-17T00:21:55.658656986+00:00"
---

## Summary

bash -lc 'git diff --check 65759498..HEAD && target/debug/atelier check atelier-t876 && target/debug/atelier check && test "$(git rev-parse HEAD)" = f205bb1fd35f25c3687173c3ad16f630bf6bdbb4'

## Command

```console
bash -lc 'git diff --check 65759498..HEAD && target/debug/atelier check atelier-t876 && target/debug/atelier check && test "$(git rev-parse HEAD)" = f205bb1fd35f25c3687173c3ad16f630bf6bdbb4'
```

Exit status: 1

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
