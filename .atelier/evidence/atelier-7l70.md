---
created_at: "2026-11-11T08:28:08.829458199+00:00"
id: "atelier-7l70"
evidence_type: "test"
commit: "2f729bef46fa930082eb0a54af99f158ed15b9eb"
result: "pass"
captured_at: "2026-11-11T08:28:05.587023727+00:00"
command: "cargo nextest run -j 4 --manifest-path next/Cargo.toml -p atelier-desk"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-va99"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-va99"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "SIGTERM and SIGINT stop atelier-desk cleanly and revoke its T3 Code session (tests/signals.rs), and built_from is the build's HEAD (tests/desk.rs)"
updated_at: "2026-11-11T08:28:09.003677398+00:00"
---

## Summary

SIGTERM and SIGINT stop atelier-desk cleanly and revoke its T3 Code session (tests/signals.rs), and built_from is the build's HEAD (tests/desk.rs)

## Command

```console
cargo nextest run -j 4 --manifest-path next/Cargo.toml -p atelier-desk
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 3726
SHA-256: 40c1d68703b4ef9eed4c6c85d82e0c343cba55f73d5fed9ae4d02ac4e2af8bad
Test summary: Summary [   1.015s] 32 tests run: 32 passed, 0 skipped
