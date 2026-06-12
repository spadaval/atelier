---
created_at: "2026-06-12T19:39:47.942112843+00:00"
id: "atelier-diom"
issue_type: "task"
labels:
- "closeout"
- "evidence"
- "validation"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-9pkx"
  - kind: "issue"
    id: "atelier-pyre"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Enforce issue closeout proof gates"
updated_at: "2026-06-12T23:06:00.163574620+00:00"
---

## Description

Make issue closeout enforce the validation policy. A structurally valid issue
with an Outcome and Evidence section should not be closable as "done" when the
required proof is missing or unrelated.

## Outcome

- Issue closeout and equivalent completion paths check the validation routing
  policy for the target issue.
- Closeout blocks when required proof is absent, unrelated to the issue, or only
  recorded as an implementation summary.
- Diagnostics name the missing proof and point to the domain command for
  recording or attaching it.
- Low-risk docs-only, tracker-only, and mechanical work can use a durable note
  when policy allows it; higher-risk work requires first-class evidence or a
  separate validation item.
- Mission and epic closeout can trust closed child issues only after issue
  closeout has checked their required proof.

## Evidence

- CLI integration tests for closing an issue with missing proof, unrelated
  evidence, allowed note proof, and attached first-class evidence.
- Transcript coverage showing the closeout diagnostic and recovery command.
- Regression test proving closed children without required proof do not satisfy
  parent closeout.
