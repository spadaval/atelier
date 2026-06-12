---
acceptance: []
created_at: "2026-06-11T20:28:37.436179178+00:00"
evidence_required: []
id: "atelier-qd2t"
issue_type: "task"
labels:
- "feature"
- "mission"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-0006"
  - kind: "issue"
    id: "atelier-8ig1"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Enforce active mission context in work commands"
updated_at: "2026-06-11T21:16:29.009591502+00:00"
---

Implement or extend CLI behavior so active mission context guides ordinary
orchestration commands. Scope: mission start/switch/status behavior; no-ID
mission status defaulting to the active mission; work start warning or blocking
when an issue does not advance the active mission under strict workflows;
worktree helpers carrying active mission context; and transition-driven
completion/closeout behavior.

Mission start/focus behavior should include a readiness gate for unresolved
high-consequence product, architecture, persistence, security, migration, public
contract, data-retention, or validation-policy choices. The command should block
or warn according to workflow policy when open artifact-update tasks linked to
the mission affect those areas. Local execution choices can remain mission work
when they do not undermine autonomous execution.

Completion commands should trigger configured workflow validators before marking
an issue, epic, or mission done. Missing evidence must be reported explicitly,
including which record or criterion needs proof and which command can attach it.
The implementation should support an explicit waiver/backup path for rare cases
where evidence cannot be produced, with a required reason and visible durable
record of the waiver. Status output should present action options with context,
not claim there is only one correct next step.

Out of scope: supervising live agent processes.

Acceptance: tests cover one active mission, switching, multiple-active lint
failure, work start inside/outside active mission, non-mission lightweight issue
work, worktree status showing mission context, completion blocked by missing
evidence, completion after evidence is attached, explicit waiver behavior, and
mission start/focus behavior when unresolved high-consequence artifact-update
tasks are linked to the mission.
