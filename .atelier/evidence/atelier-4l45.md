---
created_at: "2026-06-15T15:49:35.790056895+00:00"
id: "atelier-4l45"
evidence_type: "test"
captured_at: "2026-06-15T15:49:35.790029157+00:00"
target:
  kind: "issue"
  id: "atelier-okz2"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-okz2"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Root start no longer writes runtime work_associations and close transitions no longer finish them. Root status and mission active-work rendering derive current work from canonical active-category issue status. Temp-repo transcript proved status still shows the in_progress issue after deleting .atelier/runtime/state.db and rebuilding, with zero work_associations rows; stale runtime work_associations row transcript showed Current work: none. cargo check, cargo build, atelier lint atelier-okz2, atelier export --check, and git diff --check passed with existing warning debt."
updated_at: "2026-06-15T15:49:37.993716404+00:00"
---

Root start no longer writes runtime work_associations and close transitions no longer finish them. Root status and mission active-work rendering derive current work from canonical active-category issue status. Temp-repo transcript proved status still shows the in_progress issue after deleting .atelier/runtime/state.db and rebuilding, with zero work_associations rows; stale runtime work_associations row transcript showed Current work: none. cargo check, cargo build, atelier lint atelier-okz2, atelier export --check, and git diff --check passed with existing warning debt.
