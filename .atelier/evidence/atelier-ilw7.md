---
created_at: "2026-06-13T23:31:17.348934766+00:00"
id: "atelier-ilw7"
evidence_type: "validation"
captured_at: "2026-06-13T23:31:17.348784148+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-c9ej"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Docs/help command freshness check passes: target/debug/atelier workflow check reports Docs/Help Drift: clear; focused nextest command-surface tests passed; current-source lint/export checks pass"
updated_at: "2026-06-13T23:31:17.348934766+00:00"
---

Docs/help command freshness check passes:

- `target/debug/atelier workflow check` reports `Docs/Help Drift: clear`.
- `cargo nextest run test_workflow_check_reports_policy_and_issue_record_health test_workflow_check_rejects_stale_agent_guidance_commands test_workflow_check_rejects_stale_agent_guidance_options test_mission_closeout_blocks_undeferred_obsolete_command_test` passed 4 tests.
- `target/debug/atelier lint`, `target/debug/atelier lint atelier-c9ej`, and `target/debug/atelier export --check` passed with the current source-built binary.
- `git diff --check` passed.

Residual repository caveats outside this issue scope:

- The installed `atelier` binary is stale relative to the current typed evidence front matter and reported old-schema `data` diagnostics before `target/debug/atelier` rebuilt the projection.
- `cargo fmt -- --check` reports unrelated formatting drift in `src/commands/agent_factory.rs`, `src/main.rs`, and later concurrent hunks in `tests/cli_integration.rs`.
