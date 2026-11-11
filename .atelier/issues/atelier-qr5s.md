---
created_at: "2026-11-10T00:56:04.626625678+00:00"
id: "atelier-qr5s"
issue_type: "mission"
labels:
- "redesign"
strategy:
  path: docs/strategies/redesign-3-cutover.md
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Move all five repositories and Agent Factory onto the new loop, one cutover per host"
updated_at: "2026-11-11T02:46:41.651348543+00:00"
---

## Outcome

On this host, atelier, astromech, demiurge, life-on-rails and cosplay-studio run on the new records and the new binary. Agent Factory is rewritten against the new tool, and each procedure it keeps has an eval scenario it changed. The old workspace is deleted, and the site with it: the new binary has no `serve`, and the Desk (mission atelier-z4cr) replaces it outside this mission. The old product docs and the ADRs are replaced by the design record. Sumeet has a runbook to convert his work host.

## Background

One of four missions that redesign Atelier, in order: baseline, core loop, cutover, gardener. Sumeet set the direction in conversation on 2026-11-09, after an assessment of the project. The target design is in the [product specification](docs/strategies/redesign-product.md) and the [architecture specification](docs/strategies/redesign-architecture.md); this mission's own design is in its strategy. Other Atelier work is paused while the four run.

## Notes

- 2026-11-10 [note] 2026-11-10: orchestration started, unattended, on Sumeet's go-ahead of 2026-11-10. This run prepares the cutover and does not perform it: steps A to E go to their merges, steps F and G are planned, and the switch (atelier-u1ko) stops at an approval on its branch until Sumeet names the time. Steps A (atelier-orb2), C (atelier-5zre), D (atelier-mlwx) and E (atelier-oci2) are planned and go to their planning reviews. Found while planning E: mission 2 never copied the site into next/, so the new binary already has no serve, and step E only proves that and corrects the two specifications. Implementers are Claude Opus 5.5, reviewers GPT-6.1 Sol; each item has its own worktree under ~/.t3/worktrees/atelier/<id>.
