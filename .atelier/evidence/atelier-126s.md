---
created_at: "2026-06-14T07:10:48.723938872+00:00"
id: "atelier-126s"
evidence_type: "validation"
captured_at: "2026-06-14T07:10:48.723837048+00:00"
target:
  kind: "issue"
  id: "atelier-xbr0"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-xbr0"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Mission create help names generated mission sections and states that --body maps to the Intent section; docs/product/work-model.md shows a mission create example with --constraint, --risk, and --validation. Proof: cargo nextest run --test cli_integration -E 'test(test_mission_create_help_names_generated_sections) or test(test_first_class_records_export_rebuild_and_validate)'; cargo fmt -- --check; git diff --check; atelier lint."
updated_at: "2026-06-14T07:10:51.243667199+00:00"
---

Mission create help names generated mission sections and states that --body maps to the Intent section; docs/product/work-model.md shows a mission create example with --constraint, --risk, and --validation. Proof: cargo nextest run --test cli_integration -E 'test(test_mission_create_help_names_generated_sections) or test(test_first_class_records_export_rebuild_and_validate)'; cargo fmt -- --check; git diff --check; atelier lint.
