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
    id: "atelier-nafc"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-qr5s"
    type: "advances"
  - kind: "issue"
    id: "atelier-yptz"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Convert the site to the new records"
updated_at: "2026-11-10T01:29:40.511481020+00:00"
---

## Goal

Keep the dashboard working across the layout change by converting the site to the new records just before the old commands are deleted.

## Done when

- [ ] Provisional: the site's loader reads the three record kinds, its responses drop what the new model lacks, and its fixtures are regenerated in the new layout. `atelier-next serve` serves it, the web typecheck, unit and Playwright jobs run against it and pass in the change that does this, and `serve` is removed from the old binary in the tree.

## Context

Preliminary. This step is planned in full at its boundary, with one planning review, as `docs/strategies/redesign-3-cutover.md` says. Target design: `docs/strategies/redesign-product.md` and `docs/strategies/redesign-architecture.md`, Site.

## Unknowns

- How much of the brief and debrief view code survives; settled by reading the site's code when this step is planned.
