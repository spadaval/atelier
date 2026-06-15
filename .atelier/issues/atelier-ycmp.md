---
created_at: "2026-06-15T05:11:28.677167383+00:00"
id: "atelier-ycmp"
issue_type: "epic"
labels:
- "fuzz"
- "rewrite"
- "tests"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-fchz"
  children:
  - kind: "issue"
    id: "atelier-7vfj"
  - kind: "issue"
    id: "atelier-uz8g"
  - kind: "issue"
    id: "atelier-v64l"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-15T08:18:55.204262331+00:00"
status: "done"
title: "Epic: Stratify tests and fuzz targets by crate boundary"
updated_at: "2026-06-15T08:18:55.204262331+00:00"
---

## Description

Restructure tests and fuzz targets so each new crate owns focused invariants while CLI integration coverage remains targeted at visible operator workflows.

## Outcome

- The large CLI integration suite is split into focused workflow or fixture groups with shared helpers.
- Domain, records, workflow, and SQLite crates each have local tests for their owned invariants.
- Fuzz harnesses target the new internal crate APIs instead of the old single-crate `atelier::db::Database` surface.
- CLI transcript/golden coverage remains for stable visible workflows and rejected-command behavior.

## Evidence

- Child issue proof shows integration-test stratification, crate-level test additions, and fuzz retargeting.
- `cargo nextest run` and applicable fuzz build checks pass.
- Test inventory or focused search proves old single-crate API assumptions were removed from tests and fuzz targets.
