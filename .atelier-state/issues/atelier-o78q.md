---
acceptance: []
blocks:
- "atelier-u8xc"
created_at: "2026-06-10T20:59:15.257035626+00:00"
depends_on:
- "atelier-o54s"
evidence_required: []
id: "atelier-o78q"
issue_type: "epic"
labels:
- "cli"
- "human-output"
links: []
parent: null
priority: "P2"
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Add compact issue hierarchy output"
updated_at: "2026-06-10T20:59:15.257035626+00:00"
---

Provide a smaller hierarchy-oriented view for routine scanning so users are not forced to choose between flat lists and the full `atelier issue tree` dump.

Why:
Users need to see the shape of related issues, parents, and progress without overwhelming terminal output. The existing tree command is useful for exhaustive inspection but too large as the only hierarchy view.

Scope:
- Design and implement a compact hierarchy view or mode for issues, using the shared human-output formatter conventions.
- Include progress summaries and collapsed/limited children where appropriate.
- Make the relationship between the compact hierarchy view and the existing full tree explicit.
- Preserve JSON behavior and existing full-tree behavior unless a separate migration approves changes.

Out of scope:
- Mission Control TUI.
- Replacing all list/ready output with a tree.
- Altering durable hierarchy semantics.

Acceptance criteria:
- Users can inspect parent/child issue shape at a glance without massive output.
- The compact view has predictable limits and clear cues when content is omitted or collapsed.
- Tests cover deep trees, wide sibling sets, closed/open mixed subtrees, and narrow terminal widths.

Recommended subskill: agent-factory implement.
