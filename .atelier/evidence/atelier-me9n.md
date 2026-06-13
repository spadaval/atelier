---
created_at: "2026-06-13T04:26:34.666239952+00:00"
id: "atelier-me9n"
evidence_type: "test"
captured_at: "2026-06-13T04:26:34.666217407+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: null
agent_identity: null
independence_level: null
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-g3k0"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Projection rebuild temp journal files are ignored by query, lint, export, and doctor. Fixed runtime path classification for rebuild-tmp-journal and validated with cargo test storage_layout::tests::rebuild_temp_database_paths_are_local_atelier_paths plus cargo test --test cli_integration test_rebuild_temp_files_are_ignored_by_query_lint_export_and_doctor; cargo fmt -- --check passed."
updated_at: "2026-06-13T04:26:35.764847426+00:00"
---

Projection rebuild temp journal files are ignored by query, lint, export, and doctor. Fixed runtime path classification for rebuild-tmp-journal and validated with cargo test storage_layout::tests::rebuild_temp_database_paths_are_local_atelier_paths plus cargo test --test cli_integration test_rebuild_temp_files_are_ignored_by_query_lint_export_and_doctor; cargo fmt -- --check passed.
