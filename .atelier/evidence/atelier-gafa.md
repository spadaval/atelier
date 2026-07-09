---
created_at: "2026-07-09T16:34:50.258435049+00:00"
id: "atelier-gafa"
evidence_type: "test"
captured_at: "2026-07-09T16:34:49.669622368+00:00"
command: "bash -lc 'atelier check atelier-2uim && git diff --check && test \"$(rg -c '\"'\"'^\\| `atelier-(a44d|ql9k|1mga)|^\\| Legacy direct'\"'\"' .atelier/evidence/atelier-uxgk.md)\" -eq 13'"
exit_status: "0"
agent_identity: "agent-factory.validate"
target:
  kind: "issue"
  id: "atelier-2uim"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-2uim"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "bash -lc 'atelier check atelier-2uim && git diff --check && test \"$(rg -c '\"'\"'^\\| `atelier-(a44d|ql9k|1mga)|^\\| Legacy direct'\"'\"' .atelier/evidence/atelier-uxgk.md)\" -eq 13'"
updated_at: "2026-07-09T16:34:54.668032307+00:00"
---

## Summary

bash -lc 'atelier check atelier-2uim && git diff --check && test "$(rg -c '"'"'^\| `atelier-(a44d|ql9k|1mga)|^\| Legacy direct'"'"' .atelier/evidence/atelier-uxgk.md)" -eq 13'

## Command

```console
bash -lc 'atelier check atelier-2uim && git diff --check && test "$(rg -c '"'"'^\| `atelier-(a44d|ql9k|1mga)|^\| Legacy direct'"'"' .atelier/evidence/atelier-uxgk.md)" -eq 13'
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
