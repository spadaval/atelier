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
    id: "atelier-mlwx"
    type: "blocked_by"
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
title: "Rebuild the site on the new records, in the new workspace"
updated_at: "2026-11-10T04:45:25.381737978+00:00"
---

## Goal

Keep the dashboard working across the layout change by rebuilding the site on the new records, in the new workspace, before the old one is deleted.

## Done when

- [ ] Provisional: the site's loader reads the three record kinds, its responses drop what the new model lacks, and its fixtures are regenerated in the new layout. The site crate and its web app are copied into the new workspace; the new binary's `serve` serves it, and the web typecheck, unit and Playwright jobs run against it and pass. The old workspace's site is not changed.

## Context

Preliminary. This step is planned in full at its boundary, with one planning review, as `docs/strategies/redesign-3-cutover.md` says. Target design: `docs/strategies/redesign-product.md` and `docs/strategies/redesign-architecture.md`, Site.

## Unknowns

- How much of the brief and debrief view code survives; settled by reading the site's code when this step is planned.
