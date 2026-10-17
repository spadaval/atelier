---
created_at: "2026-10-17T10:00:37.726457567+00:00"
id: "atelier-sl01"
issue_type: "bug"
labels:
- "cli-output"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-uf97"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Say a transition needing an argument the same way when it runs and when it is shown"
updated_at: "2026-10-17T10:00:37.726457567+00:00"
---

## Goal

Mission atelier-o3lm validation (evidence atelier-ba8q) found two wording gaps:
- Views call `supersede` allowed and needing `--reason`, but running it without `--reason` says `Transition 'supersede' is blocked … missing required field close_reason`.
- `human-cli-output.md` says a refused `start` or `issue transition` "prints no transition report", which holds only under `-q`; without it the report still prints on stdout.

## Done when

- A transition run without its required argument refuses with wording that matches "allowed, needs --reason" (for example "needs --reason") and one recovery step.
- The doc sentence says exactly when the report is printed.
- Tests cover both.
