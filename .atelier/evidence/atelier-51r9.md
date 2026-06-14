---
created_at: "2026-06-13T23:32:17.797940201+00:00"
id: "atelier-51r9"
evidence_type: "validation"
captured_at: "2026-06-13T23:32:17.797847844+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-5a73"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Implemented exact issue status filtering and separate workflow category filtering. Proof: cargo fmt -- --check passed; cargo nextest run test_issue_orientation_uses_workflow_categories_and_exact_statuses passed; target/debug/atelier issue list --help shows --status exact workflow status and --category derived workflow category; target/debug/atelier issue list --status in_progress lists active/in_progress rows; target/debug/atelier issue list --category in_progress rejects the old ambiguous alias; target/debug/atelier lint atelier-5a73 passed; target/debug/atelier export --check passed after runtime rebuild; git diff --check passed."
updated_at: "2026-06-13T23:32:21.607937567+00:00"
---

Implemented exact issue status filtering and separate workflow category filtering. Proof: cargo fmt -- --check passed; cargo nextest run test_issue_orientation_uses_workflow_categories_and_exact_statuses passed; target/debug/atelier issue list --help shows --status exact workflow status and --category derived workflow category; target/debug/atelier issue list --status in_progress lists active/in_progress rows; target/debug/atelier issue list --category in_progress rejects the old ambiguous alias; target/debug/atelier lint atelier-5a73 passed; target/debug/atelier export --check passed after runtime rebuild; git diff --check passed.
