---
acceptance: []
created_at: "2026-06-11T20:10:00.310771857+00:00"
evidence_required: []
id: "atelier-7n3w"
issue_type: "epic"
labels:
- "cache"
- "projection"
- "sqlite"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-24sg"
  children:
  - kind: "issue"
    id: "atelier-22z4"
  - kind: "issue"
    id: "atelier-cd1l"
  - kind: "issue"
    id: "atelier-eprw"
  - kind: "issue"
    id: "atelier-t0s4"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Centralize projection freshness and command access modes"
updated_at: "2026-06-11T23:59:23.651878444+00:00"
---

Centralize projection access and freshness so commands do not make ad hoc choices between stale SQLite, fresh projection, or canonical Markdown reads.

Acceptance criteria:
- Command access modes are explicit: projection query, canonical mutation, runtime-only, and health/repair.
- All projection-backed query commands check source metadata and refresh from Markdown when safe.
- Invalid canonical Markdown causes actionable failures with atelier lint guidance rather than stale SQLite reads.
- Detail-heavy commands resolve/query through ProjectionIndex and hydrate full bodies from canonical Markdown.
- Runtime-only commands and doctor/repair paths are clearly separated from canonical mutation and projection query paths.
