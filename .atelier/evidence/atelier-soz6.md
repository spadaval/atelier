---
created_at: "2026-10-23T15:53:37.286041766+00:00"
id: "atelier-soz6"
evidence_type: "test"
commit: "fd8ad78e1bfc812396803e7a1f2e6a40784cc95a"
result: "pass"
captured_at: "2026-10-23T15:52:55.905951002+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks"
exit_status: "0"
agent_identity: "atelier merge --sync"
target:
  kind: "issue"
  id: "atelier-0uaj"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-0uaj"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full workspace suite with test hooks passes after the atelier-0uaj changes (rerun after merging master)"
updated_at: "2026-10-23T15:53:37.286683214+00:00"
---

## Summary

Full workspace suite with test hooks passes after the atelier-0uaj changes (rerun after merging master)

## Command

```console
cargo nextest run -j 4 --workspace --features test-hooks
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 116718
SHA-256: 0d2d697025f493c8030ded8756ea57963bdb6f3e45d302202ae677bf9f64fad8
Test summary: Summary [  39.366s] 928 tests run: 928 passed (1 slow), 0 skipped
