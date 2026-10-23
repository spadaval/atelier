---
created_at: "2026-10-23T15:46:02.926530468+00:00"
id: "atelier-erv1"
evidence_type: "test"
commit: "e52310719a7fa54f70f69632d3b9ad301f7ce855"
result: "pass"
captured_at: "2026-10-23T15:45:11.799675105+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks"
exit_status: "0"
agent_identity: "claude-implementer-imlh"
target:
  kind: "issue"
  id: "atelier-imlh"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-imlh"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "full workspace suite with test-hooks passes on feature/atelier-imlh after the atelier-ytks fix"
updated_at: "2026-10-23T15:46:02.926944770+00:00"
---

## Summary

full workspace suite with test-hooks passes on feature/atelier-imlh after the atelier-ytks fix

## Command

```console
cargo nextest run -j 4 --workspace --features test-hooks
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 115752
SHA-256: 4d595e4c6fff66e8d807abe5d210b06f58c36af4f57822a31212656fd9fc7fec
Test summary: Summary [  49.084s] 920 tests run: 920 passed (1 slow), 0 skipped
