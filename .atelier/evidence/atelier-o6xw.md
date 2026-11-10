---
created_at: "2026-11-10T20:42:27.756441991+00:00"
id: "atelier-o6xw"
evidence_type: "test"
commit: "6d4a628cd3b2775a9455721076fc583ff007a15c"
result: "pass"
captured_at: "2026-11-10T20:42:20.049285600+00:00"
command: "sh -c 'cd next && cargo fmt --check && TMPDIR=/home/supadava/.cache/atelier-fcb7/tmp cargo clippy -j 4 --all-targets -- -D warnings && TMPDIR=/home/supadava/.cache/atelier-fcb7/tmp cargo nextest run --build-jobs 4 --test-threads 4'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-jciw"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-jciw"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "next/: cargo fmt --check, clippy -D warnings and the full nextest suite (152 tests) pass at the Report's commit"
updated_at: "2026-11-10T20:42:27.901726058+00:00"
---

## Summary

next/: cargo fmt --check, clippy -D warnings and the full nextest suite (152 tests) pass at the Report's commit

## Command

```console
sh -c 'cd next && cargo fmt --check && TMPDIR=/home/supadava/.cache/atelier-fcb7/tmp cargo clippy -j 4 --all-targets -- -D warnings && TMPDIR=/home/supadava/.cache/atelier-fcb7/tmp cargo nextest run --build-jobs 4 --test-threads 4'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 17768
SHA-256: fd78df51a57958c0f3f8b6422a0e2ea9d65ab569d4bc96136eec97d5b89a3a61
Test summary: Summary [   6.830s] 152 tests run: 152 passed, 0 skipped
