---
created_at: "2026-07-06T20:37:49.376591376+00:00"
id: "atelier-3v2d"
issue_type: "task"
labels:
- "migration"
- "mission-review"
- "subskill-implement"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-07-16T20:50:02.720696717+00:00"
status: "done"
title: "Migrate existing mission readiness state"
updated_at: "2026-07-16T20:50:02.720696717+00:00"
---

## Description

Assigned subskill: implement. Apply the contract's explicit migration treatment to existing draft and ready mission records and any workflow configuration or canonical review data. Do not silently fabricate reviewer identity, approval, or dependency completeness. Preserve completed historical mission records unless the approved contract requires a documented transformation.

## Outcome

- After migration, every nonterminal mission has an explicit, inspectable readiness state under the new contract, and no legacy mission is silently treated as independently approved.
- Migration is deterministic, rebuild-safe, documented, and covered for repositories with draft, ready, active, and terminal missions.

## Evidence

- Migration tests cover fixtures for legacy draft, ready, active, and terminal missions and record the post-migration inspectable state for each; no fixture gains a fabricated reviewer, approval, or dependency-completeness fact.
- Running migration twice produces the same canonical records, and rebuilding the derived cache from those records preserves the migrated state and review provenance.
- Focused migration tests and a repository-level transcript record the chosen grandfathering/blocking behavior, rollback or recovery instructions, and any intentionally unchanged terminal history as first-class evidence.
