---
created_at: "2026-06-15T21:32:01.323862375+00:00"
id: "atelier-rdyl"
issue_type: "task"
labels:
- "migration"
- "mission"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-9p3t"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-16T20:39:18.572700459+00:00"
status: "done"
title: "Migrate mission terminal notes section away from Closeout Notes"
updated_at: "2026-06-16T20:39:18.572700459+00:00"
---

## Description

Migrate the persisted mission notes section that currently renders as Closeout Notes to the lifecycle-neutral section chosen by the product contract.

## Outcome

- Mission Markdown no longer renders or requires `## Closeout Notes` for newly closed missions.
- Existing committed missions with `## Closeout Notes` are migrated or explicitly handled by a hard state migration with no compatibility alias unless the product contract requires one.
- Mission close or complete records the reason in the new terminal notes section.
- Export, rebuild, parse, render, and lint behavior agree on the new section name.

## Evidence

- Command transcript from `rg -n "## Closeout Notes|closeout_notes|CloseoutNotes" crates/atelier-records/src .atelier/missions` returns no live matches after migration.
- Focused record-store tests cover parsing/rendering the new section and closed mission round trips.
- `atelier lint` and `atelier export --check` pass.
