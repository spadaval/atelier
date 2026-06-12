---
acceptance: []
created_at: "2026-06-09T19:47:13.578686404+00:00"
evidence_required: []
id: "atelier-001p"
issue_type: "task"
labels:
- "assignee:root"
- "diagnostics"
- "telemetry"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-001q"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Define telemetry storage, redaction, and retention policy"
updated_at: "2026-06-11T13:07:09.145263865+00:00"
---

Decide the local diagnostics folder contract, workspace identity fields, redaction defaults, opt-out controls, retention behavior, and whether summaries ever project into Mission Control.

Acceptance: policy is documented, compatible with issue atelier-000i, names local-only versus exported data, and gives implementers exact fields and defaults.
