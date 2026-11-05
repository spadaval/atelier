---
created_at: "2026-11-05T00:20:03.576503033+00:00"
id: "atelier-9zmz"
evidence_type: "test"
commit: "79a5cf61af2568a1b2d7abf6ff276702232a92bd"
result: "pass"
captured_at: "2026-11-05T00:19:58.803876409+00:00"
command: "cargo nextest run -p atelier-cli a_debrief_from_a_branch_that_merged_the_base"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-jnec"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-jnec"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "A debrief from a work branch that touched a scope record before merging the base matches the base's start, deviations and counts"
updated_at: "2026-11-05T00:20:03.578119160+00:00"
---

## Summary

A debrief from a work branch that touched a scope record before merging the base matches the base's start, deviations and counts

## Command

```console
cargo nextest run -p atelier-cli a_debrief_from_a_branch_that_merged_the_base
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 579
SHA-256: 6838dc5e7be7e4744df7a95f1e0d224a9e90d81f248e1311a82bc6eb889c7a62
Test summary: Summary [   1.538s] 1 test run: 1 passed, 741 skipped
