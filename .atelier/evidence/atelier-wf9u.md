---
created_at: "2026-11-11T03:36:15.906901747+00:00"
id: "atelier-wf9u"
evidence_type: "test"
commit: "92b63de6c93b2986a507fbcbdbe769a0149d2bd9"
result: "pass"
captured_at: "2026-11-11T03:36:02.321754292+00:00"
command: "sh -c 'cd next && cargo fmt -- --check && cargo clippy -j 4 --all-targets -- -D warnings && cargo nextest run -j 4'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-1oel"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-1oel"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "next/: fmt --check, clippy -D warnings and nextest (239 tests) pass after the atelier-s6xa fix"
updated_at: "2026-11-11T03:36:16.062560345+00:00"
---

## Summary

next/: fmt --check, clippy -D warnings and nextest (239 tests) pass after the atelier-s6xa fix

## Command

```console
sh -c 'cd next && cargo fmt -- --check && cargo clippy -j 4 --all-targets -- -D warnings && cargo nextest run -j 4'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 28722
SHA-256: 9477fc026fb4da05fb0f1691b03945bafa13289bc4a5bb9dd547f582083a2443
Test summary: Summary [  12.837s] 239 tests run: 239 passed, 0 skipped
