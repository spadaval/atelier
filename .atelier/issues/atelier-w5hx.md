---
created_at: "2026-10-30T06:52:24.678869557+00:00"
id: "atelier-w5hx"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-6306"
    type: "implements"
  - kind: "issue"
    id: "atelier-r6bt"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Remove one-way decision notes and migrate the existing ones"
updated_at: "2026-10-30T06:52:24.678869557+00:00"
---

## Goal

Carry out the decision to remove one-way decision notes: one way to record a costly-to-reverse choice, the `decision` record.

## Done when

`atelier issue note --help` has no `--one-way`, and passing it is a usage error; `work decisions --help` and its output list decision records first, then notes, with no one-way group; the debrief has no one-way section; the six `[decision one-way]` notes in atelier-fip8, atelier-ho7o and atelier-sfsq read `[decision]` and `git grep -n "one-way\|one_way\|ONE_WAY" crates docs/product` prints nothing; `atelier check`, `cargo nextest run` and the test-hooks suite pass.

## Scope

crates/atelier-core (note tag), atelier-app decisions.rs and mission_review.rs, atelier-cli issue_cli.rs, issue.rs, work.rs, mission_review.rs, human_output.rs; docs/product/work-model.md, cli-surface.md, human-cli-output.md, mission-reviews.md; the one-way lines in docs/strategies/review-and-introspection.md (atelier-r6bt's strategy), recording the change and its authority there.

## Context

Decision record linked as `implements`. Skill rule: `skills/agent-factory/references/tracker-records.md`, Decision Notes, in spadaval/.agents.
