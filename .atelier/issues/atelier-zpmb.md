---
created_at: "2026-11-09T00:45:58.188376772+00:00"
id: "atelier-zpmb"
issue_type: "feature"
labels:
- "fleet-dashboard"
workflow_branch:
  branch_base: master
  work_branch: atelier-zpmb
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
status: "in_progress"
title: "Serve every mission's brief and debrief as JSON from its base commit: atelier serve, read-only, for every repository here"
updated_at: "2026-11-09T16:33:53.361851600+00:00"
---

## Goal

Serve every mission's brief and debrief as JSON, current from each repository's base branch, from a read-only `atelier serve` that finds every Atelier repository on this host. The app that shows them as pages is atelier-yn69. This tests the mission's riskiest assumption first: that these views can be built from a base commit alone, without a checkout, cheaply enough for the host T3 Code runs on.

## Done when

- [x] A new crate `crates/atelier-site` holds discovery, the API routes, the cache and load limits, and `atelier serve [--bind <addr>] [--root <dir>...]` in `atelier-cli` runs it, bound to loopback by default; `atelier-site` does not depend on `atelier-cli`. HTTP is `axum` on `tokio` and `tower`, per decision atelier-c5vk.
- [x] `atelier-app` builds a `RecordIndex` from a commit's tree in memory (no temporary directory), and the brief and debrief views take one captured commit and read everything from it: records, workflow policy, config, the strategy's text and criteria, and the history that criteria marks, receipt ages and `generated_from` are computed against, with no `HEAD` lookup and no working-tree read. A test runs against a repository whose common Git directory's `HEAD` is a work branch that diverges from the base, with a different strategy, different criteria and a receipt only on that branch, and asserts the views' strategy text, criteria, marks, receipt ages and `generated_from` all match the base commit.
- [x] Discovery scans the roots (default `~/projects`) for `.atelier/`, shows each repository once however many worktrees it has (by Git common directory), and reads each one's base branch without a checkout: it takes the repository's default branch (`refs/remotes/origin/HEAD`, else whichever one of local `main` and `master` exists), reads the workflow policy committed there, and uses the `base_branch` that policy names, which must name itself in its own committed policy. A repository with no default branch, both `main` and `master` and no `origin/HEAD`, or a policy whose base does not name itself shows that error on its own page; nothing falls back to `HEAD` or a working tree. Tests cover a repository and two worktrees, one detached and behind, which shows one repository with the base branch's records; a base named neither `main` nor `master` found through `origin/HEAD`; a main checkout on a work branch whose policy names a different base, which does not change the base; and each error case; a repository whose records fail to load returns its error from its own routes while the others answer.
- [x] `GET /api/repos` returns each repository with its counts and its missions, each with id, title, status and the review to show (`brief` while the mission is in its workflow's initial status, `debrief` afterwards, chosen by the server from the workflow policy); `/api/<repo>/mission/<id>` returns that summary for one mission; and `/api/<repo>/mission/<id>/brief`, `/api/<repo>/mission/<id>/debrief` and `/api/<repo>/mission/<id>/strategy` return the `atelier-app` brief and debrief view models serialized with `serde`, the strategy's Markdown as text, and the base commit read; unknown repositories and missions are 404s with a JSON error. Fixture tests cover a draft mission, a closed one, a mission with a strategy and one without, a workflow whose mission initial status is not `draft` with a later status in the same category, and assert the brief and debrief responses carry the same title, criteria and item counts as `atelier brief -q` and `atelier debrief -q` on the fixture.
- [x] TypeScript types for every API response are generated from the Rust types with `ts-rs` into `crates/atelier-site/web/src/api/`, and each route's fixture responses are written as JSON under `crates/atelier-site/web/src/fixtures/`; both are checked in, and a Rust test fails when either is stale. These two directories are all this item puts under `web/`; atelier-yn69 builds the app around them.
- [x] Read-only proof: with each fixture repository's working tree and `.git` directory made unwritable, every route returns 200 and the same body as with them writable, and refs, the index and `git status` are unchanged.
- [x] Load bound: only work that depends on nothing but the base commit is cached, keyed by repository and base commit, keeping only the latest commit per repository; at most two view builds run at once, and a request that waits more than 5 seconds gets a 503. On this host, 20 concurrent cold-cache requests for the debrief of this repository's largest mission keep the server's peak RSS under 300 MB, and T3 Code's web endpoint keeps answering in under 1 second; a warm request returns in under 100 ms. The Report records the numbers.
- [x] `docs/architecture/` gains a page for the site covering the loader, discovery, the API and the load limits.

## Scope

Read-only, JSON only. No app, pages or embedded assets (atelier-yn69), no board, item or inbox routes (atelier-ahpf, atelier-uma8), no attribution or T3 data, no removal of `--html` (atelier-59fz).

## Context

Decisions atelier-c5vk and atelier-8p7u and the Design in `docs/strategies/fleet-dashboard.md` govern; decision atelier-7bs1 allows the server even before ADR 0030 (atelier-tu8c) updates `PRODUCT_INTENT.md`'s Boundaries. Blocked by atelier-t7pb, which makes the debrief one history walk instead of one per item. The views read the checkout today in these places, each of which must take the captured commit instead: `RecordIndex::load` reads a directory and validates against the working tree's `workflow.yaml` (`crates/atelier-app/src/record_index.rs:1232`); `crates/atelier-app/src/mission_review.rs` reads the strategy from the working tree (lines 887 and 1443) and pins history with `read.pinned("HEAD")` (lines 360 and 1470); and `crates/atelier-app/src/criteria.rs` reads the strategy's criteria from the working tree (line 48, `mission_criteria`) and runs `git rev-parse HEAD` for receipt history (line 249, in `mission_proof_until`). Against a common Git directory, `HEAD` is whatever the main checkout has out, not the base. `git_freshness` already reads blobs with `git cat-file --batch`; `Snapshot::at` in `commands/git.rs` extracts a tracker with `git archive` into a temporary directory, which the site must not do (`/tmp` here is a small quota'd tmpfs).

