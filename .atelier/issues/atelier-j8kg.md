---
created_at: "2026-10-22T19:58:07.934737612+00:00"
id: "atelier-j8kg"
issue_type: "task"
labels:
- "review"
workflow_branch:
  branch_base: master
  work_branch: task/atelier-j8kg
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
closed_at: "2026-10-22T23:22:42.532156031+00:00"
status: "done"
title: "Match review effort to the change"
updated_at: "2026-10-22T23:22:42.532156031+00:00"
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

## Close Reason

### Outcome
Review costs no more ceremony than the change needs, under one standard for code and prose. A trivial change lands on the base with a visible exemption pinned to exactly its commit, with no owner issue or work branch: `atelier review trivial --base <commit>`; land counts only that commit, never a later one or a done claim, and review list, review show and debriefs show it (atelier-mbrf). Low-priority follow-ups batch as children of one parent with one approval; Atelier already supported this, so the Agent Factory orchestrate guidance now says so and a test guards it (atelier-u6f3). Delivers atelier-k8h8 S8's shared-review part and S9's base-exemption part.

### Deviations
u6f3 needed no Atelier code. `review trivial` now takes one or two positionals so `--base` can omit the owner; existing invocations are unchanged.

### Decisions
See the decision notes on atelier-mbrf (storage as a base-targeted trivial_exemption; debrief time window) and atelier-u6f3.

### Surprises
Clap's optional-positional mode misread the existing `trivial <id> --identity X "why"` form, which shaped the argument change.

### Open items
Older binaries reject a review file without `target`; reinstall after merge, before anyone records a base exemption. The vendored skill edits need syncing upstream.
