---
created_at: "2026-07-16T22:32:24.303054457+00:00"
id: "atelier-pt92"
evidence_type: "validation"
captured_at: "2026-07-16T22:32:17.708262392+00:00"
command: "target/debug/atelier man planner"
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
  - kind: "issue"
    id: "atelier-qi40"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "target/debug/atelier man planner"
updated_at: "2026-07-16T23:52:03.466099812+00:00"
---

## Summary

target/debug/atelier man planner

## Command

```console
target/debug/atelier man planner
```

Exit status: 0

## Stdout

Bytes: 1126
Truncated: no

```text
Atelier Man: Planner
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
  1. atelier work mission <mission-id> - Inspect the authored mission graph.
  2. atelier issue plan-review <mission-id> request - Hand the exact graph to review.
  3. atelier issue show <mission-id> - Follow the current review diagnosis.

Normal Loop
-----------
  atelier work mission <mission-id>
  atelier issue plan-review <mission-id> request
  atelier issue plan-review <mission-id> rework
  atelier issue plan-review <mission-id> resolve <decision-id> --disposition "..."
  atelier issue show <mission-id>

Not Usually For This Role
-------------------------
  approval of a graph you authored or materially edited
```

## Stderr

Bytes: 0
Truncated: no

```text
```
