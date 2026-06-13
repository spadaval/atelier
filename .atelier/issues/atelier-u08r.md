---
created_at: "2026-06-13T20:37:00.321850683+00:00"
id: "atelier-u08r"
issue_type: "task"
labels:
- "cleanup"
- "cli"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "validation"
title: "Remove predecessor and compatibility command leakage"
updated_at: "2026-06-13T23:18:46.731788333+00:00"
---

## Description

Remove or explicitly hide predecessor command paths that split one operator job across multiple verbs or keep old command habits alive. Audit includes visible evidence add/capture, hidden issue quick/subissue/search/block/relate/tree/tested helpers, hidden work/workflow diagnostics, and compatibility messages that point to removed commands.

## Outcome

- Normal help teaches only the approved command surface.
- Removed predecessor commands fail as unknown or are hidden with a documented temporary owner and reason.
- Compatibility messages do not point users at obsolete or contradictory workflows.

## Evidence

- Focused help and negative-command transcripts prove removed surfaces are absent or rejected.
- Residue search for issue_compat_guidance, hidden predecessor variants, evidence add/capture, workflow/work helper exposure, and old command names is attached.
- Relevant CLI tests pass.
