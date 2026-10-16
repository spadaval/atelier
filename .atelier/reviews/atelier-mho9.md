---
schema: atelier.review
schema_version: 1
id: atelier-mho9
title: 'crates/atelier-cli/src/commands/bundle.rs:499 and docs/spec/bundle/schema.md:129: an update that sets parent on a record that already has a different parent passes preview (it lists the new parent edge) but fails apply at the staged index with ''Issue X appears under multiple parents''. Rollback is correct, so no data is damaged, but preview and apply disagree, and the docs say only that parent ''adds links''. Re-parenting after review, a likely revision, can''t be done through a bundle. Fix: either check the existing parent in validate_bundle_hierarchy for update entries and fail preview with a pointer to ''atelier issue update --parent'', or implement replace semantics the way update_lifecycle does (remove_issue_child, then add_issue_child). Document the result.'
target: atelier-4n7v
identity: Independent reviewer (Claude subagent)
commit: a9234343ba772ab7e86fa682752f752f73f4af7e
created_at: 2026-10-16T00:40:13.409417026Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-cli/src/commands/bundle.rs:499 and docs/spec/bundle/schema.md:129: an update that sets parent on a record that already has a different parent passes preview (it lists the new parent edge) but fails apply at the staged index with 'Issue X appears under multiple parents'. Rollback is correct, so no data is damaged, but preview and apply disagree, and the docs say only that parent 'adds links'. Re-parenting after review, a likely revision, can't be done through a bundle. Fix: either check the existing parent in validate_bundle_hierarchy for update entries and fail preview with a pointer to 'atelier issue update --parent', or implement replace semantics the way update_lifecycle does (remove_issue_child, then add_issue_child). Document the result.
