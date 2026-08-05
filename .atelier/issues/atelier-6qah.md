---
created_at: "2026-07-28T16:39:31.725432928+00:00"
id: "atelier-6qah"
issue_type: "epic"
labels:
- "agent-factory"
- "docs"
- "validation"
review:
  kind: pull_request
  number: 78
  provider: forgejo
fields:
  workflow_branch:
    branch_base: mission/atelier-5kej
    integration_target: mission/atelier-5kej
    merge_strategy: squash
    owner_issue_id: atelier-6qah
    owner_kind: epic
    review_target: mission/atelier-5kej
    work_branch: epic/atelier-6qah
priority: "P2"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-2pvp"
  - kind: "issue"
    id: "atelier-ekg7"
  - kind: "issue"
    id: "atelier-lho9"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-08-05T20:39:28.651591035+00:00"
status: "done"
title: "Align Agent Factory guidance and prove the adaptive loop"
updated_at: "2026-08-05T20:39:28.651591035+00:00"
---

## Description

Bring the repository-owned Agent Factory binding, Atelier help, and end-to-end behavior into agreement after the product increments settle.

Known constraints: Agent Factory owns portable orchestration; Atelier owns repository-specific tracker mechanics; neither should duplicate the other's command manual.

Expansion condition: expand when the legibility and replanning outcomes are implemented and stable enough for scenario validation.

## Outcome

- The embedded Agent Factory binding routes strategy, replanning, and expansion behavior through the correct Atelier surfaces.
- A fresh agent can create a strategic mission, execute the current horizon, expand an epic from evidence, reconcile changed work, and continue without private context.
- Residue checks find no conflicting frozen-plan, whole-graph-completeness, or planning-approval guidance.

## Evidence

- `atelier-lho9` records semantic parity across the repository Agent Factory binding, Atelier role guidance, and durable product/architecture sources.
- `atelier-ekg7` independently executes the complete adaptive loop from a disposable repository at the exact candidate revision.
- Focused command-surface and tracker checks pass, and any contradictory behavior is repaired before this epic closes.
