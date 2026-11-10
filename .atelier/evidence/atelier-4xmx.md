---
created_at: "2026-11-10T19:14:04.240475491+00:00"
id: "atelier-4xmx"
evidence_type: "test"
commit: "88863e85edce1cd566986063fe2bd56692b2e050"
result: "pass"
captured_at: "2026-11-10T19:14:00.102115119+00:00"
command: "bash -c 'cd next && cargo fmt --check && cargo clippy -j 4 --all-targets -q -- -D warnings && cargo nextest run -j 4 --no-fail-fast 2>&1 | tail -3'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-67nn"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-67nn"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "next/: cargo fmt --check, clippy -D warnings, nextest (105 tests)"
updated_at: "2026-11-10T19:14:04.384497292+00:00"
---

## Summary

next/: cargo fmt --check, clippy -D warnings, nextest (105 tests)

## Command

```console
bash -c 'cd next && cargo fmt --check && cargo clippy -j 4 --all-targets -q -- -D warnings && cargo nextest run -j 4 --no-fail-fast 2>&1 | tail -3'
```

Exit status: 0

## Stdout

Bytes: 206
SHA-256: 032fda21f87756d4dd123ff9be7a1555420930db741c6b9e6c4baa11850f9481
Test summary: Summary [   3.504s] 105 tests run: 105 passed, 0 skipped

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
