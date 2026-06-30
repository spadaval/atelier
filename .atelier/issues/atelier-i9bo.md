---
created_at: "2026-06-29T17:47:20.256398457+00:00"
id: "atelier-i9bo"
issue_type: "task"
labels:
- "reliability"
- "validation"
- "workflow"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Implement mission-start baseline gate"
updated_at: "2026-06-29T17:47:20.256398457+00:00"
---

## Description

Mission execution establishes repository health before implementation starts. The configured baseline check set is run before a mission can enter active execution. If the baseline is red, the mission remains draft/blocked or records an explicit human-approved waiver with the reason and owner.

## Outcome

The mission workflow implements a mission-start baseline gate. Readiness and transition output identify the configured baseline checks that ran, including at least the tracker check, whitespace check, and default test suite unless workflow configuration overrides them. If baseline checks are failing, the mission stays draft/blocked or records an explicit waiver that names the failure, reason, approver, and owner.
