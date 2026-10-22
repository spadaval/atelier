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

Review costs no more ceremony than the change needs, under one standard for code and prose. Low-priority follow-ups share one branch and one approval. A trivial change lands on the base with a visible exemption, with no issue or work branch.

## Done when

- Each child's Done when holds, shown by the Agent Factory skill text (vendored and upstream in spadaval/.agents) and land tests, with independent review.
- Delivers atelier-k8h8 S8's shared-review part (atelier-u6f3) and S9's base-exemption part (atelier-mbrf).

## Context

On 2026-10-22 the human decided that review keeps one standard for code and prose. atelier-1mn2 (lighter review for docs-only changes) was superseded, and atelier-mbrf was reframed as an owner-free trivial exemption.

## Background

atelier-u6f3's one approval covering several children, and atelier-mbrf's base-commit exemption, change the review gate. They come after the gate-agreement increment.
