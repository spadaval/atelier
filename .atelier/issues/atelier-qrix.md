---
acceptance: []
created_at: "2026-06-11T20:10:56.410702383+00:00"
evidence_required: []
id: "atelier-qrix"
issue_type: "validation"
labels:
- "fixtures"
- "validation"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Add hard-failure fixtures for invalid Markdown records"
updated_at: "2026-06-12T00:07:06.603846521+00:00"
---

Add fixtures/tests for duplicate IDs, ID/path mismatch, invalid schema/version, missing references, dependency cycles, invalid status/type/priority, malformed activity sidecar, unsupported committed file, and malformed front matter. Acceptance: each failure has a stable diagnostic.
