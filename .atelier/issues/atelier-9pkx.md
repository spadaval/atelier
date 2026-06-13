---
created_at: "2026-06-12T19:39:59.450087808+00:00"
id: "atelier-9pkx"
issue_type: "validation"
labels:
- "reliability"
- "tests"
- "validation"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-v9id"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-12T23:59:50.155104498+00:00"
status: "done"
title: "Validate validation failure scenarios end to end"
updated_at: "2026-06-12T23:59:50.155104498+00:00"
---

## Description

Validate the validation process itself with scenarios that should fail when
agents try to close work without real proof. This is a behavioral check of the
process, not a code review of one implementation slice.

## Outcome

- Scenarios cover missing proof, vague Evidence text, unrelated attached
  evidence, closed children with an unproven parent outcome, broad green tests
  that do not exercise the claim, docs/help drift, and a high-risk slice being
  validated only by its implementer.
- Each scenario is classified as pass, fail, blocked, deferred, or
  not-applicable with a concrete reason.
- Failures create or identify follow-up tracker items before mission closeout.
- The mission-wide adversarial validation pass cannot run as final proof until
  these validation-system scenarios are complete or explicitly deferred with an
  owner.

## Evidence

- Focused tests or transcripts for each validation failure scenario.
- Evidence record attached to this validation issue with the scenario matrix
  and result classifications.
- `atelier link list` or `atelier issue show` transcripts identify follow-up
  items for any scenario that still fails or is deferred.
