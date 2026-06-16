---
created_at: "2026-06-16T15:59:38.127626538+00:00"
id: "atelier-f7vd"
issue_type: "feature"
labels:
- "cli"
- "dependencies"
- "mission"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-em15"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-16T18:25:33.813931477+00:00"
status: "done"
title: "Sort mission status work groups by blocker order"
updated_at: "2026-06-16T18:25:33.813931477+00:00"
---

## Description

Update mission status so linked work groups and next-action suggestions respect issue blockers and display readable action state. Mission status is the main operator surface for coordinated work, so it should point agents toward the work that unlocks other work.

## Outcome

- `atelier mission status <mission-id>` orders visible issue rows inside ready, active, blocked, linked-work, or similar work groups with the shared blocker-aware rule where those groups display issue rows.
- Mission status work rows use a single readable state label instead of duplicate `category/status` strings.
- When a blocker and its dependent land in different mission status buckets, the bucket structure remains intact, but next-action guidance names the actionable blocker before dependent work.
- Blocked work rows show blocked state, count, or drill-down guidance; exact issue blocker IDs remain available in blocker/detail sections.
- Mission blockers and issue blockers remain distinct in the output.
- Evidence gaps, workflow state, parent context, and next commands remain visible after the ordering change.
- Quiet or compact mission status output preserves the same selected next-work order where it emits issue IDs.

## Evidence

- CLI test or transcript shows a mission-linked dependency chain where mission status names the blocker as the next actionable work before the dependent issue.
- CLI transcript or focused test for `atelier mission status <mission-id>` proves mission status rows use readable state and do not contain duplicate category/status tokens.
- CLI test or transcript shows blocked work exposes a drill-down path for exact blocker IDs and does not collapse mission blockers into issue blockers.
- Regression test proves evidence-gap and workflow-state cues remain visible in mission status output.
- `atelier lint atelier-f7vd`, `atelier export --check`, focused cargo tests, and `git diff --check` pass.

## Notes

Mission status should become easier to act on, not longer by default.
