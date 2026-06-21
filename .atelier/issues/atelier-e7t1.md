---
created_at: "2026-06-21T17:24:17.004069237+00:00"
id: "atelier-e7t1"
issue_type: "feature"
labels:
- "issue-list"
- "issue-table"
- "mission-rework"
- "status"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-2kfb"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-21T18:51:15.481001730+00:00"
status: "done"
title: "Split mission discovery out of issue status"
updated_at: "2026-06-21T18:51:15.481001730+00:00"
---

## Description

Remove the no-argument mission/objective dashboard behavior from issue status. Root status should remain an orientation surface, not a second issue browser: it may name the current top-level work context and point at the right inspection command. Grouped browsing and flat record discovery should be explicit command shapes so users are not surprised by layout inference. A fresh agent must be able to discover current mission/objective records, see their IDs and titles, and choose a focused issue status or issue show command without relying on a hidden no-argument issue status mode.

## Outcome

- atelier status stays compact and orientation-focused, with only enough mission/objective context to explain where the user is and which command to run next.
- atelier issue list preserves grouped context as the default broad work-browsing view.
- A separate explicit table/inventory surface, such as atelier issue table, shows homogeneous record queries such as missions/objectives with IDs, titles, health, and ready/blocked counts.
- The table/inventory surface supports filters for mission/objective discovery without making issue list infer a different layout from the same command.
- atelier issue list can show work grouped under a selected mission/objective when the user asks for grouped context.
- atelier issue status with an ID remains the focused status page for one issue/objective.
- No-argument atelier issue status is removed, demoted, or redirected according to the mission command and workflow authority contract.
- Help, role guides, and next-action text route mission discovery through the explicit table/inventory surface or grouped issue list, not an omitted argument on issue status.

## Evidence

- Focused CLI tests or transcripts show atelier status remains compact and routes mission/objective discovery to the explicit table/inventory surface or grouped issue list.
- Focused CLI tests or transcripts show broad issue list output remains grouped.
- Focused CLI tests or transcripts show the explicit table/inventory surface exposes mission/objective choices without requiring a known mission ID.
- Focused CLI tests or transcripts show issue list does not infer table layout from filters alone.
- Focused CLI tests or transcripts show issue list can render work grouped under a selected mission/objective.
- Focused CLI tests or transcripts show issue status with an ID still renders focused objective status.
- Command-surface search transcript over help output, role guides, and docs shows mission/objective discovery no longer depends on no-argument issue status.
- atelier lint and git diff --check pass.
