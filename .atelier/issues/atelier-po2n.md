---
created_at: "2026-06-10T03:50:40.571741313+00:00"
id: "atelier-po2n"
issue_type: "task"
labels:
- "architecture"
- "assignee:root"
- "freshness"
- "projection"
- "sqlite"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-a4ps"
  - kind: "issue"
    id: "atelier-e2vh"
  - kind: "issue"
    id: "atelier-hdhk"
  - kind: "issue"
    id: "atelier-tfn8"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-10T20:14:20.047794054+00:00"
status: "done"
title: "Extract ProjectionIndex freshness and reindex semantics"
updated_at: "2026-06-10T20:14:20.047794054+00:00"
---

## Description

Define and implement the first ProjectionIndex boundary for rebuildable SQLite query indexes.

Scope:
- Separate canonical projection/index responsibilities from local runtime tables in code structure and health reporting.
- Add source freshness metadata for indexed Markdown records using content hashes where needed and mtime/size only as hints.
- Define command behavior when issue list/ready/search would read stale projection data.
- Support full rebuild first; targeted reindex may be added if small and well-tested.
- Do not migrate all mutating commands in this slice.

## Outcome

ProjectionIndex has an explicit module/API boundary; stale Markdown versus SQLite index state is detected before orchestration-relevant queries; stale-query behavior is deterministic and actionable; rebuild still recreates queryable issue/dependency/link state from .atelier-state; tests cover fresh, stale, missing, and repaired projection cases.

## Evidence

- cargo fmt -- --check
- cargo test projection or equivalent focused tests
- cargo test
- ./target/debug/atelier issue ready
- ./target/debug/atelier export --check
- ./target/debug/atelier doctor
