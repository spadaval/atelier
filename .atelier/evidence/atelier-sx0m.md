---
created_at: "2026-11-11T08:05:31.384901679+00:00"
id: "atelier-sx0m"
evidence_type: "test"
commit: "8967c46eae2cdd361a3f26fe473493856984722d"
result: "pass"
captured_at: "2026-11-11T08:05:02.484154198+00:00"
command: "pnpm e2e --repeat-each 50 --workers 8 -g 'drops an answer'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-l2aj"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-l2aj"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Flake check at 8967c46ea: the CI failure 'the inbox drops an answer's not-committed mark once a commit holds it' run 100 times (50 per width) at 8 workers, which failed 7 of 60 before the fix"
updated_at: "2026-11-11T08:05:31.562622285+00:00"
---

## Summary

Flake check at 8967c46ea: the CI failure 'the inbox drops an answer's not-committed mark once a commit holds it' run 100 times (50 per width) at 8 workers, which failed 7 of 60 before the fix

## Command

```console
pnpm e2e --repeat-each 50 --workers 8 -g 'drops an answer'
```

Exit status: 0

## Stdout

Bytes: 12001
SHA-256: 8fcdc2d927ee81a55ec8c01b8967af0b29f4dbfcfc24dbc533911d97b994a156

## Stderr

Bytes: 68
SHA-256: 7c53158a98c6bea9b31500539a23fecbd5e032d0fdf5db0c8ba4c1a7c4b54465
