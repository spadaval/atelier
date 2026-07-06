---
created_at: "2026-06-30T19:37:00.731684007+00:00"
id: "atelier-2cyq"
evidence_type: "test"
captured_at: "2026-06-30T19:37:00.731666939+00:00"
target:
  kind: "issue"
  id: "atelier-akxm"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-akxm"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo fmt -- --check; cargo test -p atelier-cli workflow --lib; cargo test -p atelier-workflow --lib; cargo test -p atelier-cli --test cli_integration provider_review_open_action_reads_workflow_config_and_env_secret -- --nocapture; cargo test -p atelier-cli --test cli_integration request_review_preserves_review_artifact_field -- --nocapture; cargo test -p atelier-cli --test cli_integration provider_request_review_pushes_source_before_opening_pr -- --nocapture; cargo build -p atelier-cli; git diff --check; target/debug/atelier check atelier-akxm; target/debug/atelier check atelier-sszj; target/debug/atelier check"
updated_at: "2026-06-30T19:37:06.442181933+00:00"
---

cargo fmt -- --check; cargo test -p atelier-cli workflow --lib; cargo test -p atelier-workflow --lib; cargo test -p atelier-cli --test cli_integration provider_review_open_action_reads_workflow_config_and_env_secret -- --nocapture; cargo test -p atelier-cli --test cli_integration request_review_preserves_review_artifact_field -- --nocapture; cargo test -p atelier-cli --test cli_integration provider_request_review_pushes_source_before_opening_pr -- --nocapture; cargo build -p atelier-cli; git diff --check; target/debug/atelier check atelier-akxm; target/debug/atelier check atelier-sszj; target/debug/atelier check
