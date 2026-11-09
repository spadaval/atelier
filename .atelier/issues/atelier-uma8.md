---
created_at: "2026-11-09T06:41:35.317557607+00:00"
id: "atelier-uma8"
issue_type: "feature"
labels:
- "fleet-dashboard"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-ahpf"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-yfqu"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "List what waits on Sumeet across every repository, linking into the site's pages"
updated_at: "2026-11-09T06:41:35.317557607+00:00"
---

## Goal

Give Sumeet one queue of what waits on him across every repository on this host, linking into the brief, debrief, board and item pages, so a notification takes him straight to the page he must read. The inbox is not the home page.

## Done when

- [ ] Provisional: `/inbox` lists, grouped by repository, exactly the open items `atelier status` on each base branch lists under Needs a human, and each draft mission with a link to its brief. A fixture test shows the set matches `status` with two repositories.
- [ ] Provisional: each entry links to its item page, and an item linked to a mission (any role) also links to that mission's page, which opens the brief or debrief by the mission's status. A fixture test covers a decision, a loose end that names a mission, and a draft mission.
- [ ] Provisional: `/` and every board link to `/inbox` and show its count.
- [ ] Run against this host's repositories, every inbox link returns 200; the Report records the command and counts.

## Scope

Read-only. No new link role or record field to choose an entry's target.

## Context

Planned at the evidence boundary after atelier-zpmb and atelier-ahpf: they show how status-like views cost when read from refs, and the item page this item links to. Sumeet's 2026-11-08 direction on atelier-yfqu: "the inbox is a notification queue linking into those pages, not the home page."

## Unknowns

- Whether listing every draft mission is noisy; if it is, show only drafts with a recorded planning approval, and say so in the Report.
