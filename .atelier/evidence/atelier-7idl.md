---
created_at: "2026-11-09T18:39:54.606456106+00:00"
id: "atelier-7idl"
evidence_type: "test"
commit: "fbd16d2958cd358a0ef7eebfe3755050b3ee835b"
result: "pass"
captured_at: "2026-11-09T18:39:49.405031220+00:00"
command: "cargo nextest run --build-jobs 4 -j 4 -E 'package(atelier-site) or binary(serve)'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-yn69"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-yn69"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Embedded app routing: asset content types, missing asset 404, index for pages, built or not; every API route"
updated_at: "2026-11-09T18:39:54.740059573+00:00"
---

## Summary

Embedded app routing: asset content types, missing asset 404, index for pages, built or not; every API route

## Command

```console
cargo nextest run --build-jobs 4 -j 4 -E 'package(atelier-site) or binary(serve)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 3776
SHA-256: 4968d382dc30817e2acad8bdafdec59966b461e303c9f63d158bad553b37721e
Test summary: Summary [   1.973s] 28 tests run: 28 passed, 1 skipped
