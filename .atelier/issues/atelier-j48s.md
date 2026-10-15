---
created_at: "2026-10-15T23:47:13.909104670+00:00"
id: "atelier-j48s"
issue_type: "bug"
labels:
- "adoption"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Don't advise rebasing a reviewed work branch"
updated_at: "2026-10-15T23:47:13.909104670+00:00"
---

## Goal

`atelier merge` refuses a diverged branch with "merge_diverged: ... Update/rebase the work branch, obtain fresh proof and review". Rebasing rewrites the commits that review records pin, so every finding and approval then "pins a commit outside the work branch" and the branch can never merge without rewriting history again. Merging the base into the work branch keeps the pinned commits and works.

## Done when

`merge_diverged` and the reviews-and-merge docs tell users to merge the base into the work branch once review records exist (rebase only before any review), and the "pins a commit outside the work branch" error explains the cause and the recovery (reset to the pre-rebase tip, then merge the base). A test covers the advice.

## Context

Found 2026-10-15 while landing the adoption fixes: atelier-ha6k and atelier-3mjv were rebased after review findings and had to be reset to their pre-rebase tips and re-merged.
