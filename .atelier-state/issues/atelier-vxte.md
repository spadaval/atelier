---
acceptance: []
blocks:
- "atelier-pakd"
created_at: "2026-06-10T16:09:12.894589289+00:00"
depends_on: []
evidence_required: []
id: "atelier-vxte"
issue_type: "task"
labels:
- "cli"
- "dependencies"
- "hierarchy"
- "issue-show"
links: []
parent: "atelier-pd0w"
priority: "P1"
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Add hierarchy and dependency context to issue show"
updated_at: "2026-06-10T16:09:12.894589289+00:00"
---

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

Acceptance criteria:
- Parent title appears in human output when a parent exists.
- Subissue progress/counts are correct for direct children.
- Dependency rows include title, status, and priority for each referenced issue.
- Open blockers are clearly distinguishable in plain ASCII output.
- Empty hierarchy/dependency sections are either omitted or rendered as `(none)` consistently with the core view conventions.
- Focused helper tests cover dependency row formatting and subissue summary counts.

Recommended subskill: agent-factory implement.
