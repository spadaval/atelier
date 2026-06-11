---
acceptance: []
created_at: "2026-06-08T17:33:27+00:00"
evidence_required: []
id: "atelier-000l"
issue_type: "task"
labels:
- "config"
- "feature"
- "spec"
- "validator"
- "workflow"
priority: "P3"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-0006"
  - kind: "issue"
    id: "atelier-0008"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Implement configurable workflows and validator-backed transitions"
updated_at: "2026-06-11T21:17:57.070914297+00:00"
---

Implement configurable workflows and validator-backed transitions from the repository-owned workflow configuration contract and first-class workflow validator evaluation surface. Support record types, phases, transitions, required fields, done requirements, setup/validation hooks, and validators such as tests_passed, durable_state_current, review_complete, evidence_attached, validation_criteria_satisfied, and no_blocking_lints. Transition enforcement must support ergonomic command wrappers such as start/finish/complete once the worker transition command model is decided.

Acceptance:
Workflow configuration is loaded from the chosen repo-owned contract and validated with stable errors. Invalid transitions fail with actionable errors. Workflow validator results are consumed for machine-readable start/close enforcement. Missing evidence reports the exact record or criterion that needs proof. Setup and validation hooks follow configured timeout/failure semantics. Tests cover allowed transition, rejected transition, required evidence, invalid config, lightweight workflow, strict workflow, and waiver behavior.
