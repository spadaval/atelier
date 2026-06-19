---
created_at: "2026-06-17T21:50:11.893679197+00:00"
id: "atelier-wmcm"
evidence_type: "test"
captured_at: "2026-06-17T21:50:11.893665914+00:00"
target:
  kind: "issue"
  id: "atelier-2y4u"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-2y4u"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Ready-list bug fix proof: cargo test -p atelier-cli --test cli_integration test_issue_list_ready_still_shows_ready_children_when_another_issue_is_active -- --nocapture passed; target/debug/atelier issue list --ready shows 17 ready items instead of No issues found while atelier-2y4u is active; target/debug/atelier lint atelier-2y4u, cargo fmt -- --check, and git diff --check passed."
updated_at: "2026-06-17T21:50:15.794758908+00:00"
---

Ready-list bug fix proof: cargo test -p atelier-cli --test cli_integration test_issue_list_ready_still_shows_ready_children_when_another_issue_is_active -- --nocapture passed; target/debug/atelier issue list --ready shows 17 ready items instead of No issues found while atelier-2y4u is active; target/debug/atelier lint atelier-2y4u, cargo fmt -- --check, and git diff --check passed.
