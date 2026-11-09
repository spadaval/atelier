---
created_at: "2026-11-09T20:25:21.623355353+00:00"
id: "atelier-4bzu"
evidence_type: "test"
commit: "132006b728303cb90ff8815b499dd4388966856b"
result: "pass"
captured_at: "2026-11-09T20:25:19.879972180+00:00"
command: "cargo nextest run -j 4 -p atelier-site"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-ahpf"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ahpf"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "atelier-site unit tests, including current TypeScript types"
updated_at: "2026-11-09T20:25:21.824976418+00:00"
---

## Summary

atelier-site unit tests, including current TypeScript types

## Command

```console
cargo nextest run -j 4 -p atelier-site
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 1101
SHA-256: 98a1740e92af8759a93be41704a8fb789fdb971426829a32413d73d1969064ba
Test summary: Summary [   1.212s] 8 tests run: 8 passed, 0 skipped
