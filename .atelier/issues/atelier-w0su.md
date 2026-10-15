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

## Context

`crates/atelier-cli/src/commands/merge.rs` (base check), `crates/atelier-cli/src/commands/session.rs` (`land_base_mismatch`), `crates/atelier-cli/src/commands/start.rs` (base selection), ADR 0016, ADR 0020.
