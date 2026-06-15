---
created_at: "2026-06-13T22:48:25.056969042+00:00"
id: "atelier-1bq9"
evidence_type: "validation"
captured_at: "2026-06-13T22:48:25.056932038+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
follow_up_ids: []
residual_risks: []
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-vu88"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Demolition complete in isolated worktree /root/atelier/.atelier-worktrees/atelier-vu88: deleted src/lock_check.rs, src/locks.rs, src/sync.rs, and src/daemon.rs; removed lock/sync checks from next and issue-create --work paths; removed .locks-cache from runtime ignore/layout rules; updated Claude hook guidance to stop calling removed atelier locks/sync commands. Residue scan now leaves only docs/provenance/test/removed-root references for daemon plus a docs-only historical atelier sync mention. Focused proof: cargo nextest run test_next_command; cargo nextest run test_issue_next_uses_current_workflow_commands; cargo nextest run test_create_issue_with_work_prints_canonical_path; cargo nextest run test_root_start_applies_workflow_transition_and_records_active_work; removed-command transcript shows atelier locks/sync/daemon all fail as unrecognized subcommands; cargo fmt -- --check, atelier lint, and atelier export --check passed. Current visible next flow is atelier issue next/status guidance rather than a root atelier next command."
updated_at: "2026-06-13T22:48:26.544768057+00:00"
---

Demolition complete in isolated worktree /root/atelier/.atelier-worktrees/atelier-vu88: deleted src/lock_check.rs, src/locks.rs, src/sync.rs, and src/daemon.rs; removed lock/sync checks from next and issue-create --work paths; removed .locks-cache from runtime ignore/layout rules; updated Claude hook guidance to stop calling removed atelier locks/sync commands. Residue scan now leaves only docs/provenance/test/removed-root references for daemon plus a docs-only historical atelier sync mention. Focused proof: cargo nextest run test_next_command; cargo nextest run test_issue_next_uses_current_workflow_commands; cargo nextest run test_create_issue_with_work_prints_canonical_path; cargo nextest run test_root_start_applies_workflow_transition_and_records_active_work; removed-command transcript shows atelier locks/sync/daemon all fail as unrecognized subcommands; cargo fmt -- --check, atelier lint, and atelier export --check passed. Current visible next flow is atelier issue next/status guidance rather than a root atelier next command.
