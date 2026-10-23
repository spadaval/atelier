---
created_at: "2026-10-23T17:32:28.703889380+00:00"
id: "atelier-0aco"
evidence_type: "test"
commit: "d5a4bebcd353188a3cc4880716c0732a5f494519"
result: "pass"
captured_at: "2026-10-23T17:31:43.416701056+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks"
exit_status: "0"
agent_identity: "claude-implementer-buqu"
target:
  kind: "issue"
  id: "atelier-buqu"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-buqu"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full workspace suite with test-hooks passes (952 tests) after the S5 hook, schema and zegg changes"
updated_at: "2026-10-23T17:32:28.704139366+00:00"
---

## Summary

Full workspace suite with test-hooks passes (952 tests) after the S5 hook, schema and zegg changes

## Command

```console
cargo nextest run -j 4 --workspace --features test-hooks
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 119560
SHA-256: a6603552ce8c987ca5be46e4bac81a8210b0780e1020b003813b37c2ec240477
Test summary: Summary [  43.512s] 952 tests run: 952 passed (1 slow), 0 skipped
