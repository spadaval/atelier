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
updated_at: "2026-11-10T14:16:23.996269943+00:00"
---

## Outcome

One report gives each scorecard measure for the five Atelier repositories on this host, from 2026-10-14 to the moment its inputs were frozen, with its method, its limits and a proposed target for each cost measure. The scripts that produced it are in `scripts/scorecard/`, and a fresh agent can rerun them. Later missions are judged against this report.

## Background

One of four missions that redesign Atelier, in order: baseline, core loop, cutover, gardener. Sumeet set the direction in conversation on 2026-11-09, after an assessment of the project. The target design is in the [product specification](docs/strategies/redesign-product.md) and the [architecture specification](docs/strategies/redesign-architecture.md); this mission's own design is in its strategy. Other Atelier work is paused while the four run.

## Notes

- 2026-11-10 [note] 2026-11-10: Orchestration started on Sumeet's go-ahead (review atelier-b2t8). atelier-scx2 (freeze the inputs) goes first and alone; z5bp, 5fsj and 3kr7 follow in parallel once it merges. Implementers are Opus 5.5, reviewers GPT-6.1 Sol.
- 2026-11-10 [note] 2026-11-10: atelier-scx2 merged (pull request #92, merge 649aafc8, one review round by Sol, no findings). The inputs are frozen at cutoff 2026-11-10T05:14:58Z in ~/.local/share/atelier-baseline/2026-11/: T3's database (4.13 GB, integrity_check ok), 4,053 Claude transcript files from 31 directories, and 143 Codex sessions. Codex kept only 2026-11-08 to 2026-11-10, so Codex token coverage will be three days, not the window. A wake-up from the implementer was lost for about nine hours; from here children are waited on inside the turn. atelier-z5bp, atelier-5fsj and atelier-3kr7 start now, in parallel.
