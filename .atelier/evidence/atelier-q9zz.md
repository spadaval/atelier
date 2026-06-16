---
created_at: "2026-06-13T23:46:33.499971+00:00"
id: "atelier-q9zz"
evidence_type: "validation"
captured_at: "2026-06-13T23:46:33.499971+00:00"
proof_scope: "Agent Factory command recipe replacement for atelier-4ykl"
agent_identity: "root"
independence_level: "implementer"
residual_risks:
- "Repository-level atelier lint/export/workflow checks are blocked by unrelated malformed .atelier/evidence/atelier-06rb.md missing required front matter."
- "A later focused nextest rerun is blocked by concurrent unstaged source changes in src/record_store.rs that reference missing normalized_milestone_data, normalized_plan_data, write_yaml_i64, and write_plan_revisions functions."
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-4ykl"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Agent Factory command recipe replacement implemented; focused command-surface tests passed before concurrent source edits, but final repo checks are blocked by unrelated tracker and compile failures."
updated_at: "2026-06-13T23:47:23.787685+00:00"
---

Agent Factory command recipe replacement for atelier-4ykl:

- Diff review: `AGENTFACTORY.md` no longer carries the explicit `## Checks` command list or the stale worktree recovery recipe using `atelier worktree remove <issue-id>` / `atelier worktree for <issue-id>`. The binding now routes proof, closeout, health, transition, recovery, and validation check selection to Atelier-owned validation, product docs, workflow policy, status/help, and issue Evidence sections.
- Freshness guard: `tests/cli_integration.rs::test_agent_factory_guidance_avoids_raw_workflow_validate_commands` now asserts the binding has `## Validation Routing` and does not reintroduce `## Checks`, `atelier worktree remove <issue-id>`, or the extended nextest recipe.
- Passing proof before concurrent source edits: `cargo nextest run test_agent_factory_guidance_avoids_raw_workflow_validate_commands test_workflow_check_reports_policy_and_issue_record_health test_workflow_check_rejects_stale_agent_guidance_commands test_workflow_check_rejects_stale_agent_guidance_options` passed with 4 tests run, 4 passed. `git diff --check` passed. `cargo fmt -- --check` passed.
- Blocked required repository checks: `atelier workflow check`, `atelier lint`, and `atelier export --check` fail before checking this slice because canonical tracker Markdown is invalid: `.atelier/evidence/atelier-06rb.md` is missing required front matter key `data`.
- Blocked final focused test rerun: the same `cargo nextest run ...` command later failed at compile time after concurrent unstaged source changes appeared outside this slice. The compile errors are missing `normalized_milestone_data`, `normalized_plan_data`, `write_yaml_i64`, and `write_plan_revisions` in `src/record_store.rs`.
