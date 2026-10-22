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
  relates:
  - kind: "review"
    id: "atelier-7neu"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T20:19:01.150816412+00:00"
status: "done"
title: "Tie the stale-proof note to dirty-checkout staleness, not gate text"
updated_at: "2026-10-16T20:19:01.150816412+00:00"
---

## Goal

land's 'Uncommitted changes make passing proof stale' note fires when any failed gate reason contains 'does not prove the current checkout' and the checkout is dirty. Historical (retained) claims get the same reason but are checked without the working tree, so an unrelated uncommitted file can be blamed, and the CLI matches app-layer prose.

## Done when

NewestFailure says whether the pass is stale because of the dirty checkout; the note keys on that flag; historical stale passes are worded without 'current checkout'. A test covers a retained claim whose pass is stale from a later commit while an unrelated file is dirty.

## Context

Deferred finding atelier-7neu from the atelier-749l review (approval atelier-71io). crates/atelier-cli/src/commands/session.rs, crates/atelier-app/src/review_gate.rs, crates/atelier-app/src/evidence_freshness.rs.

## Close Reason

NewestFailure records whether the newer pass would still prove the claim with uncommitted changes set aside; the gate node carries it as stale_from_checkout and land's dirty-paths note keys on that flag, not gate text. Passes stale only from the checkout are worded as recorded before the uncommitted changes; others as no longer proving the claim after later commits. Tests: land_names_uncommitted_files_that_make_a_newer_pass_stale and a_retained_claim_whose_pass_later_commits_staled_does_not_blame_the_checkout (which failed on the text-matching code).
