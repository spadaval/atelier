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
    id: "atelier-nafc"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-yptz"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Convert the site to the new records in place"
updated_at: "2026-11-10T01:14:41.491387566+00:00"
---

## Goal

Keep the dashboard working across the layout change by converting the site in place, last, so the time a reinstall could break the old dashboard is short.

## Done when

- [ ] Provisional: the site's loader reads the three record kinds, its responses drop what the new model lacks, its fixtures are regenerated in the new layout, and the web typecheck, unit and Playwright jobs pass in the change that does it. `AGENTS.md` says not to run `cargo install` from the tree until the cutover.

## Context

Preliminary. This step is planned in full at its boundary, with one planning review, as `docs/strategies/redesign-2-core.md` says. Target design: `docs/strategies/redesign-product.md` and `docs/strategies/redesign-architecture.md`.

## Unknowns

- How much of the brief and debrief view code survives; settled by reading the site's code when this step is planned.
