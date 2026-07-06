---
created_at: "2026-06-30T18:56:41.669978346+00:00"
id: "atelier-8va5"
evidence_type: "test"
captured_at: "2026-06-30T18:56:41.669965464+00:00"
target:
  kind: "issue"
  id: "atelier-ms7i"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ms7i"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo fmt -- --check; cargo test -p atelier-workflow --lib; cargo test -p atelier-cli workflow --lib; cargo test -p atelier-cli --test cli_integration test_epic_start_from_mission_branch_uses_current_branch_base -- --nocapture; target/debug/atelier check atelier-ms7i; git diff --check all passed"
updated_at: "2026-06-30T18:56:47.281831042+00:00"
---

cargo fmt -- --check; cargo test -p atelier-workflow --lib; cargo test -p atelier-cli workflow --lib; cargo test -p atelier-cli --test cli_integration test_epic_start_from_mission_branch_uses_current_branch_base -- --nocapture; target/debug/atelier check atelier-ms7i; git diff --check all passed
