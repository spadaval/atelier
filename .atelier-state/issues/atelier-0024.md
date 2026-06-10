---
acceptance: []
blocks: []
created_at: "2026-06-10T00:33:49.477953835+00:00"
depends_on: []
evidence_required: []
id: "atelier-0024"
issue_type: "task"
labels:
- "assignee:root"
- "epic"
- "identity"
- "markdown"
- "migration"
- "storage"
links: []
parent: "atelier-000j"
priority: "P1"
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Epic: Markdown-only canonical state and record identity cutover"
updated_at: "2026-06-10T00:34:46.403574285+00:00"
---

Cut over Atelier's at-rest model to Markdown record files as the canonical source, with project-scoped random record IDs and no generated aggregate source-of-truth files. This epic covers the hard ID migration plus removing manifest.json and graph.json from the canonical projection model.

## Scope

- Reuse atelier-0023 for project-scoped random record IDs such as atelier-z1p8.
- Make each record Markdown file own its canonical metadata in YAML front matter.
- Remove manifest.json as canonical inventory/hash state; rebuild discovers and validates record files directly.
- Move graph relationships out of graph.json and into record-owned front matter or per-record link metadata.
- Migrate existing .atelier-state files, SQLite rows, tests, fixtures, docs, command parsing, JSON output, export, rebuild, and lint.

## Acceptance

The repository can rebuild SQLite from discovered Markdown records without manifest.json or graph.json; issue and future first-class record files contain their own canonical metadata and links; IDs use the project-scoped random format globally; export/check/lint detect stale, malformed, duplicate, and dangling records without aggregate manifest state; tests and fixtures prove migration from current typed-prefix IDs plus manifest/graph state.
