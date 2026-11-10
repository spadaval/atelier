---
created_at: "2026-11-10T05:20:52.730765352+00:00"
id: "atelier-uqa3"
evidence_type: "validation"
commit: "2dfeef82d317b0bb97a63633bc74feb58157d392"
result: "pass"
captured_at: "2026-11-10T05:19:26.879482424+00:00"
command: "nice -n 19 ionice -c3 sqlite3 'file:/home/supadava/.local/share/atelier-baseline/2026-11/t3/statev2.sqlite?mode=ro' 'PRAGMA integrity_check;'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-scx2"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-scx2"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "PRAGMA integrity_check on the snapshot's copy of T3's database (read-only, low priority)"
updated_at: "2026-11-10T05:20:52.884639164+00:00"
---

## Summary

PRAGMA integrity_check on the snapshot's copy of T3's database (read-only, low priority)

## Command

```console
nice -n 19 ionice -c3 sqlite3 'file:/home/supadava/.local/share/atelier-baseline/2026-11/t3/statev2.sqlite?mode=ro' 'PRAGMA integrity_check;'
```

Exit status: 0

## Stdout

Bytes: 3
SHA-256: dc51b8c96c2d745df3bd5590d990230a482fd247123599548e0632fdbf97fc22

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
