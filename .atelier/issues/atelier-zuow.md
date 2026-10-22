---
created_at: "2026-10-22T19:58:07.838889696+00:00"
id: "atelier-zuow"
issue_type: "feature"
labels:
- "evidence"
- "review"
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-zuow
priority: "P2"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-4ixu"
  - kind: "issue"
    id: "atelier-rc8p"
  - kind: "issue"
    id: "atelier-tq1j"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Record and request review with less hand work"
updated_at: "2026-10-22T20:32:26.676982290+00:00"
---

## Goal

A step's records land in one commit, and `atelier review request` writes the reviewer brief. A follow-up records where it was found with a `discovered_from` link instead of prose.

## Done when

- Each child's Done when holds, shown by CLI tests, with independent review.
- Delivers atelier-k8h8 S9's record-commit part (atelier-4ixu) and S8's tooling part (atelier-rc8p, atelier-tq1j).

## Background

The children change the review, evidence and issue-create commands and the debrief, none of which the other first-wave increments touch. Mission atelier-rc8j later reworks how these commands report; keep output changes to what each child needs.
