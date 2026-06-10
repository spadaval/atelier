---
acceptance: []
blocks:
- "atelier-0006"
- "atelier-0008"
created_at: "2026-06-08T17:33:27+00:00"
depends_on:
- "atelier-0001"
- "atelier-000h"
- "atelier-000s"
- "atelier-000u"
- "atelier-001k"
evidence_required: []
id: "atelier-000l"
issue_type: "task"
labels:
- "config"
- "feature"
- "spec"
- "validator"
- "workflow"
links: []
parent: "atelier-000r"
priority: "P3"
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Implement configurable workflows and validator-backed transitions"
updated_at: "2026-06-09T19:41:43.861375930+00:00"
---

Implement configurable workflows and validator-backed transitions from the repository-owned workflow configuration contract and first-class workflow validator evaluation surface. Support record types, phases, transitions, required fields, done requirements, setup/validation hooks, and validators such as tests_passed, durable_state_current, review_complete, evidence_attached, validation_criteria_satisfied, and no_blocking_lints.

Acceptance:
Workflow configuration is loaded from the chosen repo-owned contract and validated with stable errors. Invalid transitions fail with actionable errors. Workflow validator results are consumed for machine-readable start/close enforcement. Setup and validation hooks follow configured timeout/failure semantics. Tests cover allowed transition, rejected transition, required evidence, invalid config, lightweight workflow, strict workflow, and waiver behavior.
