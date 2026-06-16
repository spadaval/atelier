---
created_at: "2026-06-14T00:15:52.509686974+00:00"
id: "atelier-imuu"
evidence_type: "validation"
captured_at: "2026-06-14T00:15:52.509585890+00:00"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ggls"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Diff maps src/db/mod.rs schema helpers to current_schema_version/install_core_schema/apply_schema_migrations and v15 copy helpers; src/commands/rebuild.rs projection loading to ProjectionLoader and IssueRelationshipProjection. Passed: cargo build; cargo fmt -- --check; git diff --check; cargo nextest run -E 'test(refresh_projection_preserves_valid_runtime_state) or test(rebuild_recreates_canonical_projection_and_resets_runtime_state) or test(rebuild_accepts_issue_activity_sidecars) or test(test_first_class_records_export_rebuild_and_validate) or test(test_bulk_plan_apply_records_links_export_and_rebuild)'; cargo clippy --all-targets -- -W clippy::too_many_lines showed no owned-file too_many_lines warnings; target/debug/atelier rebuild; target/debug/atelier lint; target/debug/atelier export --check."
updated_at: "2026-06-14T00:15:54.473676927+00:00"
---

Diff maps src/db/mod.rs schema helpers to current_schema_version/install_core_schema/apply_schema_migrations and v15 copy helpers; src/commands/rebuild.rs projection loading to ProjectionLoader and IssueRelationshipProjection. Passed: cargo build; cargo fmt -- --check; git diff --check; cargo nextest run -E 'test(refresh_projection_preserves_valid_runtime_state) or test(rebuild_recreates_canonical_projection_and_resets_runtime_state) or test(rebuild_accepts_issue_activity_sidecars) or test(test_first_class_records_export_rebuild_and_validate) or test(test_bulk_plan_apply_records_links_export_and_rebuild)'; cargo clippy --all-targets -- -W clippy::too_many_lines showed no owned-file too_many_lines warnings; target/debug/atelier rebuild; target/debug/atelier lint; target/debug/atelier export --check.
