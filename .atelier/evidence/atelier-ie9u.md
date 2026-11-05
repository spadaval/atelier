---
created_at: "2026-11-05T00:41:01.279120491+00:00"
id: "atelier-ie9u"
evidence_type: "test"
commit: "37a116ae12ab443b4e69dfcd63e24c214cc7217f"
result: "pass"
captured_at: "2026-11-05T00:40:57.114209632+00:00"
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
title: "A debrief from a work branch that merged the base matches the base's start, deviations and counts"
updated_at: "2026-11-05T00:41:01.279448543+00:00"
---

## Summary

A debrief from a work branch that merged the base matches the base's start, deviations and counts

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
SHA-256: 8840984c6028b95cbb85690c89076743e28cde19756ad41528d06b9ffcfd7d03
Test summary: Summary [   1.579s] 1 test run: 1 passed, 742 skipped
