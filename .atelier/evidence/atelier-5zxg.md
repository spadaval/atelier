---
created_at: "2026-06-13T23:57:51.879361294+00:00"
id: "atelier-5zxg"
evidence_type: "validation"
captured_at: "2026-06-13T23:57:51.879264247+00:00"
command: null
exit_status: null
path: "docs/architecture/markdown-first-record-store.md"
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-k3vs"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Activity sidecar ownership is defined in markdown-first architecture docs: src/activity.rs owns schema, parsing, ID allocation, validation, and create-new writes; activity_log.rs is only the CLI adapter; export/rebuild/lint/history consume sidecars through that API; projection freshness intentionally skips .activity files. Focused checks passed: cargo nextest run test_canonical_export_preserves_issue_activity_sidecars rebuild_temp_database_paths_are_local_atelier_paths; atelier lint atelier-k3vs; atelier export --check; git diff --check."
updated_at: "2026-06-13T23:57:54.348399085+00:00"
---

Activity sidecar ownership is defined in markdown-first architecture docs: src/activity.rs owns schema, parsing, ID allocation, validation, and create-new writes; activity_log.rs is only the CLI adapter; export/rebuild/lint/history consume sidecars through that API; projection freshness intentionally skips .activity files. Focused checks passed: cargo nextest run test_canonical_export_preserves_issue_activity_sidecars rebuild_temp_database_paths_are_local_atelier_paths; atelier lint atelier-k3vs; atelier export --check; git diff --check.
