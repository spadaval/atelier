---
created_at: "2026-11-07T02:15:12.216315201+00:00"
id: "atelier-at5c"
evidence_type: "test"
commit: "902a5a6352d1e5510df1f9b1669124741dd8fea3"
result: "pass"
captured_at: "2026-11-07T02:15:09.241772207+00:00"
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
updated_at: "2026-11-07T02:15:12.217554260+00:00"
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
SHA-256: c5acd8cbf00f321288715d628c49dc2b7cd24f2fc0cdff81be9ed455233f267b
Test summary: Summary [   0.749s] 2 tests run: 2 passed, 0 skipped
