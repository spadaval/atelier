---
created_at: "2026-06-20T00:23:58.064244109+00:00"
id: "atelier-nmgk"
evidence_type: "validation"
captured_at: "2026-06-20T00:23:58.064230146+00:00"
target:
  kind: "issue"
  id: "atelier-0d5k"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-0d5k"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Migrated review artifact transition identifiers to namespaced review.open/review.link across workflow parser, CLI action planning/execution, active workflow config, starter/rebuild workflow text, docs, and tests. Validation passed: cargo test -p atelier-workflow; cargo test -p atelier-cli commands::workflow::tests; cargo test -p atelier-cli --test cli_integration provider_review_open_action_reads_workflow_config_and_env_secret; cargo build -p atelier-cli; cargo fmt -- --check; target/debug/atelier lint atelier-0d5k; target/debug/atelier workflow check; target/debug/atelier issue transition atelier-0d5k --options; rg -n 'review_artifact_(open|link)' crates docs SPEC.md .atelier/workflow.yaml returned no matches; git diff --check."
updated_at: "2026-06-20T00:24:02.610456298+00:00"
---

Migrated review artifact transition identifiers to namespaced review.open/review.link across workflow parser, CLI action planning/execution, active workflow config, starter/rebuild workflow text, docs, and tests. Validation passed: cargo test -p atelier-workflow; cargo test -p atelier-cli commands::workflow::tests; cargo test -p atelier-cli --test cli_integration provider_review_open_action_reads_workflow_config_and_env_secret; cargo build -p atelier-cli; cargo fmt -- --check; target/debug/atelier lint atelier-0d5k; target/debug/atelier workflow check; target/debug/atelier issue transition atelier-0d5k --options; rg -n 'review_artifact_(open|link)' crates docs SPEC.md .atelier/workflow.yaml returned no matches; git diff --check.
