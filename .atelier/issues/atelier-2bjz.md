---
created_at: "2026-11-04T23:12:48.381558151+00:00"
id: "atelier-2bjz"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: task/atelier-2bjz
priority: "P3"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-juyb"
  - kind: "issue"
    id: "atelier-s5x4"
  - kind: "issue"
    id: "atelier-w5hx"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-r6bt"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Make decision records the one way to record a costly choice"
updated_at: "2026-11-04T23:18:13.478619588+00:00"
---

## Goal

Carry out atelier-6306 (atelier-w5hx), with the two decision fixes that touch the same code: decided() without a workflow policy (atelier-s5x4) and refusing misspelled decision-note labels (atelier-juyb). One branch and one review for the three, in the order s5x4, w5hx, juyb, because they edit `decisions.rs` and `DecisionNote::parse` in turn.

## Done when

Every child is terminal with its own Done when met and proven, and one independent review of the branch is approved here.
