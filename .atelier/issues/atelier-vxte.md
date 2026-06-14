---
created_at: "2026-06-10T16:09:12.894589289+00:00"
id: "atelier-vxte"
issue_type: "task"
labels:
- "cli"
- "dependencies"
- "hierarchy"
- "issue-show"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-pakd"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-10T17:29:23.402451775+00:00"
status: "done"
title: "Add hierarchy and dependency context to issue show"
updated_at: "2026-06-10T17:29:23.402451775+00:00"
---

## Description

Add hierarchy and dependency context to the richer human `issue show` view.

What:
- Show parent context with parent ID and title when present.
- Show direct subissue summary counts by status and priority.
- Render direct subissue rows sorted by status/actionability, priority, then stable ID/title ordering.
- Expand `Blocked by` and `Blocking` rows to include ID, status, priority, and title rather than just IDs.
- Make blocked/open blockers visually obvious in plain ASCII output.

Out of scope:
- Recursive tree rendering beyond direct subissues.
- Grouped work queue views.
- New persistence fields.
- Terminal-width measurement dependency for v1.
- Parent title appears in human output when a parent exists.
- Subissue progress/counts are correct for direct children.
- Dependency rows include title, status, and priority for each referenced issue.
- Open blockers are clearly distinguishable in plain ASCII output.
- Empty hierarchy/dependency sections are either omitted or rendered as `(none)` consistently with the core view conventions.
- Focused helper tests cover dependency row formatting and subissue summary counts.

Recommended subskill: agent-factory implement.
Evidence was not specified in the legacy issue record.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
