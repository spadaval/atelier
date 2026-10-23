---
created_at: "2026-10-23T17:37:08.986953112+00:00"
id: "atelier-nmi2"
evidence_type: "test"
commit: "2957f128bf09b4ff608a939d3a8d3fe80f407ced"
result: "pass"
captured_at: "2026-10-23T17:36:23.617727932+00:00"
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
title: "Full workspace suite with test-hooks passes (953 tests)"
updated_at: "2026-10-23T17:37:08.987406839+00:00"
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
SHA-256: b828b12f8b7738fa7d0dba33033b30ef67987b929f068dee2ec93b022a91e780
Test summary: Summary [  43.581s] 953 tests run: 953 passed (1 slow), 0 skipped
