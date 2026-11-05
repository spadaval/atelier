---
created_at: "2026-11-05T01:46:08.593188966+00:00"
id: "atelier-ma6l"
evidence_type: "test"
commit: "bdf20b691e3194093bc91ed14ea462e99e1707b9"
result: "pass"
captured_at: "2026-11-05T01:46:03.092447034+00:00"
command: "cargo nextest run -p atelier-cli a_debrief_from_a_branch_that_merged_the_base"
exit_status: "0"
agent_identity: "atelier merge --sync"
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
title: "A debrief from a work branch that merged the base matches the base's start, deviations and counts (rerun after merging master)"
updated_at: "2026-11-05T01:46:08.593922244+00:00"
---

## Summary

A debrief from a work branch that merged the base matches the base's start, deviations and counts (rerun after merging master)

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
SHA-256: 9932c681bd223012040e9973bc33d56d2e971d5eb10a23b8ea5e21538497511b
Test summary: Summary [   1.729s] 1 test run: 1 passed, 750 skipped
