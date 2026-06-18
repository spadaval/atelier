---
created_at: "2026-06-18T00:01:33.985251715+00:00"
id: "atelier-awfy"
evidence_type: "validation"
captured_at: "2026-06-18T00:01:33.985242118+00:00"
agent_identity: "codex"
target:
  kind: "issue"
  id: "atelier-o97w"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-o97w"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Added visible session begin/show/list/end commands and root help entry. Focused tests cover begin/show/list/end round trip and invalid role failure. Validation: cargo test -p atelier-cli sessions; cargo test -p atelier-records session; cargo fmt -- --check; cargo check -p atelier-cli; cargo build -p atelier-cli; target/debug/atelier lint atelier-o97w; target/debug/atelier session --help; target/debug/atelier --help | rg session; git diff --check."
updated_at: "2026-06-18T00:01:38.067008364+00:00"
---

Added visible session begin/show/list/end commands and root help entry. Focused tests cover begin/show/list/end round trip and invalid role failure. Validation: cargo test -p atelier-cli sessions; cargo test -p atelier-records session; cargo fmt -- --check; cargo check -p atelier-cli; cargo build -p atelier-cli; target/debug/atelier lint atelier-o97w; target/debug/atelier session --help; target/debug/atelier --help | rg session; git diff --check.
