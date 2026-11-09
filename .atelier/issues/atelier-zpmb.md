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
    id: "atelier-yfqu"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Serve every mission's brief and debrief from its base branch: atelier serve, read-only, for every repository here"
updated_at: "2026-11-09T06:42:06.971538365+00:00"
---

## Goal

Serve every mission's brief and debrief as pages, current from each repository's base branch, from a read-only `atelier serve` that finds every Atelier repository on this host. This is the site's primary use, and it tests the mission's riskiest assumption first: that these views can be built from a base commit alone, without a checkout, cheaply enough for the host T3 Code runs on.

## Done when

- [ ] A new crate `crates/atelier-site` holds the routes, page renderers, cache and load limits, and `atelier serve [--bind <addr>] [--root <dir>...]` in `atelier-cli` runs it, bound to loopback by default; `atelier-site` does not depend on `atelier-cli`. HTTP is `axum` on `tokio` and `tower`, per decision atelier-c5vk.
- [ ] `atelier-app` builds a `RecordIndex` from a commit's tree in memory (no temporary directory), and the brief and debrief views take one captured commit and read everything from it: records, workflow policy, config, the strategy's text and criteria, and the history that criteria marks, receipt ages and `generated_from` are computed against, with no `HEAD` lookup and no working-tree read. A test runs against a repository whose common Git directory's `HEAD` is a work branch that diverges from the base, with a different strategy, different criteria and a receipt only on that branch, and asserts the views' strategy text, criteria, marks, receipt ages and `generated_from` all match the base commit.
- [ ] Discovery scans the roots (default `~/projects`) for `.atelier/`, shows each repository once however many worktrees it has (by Git common directory), and reads each from its base branch's commit. A test with a repository and two worktrees, one detached and behind, shows one repository with the base branch's records; a repository whose records fail to load shows the error on its own page while the others render.
- [ ] `/` lists the repositories with a link to each mission. `/<repo>/mission/<id>` shows the brief while the mission is in its workflow's initial status and the debrief afterwards, linking both (`/brief`, `/debrief`) and the strategy rendered from Markdown (`/strategy`). Fixture tests cover a draft mission, a closed one, a mission with a strategy and one without, a title holding `<script>` rendered escaped on every route, and a strategy whose Markdown holds raw HTML and links and images with `javascript:`, `data:` and `vbscript:` destinations, which render as text while relative, fragment, `http`, `https` and `mailto` links are kept.
- [ ] The brief and debrief HTML renderers move from `atelier-cli` into `atelier-site` as functions that return HTML and write nothing; until atelier-59fz removes it, `brief --html` and `debrief --html` write those functions' output, and their existing tests pass.
- [ ] Read-only proof: with each fixture repository's working tree and `.git` directory made unwritable, every route returns 200 and the same HTML as with them writable, and refs, the index and `git status` are unchanged.
- [ ] Load bound: only work that depends on nothing but the base commit is cached, keyed by repository and base commit, keeping only the latest commit per repository; at most two renders run at once, and a request that waits more than 5 seconds gets a 503 page. On this host, 20 concurrent cold-cache requests for the debrief of this repository's largest mission keep the server's peak RSS under 300 MB, and T3 Code's web endpoint keeps answering in under 1 second; a warm request returns in under 100 ms. The Report records the numbers.
- [ ] The repository ships a systemd user unit for `atelier serve` with `Restart=on-failure`, `MemoryMax=` and `CPUQuota=`; `docs/` says how to install it, restart it after `cargo install`, and publish it with `tailscale serve` without changing T3 Code's published port; `docs/architecture/` gains a page for the site.
- [ ] Every page is server-rendered HTML with no front-end framework and reads at a 390 px wide viewport. Run against this host's repositories, every mission's brief and debrief page returns 200; the Report records the command and counts.

## Scope

Read-only. No board, item or inbox pages (atelier-ahpf, atelier-uma8), no attribution or T3 data, no removal of `--html` (atelier-59fz).

## Context

Decision atelier-c5vk and the Design in `docs/strategies/fleet-dashboard.md` govern; decision atelier-7bs1 allows the server even before ADR 0030 (atelier-tu8c) updates `PRODUCT_INTENT.md`'s Boundaries. The renderers are in `crates/atelier-cli/src/review_page.rs` and `commands/mission_review.rs` (`render_html`, `render_brief_html`, `write_review_page`); the views read the checkout today in these places, each of which must take the captured commit instead: `RecordIndex::load` reads a directory and validates against the working tree's `workflow.yaml` (`crates/atelier-app/src/record_index.rs:1232`); `crates/atelier-app/src/mission_review.rs` reads the strategy from the working tree (lines 887 and 1443) and pins history with `read.pinned("HEAD")` (lines 360 and 1470); and `crates/atelier-app/src/criteria.rs` reads the strategy's criteria from the working tree (line 48, `mission_criteria`) and runs `git rev-parse HEAD` for receipt history (line 249, in `mission_proof_until`). Against a common Git directory, `HEAD` is whatever the main checkout has out, not the base. `git_freshness` already reads blobs with `git cat-file --batch`; `Snapshot::at` in `commands/git.rs` extracts a tracker with `git archive` into a temporary directory, which the site must not do (`/tmp` here is a small quota'd tmpfs). The planner's spike: `atelier debrief atelier-k8h8 -q` took 7.7 s and 64 MB from a checkout.

## Unknowns

- Whether the views read any checkout state beyond the sites listed in Context; the divergent-`HEAD` test and the read-only proof find out.
