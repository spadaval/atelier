---
created_at: "2026-10-23T16:29:51.403418024+00:00"
id: "atelier-ws5o"
evidence_type: "test"
commit: "7582a2221784baf5591841f72de9706bfa66ecec"
result: "pass"
captured_at: "2026-10-23T16:29:08.072713175+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks"
exit_status: "0"
agent_identity: "claude-implementer-0uaj"
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
title: "Full workspace suite with test hooks passes after the atelier-61il and atelier-vlhn fixes"
updated_at: "2026-10-23T16:29:51.403701874+00:00"
---

## Summary

Full workspace suite with test hooks passes after the atelier-61il and atelier-vlhn fixes

## Command

```console
cargo nextest run -j 4 --workspace --features test-hooks
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 117645
SHA-256: 0fba61790edccee53000e6561d215b94e30a470cfe43e46818ef5d5af96d2bb0
Test summary: Summary [  41.498s] 936 tests run: 936 passed (1 slow), 0 skipped
