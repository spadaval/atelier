---
created_at: "2026-06-12T04:58:55.521376067+00:00"
id: "atelier-wpyb"
issue_type: "epic"
labels:
- "cli"
- "rework"
- "workflow"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-cany"
  - kind: "issue"
    id: "atelier-f3p6"
  - kind: "issue"
    id: "atelier-sckq"
  - kind: "issue"
    id: "atelier-sdmo"
  - kind: "issue"
    id: "atelier-trr2"
  - kind: "issue"
    id: "atelier-u4nx"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Repair status start history prime and transition surfaces"
updated_at: "2026-06-13T00:01:19.768971860+00:00"
---

## Description

Repair missing or underpowered workflow signpost surfaces from the previous
mission: root status, normal start flow, history, prime, and issue transition
options. Use the closed signpost decisions in `atelier-rqvv`, `atelier-v02t`,
`atelier-vr9g`, `atelier-hggl`, and `atelier-bzts`; this epic should implement
or explicitly supersede those decisions, not reopen them implicitly.

## Outcome

- Root `atelier status` shows checkout state, active work, active mission,
  tracker health, recent relevant activity, blockers, ready work, and
  active-mission-scoped next actions.
- `atelier start <issue-id>` exists as the normal work entrypoint.
- The `atelier work` command group is exploded or hidden from the normal
  workflow; start, finish, and current-work orientation move to domain/root
  surfaces.
- Issue transition/options output exists so users can ask what an issue can do
  next without knowing internal validator commands.
- History and prime surfaces follow their closed contracts or have new
  superseding tracker items.
- User-facing next actions stop pushing normal users toward raw workflow
  validator commands when a status/transition surface should own that guidance.
- Any remaining workflow-validator implementation is hidden behind domain
  status/transition/closeout surfaces instead of appearing in normal help.
- Help, docs, and tests agree on the implemented surfaces.

## Evidence

- Transcript tests cover empty and active root status, start behavior, finish or
  equivalent current-work completion behavior, issue transition/options output,
  and retained or removed history/prime behavior.

- Negative transcripts prove normal next actions do not route users to raw
  workflow-validator commands when a domain status, transition, start, or
  closeout surface owns the answer.

- Tests cover the absence or replacement behavior for intentionally removed
  surfaces.

- Docs/help parity checks show Agent Factory guidance and repository docs match
  the implemented status, start, finish/current-work, transition, history, and
  prime surfaces.

- Run focused CLI integration tests for the repaired workflow signposts plus
  `atelier lint`.

## Notes

The product decisions for status, mission status/show, transitions, history, and
prime are in closed decision records. This epic owns implementation and
validation against those decisions.
