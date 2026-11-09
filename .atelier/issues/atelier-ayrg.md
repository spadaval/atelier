---
created_at: "2026-11-09T18:00:38.351435421+00:00"
id: "atelier-ayrg"
issue_type: "bug"
labels:
- "review"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-1gb3"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Count fix_now findings resolved by a later approval as fixed in the debrief: atelier-1gb3's shows 16 of 16 awaiting a decision"
updated_at: "2026-11-09T18:00:38.351435421+00:00"
---

## Goal

The debrief of mission atelier-1gb3 says "16 findings: 16 awaiting a decision", though land passes and every finding was a fix_now finding resolved by an approval after its fix. `crates/atelier-app/src/mission_review.rs` counts a finding as fixed only with an explicit fix_now disposition, so a finding resolved the normal way (recommended fix_now, then a later approval) counts as undisposed. The debrief then tells the human there are 16 open decisions that do not exist.

## Done when

- [ ] A fix_now finding with no disposition, followed by an approval of its target, counts as fixed in the debrief and the mission brief, with the same rule `findings.disposed` uses.
- [ ] A finding still awaiting a decision under `findings.disposed` keeps counting as awaiting, and a test covers both cases.
