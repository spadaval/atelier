---
created_at: "2026-11-11T02:17:26.233459064+00:00"
id: "atelier-94pv"
issue_type: "task"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-uwxy"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-z4cr"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Move the spike's app to desk/ on the new API: the mission page with its brief and debrief, items and the inbox"
updated_at: "2026-11-11T02:17:26.233459064+00:00"
---

## Goal

Preliminary. Carry the spike's mission-first pages into `desk/` at the repository root, reading `atelier-desk`'s API, with unit and browser tests on fixtures.

## Done when

- [ ] Provisional: `desk/` builds, typechecks and passes its unit and Playwright tests in CI; the mission page shows what it is for, its documents, its items by status and its history at desktop and phone widths; `atelier-desk` carries the built app.

## Unknowns

- Which spike pages are carried over; settled by atelier-1ie1.
