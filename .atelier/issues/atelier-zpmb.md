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
updated_at: "2026-11-09T01:04:24.223687973+00:00"
---

## Goal

Add `atelier serve`, a read-only HTTP server that shows every Atelier repository on this host, with an inbox of everything waiting on the human, so he can open any document meant for him, such as a mission debrief, in one click from his phone. It must stay light: this host also runs the T3 Code server his phone depends on.

## Done when

- [ ] `atelier serve [--bind <addr>] [--root <dir>...]` finds repositories by scanning the roots (default `~/projects`) for `.atelier/`, counts each repository once however many worktrees it has (by Git common directory), and reads records from each repository's base branch ref, never from a checkout. A test with a repository and two worktrees, one detached and behind, shows one repository with the base branch's records.
- [ ] `/` is the inbox: exactly the open items each repository's `atelier status` on its base branch lists under Needs a human, grouped by repository. Each entry's primary link opens its reading target directly: when the item links a mission (any `relates` link to a mission record), that mission's rendered debrief; otherwise the item's page. A secondary link always opens the item page. A fixture with a needs-human item linked to a finished mission asserts the inbox link returns that mission's debrief HTML, and a fixture test checks the inbox set matches `status`.
- [ ] `/<repo>/item/<id>` renders the item's record (body, status, links, reviews and findings), and `/<repo>/mission/<id>/debrief` renders the debrief through a rendering function that returns HTML and writes nothing; the CLI's `debrief --html` writes that function's output to its file. Fixture tests cover both routes.
- [ ] Read-only proof: with each fixture repository's working tree, `.git` directory and `.atelier/runtime/` made unwritable, every route returns 200 and the same HTML as with them writable, and refs and `git status` are unchanged.
- [ ] Load bound: rendered pages are cached by repository and base branch commit, so a page is computed once per base commit; at most two renders run at once, further requests wait or get a 503 page within 5 seconds; and with 20 concurrent requests for this repository's largest mission debrief on a cold cache, the server's peak RSS stays under 300 MB and T3 Code's web endpoint keeps answering in under 1 second. The Report records the numbers.
- [ ] The repository ships a systemd user unit for `atelier serve` with `Restart=on-failure`, `MemoryMax=` and `CPUQuota=` set, and `docs/` says how to install it and publish it with `tailscale serve` beside T3 Code's published port.
- [ ] Every page is server-rendered HTML without a front-end framework and readable at a 390 px wide viewport. Run against this host's repositories, every inbox link returns 200; the Report records the command and counts.

## Scope

Read-only. No repository overview or mission strategy pages (atelier-ahpf), no attribution or T3 data. Keep new dependencies small.

## Context

ADR 0030 governs. `crates/atelier-cli/src/commands/mission_review.rs` holds the debrief renderer, whose `write_review_page` writes `.atelier/runtime/reviews/<id>-debrief.html`; the debrief traverses Git history (`crates/atelier-app/src/mission_review.rs`, `record_history.rs`), so it is the expensive page. Cross-branch status already reads records at a ref. Planning review findings atelier-bihd, atelier-5yz1, atelier-ntq9 and atelier-8j0j shaped these lines.

## Unknowns

- Which HTTP library fits the workspace's dependency budget; choose and say why in the Report.
