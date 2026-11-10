---
created_at: "2026-11-10T00:56:08.595125805+00:00"
id: "atelier-oci2"
issue_type: "task"
labels:
- "redesign"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-fcb7"
    type: "advances"
  - kind: "issue"
    id: "atelier-yptz"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Have the site read the new records, and show what changed from the plan"
updated_at: "2026-11-10T00:56:08.595125805+00:00"
---

## Goal

Keep the site working across the layout change, and give the result view its comparison against the commit a mission was made ready at.

## Done when

- [ ] Provisional: the site's API tests and Playwright suite pass against fixture repositories in the new layout, and a mission's result page lists items added, dropped or redefined since its ready commit.

## Context

Preliminary. This step is planned in full at its boundary, with one planning review, as `docs/strategies/redesign-2-core.md` says. Target design: `docs/strategies/redesign-product.md` and `docs/strategies/redesign-architecture.md`.

## Unknowns

- How much of the brief and debrief view code the new records make unnecessary.
