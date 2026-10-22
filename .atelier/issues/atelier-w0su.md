---
created_at: "2026-10-15T19:53:46.480046082+00:00"
id: "atelier-w0su"
issue_type: "feature"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Support a mission integration branch as a merge base"
updated_at: "2026-10-16T21:57:35.557263517+00:00"
---

## Goal

Agent Factory allows at most one integration branch per mission, and its migrate procedure relies on one for isolated rewrites. Atelier `merge` and `land` reject any owner whose recorded base differs from the policy `base_branch`, while `start` records a non-base current branch as the base. Decide and implement how a mission integration branch serves as a gated merge base and how it later integrates into the default branch.

## Done when

An epic started from a mission integration branch passes `land` and `atelier merge` into that branch under base policy gates, and the integration branch itself merges into the default branch through a gated path; tests cover both, or a decision record explains why the skill should stop recommending integration branches instead.

## Context

`crates/atelier-cli/src/commands/merge.rs` (base check), `crates/atelier-cli/src/commands/session.rs` (`land_base_mismatch`), `crates/atelier-cli/src/commands/start.rs` (base selection), ADR 0016, ADR 0020.

## Design

Deferred by Sumeet (2026-10-16): an advanced feature, planned now and built later. The primary use case is working on several missions in parallel, each in its own worktree, without their unfinished work meeting on the default branch.

### Model

- A mission owns an integration branch, `mission/<id>`, recorded as the mission's `workflow_branch` with the default branch as its base. `atelier start <mission>` creates it; missions are currently exempt from branch ownership.
- Work that `advances` the mission is started from the mission branch, and its `branch_base` is `mission/<id>`. It merges into the mission branch through the same gates as today (`land`, `atelier merge`, review, proof, fast-forward), so reviewed slices accumulate there.
- The trusted policy is always the default branch's, even for gates into a mission branch, so a mission branch cannot weaken its own merge policy. Policy changes reach mission branches by merging the default branch in, which n7g1 and 1isk already keep cheap.
- `atelier merge <mission>` integrates the mission branch into the default branch. Gates: all scoped work terminal; every non-record commit on the mission branch covered by an approval of an item that merged into it (the check `land` on the base already makes for unpushed base commits); merges of the default branch into it clean; the mission's own done claim and proof; and a mission-level independent validation if its Success criteria call for one.
- `land` on a mission branch behaves as on a base: planning commits (records, strategy) may go there directly, other commits must be covered by a reviewed item.
- `prime`, `status` and the merged label say "merged into mission/<id>, not into <default>" for work that reached the mission branch only.

### Slices

1. Decision record: the trust model above (default branch's policy everywhere; which branches count as integration bases and how they are registered) and whether a mission merge needs its own review.
2. `atelier start` for missions (integration branch and record) and for work started from a mission branch.
3. `land` and `atelier merge` accept a recorded mission integration base for work that advances that mission; base-commit checks on mission branches.
4. `atelier merge <mission>` into the default branch, with the gates above.
5. Labels in `prime`/`status`, docs (reviews-and-merge, work-model, sessions-and-hooks), and an Agent Factory skill resync: its "Atelier Limits" section says integration branches are unsupported (ADR 0020).

### Open questions

- Can work advance more than one mission? It then needs one integration base; the simplest rule is the mission it was started from.
- Is a mission branch kept after merging, or deleted like work branches?
- Do two missions ever need each other's unmerged work (stacking), or is that ruled out?
