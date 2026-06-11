---
acceptance: []
created_at: "2026-06-11T20:10:56.796183597+00:00"
evidence_required: []
id: "atelier-ca32"
issue_type: "task"
labels:
- "assignee:root"
- "cache"
- "docs"
- "doctor"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Separate atelier doctor runtime health from canonical lint"
updated_at: "2026-06-11T23:55:23.316750821+00:00"
---

Keep doctor focused on runtime/install/cache health: cache exists, cache is fresh, rebuild possible, ignored paths correct, and runtime diagnostics. Acceptance: doctor complements lint without becoming the durability gate.
