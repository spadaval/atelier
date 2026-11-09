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
updated_at: "2026-11-09T01:10:58.834684353+00:00"
---

## Goal

Add `atelier serve`, a read-only HTTP server that shows every Atelier repository on this host, with an inbox of everything waiting on the human, so he can open any document meant for him, such as a mission debrief, in one click from his phone. It must stay light: this host also runs the T3 Code server his phone depends on.

## Done when

- [ ] `atelier serve [--bind <addr>] [--root <dir>...]` finds repositories by scanning the roots (default `~/projects`) for `.atelier/`, counts each repository once however many worktrees it has (by Git common directory), and reads records from each repository's base branch ref, never from a checkout. A test with a repository and two worktrees, one detached and behind, shows one repository with the base branch's records.
- [ ] `/` is the inbox: exactly the open items each repository's `atelier status` on its base branch lists under Needs a human, grouped by repository. An item names what the human should read with a new `reads` link role to a mission (`atelier issue link <item> <mission> --role reads`); membership and context links (`advances`, `related` and the rest) never choose a target. An entry's primary link opens the `reads` mission's rendered debrief; with no `reads` link it opens the item's page, which for a decision shows its Question and Options; `atelier check` warns when an item has more than one `reads` link, and the inbox then opens the item page. A secondary link always opens the item page. Fixture tests: a debrief read with a `reads` link and an unrelated `related` mission link opens the `reads` mission's debrief HTML; an item with only a `related` mission link opens its item page; a decision that advances a mission opens its item page; and the inbox set matches `status`.
- [ ] `/<repo>/item/<id>` renders the item's record (body, status, links, reviews and findings), and `/<repo>/mission/<id>/debrief` renders the debrief through a rendering function that returns HTML and writes nothing; the CLI's `debrief --html` writes that function's output to its file. Fixture tests cover both routes.
- [ ] Read-only proof: with each fixture repository's working tree, `.git` directory and `.atelier/runtime/` made unwritable, every route returns 200 and the same HTML as with them writable, and refs and `git status` are unchanged.
- [ ] Load bound: only work that depends on nothing but the base commit, such as a mission's debrief and record parsing, is cached, keyed by repository and base commit. Anything read from other refs (branches, review state, how far behind) is recomputed on each request or keyed by the commits of every ref it read, and anything that depends on the date is keyed by the date. At most two renders run at once; further requests wait or get a 503 page within 5 seconds. With 20 concurrent requests for this repository's largest mission debrief on a cold cache, the server's peak RSS stays under 300 MB and T3 Code's web endpoint keeps answering in under 1 second. The Report records the numbers.
- [ ] The repository ships a systemd user unit for `atelier serve` with `Restart=on-failure`, `MemoryMax=` and `CPUQuota=` set, and `docs/` says how to install it and publish it with `tailscale serve` beside T3 Code's published port.
- [ ] Every page is server-rendered HTML without a front-end framework and readable at a 390 px wide viewport. Run against this host's repositories, every inbox link returns 200; the Report records the command and counts.

## Scope

Read-only. No repository overview or mission strategy pages (atelier-ahpf), no attribution or T3 data. Keep new dependencies small.

## Context

ADR 0030 governs. `crates/atelier-cli/src/commands/mission_review.rs` holds the debrief renderer, whose `write_review_page` writes `.atelier/runtime/reviews/<id>-debrief.html`; the debrief traverses Git history (`crates/atelier-app/src/mission_review.rs`, `record_history.rs`), so it is the expensive page. Cross-branch status already reads records at a ref. Planning review findings atelier-bihd, atelier-mhb4, atelier-5yz1, atelier-ntq9 and atelier-8j0j shaped these lines.

## Unknowns

- Which HTTP library fits the workspace's dependency budget; choose and say why in the Report.
