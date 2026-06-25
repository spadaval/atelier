---
created_at: "2026-06-24T19:26:03.410099146+00:00"
id: "atelier-fasv"
issue_type: "epic"
labels: []
review:
  kind: pull_request
  number: 26
  provider: forgejo
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-47cp"
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-z0ll"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
status: "review"
title: "Implement centralized issue read and work view pipeline"
updated_at: "2026-06-25T01:11:24.730475061+00:00"
---

## Description

Build the reusable read pipeline that powers work views, issue detail sections, objective rollups, status panels, and bounded render output. The pipeline consumes typed facts from domain/app services for workflow readiness, work rows, objective scope, evidence gates, relationships, history, checkout/status decisions, bundle previews, issue lifecycle state, and review lifecycle state. It should model sources, selectors/scopes, view models, panels, ordering, grouping, and next actions without exposing internal view terminology as public command language or duplicating domain truth in renderers.

Constraints:
- Start only after the public command/read contract in `atelier-55tk` is settled.
- Do not move workflow, evidence, objective, relationship, history, bundle, review, or issue lifecycle truth into renderers.
- Do not expose view/panel/source internals as public CLI vocabulary.
- Keep full objective-scoped work selection out of this issue unless the contract explicitly requires it.

## Outcome

A shared read/render pipeline powers checkout-oriented status, operational work views, issue detail panels, objective rollups, linked-issue/blocker views, review/evidence panels, ordering, grouping, and next-action hints while keeping internal view terminology out of the public CLI. The pipeline renders domain/app summaries instead of reimplementing workflow, evidence, objective, relationship, history, status, bundle, issue lifecycle, or review decisions in CLI code.

## Evidence

- Focused domain/app tests prove workflow readiness, work rows, objective scope, evidence gates, relationship summaries, history scope, status decisions, bundle summaries, issue lifecycle state, and review state can be evaluated without CLI renderers.
- Focused CLI tests or transcripts prove `status`, `work`, and `issue show` render from the shared pipeline and keep output bounded.
- `rg` proof over CLI renderer modules shows they no longer reimplement the extracted decisions.
