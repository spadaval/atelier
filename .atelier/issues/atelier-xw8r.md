---
created_at: "2026-10-17T01:10:57.966093266+00:00"
id: "atelier-xw8r"
issue_type: "bug"
labels:
- "cli-output"
- "start"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-4pzj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Point start hints at atelier start, not the record-only transition"
updated_at: "2026-10-17T01:10:57.966093266+00:00"
---

## Goal

`atelier start <mission>` says `No item atelier-4pzj is on the base branch master or on a branch named atelier-4pzj.` and points at `atelier status`, without saying it is a mission or naming its items (checked 2026-11-11 with the new binary in a converted scratch clone).

## Done when

- [ ] `atelier start <mission>` says it is a mission and lists the mission's items that can start (ID and title, up to 5), or says none can.
- [ ] A CLI test covers the mission error.

## Context

Rewritten for the new tool in the cutover sort (atelier-5zre): the new tool has no record-only `issue transition`, and `status` names `atelier start <id>`, so the hint and warning parts left; the mission error remains.
