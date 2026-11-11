---
created_at: "2026-11-11T08:38:17.612493262+00:00"
id: "atelier-3rjt"
evidence_type: "test"
commit: "f47a181fee4b844ad3afb079ad6d3ff25d83c8c5"
result: "pass"
captured_at: "2026-11-11T08:38:00.951190592+00:00"
command: "cargo nextest run -j 4 --manifest-path next/Cargo.toml"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-va99"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-va99"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "The whole next/ workspace: 285 tests, including tests/signals.rs (SIGTERM and SIGINT revoke the token) and built_from"
updated_at: "2026-11-11T08:38:17.782951472+00:00"
---

## Summary

The whole next/ workspace: 285 tests, including tests/signals.rs (SIGTERM and SIGINT revoke the token) and built_from

## Command

```console
cargo nextest run -j 4 --manifest-path next/Cargo.toml
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 33711
SHA-256: 142d8bc9996152380f1be3df661fe0d513cdc8d10994c5b3e4c6ed8c22ca1004
Test summary: Summary [  14.318s] 285 tests run: 285 passed, 0 skipped
