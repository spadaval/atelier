---
created_at: "2026-11-09T00:45:58.429754395+00:00"
id: "atelier-ahpf"
issue_type: "feature"
labels:
- "fleet-dashboard"
workflow_branch:
  branch_base: master
  work_branch: t3/fleet-board-item-pages
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-yfqu"
    type: "advances"
  - kind: "issue"
    id: "atelier-yn69"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Answer where are we, what's done and what's next: a board per repository and a page per item"
updated_at: "2026-11-09T20:03:36.231713120+00:00"
---

## Goal

Answer "where are we, what's done and what's next?" for each repository without an agent: a board per repository in the site's app, with a page per item, read from the base branch through the JSON API.

## Done when

- [ ] `GET /api/<repo>/board` and `/api/<repo>/item/<id>` return the `atelier-app` views the board and item pages show, with generated types and fixture JSON as atelier-zpmb set up. `/<repo>` is the board: each open mission with its progress (items done of planned, criteria proven) linking its mission page; all open work grouped by mission, with work in no mission grouped by label, each with its status; work in progress with its branch, review state (unreviewed, approval current or stale, findings open) and how far it is behind; ready work in the order `atelier work ready` gives; and items done in the last 14 days. A Rust fixture test covers each section's data, and a Vitest test renders each section from the fixture JSON.
- [ ] `/<repo>/item/<id>` renders the item's body (Markdown), status, links, notes, reviews and findings, and a decision shows its Question and Options. A fixture test covers a task, a decision and a loose end.
- [ ] with a warm cache and the base unchanged, moving an owned work branch from unreviewed to approved, then to approval stale by a new code commit, changes the board API's review state on the next request to match `atelier status` each time; with an injected clock, an item leaves the recently-done section once it is more than 14 days old.
- [ ] `/` shows each repository's counts and links its board; the board links each mission and item; brief, debrief and item pages link back to the board.
- [ ] Load: on this host, served by a release build, a board request for this repository takes under 3 s cold and under 500 ms when neither the base nor any work branch has moved, within atelier-zpmb's two-build cap. The Report records the numbers.
- [ ] The Playwright suite loads the board and an item page of each kind at 390 px. Served from this repository's `master`, `/atelier` lists the same in-progress, ready and needs-human items as `atelier status` run on `master`; the Report records the comparison.

## Context

Planned at the evidence boundary after atelier-zpmb and atelier-yn69, which show whether views read from a base commit and other refs are cheap enough and how the shared views must change. `atelier status` costs 1.6 s here today, most of it reading each work branch's tracker (atelier-3i7x). Reuse the app layer's views so fixes from mission atelier-4pzj (such as atelier-om6l) reach the board too. Sumeet's 2026-11-08 direction on atelier-yfqu: "a backlog board per repository (missions with progress, all open work by mission, label and status, recently done) answering where are we, what's done, what's next."

## Unknowns

- What a board request costs. Branch and review state come from each owned work branch's records at a pinned commit of that branch, read afresh per request as atelier-zpmb does for refs; `atelier status` takes 1.6 s here today, most of it reading each work branch's tracker (atelier-3i7x). If a cold board request takes over 3 s, cache what depends only on the base commit and each branch's commit, keyed by those commits.
