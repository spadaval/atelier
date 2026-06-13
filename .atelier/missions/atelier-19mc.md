---
created_at: "2026-06-13T02:33:40.288462525+00:00"
id: "atelier-19mc"
labels:
- "mission"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-2wbz"
    type: "advances"
  - kind: "issue"
    id: "atelier-bfuv"
    type: "advances"
  - kind: "issue"
    id: "atelier-ey4y"
    type: "advances"
  - kind: "issue"
    id: "atelier-qf35"
    type: "advances"
schema: "atelier.mission"
schema_version: 1
status: "ready"
title: "Overhaul proof, evidence, delegation, and operator CLI"
updated_at: "2026-06-13T02:35:51.356708440+00:00"
---

## Intent

Make Atelier's operating model cheaper, clearer, and harder to fool. The mission defines strong proof, redesigns evidence capture around claim mapping, improves Agent Factory delegation and model routing, and collapses mission/operator CLI surfaces into fewer contextual commands.

## Constraints

- Use docs-first contracts for workflow policy, public CLI semantics, evidence schema, and Agent Factory rules before implementation.
- Use focused tests or command transcripts for each changed public behavior; broad suites are supporting evidence, not proof by themselves.
- Default subagent assignments to cheaper/faster models unless a higher-reasoning model is justified by ambiguity or cross-cutting risk.
- Prefer fewer, stronger operator commands over new narrow commands; normal users should not need raw workflow diagnostics for closeout.

## Risks

- Changing proof and evidence policy can make existing tracker records appear under-proven unless migration and compatibility expectations are explicit.
- Collapsing command surfaces can hide useful drill-down data unless compact and verbose modes are designed together.

## Validation

- Mission defines a durable strong-proof contract and updates validation guidance so weak proof is distinguishable from strong proof, including support for subjective qualitative validation and hard quantitative validation where practical.
- Evidence records can map proof to concrete issue, epic, and mission claims, with command-backed capture for transcripts and tests.
- Mission/operator CLI gives a concise default answer for state, blockers, missing evidence, next action, and closeout readiness, with verbose audit drill-down available when needed.
- Agent Factory guidance routes delegation by bounded slice, proof need, and model cost, with 5.4 Mini as the default for routine evidence-producing work.
- Focused tests, docs/help parity checks, tracker lint/export/doctor, and an independent closeout validation prove the new model end to end.
