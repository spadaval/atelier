---
acceptance: []
created_at: "2026-06-12T00:59:26.630485506+00:00"
evidence_required: []
id: "atelier-o8a7"
issue_type: "decision"
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
status: "closed"
title: "Decide quiet-mode contract for issues and missions"
updated_at: "2026-06-12T03:35:09.477545826+00:00"
---

Decide whether quiet mode is worth keeping for issue and mission commands, and if so what its contract is. Acceptance: quiet mode is either limited to strict composition outputs such as IDs, counts, paths, and pass/fail tokens, or deprecated for commands where it cannot stay meaningful. The decision must cover issue create/list/ready/show/update, mission create/list/show/status, and mutating acknowledgements; it must reject partial human detail views masquerading as quiet output.
