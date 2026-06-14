---
created_at: "2026-06-14T16:57:02.532403436+00:00"
id: "atelier-sims"
evidence_type: "validation"
captured_at: "2026-06-14T16:57:02.532352419+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-9soq"
  role: "validates"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-9soq"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Focused command-surface visibility coverage: hidden/advanced command references are accepted only in explicit hidden/advanced context, hidden commands fail when treated as normal workflow guidance, removed commands are accepted only in explicit removal-history context, removed commands fail when treated as normal workflow guidance, and nonexistent options still fail; verified by cargo test --test cli_integration test_workflow_check_, git diff --check, and atelier lint."
updated_at: "2026-06-14T16:57:03.996548331+00:00"
---

Focused command-surface visibility coverage: hidden/advanced command references are accepted only in explicit hidden/advanced context, hidden commands fail when treated as normal workflow guidance, removed commands are accepted only in explicit removal-history context, removed commands fail when treated as normal workflow guidance, and nonexistent options still fail; verified by cargo test --test cli_integration test_workflow_check_, git diff --check, and atelier lint.
