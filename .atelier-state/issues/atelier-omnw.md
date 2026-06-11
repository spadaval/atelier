---
acceptance: []
created_at: "2026-06-10T20:59:05.023604139+00:00"
evidence_required: []
id: "atelier-omnw"
issue_type: "epic"
labels:
- "assignee:root"
- "cli"
- "human-output"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-u8xc"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Replace flat issue queues with grouped scannable views"
updated_at: "2026-06-10T22:15:52.549031285+00:00"
---

Improve routine work-selection output so users can scan open, ready, and search results without reading a raw table dump.

Why:
`atelier issue list` and related queue surfaces should communicate priority, status, type, ownership, blockers, parent context, and readiness at a useful density. A flat table hides the shape of work and makes large queues hard to operate.

Scope:
- Redesign default human output for `atelier issue list`, `atelier issue ready`, and `atelier issue search` where applicable.
- Group and order work by useful operational dimensions such as readiness, priority, type, parent epic/mission context, or blocked state.
- Show enough hierarchy/context to choose work without requiring full `issue tree`.
- Preserve quiet and JSON modes for scripts.

Out of scope:
- Changing tracker scheduling semantics.
- Building a full-screen TUI.
- Hiding important issue records solely to make output short.

Acceptance criteria:
- Default queue output is grouped, compact, and useful on a realistic project with dozens of issues.
- Output makes blockers and parent context visible without becoming a massive tree.
- Tests cover empty queues, mixed priorities/statuses/types, blocked work, and parent-child context.

Recommended subskill: agent-factory implement.
