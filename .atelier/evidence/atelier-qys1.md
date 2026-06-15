---
created_at: "2026-06-15T08:15:03.180227085+00:00"
id: "atelier-qys1"
evidence_type: "validation"
captured_at: "2026-06-15T08:15:03.180108344+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-zwna"
  role: "validates"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-zwna"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "CLI shell proof: crates/atelier-cli/src/lib.rs contains only CLI shell contract/delegation to atelier-app, no storage/workflow/domain imports; representative CLI smoke passed for issue create, mission status, evidence capture, and doctor; full cargo nextest run --test cli_integration passed 206/206 with 1 skipped; cargo fmt -- --check, cargo check --workspace, atelier lint, and git diff --check passed."
updated_at: "2026-06-15T08:15:06.453076900+00:00"
---

CLI shell proof: crates/atelier-cli/src/lib.rs contains only CLI shell contract/delegation to atelier-app, no storage/workflow/domain imports; representative CLI smoke passed for issue create, mission status, evidence capture, and doctor; full cargo nextest run --test cli_integration passed 206/206 with 1 skipped; cargo fmt -- --check, cargo check --workspace, atelier lint, and git diff --check passed.
