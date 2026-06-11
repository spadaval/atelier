---
acceptance: []
created_at: "2026-06-11T02:44:00.405898899+00:00"
evidence_required: []
id: "atelier-zjb5"
issue_type: "epic"
labels: []
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-eq2d"
  children:
  - kind: "issue"
    id: "atelier-iv68"
  - kind: "issue"
    id: "atelier-kaei"
  - kind: "issue"
    id: "atelier-vfqo"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Epic: Remove command-result JSON mode from CLI commands"
updated_at: "2026-06-11T04:31:25.247085781+00:00"
---

Remove command-result JSON mode from the CLI after the boundary inventory is complete. Acceptance: relevant command args/help no longer expose --json for result rendering; JSON formatter/result branches and compatibility tests are removed or rewritten; quiet mode remains intentionally minimal; removed behavior has clear migration notes.
