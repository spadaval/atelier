---
created_at: "2026-11-11T01:09:27.863869910+00:00"
id: "atelier-uwab"
evidence_type: "test"
commit: "38cae6b79be7c5194494f05dab8256fd06cb013c"
result: "pass"
captured_at: "2026-11-11T01:09:26.781309891+00:00"
command: "bash -c 'cd next && cargo fmt -- --check && cargo clippy -j 4 --all-targets --locked -- -D warnings'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-efsl"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-efsl"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "next/: cargo fmt --check and clippy -D warnings clean"
updated_at: "2026-11-11T01:09:28.016825034+00:00"
---

## Summary

next/: cargo fmt --check and clippy -D warnings clean

## Command

```console
bash -c 'cd next && cargo fmt -- --check && cargo clippy -j 4 --all-targets --locked -- -D warnings'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 180
SHA-256: d2b2fd2baa2c5d18568f8433e07744468efb55ff702ab8114e812a0a4bc94096
