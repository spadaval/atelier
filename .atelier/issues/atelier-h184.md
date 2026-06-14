---
created_at: "2026-06-14T03:47:07.684316124+00:00"
id: "atelier-h184"
issue_type: "bug"
labels:
- "assignee:root"
- "mission-status"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "validation"
title: "Prevent mission status from double-counting nested work"
updated_at: "2026-06-14T07:07:20.091690115+00:00"
---

## Description

Mission status should not double-count work when an issue is reachable by more than one route, such as both direct mission advances and epic child hierarchy. The rollup should deduplicate by issue ID and should make the recommended mission-to-epic graph shape clear in output.

## Outcome

Mission status and closeout rollups count each unique issue once, regardless of duplicate relationship paths, and identify duplicate reachability as a graph hygiene warning rather than inflating totals.

## Evidence

- Focused CLI test or transcript creates a mission with an epic and a child
  issue also directly linked to the mission, then shows mission status counting
  the child once.
- Focused CLI test or transcript output reports duplicate reachability as a
  graph hygiene warning or equivalent operator-facing diagnostic rather than
  inflating totals.
- `git diff --check`, `atelier lint`, and the focused mission-status test pass.
