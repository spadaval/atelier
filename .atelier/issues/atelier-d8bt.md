---
created_at: "2026-06-19T20:14:36.021958006+00:00"
id: "atelier-d8bt"
issue_type: "validation"
labels:
- "review"
- "validation"
- "workflow"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Validate transition effects and review integration end to end"
updated_at: "2026-06-19T21:18:31.266485968+00:00"
---

## Description

Independently validate the finished transition-effect and review-integration
mission against the product contract, workflow schema, execution behavior, CLI
output, and docs/help guidance.

## Outcome

- The repository workflow policy uses the explicit effect syntax where the
  mission contract requires it.
- Transition planning, execution, review artifact opening/reuse, blocked output,
  success output, and no-implicit-review-transition behavior are proven by tests
  or command transcripts.
- Documentation and help match the implemented behavior.
- Any remaining provider limitation is recorded as follow-up work with explicit
  scope and not treated as mission completion.

## Evidence

- Validation evidence maps every mission validation bullet to a test, command
  transcript, file diff, or search transcript.
- Required command transcript includes focused workflow/effect tests,
  docs/help parity checks, `atelier lint`, and `git diff --check`.
- `atelier lint atelier-d8bt` passes.
