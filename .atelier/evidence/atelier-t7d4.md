---
created_at: "2026-06-17T22:05:04.219999323+00:00"
id: "atelier-t7d4"
evidence_type: "test"
captured_at: "2026-06-17T22:05:04.219991484+00:00"
target:
  kind: "issue"
  id: "atelier-jmmn"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-jmmn"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Bundle command proof: target/debug/atelier --help lists bundle and plan without apply; target/debug/atelier plan --help omits apply; target/debug/atelier bundle --help lists preview/apply; target/debug/atelier plan apply /tmp/nope.json fails with unrecognized subcommand 'apply'. Focused tests passed: test_bundle_apply_accepts_partial_issue_key_refs, test_plan_apply_command_is_removed, test_bundle_preview_rejects_plan_and_milestone_resources, and test_bundle_apply_records_links_export_and_rebuild. cargo check -p atelier-cli, cargo fmt -- --check, target/debug/atelier lint, target/debug/atelier mission status atelier-0v3f, and git diff --check passed with Docs/Help Drift clear."
updated_at: "2026-06-17T22:05:08.252055029+00:00"
---

Bundle command proof: target/debug/atelier --help lists bundle and plan without apply; target/debug/atelier plan --help omits apply; target/debug/atelier bundle --help lists preview/apply; target/debug/atelier plan apply /tmp/nope.json fails with unrecognized subcommand 'apply'. Focused tests passed: test_bundle_apply_accepts_partial_issue_key_refs, test_plan_apply_command_is_removed, test_bundle_preview_rejects_plan_and_milestone_resources, and test_bundle_apply_records_links_export_and_rebuild. cargo check -p atelier-cli, cargo fmt -- --check, target/debug/atelier lint, target/debug/atelier mission status atelier-0v3f, and git diff --check passed with Docs/Help Drift clear.
