---
created_at: "2026-07-06T20:42:31.006613467+00:00"
id: "atelier-cwlj"
evidence_type: "validation"
captured_at: "2026-07-06T20:42:23.084313155+00:00"
command: "bash -lc 'scripts/check_active_command_guidance.sh --self-test'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-vqhi"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-vqhi"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "short final validation capture"
updated_at: "2026-07-06T20:42:41.271843893+00:00"
---

## Summary

short final validation capture

## Command

```console
bash -lc 'scripts/check_active_command_guidance.sh --self-test'
```

Exit status: 0

## Stdout

Bytes: 122
Truncated: no

```text
active command guidance self-test passed: 106 prohibited/context-restricted example(s), including all prior quality cases
```

## Stderr

Bytes: 0
Truncated: no

```text
```
