---
created_at: "2026-11-10T15:38:52.716121395+00:00"
id: "atelier-xsw2"
evidence_type: "test"
commit: "cd0d159306365ea3d185f65c9752480b061bcd90"
result: "pass"
captured_at: "2026-11-10T15:38:52.229652641+00:00"
command: "bash -c 'cd next && cargo fmt -- --check && cargo nextest run --build-jobs 4 --test-threads 4 --locked'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-mal9"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-mal9"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "next/: cargo fmt --check and the full nextest suite (38 tests) pass"
updated_at: "2026-11-10T15:38:52.857216773+00:00"
---

## Summary

next/: cargo fmt --check and the full nextest suite (38 tests) pass

## Command

```console
bash -c 'cd next && cargo fmt -- --check && cargo nextest run --build-jobs 4 --test-threads 4 --locked'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 4328
SHA-256: b2ea47b2d75f39903f48dc0ac87893a4d82ce8b7175dbda014a033061f7deca1
Test summary: Summary [   0.057s] 38 tests run: 38 passed, 0 skipped
