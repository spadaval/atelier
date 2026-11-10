---
created_at: "2026-11-10T00:56:06.207448953+00:00"
id: "atelier-6sti"
issue_type: "task"
labels:
- "redesign"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-3kr7"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-5fsj"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-c71h"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-jah8"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Write the baseline report, and propose a target for each cost measure"
updated_at: "2026-11-10T00:56:06.207448953+00:00"
---

## Goal

Put the baseline in one place that later missions are judged against, and propose targets for Sumeet to accept or change when he reads this mission's result.

## Done when

- [ ] `docs/architecture/quality/scorecard-baseline-2026-11.md` gives every measure in the product specification's Scorecard table, per repository, with its method in one line, its coverage and its limits, and links the files under `docs/architecture/quality/scorecard-baseline/`.
- [ ] It proposes a target for process share, confusion and tokens per merged item, each with its reason, and states the constraint that defects after merge must not rise. The targets are marked as proposals for Sumeet.
- [ ] The Before column of the Scorecard table in `docs/strategies/redesign-product.md` holds the report's numbers.
- [ ] The report says which numbers differ from the assessment's and why.

## Scope

Assembly and targets. No new measurement.

## Context

Strategy: `docs/strategies/redesign-1-baseline.md`. The assessment's numbers, which the Scorecard table holds today, are in `/var/tmp/atelier-assessment/reports/`.
