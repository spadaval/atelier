---
acceptance: []
created_at: "2026-06-12T01:19:15.974791691+00:00"
evidence_required: []
id: "atelier-ia6p"
issue_type: "task"
labels:
- "cli"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Design next-action formatting helpers without generic policy"
updated_at: "2026-06-12T01:19:15.974791691+00:00"
---

Design central next-action rendering helpers that standardize labels, indentation, command text, optional why text, and empty rendering. Acceptance: helpers are explicitly non-authoritative; each command supplies its own context-specific actions and reasons, and tests prove the helper cannot degrade output into generic advice like try again or something went wrong.
