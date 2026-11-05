---
created_at: "2026-10-17T19:04:44.537094232+00:00"
id: "atelier-pth2"
issue_type: "task"
labels:
- "review"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-r6bt"
    type: "advances"
  - kind: "review"
    id: "atelier-r22g"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-05T01:23:49.044698841+00:00"
status: "done"
title: "Remove the read-time criteria mission helper and its stale comments"
updated_at: "2026-11-05T01:23:49.044698841+00:00"
---

## Goal

After atelier-ho7o stored each receipt's criteria mission, `criteria::criteria_mission()` in crates/atelier-app/src/criteria.rs has no callers, and the doc comments on `MissionProof.orphaned` and lint.rs `criteria_findings` still describe working the mission out from the target. A later reuse could bring the read-time rule back. Found by review atelier-r22g.

## Done when

The unused helper is gone and both comments say the stored `criteria_mission` decides; the build has no dead-code warning.

## Close Reason

The unused `criteria::criteria_mission` is gone, and the doc comments on `MissionProof.orphaned` and lint.rs `criteria_findings` now say the receipt's stored `criteria_mission` decides; the workspace builds with no warnings.

### Deviations
None.

### Decisions
None.

### Surprises
None. The recent master changes to criteria.rs and lint.rs had left both stale comments in place.

### Open items
None.
