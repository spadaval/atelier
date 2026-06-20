---
created_at: "2026-06-20T19:40:46.358147615+00:00"
id: "atelier-gwnq"
evidence_type: "validation"
captured_at: "2026-06-20T19:40:46.358146472+00:00"
target:
  kind: "issue"
  id: "atelier-19xa"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-19xa"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Added issue link/unlink --role for issue-objective relationship mutations. The advances role is included in issue status objective work traversal; blocked_by uses canonical issue blockers. Validation passed: cargo fmt -- --check; cargo check -p atelier-cli; cargo build -p atelier-cli; cargo test -p atelier-cli --test cli_integration setup_guidance -- --nocapture; target/debug/atelier issue --help shows link/unlink; target/debug/atelier mission status atelier-e146 reports docs/help drift clear; atelier lint atelier-19xa; git diff --check."
updated_at: "2026-06-20T19:40:49.366453092+00:00"
---

Added issue link/unlink --role for issue-objective relationship mutations. The advances role is included in issue status objective work traversal; blocked_by uses canonical issue blockers. Validation passed: cargo fmt -- --check; cargo check -p atelier-cli; cargo build -p atelier-cli; cargo test -p atelier-cli --test cli_integration setup_guidance -- --nocapture; target/debug/atelier issue --help shows link/unlink; target/debug/atelier mission status atelier-e146 reports docs/help drift clear; atelier lint atelier-19xa; git diff --check.
