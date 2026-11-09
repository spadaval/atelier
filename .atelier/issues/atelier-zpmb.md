---
created_at: "2026-11-09T00:45:58.188376772+00:00"
id: "atelier-zpmb"
issue_type: "feature"
labels:
- "fleet-dashboard"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-tu8c"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-yfqu"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Add atelier serve: one read-only site for every repository here, with an inbox of what waits on the human"
updated_at: "2026-11-09T00:45:58.188376772+00:00"
---

## Goal

Add `atelier serve`, a read-only HTTP server that shows every Atelier repository on this host, with an inbox of everything waiting on the human, so he can reach any document meant for him, such as a mission debrief, in one click from his phone.

## Done when

- [ ] `atelier serve [--bind <addr>] [--root <dir>...]` finds repositories by scanning the roots (default `~/projects`) for `.atelier/`, counts each repository once however many worktrees it has (by Git common directory), and reads records from each repository's base branch ref, never from a checkout. A test with a repository and two worktrees, one of them detached and behind, shows one repository with the base branch's records.
- [ ] `/` is the inbox: every open item in every repository that waits on the human (the `Needs a human` set `atelier status` shows), grouped by repository, each linking its item page. A fixture test checks the set matches `status` for the same repository.
- [ ] `/<repo>/item/<id>` renders the item's record (body, status, links, reviews and findings) and, for a mission, links its debrief; `/<repo>/mission/<id>/debrief` renders the debrief with the existing renderer, generated on request. Fixture tests cover both.
- [ ] Every page is server-rendered HTML without a front-end framework, readable at phone width, and the server makes no writes to any repository (a test runs every route and checks each repository's `git status` and refs are unchanged).
- [ ] Run against this host's repositories, every inbox link returns 200; the item's Report records the command and counts.

## Scope

Read-only. No repository overview or mission strategy pages (separate item), no attribution or T3 data. Keep new dependencies small; this host also runs the T3 Code server.

## Context

ADR 0030 governs. `crates/atelier-cli/src/commands/mission_review.rs` holds the debrief renderer, which today writes `.atelier/runtime/reviews/<id>-debrief.html`. Cross-branch status already reads records at a ref.

## Unknowns

- Which HTTP library fits the workspace's dependency budget; choose and say why in the Report.
