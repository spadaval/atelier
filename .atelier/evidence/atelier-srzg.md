---
created_at: "2026-11-11T07:03:13.286840917+00:00"
id: "atelier-srzg"
evidence_type: "test"
commit: "6a1a37512e371e0ebcff9281d5a0046ec74354a5"
result: "pass"
captured_at: "2026-11-11T07:03:08.172561977+00:00"
command: "bash -c 'cd next && cargo fmt -- --check && cargo clippy -j 4 --all-targets -- -D warnings 2>&1 | tail -2 && cd .. && git diff --check master'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-h3hl"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-h3hl"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "fmt, clippy and whitespace"
updated_at: "2026-11-11T07:03:13.454669351+00:00"
---

## Summary

fmt, clippy and whitespace

## Command

```console
bash -c 'cd next && cargo fmt -- --check && cargo clippy -j 4 --all-targets -- -D warnings 2>&1 | tail -2 && cd .. && git diff --check master'
```

Exit status: 0

## Stdout

Bytes: 180
SHA-256: 6110af8a38f3ca49341b52e2c3fe0ae9271d7829abc7f9fa51189ef3a2b03585

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
