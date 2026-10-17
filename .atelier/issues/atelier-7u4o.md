---
created_at: "2026-10-16T23:04:58.306346849+00:00"
id: "atelier-7u4o"
issue_type: "feature"
labels:
- "orientation"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-jcki"
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-1w8c"
    type: "advances"
  - kind: "issue"
    id: "atelier-deyy"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Make issue show brief by default and accept several IDs"
updated_at: "2026-10-16T23:04:58.306346849+00:00"
---

## Goal

Agents trim `issue show` with `| head`, `| sed -n 1,40p` and section-extracting `sed` ranges, and loop over IDs to piece a hierarchy together. The default view is too long for routine use (about 10 lines of transition readiness alone), and there is no way to see several items at once.

## Done when

`issue show <id>` defaults to a dense view: title line, status, parent and children, blockers, mission, Goal and Done when, and a one-line transition summary (allowed transitions by name; blocked ones with their reason); `--full` gives today's view. `issue show <id> <id>...` prints each in the brief view. Tests cover both, and `docs/product/human-cli-output.md` describes the brief view.

## Scope

`issue show` rendering, its flags, and tests.

## Context

ADR 0022 output principle. Transcript evidence: `issue show ... | sed -n '1,30p'`, `sed -n '/Hierarchy/,/^$/p'` loops.
