---
created_at: "2026-07-06T18:32:26.850930981+00:00"
id: "atelier-sdnv"
evidence_type: "validation"
captured_at: "2026-07-06T18:32:26.691725719+00:00"
command: "target/debug/atelier review show --issue atelier-ye11 --comments"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-ye11"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ye11"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Independent live provider validation: merged PR 47 state and current provider comments are retrievable through collapsed review show --comments."
updated_at: "2026-07-06T18:32:30.756353523+00:00"
---

## Summary

Independent live provider validation: merged PR 47 state and current provider comments are retrievable through collapsed review show --comments.

## Command

```console
target/debug/atelier review show --issue atelier-ye11 --comments
```

Exit status: 0

## Stdout

Bytes: 424
Truncated: no

```text
Review
======
Authority: configured provider review
State:     closed
Issue:     atelier-ye11
URL:       http://iiot-grand-central.cisco.com:3000/supadava/atelier/pulls/47
Number:    47
Merged:    true
Review Comments
===============
comment 230 - [P1] Collapsed provider approval is not executable: crates/atelier-app/src/fo...
comment 241 - Addressed REQUEST_CHANGES in e6ab7a7d: Forgejo comments now use list-reviews ...
```

## Stderr

Bytes: 0
Truncated: no

```text
```
