---
created_at: "2026-06-29T17:40:30.194241997+00:00"
id: "atelier-2uim"
issue_type: "validation"
labels:
- "agent-factory"
- "contract"
- "mission-review"
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
title: "Validate retained mission-planning contracts before review expansion"
updated_at: "2026-07-07T05:42:00.000000000+00:00"
---

## Description

Assigned subskill: validate. Independently compare the contract and authoring-standard artifacts produced by atelier-0zhd and atelier-wlk4 with atelier-a44d, atelier-ql9k, and atelier-1mga before dependent implementation begins. Classify which legacy rules are retained, amended, or superseded. Confirm that workflow ownership, Outcome-led planning, validator-selected proof, and evidence-as-receipt boundaries remain intact while obsolete direct `draft -> ready` readiness and Outcome-only review are not accepted as the target behavior. This issue is migrated from the closed atelier-1mga mission into the atelier-6tne contract epic; final end-to-end behavior remains owned by atelier-t876.

## Outcome

- An independent claim map identifies every applicable rule from atelier-a44d, atelier-ql9k, and atelier-1mga as retained, amended, or superseded and cites the exact updated contract or authoring-standard location.
- Any contradiction, unowned legacy validation responsibility, or reintroduction of planner-authored proof paperwork is reported as a blocking finding against atelier-6tne before implementation work proceeds.
- The result explicitly defers executable lifecycle, migration, rebuild, dependency, diagnostics, and Agent Factory behavior to atelier-t876 rather than duplicating final end-to-end validation.

## Evidence

- A first-class evidence record attached to atelier-2uim contains the independent claim matrix, exact documentation file references, affected issue IDs, `pass`, `fail`, `blocked`, or `deferred` classifications, and follow-up IDs for every blocking contradiction.
- `atelier issue show atelier-2uim` and `atelier issue transition atelier-2uim` transcripts confirm the issue is scoped under atelier-6tne, waits for atelier-wlk4, and resolves branch context through the current p4z2 graph rather than missing `mission/atelier-1mga`.
