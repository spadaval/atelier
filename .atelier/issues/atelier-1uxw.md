---
acceptance: []
created_at: "2026-06-11T20:28:37.399561843+00:00"
evidence_required: []
id: "atelier-1uxw"
issue_type: "task"
labels:
- "agent-factory"
- "docs"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-8ig1"
  - kind: "issue"
    id: "atelier-qd2t"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Make Agent Factory guidance Atelier-native"
updated_at: "2026-06-11T21:16:20.886569002+00:00"
---

Update the Agent Factory skill and procedure guidance so mission work assumes
Atelier as the durable coordination substrate. Scope includes active mission
focus, option-oriented mission/epic status, issue work selection from the active
mission graph, mission readiness rules for unresolved decisions, worker subagent
interaction flow, evidence and validator expectations, handoff requirements, and
removal or demotion of generic tracker/Beads language where Atelier is now the
product dependency.

Mission creation guidance should distinguish ordinary decision issues from
autonomy-blocking decisions. Highly consequential product, architecture,
persistence, security, data-retention, migration, or public-contract decisions
must be resolved before a mission is started. Mission-linked decision issues are
allowed only for local execution choices that can be resolved without changing
the mission's objective, safety posture, public contract, or validation policy.

The worker flow should be explicit: inspect assigned issue and parent mission or
epic context; create or locate the worktree/branch; start tracked work; implement
the owned slice; run the required proof; attach or record evidence; run the
completion/transition command; leave durable handoff notes when not complete.

Acceptance: SKILL.md and relevant procedure/standard docs tell orchestrators and
subagents to use Atelier mission, issue, work, worktree, evidence, workflow
validate, lint, doctor, and export/check surfaces; guidance tells planners to
resolve high-consequence decisions before mission start and to block mission
activation when such decisions remain; guidance matches actual CLI behavior for
transitions and evidence; legacy fallback language is clearly separated from the
Atelier-dependent path.
