---
created_at: "2026-06-18T00:08:55.333770081+00:00"
id: "atelier-7td0"
evidence_type: "test"
captured_at: "2026-06-18T00:08:55.333761948+00:00"
target:
  kind: "issue"
  id: "atelier-95wv"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-95wv"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo test -p atelier-cli sessions; cargo test -p atelier-cli test_root_start_allows_multiple_current_work_issues_in_same_worktree; cargo test -p atelier-cli test_root_start_applies_workflow_transition_and_records_active_work; cargo check -p atelier-cli; cargo fmt -- --check; cargo build -p atelier-cli; target/debug/atelier lint atelier-95wv; target/debug/atelier start --help; git diff --check"
updated_at: "2026-06-18T00:08:59.412404806+00:00"
---

cargo test -p atelier-cli sessions; cargo test -p atelier-cli test_root_start_allows_multiple_current_work_issues_in_same_worktree; cargo test -p atelier-cli test_root_start_applies_workflow_transition_and_records_active_work; cargo check -p atelier-cli; cargo fmt -- --check; cargo build -p atelier-cli; target/debug/atelier lint atelier-95wv; target/debug/atelier start --help; git diff --check
