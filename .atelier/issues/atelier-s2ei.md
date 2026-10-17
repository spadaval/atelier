---
created_at: "2026-10-17T19:00:36.351224229+00:00"
id: "atelier-s2ei"
issue_type: "feature"
labels:
- "merge"
- "retro-2026-10-17"
priority: "P1"
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
title: "Add atelier merge --sync to bring the base in, re-prove and report approval survival"
updated_at: "2026-10-17T19:00:36.351224229+00:00"
---

## Goal

Orchestrators hand-built the same macro 44+ times: commit records, `git merge master`, rerun both evidence suites, record evidence, `atelier merge`, `land`, push only on success. One command should do the catch-up step and say whether the approval survived (clean merge) or which conflict-resolved files need re-review.

## Done when

`atelier merge <id> --sync` merges the base into the owner branch, reruns the recorded evidence commands, records their receipts, and either reports the approval as kept (clean merge) or lists the conflict-resolved paths that need a fresh approval, ending in a verdict. Tests cover a clean merge, a conflicted merge, and a failing rerun.

## Context

Found in the 2026-10-17 retrospective of the 10-15..10-17 atelier runs (sessions b32c39e6, 9a04974d, d483b55c, bf433168). o3lm: 78 `git merge master` invocations, 52 merge commits, 25 suite reruns labelled 'after merging master'.
