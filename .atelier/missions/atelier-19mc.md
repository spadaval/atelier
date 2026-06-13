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
  - kind: "issue"
    id: "atelier-sv98"
    type: "advances"
schema: "atelier.mission"
schema_version: 1
status: "ready"
title: "Overhaul proof, evidence, delegation, and operator CLI"
updated_at: "2026-06-13T02:52:21.324153629+00:00"
---

## Intent

Make Atelier's operating model cheaper, clearer, and harder to fool. The mission defines strong proof, redesigns evidence capture around accountable work, improves Agent Factory delegation and model routing, and collapses the operator CLI into fewer contextual commands that real humans and agents can use without ceremony.

## Constraints

- Use docs-first contracts for workflow policy, public CLI semantics, evidence schema, and Agent Factory rules before implementation.
- Use focused tests or command transcripts for each changed public behavior; broad suites are supporting evidence, not proof by themselves.
- Use cheaper/faster models only when the orchestrator judges the assignment is bounded, low-ambiguity, and low-risk; use higher-reasoning models for complex open-ended implementation, review, architecture, and hard debugging.
- Prefer fewer, stronger operator commands over new narrow commands; normal users should not need raw workflow diagnostics for closeout.
- Treat simplicity as a safety requirement: if a process or command shape makes routine work easier to misuse, redesign it before relying on training or reminders.

## Risks

- Changing proof and evidence policy can make existing tracker records appear under-proven unless migration and compatibility expectations are explicit.
- Collapsing command surfaces can hide useful drill-down data unless compact and verbose modes are designed together.

## Validation

- Mission defines a durable strong-proof contract and updates validation guidance so weak proof is distinguishable from strong proof, including support for subjective qualitative validation and hard quantitative validation where practical.
- Mission defines where validation requirements belong across mission, epic, issue, and validation-item layers, with explicit anti-red-tape guidance to prevent duplicated requirements.
- Evidence records attach to accountable work rather than mission objectives; mission readiness derives proof from linked implementation, validation, review, and closeout work.
- Evidence recording is one simple operator workflow for manual summaries and command transcripts.
- Mission/operator CLI gives a concise default answer for state, blockers, missing evidence, next action, and closeout readiness, with verbose audit drill-down available when needed.
- Broad command consolidation removes, hides, or merges duplicate surfaces so normal workflows are simple and context-driven.
- Agent Factory guidance routes delegation by bounded slice, proof need, model capability, and cost, with clear caution that 5.4 Mini is suitable for basic bounded validation or refactor-style work only when orchestrator judgment says the task fits.
- Focused tests, docs/help parity checks, tracker lint/export/doctor, and an independent closeout validation prove the new model end to end.