## Unknowns

- Whether the views read any checkout state beyond the sites listed in Context; the divergent-`HEAD` test and the read-only proof find out.
- Whether the view models serialize as they are or need a serializable shape beside them; prefer deriving `Serialize` on the existing types, and record a decision note if a separate shape is needed.

## Notes

- 2026-11-09 [decision] The API serializes the existing view models: serde and ts-rs derives on the brief and debrief types in atelier-app and the record types in atelier-core, with each enum that carries data tagged as {kind, value}.
  Reason: The views serialize as they are, so the site cannot drift from what the CLI computes, and tagged enums give TypeScript discriminated unions.
  Rejected: A separate serializable shape beside each view: a second model to keep in step
- 2026-11-09 [decision] At most two loads or view builds run at once through a tokio semaphore around the builds, not a tower concurrency layer; tower serves in-process requests (atelier_site::Client) through ServiceExt::oneshot.
  Reason: A layer limits every request, so warm requests answered from the cache would queue behind cold builds; the Done when caps builds.
  Rejected: tower ConcurrencyLimitLayer and TimeoutLayer on the API routes
- 2026-11-09 [decision] Refs are read afresh for every request: each request scans the roots and runs one git for-each-ref per repository, and requests that arrive while a scan runs share the next one. Only the policy and config facts of a commit, the records loaded from it and the views built from it are cached.
  Reason: The strategy's Design says anything read from refs is recomputed per request or keyed by every ref it read; a warm debrief with the scan still answers in about 12 ms on a debug build.
  Rejected: Reusing a scan for two seconds: a time-keyed cache of ref state
- 2026-11-09 [decision] The base branch is read from its local branch (refs/heads/<base>); a base with no local branch is an error, never its remote-tracking ref. A repository's URL name is its committed project_slug, else its directory name, with -2 and on for a repeated name in common-directory order. Discovery searches at most four directories deep, never inside a checkout, hidden directories, node_modules or target. atelier serve binds 127.0.0.1:7341 by default.
  Reason: The CLI's commands and land read the local base, and agents push it after land; the other choices keep a scan of ~/projects cheap and URLs stable.
  Rejected: Falling back to refs/remotes/origin/<base>: it can lag or lead what land checked

