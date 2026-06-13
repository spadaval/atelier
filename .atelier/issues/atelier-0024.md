---
created_at: "2026-06-10T00:33:49.477953835+00:00"
id: "atelier-0024"
issue_type: "epic"
labels:
- "assignee:root"
- "epic"
- "identity"
- "markdown"
- "migration"
- "storage"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-0023"
  - kind: "issue"
    id: "atelier-0025"
  - kind: "issue"
    id: "atelier-0026"
  - kind: "issue"
    id: "atelier-0027"
  - kind: "issue"
    id: "atelier-0028"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-11T14:42:00.416727701+00:00"
status: "done"
title: "Epic: Markdown-only canonical state and record identity cutover"
updated_at: "2026-06-11T14:42:00.416727701+00:00"
---

## Description

Cut over Atelier's at-rest model to Markdown record files as the canonical source, with project-scoped random record IDs and no generated aggregate source-of-truth files. This epic covers the hard ID migration plus removing manifest.json and graph.json from the canonical projection model.

## Outcome

The repository can rebuild SQLite from discovered Markdown records without manifest.json or graph.json; issue and future first-class record files contain their own canonical metadata and links; IDs use the project-scoped random format globally; export/check/lint detect stale, malformed, duplicate, and dangling records without aggregate manifest state; tests and fixtures prove migration from current typed-prefix IDs plus manifest/graph state.

## Evidence

Evidence was not specified in the legacy issue record.

## Notes

### Scope

- Reuse atelier-0023 for project-scoped random record IDs such as atelier-z1p8.
- Make each record Markdown file own its canonical metadata in YAML front matter.
- Remove manifest.json as canonical inventory/hash state; rebuild discovers and validates record files directly.
- Move graph relationships out of graph.json and into record-owned front matter or per-record link metadata.
- Migrate existing .atelier-state files, SQLite rows, tests, fixtures, docs, command parsing, JSON output, export, rebuild, and lint.
