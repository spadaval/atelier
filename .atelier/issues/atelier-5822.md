---
created_at: "2026-11-06T21:26:18.777067374+00:00"
id: "atelier-5822"
issue_type: "feature"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-hwxm"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-n4mj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-07T02:51:30.720355360+00:00"
status: "superseded"
title: "Records ride with the commits they describe"
updated_at: "2026-11-07T02:51:30.720355360+00:00"
---

## Goal

Record-only commits fall to the floor that pinning forces, by letting records travel with the commit they describe or be batched, wherever pinning allows.

## Done when

Provisional: record-only commits in the lifecycle scenario are at most half the baseline (with the disposition change counted); every command that writes records commits them in the way the new rule says; nothing a gate reads becomes unpinned.

## Unknowns

The floor measured by atelier-5fus decides how much this item can reach.

## Close Reason

Dropped on Sumeet's direction (2026-11-06): the record-only commit count is a symptom, not the cost. Amending consecutive record commits reached the target (6 of 13) but could rewrite a commit already pushed by URL, and only with two scenario reorderings; batching or squashing would only hide the process after the fact and save agents no step. The real cuts came from needing fewer records and steps (atelier-hwxm, atelier-s66u). Implementation and its review (findings atelier-77xt, atelier-8c8h, atelier-b5hm, atelier-7cis) are on the unmerged branch feature/atelier-5822, PR #32, closed unmerged.
