---
created_at: "2026-06-11T00:06:58.557537388+00:00"
id: "atelier-nc18"
data: "{\"constraints\":[\"Diagnostics data stays outside .atelier-state by default, supports disable/override controls, and documents redaction and retention behavior.\"],\"evidence\":[],\"milestones\":[],\"plans\":[],\"risks\":[\"Telemetry can leak sensitive command context or create nondeterministic tracker state if storage and redaction boundaries are not explicit.\"],\"validation\":[\"Linked issues prove storage/redaction policy, command instrumentation, disabled telemetry behavior, failed-command telemetry, and stable slow-command query output.\"],\"work\":[]}"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-000i"
  - kind: "issue"
    id: "atelier-001m"
  - kind: "issue"
    id: "atelier-001p"
  - kind: "issue"
    id: "atelier-001q"
  - kind: "issue"
    id: "atelier-001r"
  attachments: []
  relates: []
schema: "atelier.mission"
schema_version: 1
status: "open"
title: "Local diagnostics and telemetry"
updated_at: "2026-06-11T00:06:58.557537388+00:00"
---

Add local-only command telemetry and diagnostics so operators can identify slow commands and performance issues without committing runtime logs into durable project state.
