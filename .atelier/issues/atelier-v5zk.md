---
created_at: "2026-11-09T07:05:36.902156827+00:00"
id: "atelier-v5zk"
issue_type: "task"
labels:
- "agent-factory"
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
status: "todo"
title: "Keep the two planning reviewers blind to each other's records, not only by timing"
updated_at: "2026-11-09T07:05:36.902156827+00:00"
---

## Goal

atelier-v3nm's replan of atelier-yfqu ran its two planning review passes in parallel in one shared checkout, and they stayed blind to each other only by timing: the Grok reviewer listed .atelier/reviews looking for parallel reviewer records 17 seconds before Sol's appeared. plan.md's Planning Review says the passes see neither the planner's reasoning nor each other's findings, but does not say how. Tell the planner how to keep them blind.

## Done when

- [ ] plan.md's Planning Review tells the planner, in one or two plain sentences, how to keep the two passes blind: each reviewer's brief says not to read the other pass's records or thread, and the planner records both passes only after both finish (or each reviewer writes from its own checkout), so neither can see the other's findings while it works.
- [ ] references/atelier.md's Planning on the Base stays consistent with it (no change if it already is).

## Scope

skills/agent-factory/procedures/plan.md, and references/atelier.md only if needed.
