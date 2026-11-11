---
created_at: "2026-10-17T01:10:57.985221680+00:00"
id: "atelier-9bm2"
issue_type: "feature"
labels:
- "cli-output"
- "work-views"
priority: "P2"
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
closed_at: "2026-11-11T03:55:35.690492853+00:00"
status: "superseded"
title: "Show the outcome, own blockers and next step in the work mission header"
updated_at: "2026-11-11T03:55:35.690492853+00:00"
---

## Goal

The `work mission` header shows the mission, its Scope, Strategy and Criteria, but omits:
- the Outcome, which docs/product/issue-inventory-and-mission-overview.md says the view shows;
- the mission's own blockers: a mission `blocked_by` an open issue shows nothing, though its `start` transition requires `blockers.transitive_none_open`;
- what moves the mission itself: a draft mission needs `ready`, a todo one `start`, and one in progress closes once all its work is merged and its validation, if it has one, is done (atelier-jnrh). The footer offers only `issue show` and `issue transition`.

## Done when

- [ ] `work mission <id>` prints the first line of the Outcome, and `Blocked by:` only when the mission has open blockers.
- [ ] A `Next:` line names the mission's own next step: `ready` for a draft, `start` for a todo mission (noting work already active), and for one in progress either what is left before close (items not merged, a validation not done) or `close` once nothing is.
- [ ] Tests cover a draft, a todo mission with active work, a blocked mission, and an in-progress mission whose work is all merged.

## Context

The close rule is described, not enforced (atelier-jnrh). The separate Status line the original reported is gone, and Strategy and Criteria are shown (atelier-stsa).

## Report

Dropped in the cutover sort (atelier-5zre), reason (b). The new `show <mission>` prints the outcome and criteria and each item's status and blockers on one page (product specification, Commands: `show`; run on atelier-4pzj in a scratch clone).
