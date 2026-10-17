---
created_at: "2026-10-17T15:43:38.142328911+00:00"
id: "atelier-qymu"
issue_type: "task"
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
title: "Integrate this repository's work with merge_commit and the full suite on the merged tree"
updated_at: "2026-10-17T15:43:38.142328911+00:00"
---

## Goal

Switch this repository from `fast_forward_only` to `merge_strategy: merge_commit`, so a reviewed branch that merges cleanly lands without a catch-up merge, and the merged tree is proven by the same checks an implementer runs (decided by Sumeet 2026-10-17 after mission atelier-k8h8).

## Done when

- `.atelier/workflow.yaml` sets `merge_strategy: merge_commit` and `merge_checks` runs, on the merged tree: `cargo fmt -- --check`, `cargo metadata --locked`, `cargo nextest run`, and `cargo nextest run --features test-hooks`, with TMPDIR inside the temporary checkout (not the quota-limited /tmp).
- The checks leave the temporary checkout clean (build output stays in the ignored `target/`).
- `atelier merge <id> --check` passes this policy on a real branch.

## Context

Under `merge_commit`, evidence at the branch head does not prove the merged tree, so the policy must carry a `shell.check` that runs real proof (docs/product/reviews-and-merge.md). Each merge does a cold build in a fresh checkout; that cost is accepted in exchange for no catch-up churn.
