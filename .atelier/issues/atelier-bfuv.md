---
created_at: "2026-06-13T02:33:42.569206086+00:00"
id: "atelier-bfuv"
issue_type: "epic"
labels:
- "evidence"
- "schema"
- "validation"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-0vjq"
  - kind: "issue"
    id: "atelier-dv3d"
  - kind: "issue"
    id: "atelier-h2tq"
  - kind: "issue"
    id: "atelier-n9up"
  - kind: "issue"
    id: "atelier-rzsg"
  - kind: "issue"
    id: "atelier-s8z0"
  - kind: "issue"
    id: "atelier-xmss"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-13T16:09:11.601797088+00:00"
status: "done"
title: "Redesign evidence capture and proof coverage"
updated_at: "2026-06-13T16:09:11.601797088+00:00"
---

## Description

Redesign evidence so proof is attached to accountable work and easy to inspect without private chat context. The model should avoid turning missions into pseudo-work items and should not require line-level claim plumbing for ordinary work.
- Evidence records identify accountable targets, proof kind, result classification, commands or artifacts, agent identity, independence level, residual risks, and follow-up IDs.
- Evidence recording uses one normal workflow for manual summaries, shell commands, tests, audits, and transcripts.
- Evidence rendering defaults to compact summaries with drill-down for full details.
- Parent closeout can verify whether required proof coverage exists through linked work and validation records, not just whether any evidence is attached.
- Evidence schema or record contract is documented before implementation.
- Focused tests prove evidence creation, attachment to accountable work, rendering, and closeout lookup.
- Command transcripts prove agents can record a test run or CLI transcript without manually writing a huge summary or choosing between duplicate evidence verbs.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
