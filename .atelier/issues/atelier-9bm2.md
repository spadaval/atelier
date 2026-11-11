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
status: "backlog"
title: "Show the outcome, own blockers and next step in the work mission header"
updated_at: "2026-11-07T21:03:27.041551199+00:00"
---

## Goal

`atelier show <mission>` prints the mission's Outcome, criteria, items by status and `Not made ready yet`, but not what moves the mission itself: a draft mission needs `ready`, a ready one with no lead needs `lead`, and one whose items are all done or dropped needs its Result and `close` (checked 2026-11-11 with the new binary in a converted scratch clone).

## Done when

- [ ] `show <mission>` ends with a `Then:` line naming the mission's own next step: `ready` for a draft, `lead` for a ready mission with no lead (noting items already started), and for one in progress either what is left before close (items not done or dropped, the Result not written) or `close` once nothing is.
- [ ] Tests cover a draft, a ready mission with no lead and an item already started, and a mission whose items are all done.

## Context

The close rule is described, not enforced (atelier-jnrh). The separate Status line the original reported is gone, and Strategy and Criteria are shown (atelier-stsa).

Rewritten for the new tool in the cutover sort (atelier-5zre): `work mission` became `show <mission>`, which already prints the Outcome and criteria; a mission has no blockers in the new records, so that line went; a todo mission's `start` became `lead`.
