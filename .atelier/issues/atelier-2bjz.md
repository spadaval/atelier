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
closed_at: "2026-11-05T00:32:27.776753591+00:00"
status: "done"
title: "Make decision records the one way to record a costly choice"
updated_at: "2026-11-05T00:32:27.776753591+00:00"
---

## Goal

Carry out atelier-6306 (atelier-w5hx), with the two decision fixes that touch the same code: decided() without a workflow policy (atelier-s5x4) and refusing misspelled decision-note labels (atelier-juyb). One branch and one review for the three, in the order s5x4, w5hx, juyb, because they edit `decisions.rs` and `DecisionNote::parse` in turn.

## Done when

Every child is terminal with its own Done when met and proven, and one independent review of the branch is approved here.

## Close Reason

Decision records are now the one way to record a costly choice: one-way decision notes are gone and the four existing ones migrated (atelier-w5hx), a `done` decision record counts as decided without a workflow policy (atelier-s5x4), and `check` refuses misspelled `Reason:`/`Rejected:` labels in decision notes (atelier-juyb); each child carries its own report and receipts.

### Deviations
The brief's overflow line follows human-cli-output.md's omitted-rows form rather than a bare `+N more` (w5hx decision note).

### Decisions
On the children (`atelier work decisions atelier-r6bt`); the label rule was narrowed after review atelier-nu4k to same-word or one-edit near-misses sharing the label's first letter.

### Surprises
The first label rule (two edits) refused common words such as Person: and Selected:.

### Open items
None.
