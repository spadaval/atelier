---
created_at: "2026-11-10T22:27:42.409837157+00:00"
id: "atelier-baxf"
evidence_type: "test"
commit: "1771fec8cea910e15036b1d0915674895136a6bd"
result: "pass"
captured_at: "2026-11-10T22:27:29.799490508+00:00"
command: "bash -c 'cd next && cargo fmt --check && TMPDIR=/home/supadava/.cache/atelier-fcb7/tmp cargo clippy -q -j 4 --all-targets -- -D warnings && TMPDIR=/home/supadava/.cache/atelier-fcb7/tmp cargo nextest run --build-jobs 4 -j 4 2>&1 | tail -3'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-pz1y"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-pz1y"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "next/ suite: fmt check, clippy -D warnings, nextest (181 tests)"
updated_at: "2026-11-10T22:27:42.568176176+00:00"
---

## Summary

next/ suite: fmt check, clippy -D warnings, nextest (181 tests)

## Command

```console
bash -c 'cd next && cargo fmt --check && TMPDIR=/home/supadava/.cache/atelier-fcb7/tmp cargo clippy -q -j 4 --all-targets -- -D warnings && TMPDIR=/home/supadava/.cache/atelier-fcb7/tmp cargo nextest run --build-jobs 4 -j 4 2>&1 | tail -3'
```

Exit status: 0

## Stdout

Bytes: 204
SHA-256: 6c1e0e5c01c069183439e6bccf73131705fddcd0eae38156522a98552bb92b2c
Test summary: Summary [   9.606s] 181 tests run: 181 passed, 0 skipped

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
