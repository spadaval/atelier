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
status: "todo"
title: "Support a mission integration branch as a merge base"
updated_at: "2026-10-15T19:53:46.480046082+00:00"
---

## Goal

Agent Factory allows at most one integration branch per mission, and its migrate procedure relies on one for isolated rewrites. Atelier `merge` and `land` reject any owner whose recorded base differs from the policy `base_branch`, while `start` records a non-base current branch as the base. Decide and implement how a mission integration branch serves as a gated merge base and how it later integrates into the default branch.

## Done when

An epic started from a mission integration branch passes `land` and `atelier merge` into that branch under base policy gates, and the integration branch itself merges into the default branch through a gated path; tests cover both, or a decision record explains why the skill should stop recommending integration branches instead.

## Design

Needs a decision from Sumeet (2026-10-16, orchestrator); not built. The Done when allows either building gated integration branches or a decision record that the skill should stop recommending them.

Since this was filed, the vendored Agent Factory skill (atelier-92mz) already says not to plan a mission integration branch or isolated rewrite branch under gated merge, and to ask the human. Merging the base into reviewed branches now keeps approvals (atelier-n7g1, atelier-1isk), so long-lived parallel branches on the default base cost less than they did, which weakens the case for integration branches.

Building it needs: `start` recording an integration branch as a base; `land` and `merge` accepting an owner whose base is a branch named as a mission's integration branch, with that branch's committed policy and gates; and a gated path from the integration branch to the default branch (its own owner, presumably the mission, reviewed as a whole). Roughly the size of atelier-n7g1 plus atelier-3mjv together.

Recommendation: record a decision that Atelier supports one base per repository for now, and keep the skill's current guidance. Revisit when a mission needs an isolated rewrite that cannot use expand-and-contract.

## Context

`crates/atelier-cli/src/commands/merge.rs` (base check), `crates/atelier-cli/src/commands/session.rs` (`land_base_mismatch`), `crates/atelier-cli/src/commands/start.rs` (base selection), ADR 0016, ADR 0020.
