---
created_at: "2026-06-10T16:00:59.309465327+00:00"
id: "atelier-6kkz"
issue_type: "task"
labels:
- "activity"
- "export"
- "projection"
- "rebuild"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-krhk"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-10T17:32:25.922110761+00:00"
status: "done"
title: "Rebuild and export issue activity sidecars"
updated_at: "2026-06-10T17:32:25.922110761+00:00"
---

## Description

Make activity sidecars canonical state for export, rebuild, and projections.

What:
- Include activity sidecar files in canonical export/check behavior once they exist.
- During rebuild, discover issue activity files and repopulate the projection/index required by `issue show`, `history`, and JSON output.
- Ensure deleting `.atelier/state.db` and rebuilding preserves activity-derived history.

Out of scope:
- Designing mission or plan activity projections.
- The one-off SQLite migration script.
- `atelier export --check` accounts for activity sidecars without spurious drift.
- `atelier rebuild` reconstructs activity projections from `.atelier-state/` alone.
- `issue show`, `atelier history`, and JSON output see the same activity before and after runtime DB deletion/rebuild.
- Tests cover rebuild from canonical sidecars and export freshness behavior.

Recommended subskill: agent-factory implement.
Evidence was not specified in the legacy issue record.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
