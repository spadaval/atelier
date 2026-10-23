---
created_at: "2026-10-23T15:34:05.526127247+00:00"
id: "atelier-2vps"
evidence_type: "test"
commit: "9f4d7ee0478cb378cac9b7072ff2de41ba86b223"
result: "pass"
captured_at: "2026-10-23T15:34:03.365312725+00:00"
command: "cargo nextest run -j 4 --features test-hooks -p atelier-cli --test merge_gates gates_name"
exit_status: "0"
agent_identity: "claude-implementer-imlh"
target:
  kind: "issue"
  id: "atelier-cisa"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-cisa"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "land and merge --check name the approval considered and why it is not current (stale by code, stale by base merge of an in-scope record, author identity), merge --sync and review list report it from the same approval_standing, and every printed fix runs"
updated_at: "2026-10-23T15:34:05.526385510+00:00"
---

## Summary

land and merge --check name the approval considered and why it is not current (stale by code, stale by base merge of an in-scope record, author identity), merge --sync and review list report it from the same approval_standing, and every printed fix runs

## Command

```console
cargo nextest run -j 4 --features test-hooks -p atelier-cli --test merge_gates gates_name
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 806
SHA-256: 1a9e471a4d2249139d3f8090a8669a73f09aa8d4e517d65559c97238290e70d7
Test summary: Summary [   0.418s] 3 tests run: 3 passed, 78 skipped
