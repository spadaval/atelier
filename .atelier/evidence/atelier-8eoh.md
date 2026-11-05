---
created_at: "2026-11-05T01:46:02.869194013+00:00"
id: "atelier-8eoh"
evidence_type: "test"
commit: "bdf20b691e3194093bc91ed14ea462e99e1707b9"
result: "pass"
captured_at: "2026-11-05T01:45:57.792190360+00:00"
command: "cargo nextest run -p atelier-cli -E 'test(/a_child_closed_on_/)'"
exit_status: "0"
agent_identity: "atelier merge --sync"
target:
  kind: "issue"
  id: "atelier-b3qt"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-b3qt"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "A child closed on its parent's branch is added before its delivery, whether the branch merges into the base or merges the base in first (rerun after merging master)"
updated_at: "2026-11-05T01:46:02.869682424+00:00"
---

## Summary

A child closed on its parent's branch is added before its delivery, whether the branch merges into the base or merges the base in first (rerun after merging master)

## Command

```console
cargo nextest run -p atelier-cli -E 'test(/a_child_closed_on_/)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 719
SHA-256: 01f8c192c9c12a5cdd1b848b5759c160b334ed324c51e5056ac067fb939dabc7
Test summary: Summary [   1.430s] 2 tests run: 2 passed, 749 skipped
