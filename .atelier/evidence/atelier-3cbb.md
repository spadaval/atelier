---
created_at: "2026-11-10T16:55:46.976224710+00:00"
id: "atelier-3cbb"
evidence_type: "test"
commit: "85abd947e7cc8402d57da8168b891a02e9f5853b"
result: "pass"
captured_at: "2026-11-10T16:55:46.580552982+00:00"
command: "bash -c 'cd next && cargo fmt -- --check && cargo clippy -j 4 --all-targets --locked -- -D warnings 2>&1 | tail -1'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-5cuc"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-5cuc"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "bash -c 'cd next && cargo fmt -- --check && cargo clippy -j 4 --all-targets --locked -- -D warnings 2>&1 | tail -1'"
updated_at: "2026-11-10T16:55:47.115954071+00:00"
---

## Summary

bash -c 'cd next && cargo fmt -- --check && cargo clippy -j 4 --all-targets --locked -- -D warnings 2>&1 | tail -1'

## Command

```console
bash -c 'cd next && cargo fmt -- --check && cargo clippy -j 4 --all-targets --locked -- -D warnings 2>&1 | tail -1'
```

Exit status: 0

## Stdout

Bytes: 72
SHA-256: 59639588a6f1826f129cbcd121edc07279dab94786f5fa5bde6b3439b2e845cc

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
