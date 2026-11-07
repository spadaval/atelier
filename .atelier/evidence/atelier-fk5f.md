---
created_at: "2026-11-07T03:43:36.326764785+00:00"
id: "atelier-fk5f"
evidence_type: "test"
commit: "2206fb88355cf43d3ac399cd9858ea9bfbcb9beb"
result: "pass"
captured_at: "2026-11-07T03:43:33.367325336+00:00"
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
updated_at: "2026-11-07T03:43:36.327474528+00:00"
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

Bytes: 809
SHA-256: bfecd76477e11fcc399e85952500f016798d3e85e452323f14e4e5875aba76fe
Test summary: Summary [   0.817s] 3 tests run: 3 passed, 0 skipped
