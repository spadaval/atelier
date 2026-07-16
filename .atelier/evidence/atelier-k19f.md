---
created_at: "2026-07-16T22:32:13.493121675+00:00"
id: "atelier-k19f"
evidence_type: "validation"
captured_at: "2026-07-16T22:32:12.830902089+00:00"
command: "target/debug/atelier work mission atelier-p4z2 --ready"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-72k4"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-72k4"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "target/debug/atelier work mission atelier-p4z2 --ready"
updated_at: "2026-07-16T22:32:13.494950785+00:00"
---

## Summary

target/debug/atelier work mission atelier-p4z2 --ready

## Command

```console
target/debug/atelier work mission atelier-p4z2 --ready
```

Exit status: 0

## Stdout

Bytes: 731
Truncated: no

```text
atelier-p4z2 [mission] in_progress - Require independent review before mission execution
========================================================================================

Status: in_progress
Work  : active 2, ready 2, blocked 2, done 8, backlog 0

Ready Work
----------
  ready atelier-h3wg [todo] high - Epic: Add independent mission review to Agent Factory
  ready atelier-l5mw [todo] high - Add the Agent Factory mission-review subskill

Next Commands
-------------
  Show mission record: atelier issue show atelier-p4z2
  Inspect transitions: atelier issue transition atelier-p4z2
  List mission ready work: atelier work mission atelier-p4z2 --ready
  List mission blockers: atelier work mission atelier-p4z2 --blocked
```

## Stderr

Bytes: 0
Truncated: no

```text
```
