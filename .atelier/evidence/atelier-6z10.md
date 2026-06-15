---
created_at: "2026-06-15T06:09:35.721460090+00:00"
id: "atelier-6z10"
evidence_type: "validation"
captured_at: "2026-06-15T06:09:35.721433753+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-wet4"
  role: "validates"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-wet4"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Status and role guidance now describe current-work sets from canonical in_progress issues: tests cover no current work, one current work item, and multiple current work items; role guide tests show status-derived current work and no active-pointer/abandon guidance; help/common command tests omit removed root abandon/repair and route to status, issue transitions, notes, evidence, workflow status, doctor, and worktree commands. Proof: cargo test --test cli_integration root_status -- --nocapture; cargo test --test cli_integration root_start -- --nocapture; cargo test --test cli_integration man_worker -- --nocapture; cargo test --test cli_integration removed -- --nocapture; cargo test --test cli_integration spec_representative -- --nocapture; atelier lint atelier-wet4; atelier export --check; git diff --check."
updated_at: "2026-06-15T06:09:37.598922450+00:00"
---

Status and role guidance now describe current-work sets from canonical in_progress issues: tests cover no current work, one current work item, and multiple current work items; role guide tests show status-derived current work and no active-pointer/abandon guidance; help/common command tests omit removed root abandon/repair and route to status, issue transitions, notes, evidence, workflow status, doctor, and worktree commands. Proof: cargo test --test cli_integration root_status -- --nocapture; cargo test --test cli_integration root_start -- --nocapture; cargo test --test cli_integration man_worker -- --nocapture; cargo test --test cli_integration removed -- --nocapture; cargo test --test cli_integration spec_representative -- --nocapture; atelier lint atelier-wet4; atelier export --check; git diff --check.
