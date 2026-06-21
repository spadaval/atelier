---
created_at: "2026-06-21T19:37:43.989787220+00:00"
id: "atelier-l3j7"
evidence_type: "validation"
captured_at: "2026-06-21T19:37:43.989771145+00:00"
target:
  kind: "issue"
  id: "atelier-kka3"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-kka3"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Removed active mission storage/projection branches: canonical registry omits mission records, init no longer creates .atelier/missions, export/rebuild/activity scanners only accept issue activity, bundle mission resources write issue_type=mission records, and mission table/status read issue-backed objectives. Checks passed: cargo fmt -- --check; cargo build -p atelier-cli --bin atelier; target/debug/atelier lint; git diff --check; focused nextest mission surface tests; code search confirms no active canonical_dir/export/rebuild .atelier/missions branch remains."
updated_at: "2026-06-21T19:37:48.881876638+00:00"
---

Removed active mission storage/projection branches: canonical registry omits mission records, init no longer creates .atelier/missions, export/rebuild/activity scanners only accept issue activity, bundle mission resources write issue_type=mission records, and mission table/status read issue-backed objectives. Checks passed: cargo fmt -- --check; cargo build -p atelier-cli --bin atelier; target/debug/atelier lint; git diff --check; focused nextest mission surface tests; code search confirms no active canonical_dir/export/rebuild .atelier/missions branch remains.
