---
created_at: "2026-10-23T17:37:54.737986639+00:00"
id: "atelier-ikdi"
evidence_type: "test"
commit: "2957f128bf09b4ff608a939d3a8d3fe80f407ced"
result: "pass"
captured_at: "2026-10-23T17:37:09.302450395+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks"
exit_status: "0"
agent_identity: "claude-implementer-buqu"
target:
  kind: "issue"
  id: "atelier-i80r"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-i80r"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full workspace suite with test-hooks passes (953 tests)"
updated_at: "2026-10-23T17:37:54.738415830+00:00"
---

## Summary

Full workspace suite with test-hooks passes (953 tests)

## Command

```console
cargo nextest run -j 4 --workspace --features test-hooks
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 119668
SHA-256: 712c8d2a99c89ff1590fa58e922ba7de07f4e9011d04e489170d624414fbb647
Test summary: Summary [  43.654s] 953 tests run: 953 passed (1 slow), 0 skipped
