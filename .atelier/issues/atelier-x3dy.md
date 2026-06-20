---
created_at: "2026-06-19T19:39:31.116857580+00:00"
id: "atelier-x3dy"
issue_type: "feature"
labels:
- "cli"
- "prune"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-iq7f"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Add prune inventory and dry-run report"
updated_at: "2026-06-19T19:39:31.116857580+00:00"
---

## Description

Add root `atelier prune` as an inventory and dry-run report. This should be
useful before any destructive pruning exists and should become the shared
candidate engine for later apply paths.

## Outcome

- Root `atelier prune` reports candidates across the retention classes selected
  by the contract.
- Dry-run output includes candidate kind, ID or path, age or lifecycle reason,
  protected/eligible status, skip reason when protected, and the apply command
  or required flag when action is possible.
- The command refuses ambiguous selectors and never mutates tracked records,
  local files, branches, or worktrees in dry-run mode.

## Evidence

- Focused CLI tests cover candidate rendering, protected skip reasons,
  selector validation, and no mutation during dry-run.
- Command transcript shows representative dry-run output for evidence,
  activity, local diagnostics/cache, branch, and worktree candidates.
