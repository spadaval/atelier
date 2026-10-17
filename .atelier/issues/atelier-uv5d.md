---
created_at: "2026-10-17T01:10:57.978370312+00:00"
id: "atelier-uv5d"
issue_type: "feature"
labels:
- "cli-output"
- "review"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-rc8j"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Show review state in review list <id>, with findings under dispositions"
updated_at: "2026-10-17T01:10:57.978370312+00:00"
---

## Goal

To answer "is atelier-bw0b approved? stale? which findings are still open?", a reader has to match 22 findings to 22 dispositions by eye across 45 unordered `review list` lines, then run `merge --check`, which only works with the work branch checked out. The list never says whether the latest approval (atelier-x16l @e2562bd6) is current or stale, or which findings still need action. `merge --check --json` already computes `supports` (approval and disposition IDs), but no text view shows it.

## Done when

`atelier review list <id>` starts with a state block, then shows findings as a tree:

```
Approval: ✓ atelier-x16l @e2562bd6 (Independent reviewer 3) current
          (or ⚠ stale since 1a2b3c4 "Tweak…": changed proof-sample.txt)
Findings: 22 · 21 resolved · 1 open
  ⚠ atelier-4cid defer, no disposition  The Stop hook runs at the end of every turn…
  ✓ atelier-1ia7 defer → fix_now atelier-g6fd @a485ef02  docs/adr/0022…:41-43 'prime … is removed'…
Other events: 3 review accounts (atelier-4dbb, atelier-dgmt, atelier-qdg9)
```

- A superseded disposition shows the chain (`no_action atelier-ul3r → no_action atelier-6o75`).
- Approval state comes from the same code as the `review.current` gate (crates/atelier-app/src/review_gate.rs), not a copy of it.
- The view works from any branch that has the records.
- A fixture test covers approved, stale, open-finding and superseded-disposition states.

## Context

Depends on the row format from the review-list item. The stale reason comes from the "which approval merge --check considered" item. atelier-sdb0 shows recent reviews in status, and this is the full view behind it. In transcripts, 4 of 5 `review list`/`review show` runs were piped to `head`.
