---
created_at: "2026-10-17T19:00:36.347242257+00:00"
id: "atelier-u6f3"
issue_type: "task"
labels:
- "retro-2026-10-17"
- "review"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Batch low-priority review follow-ups into one branch and one review"
updated_at: "2026-10-22T19:58:17.870009528+00:00"
---

## Goal

o3lm pulled 10 deferred review follow-ups back into the mission and ran each through the full lifecycle: about 370-640 record lines each for 30-180 source lines (2hh8, jx7g, ytxg, ...). That doubled the mission's scope for low-priority polish.

## Done when

The Agent Factory orchestrate guidance says to batch low-priority deferred follow-ups into one owner branch (a parent with children) reviewed once, and atelier supports closing several children on one branch with one approval. Shown by the skill text and a test of one approval covering the children.

## Context

Found in the 2026-10-17 retrospective of the 10-15..10-17 atelier runs (sessions b32c39e6, 9a04974d, d483b55c, bf433168).
