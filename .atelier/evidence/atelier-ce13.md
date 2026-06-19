---
created_at: "2026-06-18T23:33:12.952537257+00:00"
id: "atelier-ce13"
evidence_type: "validation"
captured_at: "2026-06-18T23:33:12.952535911+00:00"
target:
  kind: "issue"
  id: "atelier-vd9e"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-vd9e"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Docs now map PR workflow claims to local Atelier facts versus Forgejo-owned policy. Proof: git diff updates cli-surface, workflow-configuration, validation, and ADR 0010; rg over docs/product docs/adr .agents/skills/agent-factory found no contradictory claim that Atelier enforces Forgejo approval or branch-protection policy; target/debug/atelier man validator matches role guidance; git diff --check and target/debug/atelier lint passed."
updated_at: "2026-06-18T23:33:15.568115910+00:00"
---

Docs now map PR workflow claims to local Atelier facts versus Forgejo-owned policy. Proof: git diff updates cli-surface, workflow-configuration, validation, and ADR 0010; rg over docs/product docs/adr .agents/skills/agent-factory found no contradictory claim that Atelier enforces Forgejo approval or branch-protection policy; target/debug/atelier man validator matches role guidance; git diff --check and target/debug/atelier lint passed.
