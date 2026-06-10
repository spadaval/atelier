---
acceptance: []
blocks:
- "atelier-001r"
created_at: "2026-06-09T19:47:13.601126452+00:00"
depends_on:
- "atelier-001p"
evidence_required: []
id: "atelier-001q"
issue_type: "task"
labels:
- "cli"
- "performance"
- "task"
- "telemetry"
links: []
parent: "atelier-001m"
priority: "P2"
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Instrument CLI commands with structured duration events"
updated_at: "2026-06-09T19:47:13.601126452+00:00"
---

Add a shared command telemetry wrapper that records start, finish, duration, status, command identity, workspace identity, and optional phase timings for every agent-facing command.

Acceptance: successful and failed commands emit records when enabled, respect disabled telemetry, avoid sensitive argument capture by default, and have focused tests.
