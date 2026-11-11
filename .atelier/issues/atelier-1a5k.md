---
created_at: "2026-10-17T01:10:57.986493174+00:00"
id: "atelier-1a5k"
issue_type: "task"
labels:
- "cli-output"
- "work-views"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-4pzj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-11T03:55:34.101570533+00:00"
status: "declined"
title: "Stop work parent listing and counting the parent as its own row"
updated_at: "2026-11-11T03:55:34.101570533+00:00"
---

## Goal

`work parent` lists the parent among its own rows (crates/atelier-cli/src/commands/work.rs): `work parent atelier-ma4a` printed, on 2026-11-07, `Scope: 2 issues` and then the parent itself as the first row under Backlog Work, above its two children. On master, `work parent 1ioo` says "done 12" while `issue show 1ioo` Subissues says "11 total", and the two order the children differently.

## Done when

- [ ] The parent appears only in the header, with its status and blockers; counts cover children only, so `work parent` and `issue show` agree on the number of children.
- [ ] Children appear in the same order in both views (the shared order in docs/product/work-view-ordering.md).
- [ ] Tests cover a parent with children in different states.

## Context

Already done since the original: the header uses the mission's form with the parent's mission, and the `Proof gaps` count is gone (a test asserts it). Decision atelier-mzpn dropped naming the items without proof, since receipts gate nothing (atelier-md0v).

## Report

Dropped in the cutover sort (atelier-5zre), reason (a). It changes `work parent`, and the What is removed table removes the nine `work` views and parents.
