---
acceptance: []
created_at: "2026-06-10T03:50:49.785782245+00:00"
evidence_required: []
id: "atelier-e2vh"
issue_type: "task"
labels:
- "cli"
- "markdown"
- "migration"
- "record-store"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-a4ps"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Migrate issue mutations to Markdown-first writes"
updated_at: "2026-06-10T23:49:11.731358422+00:00"
---

Move canonical issue mutations from SQLite-first plus export to Markdown-first writes.

Scope:
- Cover issue create, update, close/reopen, labels, dependencies, and typed links where they mutate canonical issue facts.
- Write canonical Markdown through RecordStore before refreshing ProjectionIndex.
- Preserve stable human and JSON output for Agent Factory workflows.
- Keep compatibility commands working or classify each deferred path explicitly.
- Do not change local-only sessions, timers, usage, or lock behavior except for record ID references required by the mutation path.

Acceptance:
A successful canonical issue mutation is durable in .atelier-state without requiring a later SQLite export; ProjectionIndex is refreshed or marked stale with an actionable repair path; export --check and rebuild remain valid after each mutation; tests or CLI transcripts cover create, update, close/reopen, label, dependency add/remove, and typed link mutation behavior.

Validation:
- cargo fmt -- --check
- cargo test
- scripted CLI mutation round trip in a temp repo
- ./target/debug/atelier export --check
- ./target/debug/atelier lint
- ./target/debug/atelier doctor
