---
created_at: "2026-11-04T23:28:28.402420720+00:00"
id: "atelier-6zt2"
evidence_type: "test"
commit: "7b8fb1511fc10a4d6d4725d2426d15509c2f1c16"
result: "pass"
captured_at: "2026-11-04T23:28:09.060854452+00:00"
command: "cargo nextest run -p atelier-cli a_child_closed_on_its_parents_branch"
exit_status: "0"
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
title: "A child closed on its parent's branch and merged later is added no later than it is delivered"
updated_at: "2026-11-04T23:28:28.402706503+00:00"
---

## Summary

A child closed on its parent's branch and merged later is added no later than it is delivered

## Command

```console
cargo nextest run -p atelier-cli a_child_closed_on_its_parents_branch
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 640
SHA-256: 3a18664e6e9c5f55c088c905cd2a43872728f34fb6b17a4784a347c2e5874605
Test summary: Summary [   1.292s] 1 test run: 1 passed, 740 skipped
