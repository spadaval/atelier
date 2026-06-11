---
acceptance: []
created_at: "2026-06-11T20:10:55.605429835+00:00"
evidence_required: []
id: "atelier-jarw"
issue_type: "task"
labels:
- "lint"
- "markdown"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-ca32"
  - kind: "issue"
    id: "atelier-iw2l"
  - kind: "issue"
    id: "atelier-qrix"
  - kind: "issue"
    id: "atelier-unma"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Validate canonical .atelier Markdown directly in atelier lint"
updated_at: "2026-06-11T20:10:55.605429835+00:00"
---

Make lint parse committed .atelier/ records directly rather than trusting SQLite projection state. Acceptance: deleting state.db does not prevent canonical lint from validating committed records.