## Report

All eight Done when lines hold. `atelier serve` (new crate `crates/atelier-site`, axum on tokio, tower for in-process requests) finds every Atelier repository under its roots and serves each mission's brief, debrief and strategy as JSON from one captured commit of its base branch. `atelier_app::commit_records::load` builds the `RecordIndex` from the commit's tree through one `git cat-file --batch`, and `ReadContext::at_commit` makes the views read the strategy, criteria, history and `generated_from` from that commit only. Tests, all passing: `serve::divergent_head::views_read_the_base_commit_when_head_is_a_diverging_work_branch` (served views equal the CLI's views in a checkout of the base while the common directory's HEAD is a diverging work branch; with the old `git rev-parse HEAD` put back it failed, counting a receipt's age as 2 instead of 1); the seven `serve::discovery` tests (two worktrees show once with the base's records, a base found through origin/HEAD, a checkout on a work branch does not move the base, and each error case); the six `serve::api` tests (checked-in fixture JSON is current; brief and debrief titles and counts match `atelier brief -q` and `atelier debrief -q` on four fixture missions, covering draft, closed, with and without a strategy, and a `proposed` initial status with `ready` in the same category; the review choice; JSON 404s; a broken repository answers 500 on its own routes while the others answer; one build per base commit); `serve::read_only::every_route_answers_the_same_from_unwritable_repositories`; and in `atelier-site`, `typescript_types_are_current` and `builds_are_capped_and_a_long_wait_is_a_503`. The CLI's output is unchanged: `brief`, `brief -q`, `debrief` and `debrief -q` on all 22 missions of this repository plus `status` and `status -q`, 90 outputs, are identical from master's binary (94990dd3) and this branch's (receipt atelier-x5zy). Load on this host, release build, 20 concurrent cold-cache debrief requests: for atelier-k8h8 (largest debrief) all 200, slowest 1.32 s, peak RSS 65.4 MB, T3 Code's web endpoint (127.0.0.1:3773) slowest 6 ms during the load (4 ms idle), warm requests 13 ms; for atelier-rc8j (most items, 49) slowest 1.03 s, peak RSS 55.5 MB, T3 Code slowest 2 ms, warm 12 ms (receipts atelier-03ev, atelier-7nsj). The full workspace suite passes with and without `test-hooks` (atelier-mo9r, atelier-ygyw), and `scripts/smoke-repos.sh` shows no regression across 5 repositories (atelier-grj3). The comparison and load scripts used are `~/.cache/atelier-tmp-zpmb/compare.sh` and `load.sh` on this host. Size: about 3,200 hand-written lines (1,850 code and docs, 1,320 tests), above the 1,500 to 2,500 expected but under twice it; another 2,100 lines are generated TypeScript types and fixture JSON, and 430 are `Cargo.lock`. The commit-tree loader (about 300 lines in `atelier-app`) could have been its own item, but the divergent-HEAD test needs the server to reach it.

### Deviations

- `atelier serve` has a `-q` token (the URL it serves) and refuses a `--root` that is not a directory, because the quiet-token test requires every command to have both; the token is in `docs/product/human-cli-output.md`.
- Root help now lists `serve`; `setup_guidance`'s command list was updated.

### Decisions

See the four decision notes above: serializing the existing view models with `{kind, value}` enums; a semaphore around builds rather than a tower concurrency layer; refs read afresh on every request with in-flight scans shared; and the local base branch, URL names, scan limits and default port.

### Open items

- A failed load or view build is cached for its base commit like a successful one, so a transient Git failure (a process that could not start) stays until the base moves or the server restarts.
- The Rust doc comments on the view types appear in the generated TypeScript, so editing one needs `ATELIER_UPDATE_FIXTURES=1` to refresh `web/src/api/`.
