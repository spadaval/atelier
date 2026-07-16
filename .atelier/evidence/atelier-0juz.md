---
created_at: "2026-07-16T21:38:59.078075771+00:00"
id: "atelier-0juz"
evidence_type: "test"
captured_at: "2026-07-16T21:38:23.870059441+00:00"
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
title: "Split-phase command capture avoids canonical lock inversion: nested exclusive mission migration completed before evidence transaction; focused reentrant migration and mutation regression passed; cargo nextest run passed 753/753."
updated_at: "2026-07-16T21:38:59.079962144+00:00"
---

## Summary

Split-phase command capture avoids canonical lock inversion: nested exclusive mission migration completed before evidence transaction; focused reentrant migration and mutation regression passed; cargo nextest run passed 753/753.

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

