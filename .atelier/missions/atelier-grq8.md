---
created_at: "2026-06-19T18:25:27.000914161+00:00"
id: "atelier-grq8"
labels:
- "mission"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-298c"
    type: "advances"
  - kind: "issue"
    id: "atelier-ie7v"
    type: "advances"
  - kind: "issue"
    id: "atelier-qsmn"
    type: "advances"
  - kind: "issue"
    id: "atelier-um8u"
    type: "advances"
  - kind: "issue"
    id: "atelier-zr6j"
    type: "advances"
schema: "atelier.mission"
schema_version: 1
status: "closed"
title: "Workflow transition effects and explicit review integration"
updated_at: "2026-06-19T21:42:11.757201411+00:00"
---

## Intent

Define and implement explicit workflow transition effects so configured issue
transitions can run bounded, inspectable side effects without hiding workflow
state changes inside review commands. The first concrete integration is review
artifact opening/linking during an explicit issue transition, while review
merge, approval, comments, and status remain review-artifact operations.

## Constraints

- `atelier review` and provider-specific review actions must not start, close,
  block, validate, or otherwise transition Atelier issues as a side effect.
- Transition effects are declared in workflow policy, previewed in transition
  readiness output, and executed only by explicit issue transition commands.
- Validators decide whether a transition is allowed; effects perform bounded
  work after readiness succeeds and must have clear failure and recovery
  behavior.
- The v1 review effect must reuse the configured review mode boundary from ADR
  0011 and must not reintroduce `pr` compatibility aliases or hidden fallback
  behavior.
- Local canonical state, review-provider calls, and activity records must not
  leave operators guessing whether the workflow transition succeeded.

## Risks

- Hidden review side effects could undermine the explicit workflow model.
- Provider-backed review creation cannot be made perfectly atomic with local
  Markdown updates, so preflight, idempotency, and recovery text must be
  designed before implementation.
- A broad generic effect engine could become an unreviewed automation system
  instead of a small workflow-policy mechanism.
- Transition output could become opaque if validators, effects, and next
  commands are not rendered separately.

## Validation

- Product docs and an ADR define transition effects, the review boundary,
  effect failure semantics, and the v1 effect set.
- Workflow schema and planner tests accept declared transition effects, reject
  invalid effects, and expose a deterministic transition plan without executing
  it.
- CLI tests show `atelier issue transition <id> --options`, blocked transition
  output, and successful transition output naming validators, planned effects,
  applied effects, skipped effects, and recovery guidance.
- Review integration tests prove the configured review artifact can be opened
  or reused from an explicit transition effect, while review commands still do
  not transition issue workflow.
- Final validation evidence records focused tests, `atelier lint`,
  `git diff --check`, and docs/help parity for the new transition-effect
  surface.

## Terminal Notes

- Close reason: All linked epics and terminal validation work are complete with passing validation evidence.
