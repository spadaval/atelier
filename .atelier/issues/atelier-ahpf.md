---
created_at: "2026-11-09T00:45:58.429754395+00:00"
id: "atelier-ahpf"
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
    id: "atelier-yfqu"
    type: "advances"
  - kind: "issue"
    id: "atelier-zpmb"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Answer where are we, what's done and what's next: a board per repository and a page per item"
updated_at: "2026-11-09T06:42:17.601608576+00:00"
---

## Goal

Answer "where are we, what's done and what's next?" for each repository without an agent: a board per repository on `atelier serve`, with a page per item, read from the base branch.

## Done when

- [ ] Provisional: `/<repo>` is the board: each open mission with its progress (items done of planned, criteria proven) linking its mission page; all open work grouped by mission, with work in no mission grouped by label, each with its status; work in progress with its branch, review state (unreviewed, approval current or stale, findings open) and how far it is behind; ready work in the order `atelier work ready` gives; and items done in the last 14 days. A fixture test covers each section.
- [ ] Provisional: `/<repo>/item/<id>` renders the item's body (Markdown), status, links, notes, reviews and findings, and a decision shows its Question and Options. A fixture test covers a task, a decision and a loose end.
- [ ] Provisional: with a warm cache and the base unchanged, moving an owned work branch from unreviewed to approved, then to approval stale by a new code commit, changes the board's review state on the next request to match `atelier status` each time; with an injected clock, an item leaves the recently-done section once it is more than 14 days old.
- [ ] Provisional: `/` shows each repository's counts and links its board; the board links each mission and item; brief, debrief and item pages link back to the board.
- [ ] Rendered against this repository's `master`, `/atelier` lists the same in-progress, ready and needs-human items as `atelier status` run on `master`; the Report records the comparison.

## Context

Planned at the evidence boundary after atelier-zpmb, which shows whether views read from a base commit and other refs are cheap enough and how the shared views must change. `atelier status` costs 1.6 s here today, most of it reading each work branch's tracker (atelier-3i7x). Reuse the app layer's views so fixes from mission atelier-4pzj (such as atelier-om6l) reach the board too. Sumeet's 2026-11-08 direction on atelier-yfqu: "a backlog board per repository (missions with progress, all open work by mission, label and status, recently done) answering where are we, what's done, what's next."

## Unknowns

- How the board reads branch and review state from refs without a checkout, and what it costs; zpmb's loader and measurements decide.
