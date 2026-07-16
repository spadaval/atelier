---
created_at: "2026-07-16T22:32:38.301410617+00:00"
id: "atelier-iw6l"
evidence_type: "validation"
captured_at: "2026-07-16T22:32:31.564212729+00:00"
command: "target/debug/atelier man manager"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-72k4"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-72k4"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "target/debug/atelier man manager"
updated_at: "2026-07-16T22:32:38.303236070+00:00"
---

## Summary

target/debug/atelier man manager

## Command

```console
target/debug/atelier man manager
```

Exit status: 0

## Stdout

Bytes: 1333
Truncated: no

```text
Atelier Man: Manager
====================

Current State
-------------
  Repository: /root/atelier-worktrees/atelier-p2wk
  Tracker:    current
  Current work:   3 issue(s)
    atelier-72k4 - Explain mission review and dependency readiness failures [worker]
    atelier-p2wk - Epic: Enforce reviewed and dependency-safe mission readiness [worker]
    atelier-p4z2 - Mission: Require independent review before mission execution [worker]
  Ready work:     5

Most Relevant Commands
----------------------
  1. atelier work missions - Compare mission scope and readiness.
  2. atelier work mission <mission-id> - Inspect dependency-safe work.
  3. atelier issue show <mission-id> - Follow the review diagnosis.

Normal Loop
-----------
  atelier work missions
  atelier work mission <mission-id>
  atelier man work-model
  atelier work ready
  atelier work blocked
  atelier issue show <objective-id>
  atelier bundle preview <file>
  atelier bundle apply <file> --yes
  atelier issue create "..."
  atelier issue link <mission-id> <issue-id> --role advances
  atelier issue link <blocked-id> <blocker-id> --role blocked_by
  atelier status

Not Usually For This Role
-------------------------
  mission-plan approval; planners author and reviewers independently approve
  diagnostics slow, rebuild; shell loops for bulk graph creation
```

## Stderr

Bytes: 0
Truncated: no

```text
```
