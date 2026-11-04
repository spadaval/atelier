---
created_at: "2026-11-04T23:24:54.158283092+00:00"
id: "atelier-crlh"
evidence_type: "test"
commit: "4d8556a432a0fdbbba7006923554be0632c62c48"
result: "pass"
captured_at: "2026-11-04T23:24:50.851909418+00:00"
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
title: "A debrief from a work branch that merged the base reports the base's start, deviations and counts"
updated_at: "2026-11-04T23:24:54.158557153+00:00"
---

## Summary

A debrief from a work branch that merged the base reports the base's start, deviations and counts

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
SHA-256: f7eb498e60711f4ca57c7884d7f704d6bc4d9b8af21d87006a264ce1d3a8efda
Test summary: Summary [   1.387s] 1 test run: 1 passed, 739 skipped
