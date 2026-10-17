---
created_at: "2026-10-17T07:03:45.434177026+00:00"
id: "atelier-j7kw"
evidence_type: "test"
commit: "e27fd01b9a2b956767ea593e9e6f5b3ee0666748"
result: "pass"
captured_at: "2026-10-17T07:03:45.233656423+00:00"
command: "git diff --check master -- . ':!.atelier'"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "atelier-onwh"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-onwh"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "git diff --check against master finds no whitespace errors outside .atelier"
updated_at: "2026-10-17T07:03:45.434834331+00:00"
---

## Summary

git diff --check against master finds no whitespace errors outside .atelier

## Command

```console
git diff --check master -- . ':!.atelier'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 0
Truncated: no

```text
```

