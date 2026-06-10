---
acceptance: []
blocks:
- "atelier-e2vh"
- "atelier-hdhk"
- "atelier-m25t"
- "atelier-po2n"
created_at: "2026-06-10T03:50:32.442728503+00:00"
depends_on: []
evidence_required: []
id: "atelier-3mpl"
issue_type: "task"
labels:
- "architecture"
- "markdown"
- "record-store"
- "storage"
links: []
parent: "atelier-zd4d"
priority: "P1"
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Introduce RecordStore for canonical issue Markdown"
updated_at: "2026-06-10T03:50:32.442728503+00:00"
---

Build the first RecordStore slice for canonical Markdown records.

Scope:
- Add a module or equivalent boundary that discovers, loads, validates, renders, and atomically writes issue records under .atelier-state/issues.
- Reuse the canonical layout and project-scoped record ID rules already documented.
- Keep SQLite query behavior working while making the file API testable without a live tracker database.
- Do not add mission, plan, evidence, or daemon behavior in this slice.

Acceptance:
RecordStore can round-trip issue Markdown with deterministic output; path/id/schema/front matter validation errors are actionable; ID allocation checks collisions across discovered records; focused tests cover valid records, malformed front matter, path mismatch, duplicate IDs, deterministic rendering, and atomic replacement failure handling; docs references point to docs/architecture/markdown-first-record-store.md.

Validation:
- cargo fmt -- --check
- cargo test record_store or equivalent focused tests
- cargo test
- ./target/debug/atelier export --check
- ./target/debug/atelier doctor
