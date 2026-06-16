---
created_at: "2026-06-16T17:43:39.830818019+00:00"
id: "atelier-2437"
evidence_type: "validation"
captured_at: "2026-06-16T17:43:39.825227551+00:00"
command: "git show --stat --oneline db5bd17"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-jezn"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-jezn"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "repo guidance diff removes routine export checks and adds classification"
updated_at: "2026-06-16T17:43:43.415765506+00:00"
---

## Summary

repo guidance diff removes routine export checks and adds classification

## Command

```console
git show --stat --oneline db5bd17
```

Exit status: 0

## Stdout

Bytes: 368
Truncated: no

```text
db5bd17 Remove routine export check guidance
 AGENTFACTORY.md                                    | 13 ++++----
 docs/architecture/quality/validation.md            | 14 ++++----
 .../export-check-reference-classification.md       | 39 ++++++++++++++++++++++
 docs/product/command-audit/index.md                |  1 +
 4 files changed, 53 insertions(+), 14 deletions(-)
```

## Stderr

Bytes: 0
Truncated: no

```text
```
