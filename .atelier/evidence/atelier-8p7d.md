---
created_at: "2026-06-15T16:49:25.319823079+00:00"
id: "atelier-8p7d"
evidence_type: "validation"
captured_at: "2026-06-15T16:49:25.319773255+00:00"
target:
  kind: "issue"
  id: "atelier-vv2i"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-vv2i"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "App-layer boundary proof: atelier-app exposes Request, Outcome, ViewModel, and a handle entrypoint that returns data for CLI rendering without stdout/stderr rendering; crate-local app_entrypoint_returns_view_model_without_rendering passed; RUSTFLAGS=-Dwarnings cargo check --workspace --all-targets passed."
updated_at: "2026-06-15T16:49:27.410901173+00:00"
---

App-layer boundary proof: atelier-app exposes Request, Outcome, ViewModel, and a handle entrypoint that returns data for CLI rendering without stdout/stderr rendering; crate-local app_entrypoint_returns_view_model_without_rendering passed; RUSTFLAGS=-Dwarnings cargo check --workspace --all-targets passed.
