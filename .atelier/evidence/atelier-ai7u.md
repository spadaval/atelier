---
created_at: "2026-11-07T02:29:20.321877477+00:00"
id: "atelier-ai7u"
evidence_type: "test"
commit: "58cde948187fa433dfd559ecc664c6fb1666808a"
result: "pass"
captured_at: "2026-11-07T02:29:17.537687251+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --test proof_timing"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-n4mj"
  role: "validates"
criteria:
- slug: "proof-timing"
  result: "pass"
criteria_mission: "atelier-n4mj"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-n4mj"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo nextest run -j 4 -p atelier-cli --test proof_timing"
updated_at: "2026-11-07T02:29:20.322595827+00:00"
---

## Summary

cargo nextest run -j 4 -p atelier-cli --test proof_timing

## Command

```console
cargo nextest run -j 4 -p atelier-cli --test proof_timing
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 680
SHA-256: 0b24e5e4be2b9868523d8f940ed7b1d98bb7c32a65bda75ebb2d846530b628cf
Test summary: Summary [   0.773s] 2 tests run: 2 passed, 0 skipped
