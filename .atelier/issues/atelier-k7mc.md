---
created_at: "2026-07-28T16:39:31.730334911+00:00"
id: "atelier-k7mc"
issue_type: "epic"
labels:
- "agent-factory"
- "planning"
- "product-contract"
review:
  kind: pull_request
  number: 77
  provider: forgejo
fields:
  workflow_branch:
    branch_base: mission/atelier-5kej
    integration_target: mission/atelier-5kej
    merge_strategy: squash
    owner_issue_id: atelier-k7mc
    owner_kind: epic
    review_target: mission/atelier-5kej
    work_branch: epic/atelier-k7mc
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-2a1t"
  - kind: "issue"
    id: "atelier-564h"
  - kind: "issue"
    id: "atelier-8zyx"
  - kind: "issue"
    id: "atelier-f9hk"
  - kind: "issue"
    id: "atelier-iznq"
  - kind: "issue"
    id: "atelier-js22"
  - kind: "issue"
    id: "atelier-jwex"
  - kind: "issue"
    id: "atelier-puot"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Define the adaptive mission contract"
updated_at: "2026-08-05T19:19:37.543529637+00:00"
---

## Description

Own the durable product and architecture boundary between governing strategy, outcome-bearing epics, and the evidence-backed issue horizon. This epic is the first expanded evidence horizon.

## Outcome

- Product and architecture sources state what belongs in strategy, mission and epic outcomes, executable issues, and evidence.
- The contract defines typed strategy references, draft-to-ready expansion, routine graph maintenance, and preservation of completed or superseded history.
- The anti-red-tape boundary explicitly excludes plan approval, actor authorization, graph digests, frozen plans, and mandatory speculative decomposition.

## Evidence

- `docs/architecture/quality/adaptive-mission-gap-analysis.md` maps the required scenarios to existing and missing behavior.
- Focused workflow, record-store, ready-queue, bundle, and mission-dashboard tests prove the implemented contract.
- `atelier-2a1t` independently validates the resulting contract against the strategy and Zen.
