---
created_at: "2026-10-22T19:58:07.934737612+00:00"
id: "atelier-j8kg"
issue_type: "task"
labels:
- "review"
priority: "P2"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-1mn2"
  - kind: "issue"
    id: "atelier-mbrf"
  - kind: "issue"
    id: "atelier-u6f3"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-1ztv"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Match review effort to the change"
updated_at: "2026-10-22T19:58:17.943603519+00:00"
---

## Goal

Review effort matches the change. Docs-only branches get one round where only high and medium findings block. Low-priority follow-ups share one branch and one approval. Planning text reaches the base without a full work-branch cycle.

## Done when

- Each child's Done when holds, shown by the Agent Factory skill text (vendored and upstream in spadaval/.agents) and land tests, with independent review.
- Delivers atelier-k8h8 S8's review-budget part (atelier-1mn2, atelier-u6f3) and S9's planning part (atelier-mbrf).

## Unknowns

atelier-mbrf offers two rules: policy-named planning paths, or a human-approval review event. Choosing one changes what needs independent review, so the human decides before this increment is planned into todo.

## Background

atelier-u6f3's one approval covering several children, and atelier-mbrf's planning paths, change the review gate. They come after the gate-agreement increment.
