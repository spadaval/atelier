---
created_at: "2026-06-10T03:50:32.442728503+00:00"
id: "atelier-3mpl"
issue_type: "task"
labels:
- "architecture"
- "assignee:root"
- "markdown"
- "record-store"
- "storage"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-e2vh"
  - kind: "issue"
    id: "atelier-hdhk"
  - kind: "issue"
    id: "atelier-m25t"
  - kind: "issue"
    id: "atelier-po2n"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-10T18:59:54.262718968+00:00"
status: "done"
title: "Introduce RecordStore for canonical issue Markdown"
updated_at: "2026-06-10T18:59:54.262718968+00:00"
---

## Description

Build the first RecordStore slice for canonical Markdown records.

Scope:
- Add a module or equivalent boundary that discovers, loads, validates, renders, and atomically writes issue records under .atelier-state/issues.
- Reuse the canonical layout and project-scoped record ID rules already documented.
- Keep SQLite query behavior working while making the file API testable without a live tracker database.
- Do not add mission, plan, evidence, or daemon behavior in this slice.

## Outcome

RecordStore can round-trip issue Markdown with deterministic output; path/id/schema/front matter validation errors are actionable; ID allocation checks collisions across discovered records; focused tests cover valid records, malformed front matter, path mismatch, duplicate IDs, deterministic rendering, and atomic replacement failure handling; docs references point to docs/architecture/markdown-first-record-store.md.

## Evidence

- cargo fmt -- --check
- cargo test record_store or equivalent focused tests
- cargo test
- ./target/debug/atelier export --check
- ./target/debug/atelier doctor
