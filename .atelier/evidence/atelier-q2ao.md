---
created_at: "2026-06-20T00:47:09.228717383+00:00"
id: "atelier-q2ao"
evidence_type: "validation"
captured_at: "2026-06-20T00:47:09.228703069+00:00"
target:
  kind: "issue"
  id: "atelier-cin6"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-cin6"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Addressed independent review findings for atelier-cin6: project config now rejects [review.providers.forgejo.role_authors] and Forgejo role commands source role authors from workflow review.open action params; review.link is no longer accepted as a workflow action until an executor exists. Checks passed: cargo fmt -- --check; cargo test -p atelier-app project_config --lib; cargo test -p atelier-cli forgejo --lib; cargo test -p atelier-workflow action --lib; cargo build -p atelier-cli; target/debug/atelier workflow check; target/debug/atelier lint atelier-cin6; git diff --check. FORGEJO_ADMIN_TOKEN=dummy target/debug/atelier forgejo roles provision now reaches Forgejo API auth (401 dummy token) instead of local role-author config failure."
updated_at: "2026-06-20T00:47:13.804005219+00:00"
---

Addressed independent review findings for atelier-cin6: project config now rejects [review.providers.forgejo.role_authors] and Forgejo role commands source role authors from workflow review.open action params; review.link is no longer accepted as a workflow action until an executor exists. Checks passed: cargo fmt -- --check; cargo test -p atelier-app project_config --lib; cargo test -p atelier-cli forgejo --lib; cargo test -p atelier-workflow action --lib; cargo build -p atelier-cli; target/debug/atelier workflow check; target/debug/atelier lint atelier-cin6; git diff --check. FORGEJO_ADMIN_TOKEN=dummy target/debug/atelier forgejo roles provision now reaches Forgejo API auth (401 dummy token) instead of local role-author config failure.
