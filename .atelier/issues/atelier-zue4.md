---
created_at: "2026-06-12T05:11:44.119955442+00:00"
id: "atelier-zue4"
issue_type: "epic"
labels:
- "evidence"
- "reliability"
- "rework"
- "validation"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-n1ys"
  children:
  - kind: "issue"
    id: "atelier-0u2k"
  - kind: "issue"
    id: "atelier-1p83"
  - kind: "issue"
    id: "atelier-7yen"
  - kind: "issue"
    id: "atelier-8o34"
  - kind: "issue"
    id: "atelier-fx9r"
  - kind: "issue"
    id: "atelier-g18z"
  - kind: "issue"
    id: "atelier-hah9"
  - kind: "issue"
    id: "atelier-k9m8"
  - kind: "issue"
    id: "atelier-l0yk"
  - kind: "issue"
    id: "atelier-qb7m"
  - kind: "issue"
    id: "atelier-u6ax"
  - kind: "issue"
    id: "atelier-v9id"
  - kind: "issue"
    id: "atelier-w8e3"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Overhaul mission validation and reliability system"
updated_at: "2026-06-12T17:07:36.333322708+00:00"
---

## Description

Build a reliability layer for missions so agents cannot declare work complete
without proving the desired outcomes. This epic coordinates issue structure,
lint, validators, evidence capture, Agent Factory validation process, closeout
audits, and drift detection.

## Outcome

- Executable work has clear Outcome and Evidence sections.
- Lint catches vague or missing evidence expectations before work starts.
- Mission closeout maps every mission outcome to linked work and attached
  evidence.
- Validators are target-aware and use domain-specific closeout policy.
- Evidence capture is easy enough that agents attach real command transcripts
  instead of summaries.
- Validation subagents verify claims line by line instead of running broad tests
  and assuming success.
- A mission-level adversarial validation pass tries to disprove every claimed
  repair before closeout.
- Epics and missions require independent closeout audits, not just closed
  children.
- Docs, help output, tests, and Agent Factory guidance must agree before
  closeout.
- Ignored or stale tests are visible blockers with owners.
- CLI next actions are context-aware and covered by transcript tests.
- Malformed canonical records do not disable orientation or repair commands;
  they are reported as bounded, actionable degraded-state blockers.
- Projection-backed commands rebuild or fail with bounded, actionable diagnostics
  instead of dumping stale index internals or rebuild races.

## Evidence

- Child issues cover lint, validators, evidence capture, validation process,
  closeout audit, adversarial mission validation, malformed-record recovery,
  projection freshness, drift detection, ignored tests, next actions, and
  reliability status.

- Final validation attaches evidence records for command transcripts, focused
  tests, docs/help parity checks, stale-test inventory, and mission contract
  audit results.

- Mission `atelier-tcmr` cannot close until this reliability work is validated.

## Notes

This epic is intentionally broader than one command. The failure mode was

systemic: agents could close work without proof because tracker structure,

commands, docs, validators, and skill process all allowed it.
