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
updated_at: "2026-07-07T05:28:00.000000000+00:00"
---

## Description

Assigned subskill: implement. Provide canonical mission-plan review state that identifies the mission, reviewed graph revision, author or material editors, reviewer, findings, resolutions, approval, and freshness. Reuse native review event concepts where appropriate without requiring a branch merge or confusing plan approval with code review completion.

## Outcome

- Atelier can deterministically determine whether a mission has an independent approval for its exact current mission and reachable issue-graph revision.
- Material mission or graph edits make prior approval stale; non-material activity does not create spurious invalidation.
- Canonical rebuild preserves review provenance, findings, resolutions, approval identity, and freshness.

## Evidence

- A round-trip fixture records an approval by a reviewer distinct from every author/material editor, identifies the exact mission plus reachable-graph revision, rebuilds the cache from canonical records, and observes unchanged provenance, findings, resolutions, approval, and freshness.
- Focused tests change mission intent, scope roots, reachable work-item content, hierarchy, blocker/dependency edges, and closeout coverage one class at a time and observe the prior approval become stale; a note-only activity append preserves approval freshness.
- Deterministic render/rebuild comparison and malformed/provenance-loss rejection output are captured as first-class test evidence on this issue.
