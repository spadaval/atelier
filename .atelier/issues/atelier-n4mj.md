---
created_at: "2026-11-06T21:26:18.039205049+00:00"
id: "atelier-n4mj"
issue_type: "mission"
labels: []
strategy:
  path: docs/strategies/lighter-workflow.md
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "draft"
title: "Record what proves work, when it can be proven, and nothing more"
updated_at: "2026-11-06T21:26:18.039205049+00:00"
---

## Outcome

Atelier records what proves work, at the time it can be proven, and nothing more. A review finding and what was done about it are one record. Checking a fix is the review of that fix, not a fresh approval of the whole branch. Recording proof and review adds as few commits of its own as the pinning of records allows. An item's proof is split, when it is planned, into what must pass before merge and what can only be shown after, and merged work awaiting validation stays visible until validated. Statuses and issue types are the ones repositories use. No non-trivial change reaches the base without independent review of its code, and every gate that refuses today still refuses. See docs/strategies/lighter-workflow.md.

## Success criteria

- lifecycle-cost: a scenario test of one standard lifecycle (start, evidence, close, review with three findings resolved fix_now, defer and no_action, the fix checked, merge) shows review files per finding of 1 and record-only commits at most half the baseline measured on master at mission start.
- proof-timing: a scenario test shows merge refusing an unproven before-merge criterion, an after-merge criterion without a reason refused, the merged item shown awaiting validation, validation on the base completing it, and a mission refusing to close with open validation unless carried.
- gates-hold: existing gate tests in merge and land still pass, except where an ADR from this mission replaces the rule they test; each such change is listed in the debrief.
