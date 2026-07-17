---
created_at: "2026-07-06T20:37:49.358922409+00:00"
id: "atelier-p4z2"
issue_type: "mission"
labels:
- "agent-factory"
- "mission-review"
- "planning"
- "workflow"
review:
  kind: pull_request
  number: 76
  provider: forgejo
fields:
  workflow_branch:
    branch_base: master
    integration_target: master
    merge_strategy: squash
    owner_issue_id: atelier-p4z2
    owner_kind: mission
    review_target: master
    work_branch: mission/atelier-p4z2
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-6tne"
    type: "advances"
  - kind: "issue"
    id: "atelier-h3wg"
    type: "advances"
  - kind: "issue"
    id: "atelier-p2wk"
    type: "advances"
  - kind: "issue"
    id: "atelier-t876"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-07-17T00:27:55.696014444+00:00"
status: "publish_review"
title: "Mission: Require independent review before mission execution"
updated_at: "2026-07-17T00:27:55.696014444+00:00"
---

## Description

Mission planning becomes a reviewable, non-executable draft phase rather than an informal prelude to execution. Scope includes the durable planning contract, mission and issue-set authoring standards, independent review authority, exact-revision approval freshness, mission and issue dependency gates, operator diagnostics, Agent Factory routing, migration of existing mission readiness state, and independent end-to-end validation.

Non-scope: changing ordinary code-review semantics, requiring planner-authored validation transcripts, adding arbitrary approval workflow graphs, or treating mission-plan approval as merge authority.

Risks: completed repository decisions currently reject mission review states and limit readiness review to Outcome clarity; existing draft and ready missions need an explicit migration policy; the open legacy validator atelier-2uim must validate only the retained parts of those decisions rather than obsolete direct-readiness behavior; author identity and graph revision fingerprints must remain trustworthy across canonical rebuilds; and dependency enforcement cannot detect blockers that planning failed to record.

## Outcome

- A newly authored mission remains non-executable until an independent mission reviewer approves the exact current mission and reachable issue-graph revision; the author or material editor cannot supply that approval.
- Material changes to mission intent, scope, work items, relationships, dependencies, or closeout coverage make prior approval stale and prevent readiness or execution until review is renewed.
- Mission start rejects incomplete declared mission blockers through their transitive closure, and issue execution rejects incomplete declared issue dependencies, while completed prerequisites permit execution.
- Atelier's normal issue detail, work-selection, and transition output identify missing review, stale approval, unresolved findings, and direct or transitive blockers with the next corrective action.
- Agent Factory provides a distinct mission-review procedure and separates plan authorship, independent readiness review, orchestration, code review, and outcome validation.
- A single durable authoring standard explains what good missions, epics, executable issues, validation issues, dependencies, and complete issue sets look like without requiring private chat history.

## Evidence

- `atelier-t876` records a claim map from every mission Outcome line to public command transcripts, mission-plan review artifacts, migration and rebuild results, Agent Factory guidance inspection, and attached first-class evidence IDs.
- `atelier-2uim` independently reviews the contract artifacts for explicit retained, amended, and superseded rules from atelier-a44d, atelier-ql9k, and atelier-1mga before implementation epics can proceed; its evidence record must not validate the obsolete direct `draft -> ready` lifecycle as a target state.
- The end-to-end transcript includes the current regression baseline: a draft mission and its open descendants are not offered by `atelier issue show`, `atelier work ready`, or transition inspection as executable work; the same graph becomes executable only after a distinct reviewer approves its exact revision and declared dependency closure is complete.
- The validator's evidence record classifies every claim and negative scenario as `pass`, `fail`, `blocked`, `deferred`, or `not-applicable` without treating broad suite success or this planning text as execution evidence.
