---
created_at: "2026-11-11T06:03:01.525159266+00:00"
id: "atelier-kbrp"
evidence_type: "test"
commit: "ac162c6c0d31959df27e2549a297be20a6103478"
result: "pass"
captured_at: "2026-11-11T06:02:47.004253658+00:00"
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
title: "next/ workspace after the atelier-xxwm fix: fmt check, clippy with no warnings, and all tests"
updated_at: "2026-11-11T06:03:01.691602968+00:00"
---

## Summary

next/ workspace after the atelier-xxwm fix: fmt check, clippy with no warnings, and all tests

## Command

```console
bash -c 'cd next && cargo fmt -- --check && test "$(cargo clippy -j 4 --all-targets 2>&1 | grep -cE "^(warning|error)")" = 0 && cargo nextest run -j 4 2>&1 | tail -3'
```

Exit status: 0

## Stdout

Bytes: 214
SHA-256: b1270d15c080e7125b6a3fe5e997ecca5b1603542412940bfedc7486c80eb20f
Test summary: Summary [  13.604s] 266 tests run: 266 passed, 0 skipped

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
