---
created_at: "2026-10-17T20:10:21.556378400+00:00"
id: "atelier-juyb"
issue_type: "task"
labels:
- "review"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-r6bt"
    type: "advances"
  - kind: "issue"
    id: "atelier-r6bt"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Refuse misspelled field lines in decision notes"
updated_at: "2026-10-17T20:10:21.556378400+00:00"
---

## Goal

A decision note line such as `Rejectd: x` passes `check` and is folded into the reason text (independent validation of atelier-r6bt, S1).

## Done when

`check` refuses an indented line in a decision note that looks like a misspelled `Reason:` or `Rejected:` label, or the docs say why it is allowed, shown by a CLI test.
