---
created_at: "2026-10-23T17:05:54.713691660+00:00"
id: "atelier-v86w"
evidence_type: "test"
commit: "be96c10dbb108dace195f37b2659dc1623f1e752"
result: "pass"
captured_at: "2026-10-23T17:05:10.874566347+00:00"
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
title: "Full workspace suite with test hooks passes after the atelier-61il and atelier-vlhn fixes (rerun after merging master)"
updated_at: "2026-10-23T17:05:54.714333940+00:00"
---

## Summary

Full workspace suite with test hooks passes after the atelier-61il and atelier-vlhn fixes (rerun after merging master)

## Command

```console
cargo nextest run -j 4 --workspace --features test-hooks
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 118576
SHA-256: b224cabf0b7f2dca9134145d1436f24a36ab744aaef97b402a2a40e5fe510004
Test summary: Summary [  41.709s] 943 tests run: 943 passed (1 slow), 0 skipped
