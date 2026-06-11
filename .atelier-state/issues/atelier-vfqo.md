---
acceptance: []
created_at: "2026-06-11T02:45:01.354284754+00:00"
evidence_required: []
id: "atelier-vfqo"
issue_type: "task"
labels: []
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-vqsb"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Rewrite tests that asserted JSON command output"
updated_at: "2026-06-11T04:31:01.850823710+00:00"
---

Replace JSON command-output assertions with human-output, quiet-output, or direct state/projection assertions according to the inventory. Acceptance: tests cover removal behavior and the intended replacement workflow without scraping broad human text as a machine contract.
