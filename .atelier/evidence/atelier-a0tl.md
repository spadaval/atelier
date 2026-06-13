---
created_at: "2026-06-13T04:30:08.125765337+00:00"
id: "atelier-a0tl"
data: "{\"captured_at\":\"2026-06-13T04:30:08.125741185+00:00\",\"kind\":\"test\",\"path\":null,\"producer\":null,\"result\":\"pass\",\"uri\":null}"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-j6v4"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Lifecycle status changes constrained to explicit commands. Removed public issue update --status, updated closed issue guidance to issue reopen, and validated issue update help plus reopen behavior. Checks: cargo test --test cli_integration test_issue_help_uses_reduced_lifecycle_surface; cargo test --test cli_integration test_reopen_issue; cargo fmt -- --check."
updated_at: "2026-06-13T04:30:09.314237185+00:00"
---

Lifecycle status changes constrained to explicit commands. Removed public issue update --status, updated closed issue guidance to issue reopen, and validated issue update help plus reopen behavior. Checks: cargo test --test cli_integration test_issue_help_uses_reduced_lifecycle_surface; cargo test --test cli_integration test_reopen_issue; cargo fmt -- --check.
