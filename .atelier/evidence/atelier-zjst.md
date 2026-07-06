---
created_at: "2026-07-06T18:16:08.495795775+00:00"
id: "atelier-zjst"
evidence_type: "test"
captured_at: "2026-07-06T18:16:08.336029091+00:00"
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
title: "Live Forgejo PR 47 review show --comments succeeds and renders provider comment 230 after endpoint fix."
updated_at: "2026-07-06T18:16:23.809636821+00:00"
---

## Summary

Live Forgejo PR 47 review show --comments succeeds and renders provider comment 230 after endpoint fix.

## Command

```console
target/debug/atelier review show --issue atelier-ye11 --comments
```

Exit status: 0

## Stdout

Bytes: 328
Truncated: no

```text
Review
======
Authority: configured provider review
State:     open
Issue:     atelier-ye11
URL:       http://iiot-grand-central.cisco.com:3000/supadava/atelier/pulls/47
Number:    47
Merged:    false
Review Comments
===============
comment 230 - [P1] Collapsed provider approval is not executable: crates/atelier-app/src/fo...
```

## Stderr

Bytes: 0
Truncated: no

```text
```

