---
created_at: "2026-07-16T22:00:34.569226994+00:00"
id: "atelier-xfcy"
evidence_type: "test"
captured_at: "2026-07-16T21:59:58.544903639+00:00"
command: "target/debug/atelier migrate-mission-plan-review"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-3v2d"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-3v2d"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Immutable target binding and exclusive postflight prevent same-ID replacement misattribution: target identity includes schema/version, created_at, and record type; paused writer exclusion and focused capture tests pass; cargo nextest run passed 758/758."
updated_at: "2026-07-16T22:00:34.571082453+00:00"
---

## Summary

Immutable target binding and exclusive postflight prevent same-ID replacement misattribution: target identity includes schema/version, created_at, and record type; paused writer exclusion and focused capture tests pass; cargo nextest run passed 758/758.

## Command

```console
target/debug/atelier migrate-mission-plan-review
```

Exit status: 0

## Stdout

Bytes: 81
Truncated: no

```text
Independent mission plan-review cutover already applied (42 missions validated).
```

## Stderr

Bytes: 0
Truncated: no

```text
```
