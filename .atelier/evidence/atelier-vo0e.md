---
created_at: "2026-06-15T06:09:10.712353882+00:00"
id: "atelier-vo0e"
evidence_type: "validation"
captured_at: "2026-06-15T06:09:10.712307558+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-nrwh"
  role: "validates"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-nrwh"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Removed root active-pointer cleanup surfaces: root help omits abandon/repair; removed-command tests reject atelier abandon/repair with guidance; worktree repair remains scoped to stale worktree association metadata. Proof: cargo test --test cli_integration removed -- --nocapture; cargo test --test cli_integration worktree -- --nocapture; atelier lint atelier-nrwh; atelier export --check; git diff --check."
updated_at: "2026-06-15T06:09:12.670116832+00:00"
---

Removed root active-pointer cleanup surfaces: root help omits abandon/repair; removed-command tests reject atelier abandon/repair with guidance; worktree repair remains scoped to stale worktree association metadata. Proof: cargo test --test cli_integration removed -- --nocapture; cargo test --test cli_integration worktree -- --nocapture; atelier lint atelier-nrwh; atelier export --check; git diff --check.
