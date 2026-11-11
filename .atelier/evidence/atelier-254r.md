---
created_at: "2026-11-11T08:04:55.986094164+00:00"
id: "atelier-254r"
evidence_type: "test"
commit: "c58e0b76314be2ecd4fae80c864e2733cda80b52"
result: "pass"
captured_at: "2026-11-11T08:04:36.846172460+00:00"
command: "cargo nextest run --locked -j 4"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-u1ko"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-u1ko"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Root workspace at the branch head: cargo nextest run --locked passes (fmt, build and clippy -D warnings passed before it)"
updated_at: "2026-11-11T08:04:56.234343254+00:00"
---

## Summary

Root workspace at the branch head: cargo nextest run --locked passes (fmt, build and clippy -D warnings passed before it)

## Command

```console
cargo nextest run --locked -j 4
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 33365
SHA-256: c52fe1b42f7d77848a5d825392e48b27f900e4351a0ad9dd33f8f32be98ffe16
Test summary: Summary [  18.383s] 283 tests run: 283 passed, 0 skipped
