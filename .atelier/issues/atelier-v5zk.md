---
created_at: "2026-11-09T07:05:36.902156827+00:00"
id: "atelier-v5zk"
issue_type: "task"
labels:
- "agent-factory"
workflow_branch:
  branch_base: master
  work_branch: atelier-v5zk
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-fl12"
    type: "advances"
  - kind: "issue"
    id: "atelier-v3nm"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-09T07:12:08.783667593+00:00"
status: "done"
title: "Keep the two planning reviewers blind to each other's records, not only by timing"
updated_at: "2026-11-09T07:12:08.783667593+00:00"
---

## Goal

atelier-v3nm's replan of atelier-yfqu ran its two planning review passes in parallel in one shared checkout, and they stayed blind to each other only by timing: the Grok reviewer listed .atelier/reviews looking for parallel reviewer records 17 seconds before Sol's appeared. plan.md's Planning Review says the passes see neither the planner's reasoning nor each other's findings, but does not say how. Tell the planner how to keep them blind.

## Done when

- [x] plan.md's Planning Review tells the planner, in one or two plain sentences, how to keep the two passes blind: each reviewer's brief says not to read the other pass's records or thread, and the planner records both passes only after both finish (or each reviewer writes from its own checkout), so neither can see the other's findings while it works.
- [x] references/atelier.md's Planning on the Base stays consistent with it (no change if it already is).

## Scope

skills/agent-factory/procedures/plan.md, and references/atelier.md only if needed.

## Report

Both Done when lines hold. plan.md's Planning Review now tells the planner to
brief each reviewer not to read the other's records or thread, and to give each
its own detached checkout of the planning commits to record in, cherry-picking
both record commits onto the base once both finish. Proof: in a scratch clone of
this repository, `atelier review finding --commit-records` from a
`git worktree add --detach` checkout recorded and committed the finding, and
`git cherry-pick` of that commit onto master made `atelier review list` show it
(pinned to the planning commit). references/atelier.md's Planning on the Base
needs no change: reviewers still record their own `review approve` or `finding`,
and the records still land on the base.

### Decisions

- Chose the own-checkout option over the planner recording both passes after
  they finish: a review record's `--identity` is who records the event, so the
  reviewer should keep recording its own verdict.
