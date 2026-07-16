---
created_at: "2026-07-16T22:07:12.498861575+00:00"
id: "atelier-562o"
evidence_type: "test"
captured_at: "2026-07-16T22:06:38.599431025+00:00"
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
title: "Reentrant canonical lock ownership is drop-order safe: the thread-local state owns the OS file until the final guard drops; inverse, three-level non-LIFO, rejected-upgrade, and unwind tests pass; cargo nextest run passed 760/760."
updated_at: "2026-07-16T22:07:12.500770817+00:00"
---

## Summary

Reentrant canonical lock ownership is drop-order safe: the thread-local state owns the OS file until the final guard drops; inverse, three-level non-LIFO, rejected-upgrade, and unwind tests pass; cargo nextest run passed 760/760.

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

