---
created_at: "2026-10-17T01:10:57.965792795+00:00"
id: "atelier-jcki"
issue_type: "feature"
labels:
- "cli-output"
- "issue-commands"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-uf97"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Show ownership, proof, review state and mission in issue show"
updated_at: "2026-10-17T01:10:57.965792795+00:00"
---

## Goal

`docs/product/human-cli-output.md` says detail views show "proof, and ownership". `atelier issue show` (`render_issue_show_human`, `crates/atelier-cli/src/commands/issue.rs:76-149`) shows neither:
- No owner branch. atelier-28mz has `workflow_branch: task/atelier-28mz` (base master), and show prints nothing about it.
- No evidence and no reviews. After 10 review events and 8 receipts on one item, `issue show` mentions neither.
- No inherited mission. A child whose parent advances a mission shows no mission, although the inventory doc says "A linked parent includes its children in mission execution context".

Agents need `evidence list`, `review list`, `git branch` or the parent's show to fill the gaps.

## Done when

`issue show` prints, working from any branch:
- `Branch: task/atelier-28mz (base master)`, or `Branch: owned by atelier-xxxx (task/atelier-xxxx)` for a child.
- `Mission: atelier-1w8c "…"`, directly or `via parent atelier-xxxx`.
- `Review: ✓ approved atelier-hleo @c24af21 · findings 3/3 disposed` (or `⚠ stale since <commit>` / `⚠ 1 open finding` / `none`).
- `Proof: ✓ atelier-b7zn pass fresh` (or `⚠ newer fail atelier-…` / `none`).

CLI tests cover an owner with review and evidence, a child with an inherited mission, and an item with neither.

## Context

Blocked by atelier-7u4o, which reshapes `issue show` into the brief view; these lines belong in that brief view. Review/evidence freshness logic already exists for `land` and `merge --check` (`atelier_app::evidence_freshness`).
