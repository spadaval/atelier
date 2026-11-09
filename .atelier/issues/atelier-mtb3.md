---
created_at: "2026-11-09T01:04:22.768330108+00:00"
id: "atelier-mtb3"
issue_type: "validation"
labels:
- "fleet-dashboard"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-6rts"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-ahpf"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-uma8"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-yfqu"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Validate site-matches-cli: fetch every brief, debrief, board and inbox link over Tailscale"
updated_at: "2026-11-09T14:43:24.053007708+00:00"
---

## Goal

Independently show the site mission's criterion site-matches-cli holds on this host, with the site installed and published.

## Done when

- [x] An agent that did not build the site loads in a headless browser, over the Tailscale hostname, the brief and debrief page (`/<repo>/mission/<id>/brief` and `/debrief`) of every mission in every Atelier repository on this host, and each renders without an error and shows the mission's title as `atelier issue show <mission>` gives it, and the criteria and item counts that `atelier brief -q` and `atelier debrief -q` print, all run in a detached worktree of that repository's base branch.
- [x] For each repository, the board lists the same in-progress, ready and needs-human items as `atelier status` run in a detached worktree of its base branch (created under `$HOME/.cache/` and removed afterwards).
- [x] Every inbox entry's links open a page that renders without an error.
- [x] A receipt for site-matches-cli records the commit, the hostname fetched, the counts and any failures.

## Context

Runs after loose end atelier-6rts installs and publishes the site, and after atelier-yn69, atelier-ahpf and atelier-uma8 merge. Playwright is in `crates/atelier-site/web/`. A bad result files a linked bug.

## Report

All four lines hold at master `2549e116`. Receipt `atelier-ril9` records
`site-matches-cli=pass`. No bug filed: nothing failed.

Setup: the site is `atelier-serve`, fetched only at
`http://buildbot.tail47515a.ts.net:7341/`, in Playwright's headless Chromium
at a 390 px viewport, one page at a time. `/api/repos` lists 5 repositories
(astromech, atelier, cosplay-studio, lor, worldcrafter), each with no error.
Each one's base commit equals its local base tip. I checked that discovery is
complete: those 5 git common dirs are every one under `~/projects` with
`.atelier/`, and each base holds as many `issue_type: "mission"` records as
the site lists (3, 22, 10, 3, 4). For each, I ran
`git worktree add --detach $HOME/.cache/atelier-mtb3/wt/<repo> <base>` and
ran the CLI there. All 5 worktrees were removed afterwards.

- Missions: 42 missions, 84 brief and debrief pages. Every page rendered with
  no console error, no page error, no `role=alert` and no error page. Every
  page's title equals the end of line 1 of `atelier issue show <id>`. Briefs
  match all six `atelier brief -q` counts, read from the page's sections:
  criteria, items, blocked, unknowns, human_decisions and approvals. Debriefs
  match `atelier debrief -q`'s criteria and its proven, failing and unproven
  counts, read from the Results marks. The debrief page prints no item
  number, so I checked the items another way. For every mission, the item IDs
  on the page are the same set as those in `atelier debrief <id>`'s full
  output, with none missing and none extra.
- Boards: on all 5 boards, In Progress, Ready Work and Needs a Human match
  `atelier status`: the owned-branch rows and Current Work, the Ready Work
  rows shown (as the board's head) and the ready and human counts. For the
  rows that status truncates, I compared against `atelier work ready -q` (in
  order) and `atelier work human -q`. In progress: astromech has
  astromech-v6r3 and astromech-x53n, lor has lor-pmzz, and the rest have
  none. Ready: 31, 14, 12, 3, 0. Needs a human: 35, 3, 2, 8, 2.
- Inbox: `/inbox` has 93 links, 66 distinct: 50 item pages (the 50
  needs-human items), 11 mission pages and 5 boards. All rendered without an
  error.

Notes on the check itself, not the site. My first debrief pass flagged 8
pages, and each was a fault in my scraper. In 4 of them, the section "What We
Set Out To Do" also lists the mission's "Decided:" lines, so I counted
criteria from Results instead. The other 4 were item-count gaps from counting
only `span.item` IDs, which misses IDs shown in "Filed:" lines and review asks.
After the fixes, 0 mismatches remained.

Scripts and output are in `$HOME/.cache/atelier-mtb3/`, outside the
repository:

- `check.cjs` holds the page, board and inbox checks; run it as `node
  check.cjs boards|inbox|missions`.
- `ids.cjs` compares debrief item IDs.
- `diag.cjs` holds the diagnostics.
- `cli/`, `debrief-cli/` and `status/` hold the CLI output.
- `result.*.json` holds the results.

