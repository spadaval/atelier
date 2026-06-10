---
acceptance: []
blocks:
- "atelier-000g"
- "atelier-0011"
- "atelier-0012"
created_at: "2026-06-08T17:33:27+00:00"
depends_on:
- "atelier-000w"
evidence_required: []
id: "atelier-000y"
issue_type: "task"
labels:
- "feature"
- "projection"
- "spec"
- "storage"
links: []
parent: "atelier-0009"
priority: "P1"
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Implement deterministic per-record export and `export --check`"
updated_at: "2026-06-08T21:23:39+00:00"
---


Replace or augment backup-oriented export with canonical projections under `.atelier-state/`. `export --check` must compare live SQLite state with exported files and fail when projections are stale.

## Acceptance Criteria

Export output is deterministic across repeated runs; stale projection cases fail export --check; mutating command behavior is documented; tests cover no-op export, changed record export, stale check failure, and JSON/Markdown serialization stability.
