---
created_at: "2026-11-11T06:08:56.731858378+00:00"
id: "atelier-o5vf"
evidence_type: "test"
commit: "bcd2e0ce9d4763bd5ab478430f3357be723c6963"
result: "pass"
captured_at: "2026-11-11T06:08:42.148628232+00:00"
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
title: "next/ workspace after the atelier-2t1h fix: fmt check, clippy with no warnings, and all tests"
updated_at: "2026-11-11T06:08:56.896119217+00:00"
---

## Summary

next/ workspace after the atelier-2t1h fix: fmt check, clippy with no warnings, and all tests

## Command

```console
bash -c 'cd next && cargo fmt -- --check && test "$(cargo clippy -j 4 --all-targets 2>&1 | grep -cE "^(warning|error)")" = 0 && cargo nextest run -j 4 2>&1 | tail -3'
```

Exit status: 0

## Stdout

Bytes: 214
SHA-256: 32f02cb713c483194b36e90b39620242cfdb2eb16fba2df9725e33e40b8638cf
Test summary: Summary [  13.702s] 266 tests run: 266 passed, 0 skipped

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
