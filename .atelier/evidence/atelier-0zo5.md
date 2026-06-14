---
created_at: "2026-06-14T07:06:40.215505299+00:00"
id: "atelier-0zo5"
evidence_type: "validation"
captured_at: "2026-06-14T07:06:40.215377503+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-tje5"
  role: "validates"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-tje5"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Guidance includes concrete command forms and anti-examples from the transcript findings: docs/architecture/quality/validation.md lists cargo test, cargo nextest, rg, cargo fmt, git diff, atelier lint, and python3 command forms, and AGENTFACTORY.md routes shell-sensitive validation to those recipes while warning against multiple cargo test filters, unquoted rg metacharacters, and fragile command substitution."
updated_at: "2026-06-14T07:06:42.601666281+00:00"
---

Guidance includes concrete command forms and anti-examples from the transcript findings: docs/architecture/quality/validation.md lists cargo test, cargo nextest, rg, cargo fmt, git diff, atelier lint, and python3 command forms, and AGENTFACTORY.md routes shell-sensitive validation to those recipes while warning against multiple cargo test filters, unquoted rg metacharacters, and fragile command substitution.
