---
schema: atelier.review
schema_version: 2
id: atelier-v1jl
title: 'crates/atelier-app/src/record_index.rs:476 — a disposition whose finding and supersedes are both missing now reports only the finding. validate_review returns before the supersedes existence check at line 489, and collect_links now skips review attachments as checked_by_reviews, so the missing supersedes is reported nowhere. On master collect_links reported it. Fixture: disposition with finding: f4ko-fx-nofd and supersedes: f4ko-fx-nosp gives ''Check found 1 finding'' naming only nofd. This is a regression in check reporting every invalid reference. Fix: check that supersedes exists before the early return on a missing finding, as follow_up already is, and add a test.'
target: atelier-f4ko
identity: Independent reviewer (Claude subagent)
commit: 53c2cb090d43aa92080e077995e3eecf689f2fc1
created_at: 2026-10-17T09:10:20.886526871Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Orchestrator (Claude, atelier-o3lm)
    commit: 6bed161ba53b9b851e87ff94006f3644c1a4f553
    recorded_at: 2026-10-17T09:14:27.804720710Z
    resolution_commit: dfcc7b3d361225ea08dcc855c7289b196c8b94b9
    reason: validate_review checks every reference before skipping; tests cover finding plus supersedes and all four missing.
    migrated_from: atelier-4yz3
---

crates/atelier-app/src/record_index.rs:476 — a disposition whose finding and supersedes are both missing now reports only the finding. validate_review returns before the supersedes existence check at line 489, and collect_links now skips review attachments as checked_by_reviews, so the missing supersedes is reported nowhere. On master collect_links reported it. Fixture: disposition with finding: f4ko-fx-nofd and supersedes: f4ko-fx-nosp gives 'Check found 1 finding' naming only nofd. This is a regression in check reporting every invalid reference. Fix: check that supersedes exists before the early return on a missing finding, as follow_up already is, and add a test.
