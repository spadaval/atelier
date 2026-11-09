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
status: "todo"
title: "Answer where-are-we on the site: a page per repository and per mission"
updated_at: "2026-11-09T01:10:59.197116519+00:00"
---

## Goal

Answer "where are we on this project?" without an agent: a page per repository and per mission on `atelier serve`, showing what waits on the human, what is in progress, what is up next and what was recently done.

## Done when

- [ ] `/<repo>` shows, from the base branch ref: items waiting on the human; work in progress with its branch, review state (unreviewed, approval current or stale, findings open) and how far the branch is behind; ready work in the order `atelier work ready` gives; items done in the last 14 days; and each open mission with its item counts. A fixture test covers each section.
- [ ] `/<repo>/mission/<id>` renders the mission's Outcome, criteria, linked strategy file (Markdown rendered as HTML), its items by status with links, and its debrief link. A fixture test covers a mission with a strategy and one without.
- [ ] With a warm cache and `master` unchanged, moving only an owned work branch from unreviewed to approved, then to approval stale by a new code commit, changes `/<repo>`'s review state on the next request to match `atelier status` each time; and with an injected clock, an item leaves the recently-done section once it is more than 14 days old. Fixture tests cover both.
- [ ] The inbox and every page link to `/<repo>`, and `/<repo>` links each mission and item.
- [ ] Rendered against this repository's `master`, `/atelier` lists the same in-progress branches and ready items as `atelier status` run on `master`; the Report records the comparison.

## Context

Builds on the serve item's routes and record loading. Mission atelier-4pzj improves the shared views (for example atelier-om6l, what holds a mission up); reuse the app layer's views so those fixes reach the site too.
