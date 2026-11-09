---
created_at: "2026-11-09T22:11:12.786205339+00:00"
id: "atelier-0arc"
evidence_type: "test"
commit: "b29d83960e452aebe1c6a111ea3dce3dc947d857"
result: "pass"
captured_at: "2026-11-09T22:10:29.743690457+00:00"
command: "bash -c 'cargo fmt -- --check && git diff --check master... && cargo nextest run -p atelier-cli'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-fnlu"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-fnlu"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo fmt --check, git diff --check and cargo nextest run -p atelier-cli (883 tests, including the brief rendering test) pass"
updated_at: "2026-11-09T22:11:12.934928820+00:00"
---

## Summary

cargo fmt --check, git diff --check and cargo nextest run -p atelier-cli (883 tests, including the brief rendering test) pass

## Command

```console
bash -c 'cargo fmt -- --check && git diff --check master... && cargo nextest run -p atelier-cli'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 111637
SHA-256: d60b576262758c1ccff16b65948118023dbd26e52e11500afd40d56b308a9800
Test summary: Summary [  38.391s] 883 tests run: 883 passed, 1 skipped
