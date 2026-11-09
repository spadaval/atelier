---
created_at: "2026-11-09T16:42:29.923449861+00:00"
id: "atelier-mo9r"
evidence_type: "test"
commit: "b7c839c1b46c70f668c05c810bb0bdc3127944d8"
result: "pass"
captured_at: "2026-11-09T16:41:20.894244295+00:00"
command: "cargo nextest run -j 4 --build-jobs 4 --workspace"
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
title: "Full workspace suite passes, 1112 tests including the serve, discovery, divergent-HEAD, read-only and fixture tests"
updated_at: "2026-11-09T16:42:30.056029199+00:00"
---

## Summary

Full workspace suite passes, 1112 tests including the serve, discovery, divergent-HEAD, read-only and fixture tests

## Command

```console
cargo nextest run -j 4 --build-jobs 4 --workspace
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 140735
SHA-256: fa5811afc5eee062b569bdf26faeb3dd80a9ddcc6d671253ff3f93fc926a6635
Test summary: Summary [  65.849s] 1111 tests run: 1111 passed, 0 skipped
