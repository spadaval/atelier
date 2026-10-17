---
created_at: "2026-10-17T17:25:59.520021809+00:00"
id: "atelier-acrt"
issue_type: "feature"
labels:
- "review"
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
status: "backlog"
title: "Review a mission before work starts with atelier brief"
updated_at: "2026-10-17T18:53:19.646876456+00:00"
---

## Goal

S5 of docs/strategies/review-and-introspection.md: `atelier brief <mission>` reviews a mission before work starts, in the same two forms as `debrief`, so the human can approve or change the plan from one page. A plan approval is an ordinary review event on the mission (`atelier review approve <mission>`), shown in both the brief and the debrief.

## Done when

`atelier brief <mission>` prints dense Markdown, and with `--html` writes a self-contained page under `.atelier/runtime/reviews/`, with: the strategy outcome; each success criterion; the scope with sizes (size labels) and statuses; blockers; unknowns (each scoped item's Unknowns section and the strategy's Risks and Unknowns section); open human decisions (open decision records and needs-human items in scope); risks; and review events on the mission, including plan approvals. A fixture test checks both forms and that their facts match. Docs describe it.

## Design

Reuses atelier-rx22's view-model module, criteria parsing and HTML renderer.
