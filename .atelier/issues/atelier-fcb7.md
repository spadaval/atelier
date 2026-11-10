---
created_at: "2026-11-10T00:56:04.348189885+00:00"
id: "atelier-fcb7"
issue_type: "mission"
labels:
- "redesign"
strategy:
  path: docs/strategies/redesign-2-core.md
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "draft"
title: "Rebuild the work loop as 17 commands that tell each thread what it holds and what comes next"
updated_at: "2026-11-10T02:41:18.683598220+00:00"
---

## Outcome

A second binary on master, `atelier-next`, implements the product specification's records and commands. It is installed nowhere, and the old binary beside it is untouched. A fresh agent told only to run `atelier status` takes a ready item to done with an independent reviewer. A converted scratch clone of each of the five repositories passes `check`, and one item completes in two of them.

## Background

One of four missions that redesign Atelier, in order: baseline, core loop, cutover, gardener. Sumeet set the direction in conversation on 2026-11-09, after an assessment of the project. The target design is in the [product specification](docs/strategies/redesign-product.md) and the [architecture specification](docs/strategies/redesign-architecture.md); this mission's own design is in its strategy. Other Atelier work is paused while the four run.

## Notes

- 2026-11-09 [note] Amended on Sumeet's direction (2026-11-09, in conversation), before any mission was made ready: Git hooks guard Git's actions and `land` goes; every rule can be skipped on purpose with ATELIER_SKIP and a reason; `merge` loses its flags and never waits, since the harness waits; `show` and `list` are spelled out so a lead or planner loses no view; his go-ahead stays `ready --by`. The command count is 17.
