---
created_at: "2026-10-15T22:08:59.002443828+00:00"
id: "atelier-9k7o"
issue_type: "task"
labels:
- "adoption"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Decide whether the done claim should come after review"
updated_at: "2026-10-15T22:08:59.002443828+00:00"
---

## Goal

The close transition is committed on the work branch before review. Until the merge, that branch says done for work the base branch doesn't have, which misleads anyone reading another branch's records and turns a rejected review into a done → in_progress reversal.

## Done when

Sumeet decides between keeping the order (and making status output show "done on branch X, not merged"), or closing at merge time with review recording the claim. Recorded in the product docs.

## Context

Found 2026-10-15 while adopting Atelier in astromech (branch chore/atelier-tracker). Documented in the close transition's description, but easy to misread.
