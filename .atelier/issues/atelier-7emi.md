---
created_at: "2026-10-17T01:10:57.987157293+00:00"
id: "atelier-7emi"
issue_type: "bug"
labels:
- "cli-output"
- "work-views"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-4pzj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Show retired missions as retired and flag the work they leave stranded"
updated_at: "2026-10-17T01:10:57.987157293+00:00"
---

## Goal

After superseding fixture mission atelier-ba3h, `work mission atelier-ba3h` still shows `Ready Work: atelier-9vdd` and suggests "List mission ready work", with no close reason or date. `work missions` says `Linked only to done missions: 1 nonterminal issues` without naming the issue and calls a superseded mission "done". That stranded issue, 9vdd, blocks atelier-1gc5 in the open mission atelier-j76w, and no view connects them.

## Done when

- Done and superseded mission dashboards lead with `superseded 2026-10-16: <close reason>` and list leftover open work under "Left open", without ready hints.
- The overview line reads `Left open by retired missions: atelier-9vdd (blocks atelier-1gc5 in atelier-j76w)`.
- Tests cover a superseded mission with an open item that blocks another mission's work.
