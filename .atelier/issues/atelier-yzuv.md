---
created_at: "2026-10-17T01:10:57.964882526+00:00"
id: "atelier-yzuv"
issue_type: "task"
labels:
- "cli-output"
- "issue-commands"
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
title: "Stop mission issue show restating scope three times"
updated_at: "2026-11-07T21:03:28.586467482+00:00"
---

## Goal

`atelier show <mission>` prints the mission once on one page, with its Outcome, criteria and items by status, and none of the old Relationships, Evidence Gates, Subissues or Impact sections, but it prints no path to the mission's documents: in a converted scratch clone, `.atelier/missions/atelier-qr5s/design.md` exists and `show atelier-qr5s` names neither it nor the directory (review finding atelier-jaa4, checked 2026-11-11 with the new binary).

## Done when

- [ ] `show <mission>` prints the path of each of the mission's documents other than `mission.md`, e.g. `Documents: .atelier/missions/atelier-qr5s/design.md, notes.md`, and nothing when it has none.
- [ ] A rendering test covers a mission with a design document and one with none.

## Context

Extends atelier-7u4o (brief `issue show`) and atelier-deyy (dense work rows). atelier-t18i's link count goes with the Relationships line; its cycle message moved to atelier-x3ru (decision atelier-mzpn).

Rewritten for the new tool in the cutover sort (atelier-5zre): `issue show <mission> --full` became `show <mission>`, which already prints the Outcome and criteria once, its items grouped by status, and no Relationships, Evidence Gates, Subissues or Impact sections, so those parts left; the strategy path became the path of the mission's documents in its directory; the parent summary went with parents.
