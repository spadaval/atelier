---
created_at: "2026-11-10T16:50:57.883011757+00:00"
id: "atelier-ybf8"
evidence_type: "test"
commit: "74993259cc8e7b2dec7a7ccb9dd5632c948825db"
result: "pass"
captured_at: "2026-11-10T16:50:57.476338015+00:00"
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
updated_at: "2026-11-10T16:50:58.024573884+00:00"
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
