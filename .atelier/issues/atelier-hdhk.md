---
acceptance: []
created_at: "2026-06-10T03:52:35.662499167+00:00"
evidence_required: []
id: "atelier-hdhk"
issue_type: "task"
labels:
- "export"
- "migration"
- "projection"
- "rebuild"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-a4ps"
  - kind: "issue"
    id: "atelier-e2vh"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Reframe export and rebuild around Markdown-first records"
updated_at: "2026-06-10T22:54:23.191622679+00:00"
---

Rework export/check/rebuild semantics so they no longer reassert SQLite as the canonical source after Markdown-first mutation lands.

Scope:
- Keep export and export --check as compatibility, repair, and deterministic rendering commands during migration.
- Make target export/check behavior validate Markdown records and derived projections without requiring SQLite to be the freshest source of canonical facts.
- Keep rebuild as the safe way to recreate ProjectionIndex from RecordStore.
- Label any SQLite-derived comparison as transitional compatibility behavior.

Acceptance:
Docs and command behavior agree that canonical records live in Markdown; export --check detects malformed/stale rendered Markdown and derived projection drift; rebuild ignores local RuntimeState except for safe schema setup; tests cover Markdown-first check behavior, compatibility export repair, unexpected canonical files, and stale derived projections.

Validation:
- cargo fmt -- --check
- cargo test export rebuild or equivalent focused tests
- cargo test
- ./target/debug/atelier rebuild
- ./target/debug/atelier export --check
- ./target/debug/atelier doctor
