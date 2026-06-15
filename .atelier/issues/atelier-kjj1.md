---
created_at: "2026-06-15T05:11:23.268768382+00:00"
id: "atelier-kjj1"
issue_type: "epic"
labels:
- "markdown"
- "rewrite"
- "storage"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-3kap"
  - kind: "issue"
    id: "atelier-4j3k"
  - kind: "issue"
    id: "atelier-fchz"
  children:
  - kind: "issue"
    id: "atelier-8wvr"
  - kind: "issue"
    id: "atelier-rxgn"
  - kind: "issue"
    id: "atelier-y3ur"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "validation"
title: "Epic: Extract RecordStore into atelier-records"
updated_at: "2026-06-15T18:13:22.809195103+00:00"
---

## Description

Extract canonical Markdown ownership into `atelier-records` and break the current RecordStore god module into cohesive modules without changing the authored `.atelier/` record format.

## Outcome

- `atelier-records` owns canonical Markdown discovery, parsing, validation, deterministic rendering, ID allocation, atomic writes, relationship rendering, and activity sidecars.
- The existing Markdown layout and record body/front matter contracts remain stable unless the architecture contract explicitly changes them.
- RecordStore mutation APIs are clear enough for application code to avoid duplicating record-kind lists or relationship constructors.
- Oversized record-store code is split by durable ownership rather than moved mechanically.

## Evidence

- Child issue proof shows parsing/rendering extraction, mutation API extraction, and round-trip validation.
- Record round-trip tests cover issues, missions, plans, evidence, relationships, and activity sidecars.
- `atelier lint`, `atelier export --check`, and targeted RecordStore tests pass.

## Notes

- Temporary adapters used while extracting `atelier-records` must follow
  `docs/architecture/source-layout.md`: name the adapter marker, removal owner,
  removal condition, and proof that no public compatibility promise is being
  created.
