---
created_at: "2026-06-16T15:59:35.972443477+00:00"
id: "atelier-qh52"
issue_type: "feature"
labels:
- "cli"
- "dependencies"
- "graph"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-em15"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Sort graph tree children by blocker order"
updated_at: "2026-06-16T15:59:35.972443477+00:00"
---

## Description

Update hierarchy rendering so `atelier graph tree` presents children in dependency order where sibling blocker edges exist and labels row state without leaking workflow internals. The graph view should remain an inspection surface, but its order should not fight the dependency graph.

## Outcome

- `atelier graph tree` orders sibling issue children with visible open blockers before the work they unblock.
- `atelier graph tree --compact` uses the same child ordering as the default tree output.
- Tree rows use compact readable state labels and avoid duplicate `category/status` tokens.
- Blocked tree rows show blocked state or count; exact blocker IDs are available through blocker/detail drill-down rather than as long inline suffixes.
- Parent/child hierarchy remains the primary shape; dependency order only changes sibling ordering within the visible hierarchy.
- Cyclic or invalid dependency data produces deterministic output with enough issue IDs for repair.

## Evidence

- CLI test or transcript shows full tree output for a parent whose children include a blocker chain and prints the blocker first.
- CLI test or transcript shows compact tree output uses the same child order.
- CLI transcript or focused test for `atelier graph tree` proves tree rows use readable state labels and do not contain duplicate category/status tokens.
- Regression test proves unrelated hierarchy depth and parent labels remain unchanged.
- `atelier lint atelier-qh52`, `atelier export --check`, focused cargo tests, and `git diff --check` pass.

## Notes

Do not make `graph tree` compute the full execution plan. It only needs a more helpful order for visible siblings.
