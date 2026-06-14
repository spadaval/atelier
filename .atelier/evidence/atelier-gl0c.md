---
created_at: "2026-06-14T08:03:10.964370263+00:00"
id: "atelier-gl0c"
evidence_type: "validation"
captured_at: "2026-06-14T08:03:10.964256303+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-ovv0"
  role: "validates"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ovv0"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Implemented atomic worktree setup/active-work association. Proof: cargo nextest run test_work_lifecycle_human_output_and_guards test_worktree_setup_failure_does_not_associate_and_can_retry passed; target/debug/atelier lint atelier-ovv0 passed; target/debug/atelier export --check passed; git diff --check passed. cargo fmt -- --check still reports unrelated src/main.rs formatting drift."
updated_at: "2026-06-14T08:03:13.746467992+00:00"
---

Implemented atomic worktree setup/active-work association. Proof: cargo nextest run test_work_lifecycle_human_output_and_guards test_worktree_setup_failure_does_not_associate_and_can_retry passed; target/debug/atelier lint atelier-ovv0 passed; target/debug/atelier export --check passed; git diff --check passed. cargo fmt -- --check still reports unrelated src/main.rs formatting drift.
