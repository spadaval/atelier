---
created_at: "2026-06-16T15:59:42.247978043+00:00"
id: "atelier-d226"
issue_type: "feature"
labels:
- "cli"
- "dependencies"
- "status"
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
status: "in_progress"
title: "Sort root status work suggestions by blocker order"
updated_at: "2026-06-16T18:13:30.760844252+00:00"
---

## Description

Update root `atelier status` so the default orientation view surfaces prerequisite work before dependent work and labels work state plainly. This is the first command many agents run, so its next-work suggestions should reflect blockers without requiring a failed start or failed close attempt.

## Outcome

- Root `atelier status` orders ready, selectable, current, or next-work issue suggestions with the shared blocker-aware rule where it displays issue rows or emits selected IDs.
- Root status work suggestions use readable action state instead of duplicate `category/status` strings.
- Status output keeps tracker freshness, active mission, branch/work context, evidence gaps, and health cues visible after the ordering change.
- If a suggested issue is blocked by hidden work outside the current status context, status prints blocked state and a drill-down command rather than silently treating it as ready.
- Quiet status output preserves the same selected next-work order where it emits IDs.

## Evidence

- CLI test or transcript shows root status for a checkout with a visible blocker chain and names the blocker before dependent work.
- CLI transcript or focused test for root `atelier status` proves readable state labels, no duplicate category/status tokens, and drill-down guidance for a blocked suggestion outside the visible set.
- Regression test proves tracker freshness, active mission, branch/work context, evidence-gap, and health cues remain present.
- `atelier lint atelier-d226`, `atelier export --check`, focused cargo tests, and `git diff --check` pass.

## Notes

This should improve first-command visibility. It should not make `status` a verbose mission audit.
