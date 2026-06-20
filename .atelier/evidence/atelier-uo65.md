---
created_at: "2026-06-20T01:57:45.576012507+00:00"
id: "atelier-uo65"
evidence_type: "validation"
captured_at: "2026-06-20T01:57:45.576003748+00:00"
target:
  kind: "issue"
  id: "atelier-qx40"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-qx40"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Migrated workflow policy and starter examples to the cleaned-up model: task_delivery, epic_delivery, validation_delivery, and spike_review replace old starter workflow names; review and validation statuses now use the active category; archived was removed from starter terminal states; spike review now opens a review artifact before review.complete; docs/spec/examples and tests were aligned. Validation passed: cargo fmt -- --check; cargo test -p atelier-workflow; cargo test -p atelier-cli --test cli_integration issue_type; cargo test -p atelier-cli --test cli_integration request_review_preserves_review_artifact_field; cargo test -p atelier-cli commands::init --lib; cargo test -p atelier-app rebuild --lib; target/debug/atelier workflow check; target/debug/atelier lint atelier-qx40; git diff --check. Stale vocabulary search found only the intentional negative effects parser test."
updated_at: "2026-06-20T01:57:49.890376833+00:00"
---

Migrated workflow policy and starter examples to the cleaned-up model: task_delivery, epic_delivery, validation_delivery, and spike_review replace old starter workflow names; review and validation statuses now use the active category; archived was removed from starter terminal states; spike review now opens a review artifact before review.complete; docs/spec/examples and tests were aligned. Validation passed: cargo fmt -- --check; cargo test -p atelier-workflow; cargo test -p atelier-cli --test cli_integration issue_type; cargo test -p atelier-cli --test cli_integration request_review_preserves_review_artifact_field; cargo test -p atelier-cli commands::init --lib; cargo test -p atelier-app rebuild --lib; target/debug/atelier workflow check; target/debug/atelier lint atelier-qx40; git diff --check. Stale vocabulary search found only the intentional negative effects parser test.
