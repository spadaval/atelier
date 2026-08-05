---
created_at: "2026-08-05T20:11:02.960865950+00:00"
id: "atelier-2pvp"
issue_type: "bug"
labels:
- "validation-finding"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-ekg7"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-08-05T20:21:31.372434679+00:00"
status: "done"
title: "Preserve mission branch ownership for newly linked epics"
updated_at: "2026-08-05T20:21:31.372434679+00:00"
---

## Description

Repair the branch-lifecycle failure discovered by atelier-ekg7. Nested work under an epic that advances an active mission must resolve through the epic owner to the mission branch; branch preparation must not strand execution on a branch that lacks the transition target.

## Outcome

- Starting nested work under an epic linked to a mission derives the epic branch from the mission branch, including when the epic was created before the `advances` link.
- Branch preparation refuses to switch or create an owner branch that does not contain the issue being transitioned, with actionable recovery guidance.
- A command-level regression proves create, link, mission start, epic expansion, and nested-work start across the first executable horizon.

## Evidence

- Focused lifecycle tests cover mission scope through the parent epic and the full fresh-repository sequence that failed in `atelier-ekg7`.
- The corrected `atelier-ekg7` continuation scenario starts the expanded child on `epic/<epic-id>` based on `mission/<mission-id>` without missing canonical records.
- `cargo nextest run`, `atelier check`, formatting, and whitespace checks pass at the repair candidate.
