---
created_at: "2026-07-01T05:55:36.152289797+00:00"
id: "atelier-y4sq"
evidence_type: "validation"
captured_at: "2026-07-01T05:55:36.136286108+00:00"
command: "rg -n -g '*.md' 'atelier (mission status|mission list|issue status|work queue --(ready|blocked|status|category))' docs"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-p0am"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-p0am"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "p0am docs vocabulary validation"
updated_at: "2026-07-01T05:55:42.061206124+00:00"
---

## Summary

p0am docs vocabulary validation

## Command

```console
rg -n -g '*.md' 'atelier (mission status|mission list|issue status|work queue --(ready|blocked|status|category))' docs
```

Exit status: 0

## Stdout

Bytes: 230
Truncated: no

```text
docs/adr/0015-missions-are-declared-workflow-policy.md:38:- `atelier issue status <objective-id>` inspects readiness and terminal checks
docs/product/command-audit/command-surface-cut-plan.md:240:### Remove `atelier issue status`
```

## Stderr

Bytes: 0
Truncated: no

```text
```
