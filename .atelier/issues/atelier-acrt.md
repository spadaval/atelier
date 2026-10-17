---
created_at: "2026-10-17T17:25:59.520021809+00:00"
id: "atelier-acrt"
issue_type: "feature"
labels:
- "review"
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-acrt
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-r6bt"
    type: "advances"
  - kind: "issue"
    id: "atelier-rx22"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T19:49:35.968262619+00:00"
status: "done"
title: "Review a mission before work starts with atelier brief"
updated_at: "2026-10-17T19:49:35.968262619+00:00"
---

## Goal

S5 of docs/strategies/review-and-introspection.md: `atelier brief <mission>` reviews a mission before work starts, in the same two forms as `debrief`, so the human can approve or change the plan from one page. A plan approval is an ordinary review event on the mission (`atelier review approve <mission>`), shown in both the brief and the debrief.

## Done when

`atelier brief <mission>` prints dense Markdown, and with `--html` writes a self-contained page under `.atelier/runtime/reviews/`, with: the strategy outcome; each success criterion; the scope with sizes (size labels) and statuses; blockers; unknowns (each scoped item's Unknowns section and the strategy's Risks and Unknowns section); open human decisions (open decision records and needs-human items in scope); risks; and review events on the mission, including plan approvals. A fixture test checks both forms and that their facts match. Docs describe it.

## Design

Reuses atelier-rx22's view-model module, criteria parsing and HTML renderer.

## Notes

- 2026-10-17 [decision] Show unknowns only for open items in the brief
  Reason: A done item's unknowns are settled, and the brief is for deciding what work still needs
- 2026-10-17 [decision] List open decision records and needs-human items together as open human decisions
  Reason: Both are calls only the human can make, and the strategy names open human decisions without distinguishing them

## Close Reason

`atelier brief <mission>` reviews a mission's plan as dense Markdown or a self-contained HTML page: outcome, criteria, scope with sizes and blockers, open items' unknowns, open human decisions, risks and plan approvals, which the debrief shows too.

### Deviations
Built on atelier-rx22's branch before it merged, then merged with its review fixes, so the two share the view-model module, page shell and mission-events function.

### Decisions
See this item's decision notes: unknowns only for open items, and decision records with needs-human work as open human decisions.

### Surprises
Review events could already target a mission, so plan approval needed no gate change.

### Open items
None.
