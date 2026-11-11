---
created_at: "2026-10-17T01:10:57.986164681+00:00"
id: "atelier-lkuy"
issue_type: "task"
labels:
- "cli-output"
- "work-views"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-4pzj"
    type: "advances"
  - kind: "issue"
    id: "atelier-om6l"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Rewrite the Mission Overview entry in plain terms"
updated_at: "2026-11-07T21:03:27.787222355+00:00"
---

## Goal

`atelier list` heads each open mission with one plain line (`Mission atelier-qr5s (ready): …`), but orders missions by ID, so a draft mission is listed above a mission in progress, and the heading gives no count of its items by state and no path to the mission's documents, such as `.atelier/missions/atelier-qr5s/design.md`, which neither `list` nor `show <mission>` prints (checked 2026-11-11 with the new binary in a converted scratch clone).

## Done when

- [ ] `list` orders missions in progress first, then ready, then draft.
- [ ] Each mission heading gives its items by state, e.g. `10 items: 1 in progress, 1 ready, 6 blocked, 2 draft`, with correct plurals and no "Scope" or "Open blockers" label, and the path of the mission's documents other than `mission.md`, e.g. `design .atelier/missions/atelier-qr5s/design.md`.
- [ ] Each mission heading, or one line after the list, names `atelier show <mission>` for the mission in full.
- [ ] The design record describes the heading, and rendering tests are updated.

## Scope

The mission headings of `list`. The Waiting on line moved to atelier-om6l in decision atelier-mzpn.

## Context

Carries the part of atelier-1w8c's S3 that its validation found unmet.

Rewritten for the new tool in the cutover sort (atelier-5zre): `work missions` became `list`, whose heading already uses plain terms and names the mission's status; the strategy link became the path of the mission's documents in its directory.
