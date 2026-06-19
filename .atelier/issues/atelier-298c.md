---
created_at: "2026-06-19T18:26:03.985884754+00:00"
id: "atelier-298c"
issue_type: "epic"
labels:
- "cli"
- "review"
- "workflow"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-qsmn"
  children:
  - kind: "issue"
    id: "atelier-s8n2"
  - kind: "issue"
    id: "atelier-wxj5"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Epic: CLI surface cleanup for issue transitions"
updated_at: "2026-06-19T21:24:03.940772641+00:00"
---

## Description

Clean up issue transition and review-facing CLI output so operators can see
what a transition will validate, what effects it will run, and what recovery
steps exist when an effect cannot complete.

## Outcome

- `atelier issue transition <id> --options` renders validators and planned
  effects as separate concepts with bounded, scan-friendly text.
- Blocked transition output names failed validators, blocked effects, required
  fields, and the next command to repair the issue.
- Successful transition output reports the status change, applied effects,
  skipped effects, and any review artifact that was opened or reused.
- Product help and role guidance point to explicit issue transitions for
  workflow movement and to `atelier review` for review artifact discussion,
  approval, and merge.

## Evidence

- CLI integration tests or golden output cover options, blocked transition, and
  successful transition rendering for transitions with and without effects.
- Documentation/help transcript shows no guidance that review commands or PR
  provider actions transition Atelier workflow.
- `atelier lint atelier-298c` passes after child work lands.
