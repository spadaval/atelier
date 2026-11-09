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
updated_at: "2026-11-09T14:32:36.611502311+00:00"
---

## Goal

Give Sumeet one queue of what waits on him across every repository on this host, linking into the brief, debrief, board and item pages, so a notification takes him straight to the page he must read. The inbox is not the home page.

## Done when

- [ ] Provisional: `GET /api/inbox` and the `/inbox` page list, grouped by repository, exactly the open items `atelier status` on each base branch lists under Needs a human, and each draft mission with a link to its brief. A Rust fixture test shows the set matches `status` with two repositories, and a Vitest test renders it from the fixture JSON.
- [ ] Provisional: each entry links to its item page, and, under the rule the Unknowns settle, to the page of the mission it asks the human to read, which opens the brief or debrief by the mission's status. A fixture test covers a decision, a loose end that names a mission, a needs-human item related only to a superseded mission, and a draft mission.
- [ ] Provisional: `/` and every board link to `/inbox` and show its count.
- [ ] Served from this host's repositories, every inbox link opens a page that renders in Playwright without an error; the Report records the command and counts.

## Scope

Read-only. No new link role or record field to choose an entry's target.

## Context

Planned at the evidence boundary after atelier-zpmb and atelier-ahpf: they show how status-like views cost when read from refs, and the item page this item links to. Sumeet's 2026-11-08 direction on atelier-yfqu: "the inbox is a notification queue linking into those pages, not the home page."

## Unknowns

- Which missions an entry links besides its item page. Any linked mission is too loose: needs-human atelier-3km4 asks for atelier-vt8q's debrief but relates only to superseded mission atelier-gg6b (finding atelier-ei8w). Settle a rule, or link only the item page, and say which in the Report.
- Whether listing every draft mission is noisy; if it is, show only drafts with a recorded planning approval, and say so in the Report.
