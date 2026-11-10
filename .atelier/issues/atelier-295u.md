---
created_at: "2026-11-10T00:56:04.888418157+00:00"
id: "atelier-295u"
issue_type: "mission"
labels:
- "redesign"
strategy:
  path: docs/strategies/redesign-4-gardener.md
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "draft"
title: "Give the gardener `atelier log` and a readiness checklist, so a run starts from facts"
updated_at: "2026-11-10T03:58:05.871591742+00:00"
---

## Outcome

`atelier log` prints the history of acts. A gardener run starts from that, from `atelier list` and from a readiness checklist for the repository, and reads transcripts only to explain a number that moved. It ends with one page for Sumeet: the numbers, the worst spots and at most three proposals, each a root cause with the trips that show it. One run has been made in each of the five repositories.

## Background

One of four missions that redesign Atelier, in order: baseline, core loop, cutover, gardener. Sumeet set the direction in conversation on 2026-11-09, after an assessment of the project. The target design is in the [product specification](docs/strategies/redesign-product.md) and the [architecture specification](docs/strategies/redesign-architecture.md); this mission's own design is in its strategy. Other Atelier work is paused while the four run.

## Notes

- 2026-11-09 [note] Amended on Sumeet's direction (2026-11-09, in conversation), before any mission was made ready: the gardener is named as a role; the mission builds one read command, `atelier log`, the history of acts, and the scorecard's arithmetic lives in the gardener's procedure, not in a `scorecard` command; a bug item names what caused it; a deferred finding is settled with `new --from` or `drop`. The command count is 18.
