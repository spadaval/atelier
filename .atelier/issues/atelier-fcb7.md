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
updated_at: "2026-11-10T00:56:04.348189885+00:00"
---

## Outcome

A new `atelier` binary on master implements the product specification's records and commands, and is installed nowhere. A fresh agent told only to run `atelier status` takes a ready item to done with an independent reviewer. A converted scratch clone of each of the five repositories passes `check` and completes one item. The old commands, their code and their tests are gone from the tree.

## Background

One of four missions that redesign Atelier, in order: baseline, core loop, cutover, gardener. Sumeet set the direction in conversation on 2026-11-09, after an assessment of the project. The target design is in the [product specification](docs/strategies/redesign-product.md) and the [architecture specification](docs/strategies/redesign-architecture.md); this mission's own design is in its strategy. Other Atelier work is paused while the four run.
