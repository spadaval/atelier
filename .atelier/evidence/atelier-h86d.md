---
created_at: "2026-10-23T15:53:45.627042237+00:00"
id: "atelier-h86d"
evidence_type: "test"
commit: "fd8ad78e1bfc812396803e7a1f2e6a40784cc95a"
result: "pass"
captured_at: "2026-10-23T15:53:43.476530382+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(disposition_of_an_approval_says_it_is_not_a_finding)'"
exit_status: "0"
agent_identity: "atelier merge --sync"
target:
  kind: "issue"
  id: "atelier-jiz0"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-jiz0"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "disposition of an approval ID says it is an approval, not a finding, with atelier review list <owner> (rerun after merging master)"
updated_at: "2026-10-23T15:53:45.627702469+00:00"
---

## Summary

disposition of an approval ID says it is an approval, not a finding, with atelier review list <owner> (rerun after merging master)

## Command

```console
cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(disposition_of_an_approval_says_it_is_not_a_finding)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 556
SHA-256: f4b088f1d74db9fe0bb4642f6ef9d03f698e4f347cea213e0cb1a49b514fb491
Test summary: Summary [   0.137s] 1 test run: 1 passed, 721 skipped
