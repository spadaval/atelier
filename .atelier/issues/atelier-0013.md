---
acceptance: []
created_at: "2026-06-08T19:39:48+00:00"
evidence_required: []
id: "atelier-0013"
issue_type: "task"
labels:
- "agent-factory"
- "assignee:root"
- "dogfood"
- "migration"
- "mission"
- "task"
- "tracker"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-0014"
  - kind: "issue"
    id: "atelier-0015"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Dogfood Atelier as this repository's live tracker"
updated_at: "2026-06-08T22:17:07.925041568+00:00"
---


Cut /root/atelier over from Beads to Atelier after the import, command parity, and storage checks pass. This is the repository-level replacement step: AGENTFACTORY.md, AGENTS.md, docs, hooks, and validation commands should point at Atelier rather than bd.

Keep the old Beads data archived for recovery, but normal planning and execution must use Atelier.

## Acceptance Criteria

AGENTFACTORY.md declares Atelier as the tracker and lists Atelier sync/check commands; current Beads data is archived read-only; a fresh clone/worktree can rebuild Atelier state and run the tracker checks; at least one real issue update and closeout is performed through Atelier; no normal agent-factory command path requires bd.
