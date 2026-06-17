---
created_at: "2026-06-17T22:12:23.374123490+00:00"
id: "atelier-c0bq"
evidence_type: "test"
captured_at: "2026-06-17T22:12:23.374115298+00:00"
target:
  kind: "issue"
  id: "atelier-mrj5"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-mrj5"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Bundle apply safety proof: apply now stages canonical state in a temporary .atelier-bundle-stage directory, runs all writes there, installs issues/missions/evidence only after full success, and cleans the stage on success or failure. Focused tests passed: test_bundle_apply_mid_apply_failure_leaves_canonical_files_unchanged proves a relationship-phase failure leaves canonical issue/evidence files unchanged; test_bundle_apply_accepts_partial_issue_key_refs proves default issue status comes from workflow initial status; test_bundle_preview_rejects_status_outside_workflow_policy proves explicit statuses must exist in .atelier/workflow.yaml; test_bundle_apply_records_links_export_and_rebuild proves representative preview/apply output and rebuild/export behavior. cargo check -p atelier-cli, cargo fmt -- --check, target/debug/atelier lint, target/debug/atelier mission status atelier-0v3f, and git diff --check passed."
updated_at: "2026-06-17T22:12:27.427062217+00:00"
---

Bundle apply safety proof: apply now stages canonical state in a temporary .atelier-bundle-stage directory, runs all writes there, installs issues/missions/evidence only after full success, and cleans the stage on success or failure. Focused tests passed: test_bundle_apply_mid_apply_failure_leaves_canonical_files_unchanged proves a relationship-phase failure leaves canonical issue/evidence files unchanged; test_bundle_apply_accepts_partial_issue_key_refs proves default issue status comes from workflow initial status; test_bundle_preview_rejects_status_outside_workflow_policy proves explicit statuses must exist in .atelier/workflow.yaml; test_bundle_apply_records_links_export_and_rebuild proves representative preview/apply output and rebuild/export behavior. cargo check -p atelier-cli, cargo fmt -- --check, target/debug/atelier lint, target/debug/atelier mission status atelier-0v3f, and git diff --check passed.
