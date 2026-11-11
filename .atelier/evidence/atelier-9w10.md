---
created_at: "2026-11-11T01:10:10.602020854+00:00"
id: "atelier-9w10"
evidence_type: "test"
commit: "38cae6b79be7c5194494f05dab8256fd06cb013c"
result: "pass"
captured_at: "2026-11-11T01:09:58.366237995+00:00"
command: "bash -c 'set -o pipefail; cd next && cargo nextest run --locked --build-jobs 4 --test-threads 4 2>&1 | tail -3'"
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
title: "next/: full nextest suite, 225 tests"
updated_at: "2026-11-11T01:10:10.760045299+00:00"
---

## Summary

next/: full nextest suite, 225 tests

## Command

```console
bash -c 'set -o pipefail; cd next && cargo nextest run --locked --build-jobs 4 --test-threads 4 2>&1 | tail -3'
```

Exit status: 0

## Stdout

Bytes: 214
SHA-256: b35b4f81f4c8e0572d1d67fc831fd9f63c7fdded702071f131295ced005f8bf9
Test summary: Summary [  11.872s] 225 tests run: 225 passed, 0 skipped

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
