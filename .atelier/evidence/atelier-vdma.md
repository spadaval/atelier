---
created_at: "2026-06-14T16:54:16.454109504+00:00"
id: "atelier-vdma"
evidence_type: "validation"
captured_at: "2026-06-14T16:54:16.454063533+00:00"
target:
  kind: "issue"
  id: "atelier-zrmo"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-zrmo"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Focused regression proof: cargo test --test cli_integration test_issue_transition_options_and_successful_execution_follow_workflow_policy -- --nocapture passed; cargo test --test cli_integration test_issue_transition_options_do_not_write_but_blocked_transitions_do -- --nocapture passed. The first test confirmed git status --short was unchanged immediately before and after atelier issue transition <id> --options on a clean tree, then confirmed issue transition <id> start still recorded transition_applied. The second test confirmed --options leaves git status unchanged and an actually attempted blocked request_validation transition records transition_blocked activity. git diff --check passed and atelier lint reported Lint passed."
updated_at: "2026-06-14T16:54:18.169215266+00:00"
---

Focused regression proof: cargo test --test cli_integration test_issue_transition_options_and_successful_execution_follow_workflow_policy -- --nocapture passed; cargo test --test cli_integration test_issue_transition_options_do_not_write_but_blocked_transitions_do -- --nocapture passed. The first test confirmed git status --short was unchanged immediately before and after atelier issue transition <id> --options on a clean tree, then confirmed issue transition <id> start still recorded transition_applied. The second test confirmed --options leaves git status unchanged and an actually attempted blocked request_validation transition records transition_blocked activity. git diff --check passed and atelier lint reported Lint passed.
