---
created_at: "2026-11-10T00:56:03.941735385+00:00"
id: "atelier-jah8"
issue_type: "mission"
labels:
- "redesign"
strategy:
  path: docs/strategies/redesign-1-baseline.md
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Measure defects after merge and what process costs, before the redesign changes either"
updated_at: "2026-11-10T05:08:26.992715087+00:00"
---

## Outcome

One report gives each scorecard measure for the five Atelier repositories on this host, from 2026-10-14 to the moment its inputs were frozen, with its method, its limits and a proposed target for each cost measure. The scripts that produced it are in `scripts/scorecard/`, and a fresh agent can rerun them. Later missions are judged against this report.

## Background

One of four missions that redesign Atelier, in order: baseline, core loop, cutover, gardener. Sumeet set the direction in conversation on 2026-11-09, after an assessment of the project. The target design is in the [product specification](docs/strategies/redesign-product.md) and the [architecture specification](docs/strategies/redesign-architecture.md); this mission's own design is in its strategy. Other Atelier work is paused while the four run.

## Notes

- 2026-11-10 [note] 2026-11-10: Orchestration started on Sumeet's go-ahead (review atelier-b2t8). atelier-scx2 (freeze the inputs) goes first and alone; z5bp, 5fsj and 3kr7 follow in parallel once it merges. Implementers are Opus 5.5, reviewers GPT-6.1 Sol.
