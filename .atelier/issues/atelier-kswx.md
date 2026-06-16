---
created_at: "2026-06-16T15:59:29.434936279+00:00"
id: "atelier-kswx"
issue_type: "task"
labels:
- "cli"
- "dependencies"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-3s9y"
  - kind: "issue"
    id: "atelier-d226"
  - kind: "issue"
    id: "atelier-em15"
  - kind: "issue"
    id: "atelier-f7vd"
  - kind: "issue"
    id: "atelier-kzfl"
  - kind: "issue"
    id: "atelier-nqjc"
  - kind: "issue"
    id: "atelier-qh52"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Define shared blocker-aware work ordering"
updated_at: "2026-06-16T17:59:15.673687157+00:00"
---

## Description

Define the shared blocker-aware ordering and row-state display contract before updating individual command views. The goal is a reusable rule for ordered issue-like rows and readable action labels, not six local sort implementations with subtle disagreement.

## Outcome

- Target-state docs or architecture notes define how visible open blocker edges affect row order: blockers precede blocked work when both are present in the same visible set.
- The contract names how done blockers, hidden blockers outside the current view, parent/child hierarchy, workflow buckets, priority, update time, and deterministic fallback ordering interact.
- The contract defines the row state vocabulary for work views: ready, blocked, active, review, validation, done, or not-ready where needed.
- Human queue rows no longer print duplicate `category/status` tokens such as `todo/todo`; exact configured workflow status is shown only where it adds information or in detail views.
- Blocked rows use the blocked state plus a compact count or drill-down cue by default, while detail surfaces preserve exact blocker IDs for repair.
- A shared helper or equivalent reusable implementation exists for command code that orders issue-like work sets.
- Cycles or invalid dependency projections are handled deterministically with repair-oriented output rather than panics or missing rows.
- Surface issues under this epic can depend on the same contract instead of redefining the rule.

## Evidence

- Documentation file diff or code comments identify the shared ordering contract and the command surfaces expected to use it.
- Documentation file diff or focused row-rendering tests identify the shared row-state grammar and the surfaces expected to avoid duplicate category/status tokens.
- Focused tests cover a simple chain, a diamond dependency graph, a hidden blocker outside the visible set, a done blocker, deterministic tie breaking, and invalid or cyclic dependency data.
- Focused tests cover row-state labels for ready, blocked, active, review, validation, and done work.
- `atelier lint atelier-kswx`, `atelier export --check`, `cargo fmt -- --check`, focused cargo tests, and `git diff --check` pass.

## Notes

This is the contract-first blocker for the implementation slices. Keep the rule small: order visible work more helpfully, then leave deeper scheduling policy to explicit blockers, workflow state, and mission status.
