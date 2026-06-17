---
created_at: "2026-06-17T22:05:18.176513059+00:00"
id: "atelier-wsg7"
evidence_type: "test"
captured_at: "2026-06-17T22:05:18.176499038+00:00"
target:
  kind: "issue"
  id: "atelier-tkiw"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-tkiw"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Bundle v1 schema proof: implementation accepts schema atelier.bundle under resources and denies unknown plan/milestone fields through BundleResources deny_unknown_fields; mission.plans and mission.milestones are absent from BundleMission; operations are create-only via validate_operation. Focused tests passed: test_bundle_preview_rejects_plan_and_milestone_resources rejects v1 plans/milestones, test_bundle_apply_records_links_export_and_rebuild covers supported issues/missions/evidence resources and relationships, and test_bundle_apply_accepts_partial_issue_key_refs covers existing issue refs. cargo check -p atelier-cli, cargo fmt -- --check, target/debug/atelier lint, target/debug/atelier mission status atelier-0v3f, and git diff --check passed."
updated_at: "2026-06-17T22:05:21.956713145+00:00"
---

Bundle v1 schema proof: implementation accepts schema atelier.bundle under resources and denies unknown plan/milestone fields through BundleResources deny_unknown_fields; mission.plans and mission.milestones are absent from BundleMission; operations are create-only via validate_operation. Focused tests passed: test_bundle_preview_rejects_plan_and_milestone_resources rejects v1 plans/milestones, test_bundle_apply_records_links_export_and_rebuild covers supported issues/missions/evidence resources and relationships, and test_bundle_apply_accepts_partial_issue_key_refs covers existing issue refs. cargo check -p atelier-cli, cargo fmt -- --check, target/debug/atelier lint, target/debug/atelier mission status atelier-0v3f, and git diff --check passed.
