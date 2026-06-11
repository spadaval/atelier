---
acceptance: []
created_at: "2026-06-10T03:52:27.308529893+00:00"
evidence_required: []
id: "atelier-z6sc"
issue_type: "decision"
labels:
- "comments"
- "durability"
- "migration"
- "record-store"
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
title: "Decide and migrate durable notes and comments"
updated_at: "2026-06-10T22:54:23.132139132+00:00"
---

Resolve how agent notes, close reasons, claims, and comments survive Markdown-first rebuild.

Scope:
- Decide whether notes/comments become issue body sections, front matter arrays, evidence records, separate first-class records, or local runtime state.
- Audit append-notes, close reasons, claims, and comment commands for current SQLite-only durability.
- Implement the chosen migration or create implementation children if it spans multiple commands.
- Preserve Agent Factory handoff expectations for durable notes.

Acceptance:
The durable behavior for notes/comments is documented and implemented or split into ready implementation children; append-notes and close reasons either survive delete-DB/rebuild/show JSON or are explicitly reclassified as local runtime with replacement workflow; tests or transcripts cover the chosen path.

Validation:
- cargo fmt -- --check
- cargo test comments or equivalent focused tests
- cargo test
- ./target/debug/atelier export --check
- ./target/debug/atelier doctor
