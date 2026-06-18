---
created_at: "2026-06-18T22:42:55.139062526+00:00"
id: "atelier-cp7i"
issue_type: "task"
labels:
- "agent-factory"
- "docs"
- "pr"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-18T23:34:45.910516766+00:00"
status: "done"
title: "Clean PR workflow guidance drift"
updated_at: "2026-06-18T23:34:45.910516766+00:00"
---

## Description

Clean up PR workflow guidance drift found during the audit. The command audit
for role guides still omits the validator role even though product docs and the
fresh binary support it, and PR workflow guidance should point operators at the
current artifact workflow rather than stale or partial command loops.

## Outcome

- Role-guide docs consistently list `worker`, `reviewer`, `validator`,
  `manager`, and `admin` where that is the implemented command contract.
- Agent-facing guidance names the PR artifact workflow commands needed for the
  proof path without treating PR commands as Atelier workflow transitions.
- Docs no longer contradict the current `atelier man validator` behavior.

## Evidence

- Search transcript over product docs, command audits, and Agent Factory
  guidance shows no stale role list that omits `validator`.
- `target/debug/atelier man validator` transcript matches the documented role
  list and PR workflow guidance.
- `target/debug/atelier lint` passes.
