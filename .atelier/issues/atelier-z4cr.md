---
created_at: "2026-11-11T02:17:25.300286627+00:00"
id: "atelier-z4cr"
issue_type: "mission"
labels: []
strategy:
  path: docs/strategies/desk.md
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "draft"
title: "Build the Desk: the human's place to plan missions and read results, on the redesigned records"
updated_at: "2026-11-11T02:17:25.300286627+00:00"
---

## Outcome

On this host, a service reached over the tailnet shows every Atelier repository on the redesigned records. A mission has one page with what it is for, its documents, its items and what happened to them, and the agents working on it. An inbox lists what waits on Sumeet. From a page he can say go on a mission, answer an open question, and start a planning session with an agent. Each act is a commit made by an `atelier` command under his name. `atelier` itself has no site.

## Background

Planned on 2026-11-11 from Sumeet's direction in the design discussion over the dashboard spike (branch `spike/mission-control`). ADR 0032 holds the decisions and `docs/product/desk.md` the product intent. It runs beside the redesign's cutover (atelier-qr5s) and neither waits on the other: the Desk is built against converted scratch clones and goes live after this host's cutover (atelier-wd1i). The mission's area was swept by hand in place of a groom run: atelier-oci2 now deletes the old site, atelier-wpj2 is superseded by this mission, and atelier-aab1's questions are answered by the note on it except whether thread links open on the phone. Health, the feed and the database are the next mission.
