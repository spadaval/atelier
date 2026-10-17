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
status: "in_progress"
title: "Show a mission's success criteria and their proof in the mission dashboard"
updated_at: "2026-10-17T18:15:35.976980393+00:00"
---

## Goal

`work mission` shows the strategy path but not its success criteria or which are proven, so the dashboard cannot answer "how close is this mission?". The done rebuild mission showed "scoped issues without evidence 5" with no criterion context.

## Done when

`work mission <id>` lists each success criterion from the linked strategy (or the mission's Success criteria section) with a proven, unproven, or failing mark derived from linked evidence, and names unproven criteria. Concrete proof is fixed when this leaves backlog.

## Unknowns

How evidence or issues name the criterion they support (a `criteria` field, an `S3`-style tag in the summary, or a link); how criteria are parsed from free-form strategy Markdown. Needs a decision before planning.

## Notes

- 2026-10-17 [note] 2026-10-17: atelier-et6q decided. Criteria are '- <slug>: <text>' bullets (S<n> still accepted) from the strategy or the mission's Success criteria section; receipts carry --criterion <slug>=<result>; check refuses unknown criteria. This item and debrief (S4) share one parser.
- 2026-10-17 [plan] 2026-10-17: joined atelier-ho7o as its child, so the proof lookup and the dashboard marks that exercise it are built and reviewed together on feature/atelier-ho7o.
- 2026-10-17 [decision] Show each criterion's first sentence in work mission
  Reason: Strategy criteria run to a paragraph with their proof plan; the first sentence names the claim and keeps one line per item
  Rejected: the full text, wrapped
