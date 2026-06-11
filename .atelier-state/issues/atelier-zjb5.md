---
acceptance: []
blocks:
- "atelier-eq2d"
created_at: "2026-06-11T02:44:00.405898899+00:00"
depends_on:
- "atelier-esh8"
evidence_required: []
id: "atelier-zjb5"
issue_type: "epic"
labels: []
links: []
parent: null
priority: "P1"
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Epic: Remove command-result JSON mode from CLI commands"
updated_at: "2026-06-11T04:31:25.247085781+00:00"
---

Remove command-result JSON mode from the CLI after the boundary inventory is complete. Acceptance: relevant command args/help no longer expose --json for result rendering; JSON formatter/result branches and compatibility tests are removed or rewritten; quiet mode remains intentionally minimal; removed behavior has clear migration notes.
