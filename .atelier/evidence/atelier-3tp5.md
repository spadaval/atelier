---
created_at: "2026-10-17T06:12:35.182088342+00:00"
id: "atelier-3tp5"
evidence_type: "test"
commit: "e62d6ca0cb613ac05d3581752b86b5726432c927"
result: "fail"
captured_at: "2026-10-17T06:12:34.385397434+00:00"
command: "sh -c 'cargo fmt -- --check && git diff --check HEAD~2'"
exit_status: "2"
agent_identity: "claude-orchestrator"
target:
  kind: "issue"
  id: "atelier-i2qq"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-i2qq"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Formatting and whitespace checks pass"
updated_at: "2026-10-17T06:12:35.182928159+00:00"
---

## Summary

Formatting and whitespace checks pass

## Command

```console
sh -c 'cargo fmt -- --check && git diff --check HEAD~2'
```

Exit status: 2

## Stdout

Bytes: 183
Truncated: no

```text
.atelier/evidence/atelier-lv4h.md:90: new blank line at EOF.
.atelier/evidence/atelier-xxy8.md:91: new blank line at EOF.
.atelier/evidence/atelier-zf2i.md:90: new blank line at EOF.
```

## Stderr

Bytes: 0
Truncated: no

```text
```

