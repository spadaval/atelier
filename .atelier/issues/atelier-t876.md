---
created_at: "2026-07-06T20:37:49.385326564+00:00"
id: "atelier-t876"
issue_type: "validation"
labels:
- "agent-factory"
- "mission-review"
- "subskill-validate"
- "validation"
- "workflow"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Validate independent mission planning and readiness end to end"
updated_at: "2026-07-06T20:37:49.385326564+00:00"
---

## Description

Assigned subskill: validate. An independent validator derives claims from the mission and epic Outcomes, exercises the public planning, review, readiness, blocker, work-selection, transition, migration, rebuild, and Agent Factory guidance paths, and records pass, fail, blocked, deferred, or not-applicable for each claim. Validation must include negative scenarios rather than relying only on broad test-suite success.

## Outcome

- Independent proof covers rejection of unreviewed plans, author self-approval, unresolved findings, stale approvals after each material graph-change class, incomplete direct blockers, incomplete transitive blockers, and direct issue starts with incomplete dependencies.
- Independent proof covers successful approval by a distinct reviewer, completed dependency closure, non-material activity that preserves approval, deterministic rebuild, migration of representative legacy mission states, and actionable normal CLI recovery output.
- The validator reviews the mission-review rubric against representative complete and defective issue sets and confirms that Agent Factory keeps plan, mission-review, code review, orchestration, and validation responsibilities distinct.

## Evidence

Evidence was not specified in the bundle.
