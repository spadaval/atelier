---
created_at: "2026-11-05T02:27:17.031653170+00:00"
id: "atelier-01p4"
evidence_type: "test"
commit: "b646d9d4c0240d54961858b05ed6648100332f07"
result: "pass"
captured_at: "2026-11-05T02:27:00.455113944+00:00"
command: "cargo nextest run -j 4 -p atelier-app"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-ov97"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ov97"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "atelier-app tests pass at b646d9d4 with the WorkChild view model"
updated_at: "2026-11-05T02:27:17.032358603+00:00"
---

## Summary

atelier-app tests pass at b646d9d4 with the WorkChild view model

## Command

```console
cargo nextest run -j 4 -p atelier-app
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 13797
SHA-256: 10d25d2d66df359238cbad10a27c782d171c10277febf09af96d5d97bea63d37
Test summary: Summary [  15.959s] 107 tests run: 107 passed, 0 skipped
