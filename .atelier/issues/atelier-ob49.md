---
acceptance: []
created_at: "2026-06-12T00:59:17.815268905+00:00"
evidence_required: []
id: "atelier-ob49"
issue_type: "task"
labels:
- "assignee:root"
- "cli"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Define command removal and migration behavior"
updated_at: "2026-06-12T03:25:34.607296507+00:00"
---

Define how command moves and removals should be handled in this refinement pass. The default policy is direct removal of moved, inherited, or duplicate command surfaces once the replacement is clear, tested, and documented. Compatibility aliases and staged deprecations are not retained unless a human explicitly asks for a compatibility window.

Acceptance:

- Each removed or moved command has an explicit old-to-new mapping in docs and validation transcripts.
- The spec distinguishes retained product behavior from redundant ergonomic clutter.
- `issue quick` and `issue subissue` are evaluated as removal candidates once `issue create --work` and `issue create --parent <id>` cover their use cases.
- Help output and Agent Factory guidance do not recommend removed commands.
- Any retained alias requires an explicit human request and issue-backed reason.
