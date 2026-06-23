---
created_at: "2026-06-23T16:21:20.076733281+00:00"
id: "atelier-x7lq"
issue_type: "epic"
labels: []
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-2jse"
  - kind: "issue"
    id: "atelier-5m81"
  - kind: "issue"
    id: "atelier-5yd6"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-mska"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Epic: Lazy cache access and freshness"
updated_at: "2026-06-23T16:21:20.371126304+00:00"
---

## Description

Replace eager projection refresh behavior with an explicit lazy cache boundary. Commands refresh or rebuild cache only when they need cache data. Incremental repair should reparse/reindex changed record files when the changed set is small and fall back to full rebuild when source metadata is missing, changes are broad, or global invariants cannot be trusted locally.

## Outcome

- The cache layer owns freshness checks and write paths no longer call refresh-after-write helpers.
- Changed-record reindexing is the normal small-change repair path; full rebuild remains the safety fallback.

## Evidence

- Test output proves record writes can leave cache stale without failing.
- Test output proves query commands obtain cache through the cache boundary and repair/rebuild on demand.
- File changes in cache access code show Atelier-authored writes can provide exact changed record paths to avoid scanning, while external edits may be discovered through Git/status metadata, file metadata, and cache source metadata.
