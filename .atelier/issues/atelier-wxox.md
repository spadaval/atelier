---
created_at: "2026-06-23T15:22:38.615287377+00:00"
id: "atelier-wxox"
issue_type: "feature"
labels:
- "cli"
- "human-output"
- "issue"
- "workflow-state"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-3js3"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-23T23:02:41.543550918+00:00"
status: "done"
title: "Refresh issue detail and transition-option output"
updated_at: "2026-06-23T23:02:41.543550918+00:00"
---

## Description

Refresh the detail and workflow-decision surfaces.

## Outcome

- `issue show` summarizes dirty checkout state and renders recent activity as
  human sentences.
- `issue transition --options` starts with allowed or blocked decisions, then
  required inputs, blockers, planned actions, and commands.
- Detail and transition views prioritize command correctness and public recovery
  guidance: dirty checkout summaries must agree with Git, status-like detail
  must reflect current records, and config/parser failures must explain the
  current public recovery path without exposing internal storage/index details.
- Removed-command and blocked-transition guidance points at one current
  replacement path when one exists, rather than surfacing duplicate lifecycle
  verbs.
- Long dirty path lists and raw activity fields move behind focused drill-downs
  or verbose output.

## Evidence

- Before/after transcripts cover an active dirty checkout, a blocked close
  transition, stale status-like output or config failure recovery, recent
  evidence activity, and clean colorless output.
- Focused tests cover dirty path list bounding, transition footer clarity, and
  preservation of required next commands, plus public recovery guidance before
  lower-priority detail.
- `target/debug/atelier lint`, focused CLI tests, and `git diff --check` pass.
