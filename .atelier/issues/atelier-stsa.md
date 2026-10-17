---
created_at: "2026-10-16T23:04:58.690777933+00:00"
id: "atelier-stsa"
issue_type: "feature"
labels:
- "orientation"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-r6bt"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Show a mission's success criteria and their proof in the mission dashboard"
updated_at: "2026-10-17T15:57:12.202281380+00:00"
---

## Goal

`work mission` shows the strategy path but not its success criteria or which are proven, so the dashboard cannot answer "how close is this mission?". The done rebuild mission showed "scoped issues without evidence 5" with no criterion context.

## Done when

`work mission <id>` lists each success criterion from the linked strategy (or the mission's Success criteria section) with a proven, unproven, or failing mark derived from linked evidence, and names unproven criteria. Concrete proof is fixed when this leaves backlog.

## Unknowns

How evidence or issues name the criterion they support (a `criteria` field, an `S3`-style tag in the summary, or a link); how criteria are parsed from free-form strategy Markdown. Needs a decision before planning.

## Notes

- 2026-10-17 [note] 2026-10-17: atelier-et6q decided. Criteria are '- <slug>: <text>' bullets (S<n> still accepted) from the strategy or the mission's Success criteria section; receipts carry --criterion <slug>=<result>; check refuses unknown criteria. This item and debrief (S4) share one parser.
