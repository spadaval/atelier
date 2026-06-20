---
created_at: "2026-06-20T00:41:15.596774242+00:00"
id: "atelier-ts5e"
evidence_type: "review"
captured_at: "2026-06-20T00:41:15.596765478+00:00"
target:
  kind: "issue"
  id: "atelier-cin6"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-cin6"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Independent review of master..epic/atelier-cin6 found two blocking defects before validation: Forgejo role authors still remain active in .atelier/config.toml parser/provisioning even though the workflow contract moved action role attribution into .atelier/workflow.yaml, and the documented/parser-accepted review.link action has no preflight or executor path. Checks run: git diff --check master..epic/atelier-cin6; focused rg for legacy effects/review_artifact_* active identifiers; FORGEJO_ADMIN_TOKEN=dummy target/debug/atelier forgejo roles provision; target/debug/atelier workflow check; cargo test -p atelier-workflow action --lib -- --nocapture; cargo test -p atelier-cli commands::workflow::tests::provider_review_action_preflight_uses_workflow_role_authors_and_env_secret --lib -- --nocapture; cargo test -p atelier-cli --test cli_integration provider_review_open_action_reads_workflow_config_and_env_secret -- --nocapture; target/debug/atelier lint atelier-cin6; target/debug/atelier issue transition atelier-cin6 --options."
updated_at: "2026-06-20T00:41:20.338303242+00:00"
---

Independent review of master..epic/atelier-cin6 found two blocking defects before validation: Forgejo role authors still remain active in .atelier/config.toml parser/provisioning even though the workflow contract moved action role attribution into .atelier/workflow.yaml, and the documented/parser-accepted review.link action has no preflight or executor path. Checks run: git diff --check master..epic/atelier-cin6; focused rg for legacy effects/review_artifact_* active identifiers; FORGEJO_ADMIN_TOKEN=dummy target/debug/atelier forgejo roles provision; target/debug/atelier workflow check; cargo test -p atelier-workflow action --lib -- --nocapture; cargo test -p atelier-cli commands::workflow::tests::provider_review_action_preflight_uses_workflow_role_authors_and_env_secret --lib -- --nocapture; cargo test -p atelier-cli --test cli_integration provider_review_open_action_reads_workflow_config_and_env_secret -- --nocapture; target/debug/atelier lint atelier-cin6; target/debug/atelier issue transition atelier-cin6 --options.
