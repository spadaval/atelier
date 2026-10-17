---
created_at: "2026-10-17T17:25:59.393637922+00:00"
id: "atelier-rx22"
issue_type: "feature"
labels:
- "review"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-fip8"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-ho7o"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-r6bt"
    type: "advances"
  - kind: "issue"
    id: "atelier-sfsq"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Review a finished mission with atelier debrief"
updated_at: "2026-10-17T17:25:59.393637922+00:00"
---

## Goal

S4 of docs/strategies/review-and-introspection.md: `atelier debrief <mission>` reviews a finished mission from its records and Git history, as dense Markdown for agents and a self-contained HTML page for humans, both from one view model.

## Done when

The S4 criterion holds: outcome against each criterion with proof, each item's report, the decision log (one-way first), derived deviations, and follow-ups. Fixture tests of both forms (including a test that their facts match), and a run against a real completed mission.

## Unknowns

How cheaply Git history answers mission start, reopen events and scope added after start (one record-history query that scales with the mission's records; see atelier-83xi and atelier-7b6e). Whether a large debrief needs a summary-first layout.
