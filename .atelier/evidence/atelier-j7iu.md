---
created_at: "2026-06-15T18:17:23.812563153+00:00"
id: "atelier-j7iu"
evidence_type: "validation"
captured_at: "2026-06-15T18:17:23.812456301+00:00"
target:
  kind: "issue"
  id: "atelier-p45j"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-p45j"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Ready-list blocked parent header validation: cargo nextest run -p atelier-cli test_issue_list_ready_marks_blocked_parent_headers_as_context test_issue_list_ready_treats_internal_epic_blockers_as_ready test_issue_list_marks_external_epic_blockers_by_id passed; output now appends '(context; parent blocked)' to grouped blocked parent headers, keeps the child as the ready row, and issue blocked <parent-id> agrees on the blocker."
updated_at: "2026-06-15T18:17:27.281630714+00:00"
---

Ready-list blocked parent header validation: cargo nextest run -p atelier-cli test_issue_list_ready_marks_blocked_parent_headers_as_context test_issue_list_ready_treats_internal_epic_blockers_as_ready test_issue_list_marks_external_epic_blockers_by_id passed; output now appends '(context; parent blocked)' to grouped blocked parent headers, keeps the child as the ready row, and issue blocked <parent-id> agrees on the blocker.
