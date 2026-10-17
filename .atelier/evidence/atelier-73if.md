---
created_at: "2026-10-17T07:12:35.210936078+00:00"
id: "atelier-73if"
evidence_type: "test"
commit: "ab0b9724d7ef9197a7138e2509fa8c216343fe92"
result: "pass"
captured_at: "2026-10-17T07:12:35.014639676+00:00"
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
updated_at: "2026-10-17T07:12:35.211692358+00:00"
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

