---
created_at: "2026-06-10T00:34:21.482704428+00:00"
id: "atelier-0027"
issue_type: "task"
labels:
- "closeout"
- "migration"
- "storage"
- "validation"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-10T02:19:09.851705843+00:00"
status: "done"
title: "Validate markdown-only canonical state cutover"
updated_at: "2026-06-10T02:19:09.851705843+00:00"
---

## Description

Close out the markdown-only canonical state and record identity cutover. Prove that the repository no longer depends on manifest.json or graph.json as canonical source-of-truth files and that project-scoped random IDs work across commands, export/check, rebuild, lint, and fixtures.

## Outcome

cargo fmt -- --check, cargo test, git diff --check, atelier lint, atelier export --check, and atelier doctor pass after migration; residue searches find no canonical manifest.json/graph.json dependency; rebuild from discovered Markdown records succeeds in a fresh runtime DB; docs, fixtures, and .atelier-state are internally consistent; remaining compatibility gaps are tracked explicitly.

## Evidence

Evidence was not specified in the legacy issue record.
