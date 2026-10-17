---
created_at: "2026-10-17T19:00:36.341319071+00:00"
id: "atelier-jbwi"
issue_type: "feature"
labels:
- "orientation"
- "retro-2026-10-17"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Show planned versus built work and backlog health in work missions"
updated_at: "2026-10-17T19:00:36.341319071+00:00"
---

## Goal

About 85 of the 99 CLI-audit issues sit untouched in five draft missions, the backlog grows with every review pass, and the user believed brief/debrief had shipped when it was still a draft mission (33405776).

## Done when

`atelier work missions` shows per mission how much is done, active and not started, flags draft missions with no started work, and counts backlog items older than a configurable age. Rendering tests cover it.

## Context

Found in the 2026-10-17 retrospective of the 10-15..10-17 atelier runs (sessions b32c39e6, 9a04974d, d483b55c, bf433168).
