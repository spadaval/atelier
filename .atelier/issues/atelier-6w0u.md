---
created_at: "2026-06-12T19:39:42.527274179+00:00"
id: "atelier-6w0u"
issue_type: "task"
labels:
- "docs"
- "process"
- "validation"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-8o34"
  - kind: "issue"
    id: "atelier-diom"
  - kind: "issue"
    id: "atelier-u6ax"
  - kind: "issue"
    id: "atelier-v6nd"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Define validation routing policy for work items"
updated_at: "2026-06-12T19:39:42.527274179+00:00"
---

## Description

Define the proof policy for Atelier work. The policy should keep the operator
model small: ordinary work proves itself on the issue; risky, broad, or
parent-level claims need an independent check.

## Outcome

- Every executable issue type has a default proof expectation and a simple
  escalation rule for higher-risk work.
- The policy states when a durable note is enough, when first-class evidence is
  required, and when a separate validation issue must be created.
- The policy names triggers for independent validation, including migrations,
  public contracts, cross-cutting behavior, mission or epic closeout, docs/help
  parity, stale-test risk, and work where the implementer should not validate
  their own claim.
- Product docs, quality docs, and Agent Factory binding point to the same proof
  model.

## Evidence

- Documentation diff showing the proof policy and examples.
- Example matrix covering a docs-only issue, CLI behavior change, persistence
  migration, Agent Factory process change, epic closeout, and mission closeout.
- Review or validation note confirming the policy is actionable without private
  context.
