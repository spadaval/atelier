---
created_at: "2026-06-17T23:51:30.860911374+00:00"
id: "atelier-y4zj"
evidence_type: "validation"
captured_at: "2026-06-17T23:51:30.860902659+00:00"
agent_identity: "codex"
target:
  kind: "issue"
  id: "atelier-cbbx"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-cbbx"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Added docs/adr/0010-session-aware-pr-coordination-boundaries.md and linked it from docs/architecture/index.md. Content covers durable optional sessions, current-work separation, Forgejo PR artifacts, sudo-mode authorship, forge_pr typed field ownership, and linked_pr_merged/read-only PR validators. Validation: rg content check over ADR and architecture index; git diff --check -- docs/adr docs/architecture/index.md; target/debug/atelier lint atelier-cbbx."
updated_at: "2026-06-17T23:51:34.703916172+00:00"
---

Added docs/adr/0010-session-aware-pr-coordination-boundaries.md and linked it from docs/architecture/index.md. Content covers durable optional sessions, current-work separation, Forgejo PR artifacts, sudo-mode authorship, forge_pr typed field ownership, and linked_pr_merged/read-only PR validators. Validation: rg content check over ADR and architecture index; git diff --check -- docs/adr docs/architecture/index.md; target/debug/atelier lint atelier-cbbx.
