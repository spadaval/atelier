---
created_at: "2026-10-16T20:09:18.966628799+00:00"
id: "atelier-imbo"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Cover mixed code and record commits in the rebase recovery advice"
updated_at: "2026-10-16T20:09:18.966628799+00:00"
---

## Goal

The rebase recovery advice says to cherry-pick code or planning commits and leave out review and evidence records, but a single post-rebase commit can hold both (git add -A). Say to drop the record files from such a commit when cherry-picking (git cherry-pick -n, then remove .atelier/reviews and .atelier/evidence files it added).

## Done when

The error in crates/atelier-app/src/review_gate.rs and docs/product/reviews-and-merge.md cover mixed commits, and a test cherry-picks a mixed commit following the advice.

## Context

Deferred finding atelier-99k3 from the atelier-v9ve re-review (approval atelier-bulv).
