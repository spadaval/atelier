---
created_at: "2026-11-09T16:44:04.592449159+00:00"
id: "atelier-kg0r"
evidence_type: "test"
commit: "b7c839c1b46c70f668c05c810bb0bdc3127944d8"
result: "pass"
captured_at: "2026-11-09T16:44:02.607690158+00:00"
command: "cargo nextest run -j 4 --build-jobs 4 -p atelier-site"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-zpmb"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-zpmb"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Site unit tests: generated TypeScript types match web/src/api, and at most two builds run at once with a third waiting past the limit getting a 503"
updated_at: "2026-11-09T16:44:04.723631282+00:00"
---

## Summary

Site unit tests: generated TypeScript types match web/src/api, and at most two builds run at once with a third waiting past the limit getting a 503

## Command

```console
cargo nextest run -j 4 --build-jobs 4 -p atelier-site
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 608
SHA-256: 7cc9176d3d1b4bbe7409a30db0b2c197c92485880aae8f8ded62c8b8624aec5d
Test summary: Summary [   1.006s] 2 tests run: 2 passed, 0 skipped
