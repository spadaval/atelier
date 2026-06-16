---
created_at: "2026-06-16T17:41:39.305573777+00:00"
id: "atelier-oe8b"
evidence_type: "test"
captured_at: "2026-06-16T17:41:37.113394384+00:00"
command: "bash -lc 'target/debug/atelier --help | grep -q \"atelier doctor --fix\" && ! target/debug/atelier --help | grep -E \"^  export\\b|^  rebuild\\b|atelier export\" && target/debug/atelier man admin | grep -q \"atelier branch status\" && ! target/debug/atelier man admin | grep -q \"atelier export\"'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-vuqb"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-vuqb"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "root/admin help omit export from routine command surface"
updated_at: "2026-06-16T17:41:45.164526764+00:00"
---

## Summary

root/admin help omit export from routine command surface

## Command

```console
bash -lc 'target/debug/atelier --help | grep -q "atelier doctor --fix" && ! target/debug/atelier --help | grep -E "^  export\b|^  rebuild\b|atelier export" && target/debug/atelier man admin | grep -q "atelier branch status" && ! target/debug/atelier man admin | grep -q "atelier export"'
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
