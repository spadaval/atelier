---
created_at: "2026-11-11T08:27:04.283317341+00:00"
id: "atelier-ahmy"
evidence_type: "test"
commit: "c6db1865681617c05424ac4be591a1cdb5b6afdc"
result: "pass"
captured_at: "2026-11-11T08:27:02.828851426+00:00"
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
title: "SIGTERM and SIGINT stop atelier-desk cleanly and revoke its T3 Code session"
updated_at: "2026-11-11T08:27:04.457571867+00:00"
---

## Summary

SIGTERM and SIGINT stop atelier-desk cleanly and revoke its T3 Code session

## Command

```console
cargo nextest run -j 4 --manifest-path next/Cargo.toml -p atelier-desk
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 3616
SHA-256: a2dcb3f4e1d67d89b22fd0868ed14fc4f71a7abc1f77e1a7e8b74a27a97ecf27
Test summary: Summary [   1.012s] 32 tests run: 32 passed, 0 skipped
