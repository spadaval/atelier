---
acceptance: []
blocks:
- "atelier-000c"
created_at: "2026-06-08T17:33:27+00:00"
depends_on:
- "atelier-000j"
- "atelier-001a"
- "atelier-001f"
evidence_required: []
id: "atelier-000r"
issue_type: "task"
labels:
- "epic"
- "milestone"
- "spec"
- "validator"
- "workflow"
links: []
parent: null
priority: "P3"
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Milestone 5: Workflow policy, validators, lint, and waivers"
updated_at: "2026-06-09T19:41:43.938500035+00:00"
---

Add configurable workflow policy, workflow validators, lint severities, waivers, and action-aware guidance while keeping process risk-scaled and overrideable. This milestone owns strict versus lightweight process rules, not ad hoc command-specific behavior.

Acceptance:
Configurable workflows define phases, transitions, required fields, done requirements, validation criteria requirements, evidence requirements, validators, lints, severities, and waivers. Small tasks can remain lightweight by default. Validator results are machine-readable. Start and close enforcement are driven by workflow policy. Scenario evidence covers strict, lightweight, and waiver flows.
