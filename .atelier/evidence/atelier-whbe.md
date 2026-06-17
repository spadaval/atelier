---
created_at: "2026-06-17T23:49:57.527365514+00:00"
id: "atelier-whbe"
evidence_type: "validation"
captured_at: "2026-06-17T23:49:57.527350863+00:00"
agent_identity: "codex"
target:
  kind: "issue"
  id: "atelier-d7gd"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-d7gd"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Updated CONTEXT.md to define Session, Typed field, and Pull request artifact; added ambiguity notes distinguishing durable sessions from current work and local diagnostics, PR artifacts from workflow validators, and forge_pr typed fields from evidence attachments. Validation: rg -n 'Session:|Typed field:|Pull request artifact:|Durable sessions|local command diagnostics|linked_pr_merged|forge_pr typed' CONTEXT.md; git diff --check -- CONTEXT.md; target/debug/atelier lint atelier-d7gd."
updated_at: "2026-06-17T23:50:01.844785087+00:00"
---

Updated CONTEXT.md to define Session, Typed field, and Pull request artifact; added ambiguity notes distinguishing durable sessions from current work and local diagnostics, PR artifacts from workflow validators, and forge_pr typed fields from evidence attachments. Validation: rg -n 'Session:|Typed field:|Pull request artifact:|Durable sessions|local command diagnostics|linked_pr_merged|forge_pr typed' CONTEXT.md; git diff --check -- CONTEXT.md; target/debug/atelier lint atelier-d7gd.
