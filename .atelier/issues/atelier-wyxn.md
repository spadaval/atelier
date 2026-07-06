---
created_at: "2026-07-06T20:37:49.354483200+00:00"
id: "atelier-wyxn"
issue_type: "feature"
labels:
- "mission-review"
- "records"
- "subskill-implement"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-fqzt"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Represent mission-plan review provenance and revision freshness"
updated_at: "2026-07-06T20:37:49.354483200+00:00"
---

## Description

Assigned subskill: implement. Provide canonical mission-plan review state that identifies the mission, reviewed graph revision, author or material editors, reviewer, findings, resolutions, approval, and freshness. Reuse native review event concepts where appropriate without requiring a branch merge or confusing plan approval with code review completion.

## Outcome

- Atelier can deterministically determine whether a mission has an independent approval for its exact current mission and reachable issue-graph revision.
- Material mission or graph edits make prior approval stale; non-material activity does not create spurious invalidation.
- Canonical rebuild preserves review provenance, findings, resolutions, approval identity, and freshness.

## Evidence

Evidence was not specified in the bundle.
