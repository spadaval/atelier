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
    id: "atelier-f3p6"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Repair status start history prime and transition surfaces"
updated_at: "2026-06-12T05:06:28.483421954+00:00"
---

## Description

Repair missing or underpowered workflow signpost surfaces from the previous
mission: root status, normal start flow, history, prime, and issue transition
options.

## Outcome

- Root `atelier status` shows checkout state, active work, active mission,
  tracker health, recent relevant activity, blockers, ready work, and
  active-mission-scoped next actions.
- `atelier start <issue-id>` exists as the normal work entrypoint or the
  mission records a deliberate replacement with implementation and docs.
- The `atelier work` command group is exploded or hidden from the normal
  workflow; start, finish, and current-work orientation move to domain/root
  surfaces.
- Issue transition/options output exists so users can ask what an issue can do
  next without knowing internal validator commands.
- History and prime surfaces are either implemented with observable help and
  transcript behavior or explicitly removed from the target with replacement
  guidance.
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

This epic should split into implementation children once the product decision

for history and prime is confirmed.
