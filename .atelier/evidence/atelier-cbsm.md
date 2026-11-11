---
created_at: "2026-11-11T05:55:35.128423589+00:00"
id: "atelier-cbsm"
evidence_type: "test"
commit: "e676baad39f4a229b0fdb012b52e1259d3be2c00"
result: "pass"
captured_at: "2026-11-11T05:55:20.508704588+00:00"
command: "bash -c 'cd next && cargo fmt -- --check && test \"$(cargo clippy -j 4 --all-targets 2>&1 | grep -cE \"^(warning|error)\")\" = 0 && cargo nextest run -j 4 2>&1 | tail -3'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-uwxy"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-uwxy"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "next/ workspace: fmt check, clippy with no warnings, and all tests, at e676baad"
updated_at: "2026-11-11T05:55:35.294354895+00:00"
---

## Summary

next/ workspace: fmt check, clippy with no warnings, and all tests, at e676baad

## Command

```console
bash -c 'cd next && cargo fmt -- --check && test "$(cargo clippy -j 4 --all-targets 2>&1 | grep -cE "^(warning|error)")" = 0 && cargo nextest run -j 4 2>&1 | tail -3'
```

Exit status: 0

## Stdout

Bytes: 204
SHA-256: c8d6e84c86133aed60b2623faef6748a4612833e16a0ccf1674aac4cdc86977b
Test summary: Summary [  13.749s] 266 tests run: 266 passed, 0 skipped

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
