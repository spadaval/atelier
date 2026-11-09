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
    id: "atelier-t7pb"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-yfqu"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Serve every mission's brief and debrief as JSON from its base commit: atelier serve, read-only, for every repository here"
updated_at: "2026-11-09T14:43:23.392860739+00:00"
---

## Goal

Serve every mission's brief and debrief as JSON, current from each repository's base branch, from a read-only `atelier serve` that finds every Atelier repository on this host. The app that shows them as pages is atelier-yn69. This tests the mission's riskiest assumption first: that these views can be built from a base commit alone, without a checkout, cheaply enough for the host T3 Code runs on.

## Done when

- [ ] A new crate `crates/atelier-site` holds discovery, the API routes, the cache and load limits, and `atelier serve [--bind <addr>] [--root <dir>...]` in `atelier-cli` runs it, bound to loopback by default; `atelier-site` does not depend on `atelier-cli`. HTTP is `axum` on `tokio` and `tower`, per decision atelier-c5vk.
- [ ] `atelier-app` builds a `RecordIndex` from a commit's tree in memory (no temporary directory), and the brief and debrief views take one captured commit and read everything from it: records, workflow policy, config, the strategy's text and criteria, and the history that criteria marks, receipt ages and `generated_from` are computed against, with no `HEAD` lookup and no working-tree read. A test runs against a repository whose common Git directory's `HEAD` is a work branch that diverges from the base, with a different strategy, different criteria and a receipt only on that branch, and asserts the views' strategy text, criteria, marks, receipt ages and `generated_from` all match the base commit.
- [ ] Discovery scans the roots (default `~/projects`) for `.atelier/`, shows each repository once however many worktrees it has (by Git common directory), and reads each one's base branch without a checkout: it takes the repository's default branch (`refs/remotes/origin/HEAD`, else whichever one of local `main` and `master` exists), reads the workflow policy committed there, and uses the `base_branch` that policy names, which must name itself in its own committed policy. A repository with no default branch, both `main` and `master` and no `origin/HEAD`, or a policy whose base does not name itself shows that error on its own page; nothing falls back to `HEAD` or a working tree. Tests cover a repository and two worktrees, one detached and behind, which shows one repository with the base branch's records; a base named neither `main` nor `master` found through `origin/HEAD`; a main checkout on a work branch whose policy names a different base, which does not change the base; and each error case; a repository whose records fail to load returns its error from its own routes while the others answer.
- [ ] `GET /api/repos` returns each repository with its counts and its missions, each with id, title, status and the review to show (`brief` while the mission is in its workflow's initial status, `debrief` afterwards, chosen by the server from the workflow policy); `/api/<repo>/mission/<id>` returns that summary for one mission; and `/api/<repo>/mission/<id>/brief`, `/api/<repo>/mission/<id>/debrief` and `/api/<repo>/mission/<id>/strategy` return the `atelier-app` brief and debrief view models serialized with `serde`, the strategy's Markdown as text, and the base commit read; unknown repositories and missions are 404s with a JSON error. Fixture tests cover a draft mission, a closed one, a mission with a strategy and one without, a workflow whose mission initial status is not `draft` with a later status in the same category, and assert the brief and debrief responses carry the same title, criteria and item counts as `atelier brief -q` and `atelier debrief -q` on the fixture.
- [ ] TypeScript types for every API response are generated from the Rust types with `ts-rs` into `crates/atelier-site/web/src/api/`, and each route's fixture responses are written as JSON under `crates/atelier-site/web/src/fixtures/`; both are checked in, and a Rust test fails when either is stale. These two directories are all this item puts under `web/`; atelier-yn69 builds the app around them.
- [ ] Read-only proof: with each fixture repository's working tree and `.git` directory made unwritable, every route returns 200 and the same body as with them writable, and refs, the index and `git status` are unchanged.
- [ ] Load bound: only work that depends on nothing but the base commit is cached, keyed by repository and base commit, keeping only the latest commit per repository; at most two view builds run at once, and a request that waits more than 5 seconds gets a 503. On this host, 20 concurrent cold-cache requests for the debrief of this repository's largest mission keep the server's peak RSS under 300 MB, and T3 Code's web endpoint keeps answering in under 1 second; a warm request returns in under 100 ms. The Report records the numbers.
- [ ] `docs/architecture/` gains a page for the site covering the loader, discovery, the API and the load limits.

## Scope

Read-only, JSON only. No app, pages or embedded assets (atelier-yn69), no board, item or inbox routes (atelier-ahpf, atelier-uma8), no attribution or T3 data, no removal of `--html` (atelier-59fz).

## Context

Decisions atelier-c5vk and atelier-8p7u and the Design in `docs/strategies/fleet-dashboard.md` govern; decision atelier-7bs1 allows the server even before ADR 0030 (atelier-tu8c) updates `PRODUCT_INTENT.md`'s Boundaries. Blocked by atelier-t7pb, which makes the debrief one history walk instead of one per item. The views read the checkout today in these places, each of which must take the captured commit instead: `RecordIndex::load` reads a directory and validates against the working tree's `workflow.yaml` (`crates/atelier-app/src/record_index.rs:1232`); `crates/atelier-app/src/mission_review.rs` reads the strategy from the working tree (lines 887 and 1443) and pins history with `read.pinned("HEAD")` (lines 360 and 1470); and `crates/atelier-app/src/criteria.rs` reads the strategy's criteria from the working tree (line 48, `mission_criteria`) and runs `git rev-parse HEAD` for receipt history (line 249, in `mission_proof_until`). Against a common Git directory, `HEAD` is whatever the main checkout has out, not the base. `git_freshness` already reads blobs with `git cat-file --batch`; `Snapshot::at` in `commands/git.rs` extracts a tracker with `git archive` into a temporary directory, which the site must not do (`/tmp` here is a small quota'd tmpfs).

## Unknowns

- Whether the views read any checkout state beyond the sites listed in Context; the divergent-`HEAD` test and the read-only proof find out.
- Whether the view models serialize as they are or need a serializable shape beside them; prefer deriving `Serialize` on the existing types, and record a decision note if a separate shape is needed.
