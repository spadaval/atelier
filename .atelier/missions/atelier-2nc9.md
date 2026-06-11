---
created_at: "2026-06-11T18:22:52.342869541+00:00"
id: "atelier-2nc9"
data: "{\"constraints\":[\"Canonical Markdown under .atelier-state/ is the durable source of truth for every public project-state mutation.\",\"SQLite is only ProjectionIndex plus RuntimeState; mutation code must not depend on export_current_state as the normal writer.\",\"Slices must preserve existing Agent Factory issue and mission workflows while migration proceeds incrementally.\"],\"evidence\":[],\"milestones\":[],\"plans\":[],\"risks\":[\"Moving mutation ownership can regress relationships, activity sidecars, or projection freshness if canonical write and refresh boundaries are not explicit.\",\"Trying to rewrite every command at once could create broad persistence breakage; work must be split by mutation family and validated after each family.\"],\"validation\":[\"Linked epics prove issue lifecycle, issue relationships, first-class records, projection refresh, compatibility removal, and closeout validation for fresh checkout rebuild, mutation recoverability, stale projection repair, invalid Markdown failure, lint, doctor, export freshness, and full Rust test coverage.\"],\"work\":[]}"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-7dj5"
  - kind: "issue"
    id: "atelier-afir"
  - kind: "issue"
    id: "atelier-mszu"
  - kind: "issue"
    id: "atelier-p6hv"
  - kind: "issue"
    id: "atelier-ybt6"
  - kind: "issue"
    id: "atelier-yiiz"
  attachments: []
  relates: []
schema: "atelier.mission"
schema_version: 1
status: "closed"
title: "Finish RecordStore-owned durable mutations"
updated_at: "2026-06-11T18:59:55.061237338+00:00"
---

Desired state: public durable mutations write canonical Markdown through RecordStore first, then refresh SQLite as a rebuildable projection. Export remains an operator repair/sync tool, not the normal durability mechanism for command writes.
