---
acceptance: []
created_at: "2026-06-12T00:59:17.815268905+00:00"
evidence_required: []
id: "atelier-ob49"
issue_type: "task"
labels:
- "cli"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Define command removal and migration behavior"
updated_at: "2026-06-12T02:13:27.252201331+00:00"
---

Define how command moves and removals should be handled in this refinement pass. The default policy is staged deprecation or a compatibility alias for moved commands when existing operator habits or Agent Factory guidance could break. Direct removal is acceptable only when the issue proves the command is redundant, the replacement is clear, tested, and documented, and no compatibility need remains.

Acceptance:

- Each removed or moved command has an explicit old-to-new mapping in docs and validation transcripts.
- The spec distinguishes true compatibility needs from redundant ergonomic clutter.
- `issue quick` and `issue subissue` are evaluated as removal candidates once `issue create --work` and `issue create --parent <id>` cover their use cases.
- Help output and Agent Factory guidance do not recommend removed commands.
- Any direct removal or retained alias requires an explicit, issue-backed reason.
