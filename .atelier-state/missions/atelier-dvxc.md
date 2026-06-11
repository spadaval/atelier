---
created_at: "2026-06-11T15:53:53.956344586+00:00"
id: "atelier-dvxc"
data: "{\"constraints\":[\"Canonical durable records live in .atelier-state/ and are mutated through RecordStore before projection refresh.\",\"SQLite remains a rebuildable ProjectionIndex plus RuntimeState, not a competing durable source of truth.\",\"Ordinary projection-backed read commands transparently rebuild stale cache state when .atelier-state/ validates.\"],\"evidence\":[],\"milestones\":[],\"plans\":[],\"risks\":[\"Transparent rebuild could hide invalid or partially written Markdown unless validation and locking boundaries are strict.\",\"A broad command migration could regress existing Agent Factory issue and mission workflows if slices are not validated incrementally.\"],\"validation\":[\"Linked work proves durable mutation command audit, Markdown-first write migration, transparent stale projection rebuild, invalid Markdown failure behavior, concurrency safety, export freshness, lint, doctor, and rebuild-from-checkout scenarios.\"],\"work\":[]}"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-64w5"
  - kind: "issue"
    id: "atelier-8ptg"
  - kind: "issue"
    id: "atelier-ncog"
  - kind: "issue"
    id: "atelier-rr6y"
  attachments: []
  relates: []
schema: "atelier.mission"
schema_version: 1
status: "open"
title: "Finish Markdown-first storage transition"
updated_at: "2026-06-11T15:53:53.956344586+00:00"
---

Complete the second phase of the Markdown-first storage transition. The product outcome is that durable project state is owned by Markdown records, while SQLite behaves as an automatically repaired hot cache/projection for query speed and UI inputs. Operators should not need to understand or manually repair the cache during ordinary read workflows.
