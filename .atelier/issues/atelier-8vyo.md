---
created_at: "2026-06-14T02:52:08.979051706+00:00"
id: "atelier-8vyo"
issue_type: "task"
labels:
- "agent-factory"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "validation"
title: "Add Agent Factory stale-state preflight"
updated_at: "2026-06-14T07:05:22.400265487+00:00"
---

## Description

After the Agent Factory/Atelier guidance boundary is reconciled, update
portable guidance so tracker lint/export/read failures caused by canonical or
projection invalidity stop workflow mutation until state is repaired through
the bound tracker's owned surfaces.

## Outcome

Orchestrators and workers repair stale state before continuing mission mutation or closeout.

## Evidence

- File diff in Agent Factory guidance names the stale-state preflight and stop
  condition.
- The repair steps are routed to Atelier-owned status/lint/doctor/help surfaces
  rather than duplicated as a static command cookbook.
- A transcript or docs validation shows the rule is present and points at
  implemented commands.
