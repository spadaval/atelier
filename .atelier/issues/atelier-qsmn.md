---
created_at: "2026-06-19T18:26:11.587935915+00:00"
id: "atelier-qsmn"
issue_type: "epic"
labels:
- "migration"
- "validation"
- "workflow"
review:
  kind: pull_request
  number: 6
  provider: forgejo
priority: "P2"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-d8bt"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-19T21:39:13.463876106+00:00"
status: "done"
title: "Epic: Validation and workflow migration for transition effects"
updated_at: "2026-06-19T21:39:13.463876106+00:00"
---

## Description

Validate the completed transition-effect workflow and migrate repository policy,
docs, examples, and tests to the explicit effect model.

## Outcome

- The repository workflow configuration and docs use the explicit transition
  effect contract where review artifact opening belongs on an issue transition.
- Existing workflow behavior without effects remains unchanged except where the
  product contract intentionally changes transition output or review guidance.
- End-to-end validation proves the transition planner, effect engine, CLI output,
  review integration, docs/help, and tracker health agree.

## Evidence

- Independent validation evidence maps each mission validation criterion to
  command transcripts or focused test output.
- Required closeout commands include focused workflow/effect tests,
  docs/help parity checks, `atelier lint`, and `git diff --check`.
- Residual unsupported review modes or provider limitations are recorded as
  explicit follow-up work, not hidden assumptions.
