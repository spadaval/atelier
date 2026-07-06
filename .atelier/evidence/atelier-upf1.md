---
created_at: "2026-07-06T17:55:41.885707508+00:00"
id: "atelier-upf1"
evidence_type: "test"
captured_at: "2026-07-06T17:55:38.889347390+00:00"
command: "sh -c 'git diff --check master...HEAD && atelier lint atelier-v9sy && atelier lint atelier-95rr && atelier lint atelier-pxxj && atelier lint atelier-u327'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-v9sy"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-v9sy"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "sh -c 'git diff --check master...HEAD && atelier lint atelier-v9sy && atelier lint atelier-95rr && atelier lint atelier-pxxj && atelier lint atelier-u327'"
updated_at: "2026-07-06T17:55:48.087032708+00:00"
---

## Summary

sh -c 'git diff --check master...HEAD && atelier lint atelier-v9sy && atelier lint atelier-95rr && atelier lint atelier-pxxj && atelier lint atelier-u327'

## Command

```console
sh -c 'git diff --check master...HEAD && atelier lint atelier-v9sy && atelier lint atelier-95rr && atelier lint atelier-pxxj && atelier lint atelier-u327'
```

Exit status: 0

## Stdout

Bytes: 52
Truncated: no

```text
Lint passed.
Lint passed.
Lint passed.
Lint passed.
```

## Stderr

Bytes: 0
Truncated: no

```text
```
