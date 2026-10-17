---
created_at: "2026-10-17T01:10:57.989276399+00:00"
id: "atelier-krf4"
issue_type: "task"
labels:
- "cli-output"
- "cross-cutting"
workflow_branch:
  branch_base: master
  work_branch: task/atelier-krf4
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-o3lm"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Phrase counts as plain words in one order"
updated_at: "2026-10-17T07:15:51.322088093+00:00"
---

## Goal

- About 20 format strings use `(s)` ("6 ready issue(s)", "Lint found 1 issue(s)", "1 ready item(s)", "7 day(s)": status.rs:88/138, session.rs:197/487, issue.rs:1034/1042, prune.rs, git.rs, evidence.rs). `plural_suffix` already exists in human_output.rs.
- Order flips between sibling views: `work missions` "0 active, 2 ready" vs `work mission` "active 0, ready 2" vs `issue show` "Buckets: active 0, ready 2".
- `work mission atelier-r6bt` mixes both orders on one line and gets the number wrong: `Work    : active 0, ready 1, 1 need a human, blocked 0, done 0, backlog 1` (work.rs:406 plus `human_count`, work.rs:153).
- Quiet output has a `context=0` bucket that no human view names.
- `work missions` prints "Unassigned: 7 nonterminal issues" and a zero line, "Linked only to done missions: 0 nonterminal issues".

## Done when

- One order, count first (`2 ready, 3 blocked`), everywhere.
- Real plurals through `plural_suffix`, with no `(s)` left (grep test).
- "needs a human", and "open" instead of "nonterminal".
- Zero-count summary lines are omitted.
- Rendering tests updated.

## Context

Extends atelier-deyy (row tokens).
