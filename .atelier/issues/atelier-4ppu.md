---
created_at: "2026-10-17T01:36:21.325032540+00:00"
id: "atelier-4ppu"
issue_type: "task"
labels:
- "cli-output"
- "issue-commands"
priority: "P2"
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
status: "backlog"
title: "Say at close what the done claim still needs before it can merge"
updated_at: "2026-10-17T01:36:21.325032540+00:00"
---

## Goal

`atelier issue transition <id> close` succeeds while the work has no independent approval or no passing proof. The gate fires only later, at `land` or `merge`. In several astromech sessions on 2026-10-16, agents closed items with "review outstanding" in the close reason and then hit `review.current` at land. In one chain, the close went through even though the same command's evidence capture had been refused, and the close reason said proof was recorded.

## Done when

- A successful close of a work item prints, after its confirmation, what `land` and `merge` will still require for its owner. Each requirement comes with the command that satisfies it. The requirements are:
  - an independent approval;
  - dispositions for undisposed findings;
  - passing evidence where there is none;
  - a fresh pass where newer evidence failed.
- Nothing extra is printed when every requirement is met.
- Close behaviour is otherwise unchanged; this adds no new refusal.
- Tests cover the outstanding-review, missing-proof and nothing-outstanding cases.

## Context

The confirmation follows atelier-i2qq and atelier-7ebi. From the 2026-10-16 transcript retrospective.
