---
created_at: "2026-11-11T05:56:05.189212273+00:00"
id: "atelier-hfug"
evidence_type: "test"
commit: "18a1acf4c770b3444906abe43e61a975328b0724"
result: "pass"
captured_at: "2026-11-11T05:55:50.659500086+00:00"
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
title: "next/ workspace: fmt check, clippy with no warnings, and all tests"
updated_at: "2026-11-11T05:56:05.351973986+00:00"
---

## Summary

next/ workspace: fmt check, clippy with no warnings, and all tests

## Command

```console
bash -c 'cd next && cargo fmt -- --check && test "$(cargo clippy -j 4 --all-targets 2>&1 | grep -cE "^(warning|error)")" = 0 && cargo nextest run -j 4 2>&1 | tail -3'
```

Exit status: 0

## Stdout

Bytes: 204
SHA-256: 76ad01b048f42d518620df13312a4da1b2f4d33b7b9adeb38b390b5861c6c0a6
Test summary: Summary [  13.622s] 266 tests run: 266 passed, 0 skipped

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
