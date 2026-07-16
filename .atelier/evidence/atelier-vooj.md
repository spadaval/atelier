---
created_at: "2026-07-16T22:32:31.278133918+00:00"
id: "atelier-vooj"
evidence_type: "validation"
captured_at: "2026-07-16T22:32:24.574817099+00:00"
command: "target/debug/atelier man reviewer"
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
title: "target/debug/atelier man reviewer"
updated_at: "2026-07-16T22:32:31.279936441+00:00"
---

## Summary

target/debug/atelier man reviewer

## Command

```console
target/debug/atelier man reviewer
```

Exit status: 0

## Stdout

Bytes: 1227
Truncated: no

```text
Atelier Man: Reviewer
=====================

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
  1. atelier issue show <mission-id> - Inspect the exact graph and review diagnosis.
  2. atelier issue plan-review <mission-id> finding <finding-id> --affected <issue-id> - Record a graph finding.
  3. atelier issue plan-review <mission-id> approve - Approve as an independent reviewer.

Normal Loop
-----------
  atelier issue show <mission-id>
  atelier issue plan-review <mission-id> finding <finding-id> --affected <issue-id>
  atelier issue plan-review <mission-id> change-request <request-id> --affected <issue-id>
  atelier issue plan-review <mission-id> approve
  atelier history --issue <mission-id>

Not Usually For This Role
-------------------------
  authoring or materially editing the graph being approved
```

## Stderr

Bytes: 0
Truncated: no

```text
```
