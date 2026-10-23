---
created_at: "2026-10-23T16:14:27.240537237+00:00"
id: "atelier-0xal"
evidence_type: "test"
commit: "514da007a99e735a8120c1c5a84e0b72421bc0c4"
result: "pass"
captured_at: "2026-10-23T16:13:39.762672375+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks"
exit_status: "0"
agent_identity: "claude-implementer-l7nz"
target:
  kind: "issue"
  id: "atelier-l7nz"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-3ekh"
    role: "validates"
  - kind: "issue"
    id: "atelier-b05y"
    role: "validates"
  - kind: "issue"
    id: "atelier-l7nz"
    role: "validates"
  - kind: "issue"
    id: "atelier-o8ni"
    role: "validates"
  - kind: "issue"
    id: "atelier-u0ig"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full workspace suite with test hooks passes on the merged tree after bringing in master 0d05dcfb (925 tests)"
updated_at: "2026-10-23T16:14:29.281141774+00:00"
---

## Summary

Full workspace suite with test hooks passes on the merged tree after bringing in master 0d05dcfb (925 tests)

## Command

```console
cargo nextest run -j 4 --workspace --features test-hooks
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 116453
SHA-256: 2b684d7e1855bccb4afcb05d4601e95f6cf7fed0fb1394065bdfdc95ac8f9538
Test summary: Summary [  44.904s] 925 tests run: 925 passed (1 slow), 0 skipped
