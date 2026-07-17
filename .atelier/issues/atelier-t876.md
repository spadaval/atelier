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
review:
  kind: pull_request
  number: 75
  provider: forgejo
fields:
  workflow_branch:
    branch_base: mission/atelier-p4z2
    integration_target: mission/atelier-p4z2
    merge_strategy: squash
    owner_issue_id: atelier-t876
    owner_kind: issue
    review_target: mission/atelier-p4z2
    work_branch: validation/atelier-t876
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "validation"
title: "Validate independent mission planning and readiness end to end"
updated_at: "2026-07-17T00:25:21.245998289+00:00"
---

## Description

Assigned subskill: validate. An independent validator derives claims from the mission and epic Outcomes, consumes atelier-2uim's contract-compatibility result, exercises the public planning, review, readiness, blocker, work-selection, transition, migration, rebuild, and Agent Factory guidance paths, and records pass, fail, blocked, deferred, or not-applicable for each claim. Validation must include negative scenarios rather than relying only on broad test-suite success.

## Outcome

- Independent proof covers rejection of unreviewed plans, author self-approval, unresolved findings, stale approvals after each material graph-change class, incomplete direct blockers, incomplete transitive blockers, and direct issue starts with incomplete dependencies.
- Independent proof covers successful approval by a distinct reviewer, completed dependency closure, non-material activity that preserves approval, deterministic rebuild, migration of representative legacy mission states, and actionable normal CLI recovery output.
- The validator reviews the mission-review rubric against representative complete and defective issue sets and confirms that Agent Factory keeps plan, mission-review, code review, orchestration, and validation responsibilities distinct.
- The validator confirms atelier-2uim no longer appears as orphaned ready work for the closed atelier-1mga mission and that its retained-contract classifications agree with the implemented behavior.

## Evidence

- A claim matrix maps every mission and epic Outcome line to a classified `pass`, `fail`, `blocked`, `deferred`, or `not-applicable` result and to attached first-class evidence IDs containing the exact commands, fixtures, review artifacts, diff locations, and residual risks inspected.
- Negative transcripts cover unreviewed draft, author/material-editor self-approval, unresolved finding, change request, each material graph-change class, open direct and transitive mission blockers, incomplete ordinary-issue dependency, dependency cycle, and attempted lifecycle bypass.
- Positive transcripts cover distinct-reviewer exact-revision approval, completed dependency closure, note-only activity preserving freshness, configured terminal statuses, deterministic canonical rebuild, idempotent migration of draft/ready/active/terminal legacy states, and actionable normal CLI recovery output.
- The baseline regression explicitly demonstrates that `atelier issue show`, `atelier work ready`, and transition inspection no longer advertise a draft mission or its descendants as executable before current independent approval; it then demonstrates the approved dependency-safe graph becoming available.
- A fresh-context rubric audit classifies representative complete and defective issue sets and verifies Agent Factory keeps plan, mission-review, code review, orchestration, and outcome validation separate; discovered defects receive follow-up issue IDs rather than validator-authored fixes.
