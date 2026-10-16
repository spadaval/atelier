---
created_at: "2026-10-16T19:12:40.883294176+00:00"
id: "atelier-x3bj"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: task/atelier-x3bj
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Tie the stale-proof note to dirty-checkout staleness, not gate text"
updated_at: "2026-10-16T20:07:49.667065017+00:00"
---

## Goal

land's 'Uncommitted changes make passing proof stale' note fires when any failed gate reason contains 'does not prove the current checkout' and the checkout is dirty. Historical (retained) claims get the same reason but are checked without the working tree, so an unrelated uncommitted file can be blamed, and the CLI matches app-layer prose.

## Done when

NewestFailure says whether the pass is stale because of the dirty checkout; the note keys on that flag; historical stale passes are worded without 'current checkout'. A test covers a retained claim whose pass is stale from a later commit while an unrelated file is dirty.

## Context

Deferred finding atelier-7neu from the atelier-749l review (approval atelier-71io). crates/atelier-cli/src/commands/session.rs, crates/atelier-app/src/review_gate.rs, crates/atelier-app/src/evidence_freshness.rs.
