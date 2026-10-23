---
created_at: "2026-10-23T14:57:58.585503860+00:00"
id: "atelier-8z2v"
evidence_type: "test"
commit: "06272b9d4b6401c8d7fe6c7561257da0b2c9446d"
result: "pass"
captured_at: "2026-10-23T14:57:19.169350804+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks"
exit_status: "0"
agent_identity: "claude-implementer-h1nc"
target:
  kind: "issue"
  id: "atelier-h1nc"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-h1nc"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "full workspace suite with test hooks passes (898 tests) at the review-fix commit"
updated_at: "2026-10-23T14:57:58.585825239+00:00"
---

## Summary

full workspace suite with test hooks passes (898 tests) at the review-fix commit

## Command

```console
cargo nextest run -j 4 --workspace --features test-hooks
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 112854
SHA-256: 4e99792ffadd863d25b9a5a6c5fe5a911b7b3cb3639e1d99c651483509696440
Test summary: Summary [  37.709s] 898 tests run: 898 passed (1 slow), 0 skipped
