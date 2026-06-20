---
created_at: "2026-06-20T05:19:49.557582213+00:00"
id: "atelier-tpty"
evidence_type: "validation"
captured_at: "2026-06-20T05:19:49.557563960+00:00"
target:
  kind: "issue"
  id: "atelier-y8cs"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-y8cs"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Shared Forgejo config path and doctor readiness validation: cargo fmt -- --check; cargo test -p atelier-app project_config::tests::forgejo_loader --lib; cargo test -p atelier-app health::tests::review_backend_health --lib; cargo test -p atelier-cli commands::forgejo --lib; cargo check -p atelier-cli; git diff --check; target/debug/atelier doctor reported provider forgejo status ok with role authors having write access plus sudo verification; target/debug/atelier lint atelier-y8cs passed."
updated_at: "2026-06-20T05:19:54.273193685+00:00"
---

Shared Forgejo config path and doctor readiness validation: cargo fmt -- --check; cargo test -p atelier-app project_config::tests::forgejo_loader --lib; cargo test -p atelier-app health::tests::review_backend_health --lib; cargo test -p atelier-cli commands::forgejo --lib; cargo check -p atelier-cli; git diff --check; target/debug/atelier doctor reported provider forgejo status ok with role authors having write access plus sudo verification; target/debug/atelier lint atelier-y8cs passed.
