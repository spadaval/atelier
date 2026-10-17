---
created_at: "2026-10-17T19:00:36.356125467+00:00"
id: "atelier-mhx0"
issue_type: "bug"
labels:
- "retro-2026-10-17"
- "start"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Refuse a work branch whose type prefix does not match the issue"
updated_at: "2026-10-17T19:00:36.356125467+00:00"
---

## Goal

1jll's branch prefix did not match its issue type, so `start` recorded the wrong base and land later failed with land_base_mismatch ('recorded base task/atelier-1jll disagrees with trusted policy base master').

## Done when

`atelier start` on a branch whose `<type>/` prefix does not match the item's type refuses and names the right branch to create. A test covers it.

## Context

Found in the 2026-10-17 retrospective of the 10-15..10-17 atelier runs (sessions b32c39e6, 9a04974d, d483b55c, bf433168). Related: atelier-0vp3.
