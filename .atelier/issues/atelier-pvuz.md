---
created_at: "2026-06-12T20:29:52.396700675+00:00"
id: "atelier-pvuz"
issue_type: "task"
labels:
- "cli"
- "closeout"
- "evidence"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-a4sn"
  - kind: "issue"
    id: "atelier-pyre"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Expose closeout proof requirements through domain commands"
updated_at: "2026-06-12T20:29:52.396700675+00:00"
---

## Description

Expose closeout proof requirements through domain commands instead of requiring
operators to understand raw workflow validators. This is the user-facing bridge
between reliability policy and closeout behavior.

## Outcome

- Mission status and mission closeout output name missing proof, open work,
  malformed linked issues, missing evidence, stale tracker state, and dirty
  worktree state in user-facing language.
- Issue closeout output names missing issue-level proof and points to the
  appropriate evidence or note workflow.
- Domain next actions tell operators how to record or attach proof.
- Raw validator names may appear only as advanced diagnostic detail, not as the
  primary answer.
- Output stays bounded when many linked issues or evidence gaps exist.

## Evidence

- CLI transcripts for issue closeout missing proof, mission closeout missing
  proof, malformed linked issue, stale tracker state, and dirty worktree state.
- Tests proving status and closeout surfaces agree on blocker classes.
- Docs/help parity check for closeout proof commands.
