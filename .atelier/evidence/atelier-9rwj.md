---
created_at: "2026-06-13T23:57:35.708247219+00:00"
id: "atelier-9rwj"
evidence_type: "validation"
captured_at: "2026-06-13T23:57:35.708152961+00:00"
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
    id: "atelier-10qm"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Dead-code cleanup removed 37 production cargo build dead-code warnings and hidden evidence add/capture predecessor command variants. Proof: cargo build passed clean; cargo fmt -- --check passed; git diff --check passed; focused nextest slice passed 197/197; evidence/removed-alias nextest slice passed 3/3; atelier lint passed; atelier export --check reported canonical export current; atelier doctor reported rebuild_ready/projection_fresh/runtime health ok. Residual warnings are pre-existing cli_integration unused stdout variables during test compilation, not dead-code warnings from touched production paths."
updated_at: "2026-06-13T23:57:37.918297395+00:00"
---

Dead-code cleanup removed 37 production cargo build dead-code warnings and hidden evidence add/capture predecessor command variants. Proof: cargo build passed clean; cargo fmt -- --check passed; git diff --check passed; focused nextest slice passed 197/197; evidence/removed-alias nextest slice passed 3/3; atelier lint passed; atelier export --check reported canonical export current; atelier doctor reported rebuild_ready/projection_fresh/runtime health ok. Residual warnings are pre-existing cli_integration unused stdout variables during test compilation, not dead-code warnings from touched production paths.
