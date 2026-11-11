---
created_at: "2026-11-11T02:17:25.926248278+00:00"
id: "atelier-uwxy"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: atelier-uwxy
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-1ie1"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-l5ai"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-z4cr"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Serve the Desk's reading routes from commits: the atelier-desk crate, its generated API types and the app's files"
updated_at: "2026-11-11T05:41:13.371923139+00:00"
---

## Goal

Stand up the Desk's server: a new crate `next/crates/atelier-desk` whose binary serves the reading routes of `docs/architecture/desk.md` as JSON from commits, with the response types generated for the app, and serves the built app's files from a directory. After this item the app (atelier-94pv) has a real API to be built against, and a mission's data can be compared with `atelier show` on the same commit.

## Done when

- [ ] `next/crates/atelier-desk` is a member of the `next/` workspace, depending on `atelier-app` and `atelier-records`. A test in the workspace fails if any other crate depends on it.
- [ ] The binary serves `GET /api/repos`, `/api/repos/{repo}/missions` (with `?done=true`), `/api/repos/{repo}/missions/{id}`, `/api/repos/{repo}/doc?path=`, `/api/repos/{repo}/items/{id}` and `/api/inbox`, with the fields the document gives for each, except `threads`, `holds`, `pending_answers`, `can_decide` and `decide_at`, which atelier-e05n and atelier-h3hl add. It does not serve `/history`, `/api/threads` or `POST /api/act`.
- [ ] Every record it serves is read from a commit through the read-only opening atelier-l5ai adds. A test serves a scratch repository whose checkout has uncommitted record edits and gets the committed values.
- [ ] A test builds a scratch repository on the new records with at least two missions and items in four statuses, one of them started on its own branch, and checks that each mission's title, status and per-status item counts from the API equal what `atelier_app`'s view for `show <mission>` gives at the same commit, and that the inbox's `needs_human` items equal those of `list` with `needs_human` set.
- [ ] Repositories come from a configuration file the binary is given: one or more roots, each direct child directory holding a repository on the new records being served under its directory name, and the name the human acts under. A repository that does not open, or holds a record that does not read, appears in `/api/repos` with `error` and does not fail the others.
- [ ] Responses are cached by the set of commits they were read from (the base's tip and the tip of each branch named for an item), and a test shows a new commit on the base changes the response without a restart.
- [ ] Adding the crate changes nothing the tracker ships, checked in the `next/` workspace, which the switch makes the repository's only one. (The old root workspace is not the measure: its CLI already has `axum` and `tokio` through `atelier-site`.) Three checks: a test in `next/` reads `cargo metadata` and fails if `axum`, `tokio` or `ts-rs` is in the dependency tree of `atelier-cli`, `atelier-app`, `atelier-records` or `atelier-core`; `cargo install --path next/crates/atelier-cli --locked --root <a scratch directory>` installs one binary, `atelier`, with its output in the Report; and `atelier-desk`'s manifest opts the crate out of release packaging in the way the root `Cargo.toml`'s or `dist-workspace.toml`'s `dist` configuration honours, so the crate stays out when the switch gives `next/` that configuration, with the Report naming the setting. Checking the switched tree's release itself is atelier-u1ko's, not this item's.
- [ ] The response types are exported for the app with `ts-rs` into `desk/src/api/` and committed. A test fails when the committed files differ from what the types generate.
- [ ] The binary serves the files of a directory given at start (the built app) with a fallback to its `index.html` for paths outside `/api`, so `cargo build` of the workspace needs no Node. With no directory given it serves the API alone.
- [ ] It binds to the addresses it is given, loopback by default, and refuses with 403 a request whose `Host` is not one of the names it is configured to answer to. A test covers an allowed and a refused host.
- [ ] `scripts/ci-jobs.sh` does not run the old workspace's `rust` or `web` jobs for a change that touches only `desk/`, and its test covers that. The `next` job builds and tests `atelier-desk` with the workspace.
- [ ] Run against the converted clone at `~/.cache/atelier-desk/clones/atelier`, every open mission's route answers 200, and the Report gives, for each, its title, status and counts beside the new `atelier show <mission>` there.
- [ ] `docs/architecture/desk.md` says how the server is configured and started, and is corrected wherever the code as built differs from it.

## Scope

The new crate, `next/Cargo.toml` and `next/Cargo.lock`, `desk/src/api/` (generated types only), `scripts/ci-jobs.sh` with its test, and `docs/architecture/desk.md`. New dependencies of the `next/` workspace: `axum` and `tokio` for the server and `ts-rs` for the types, as the strategy names; `tower-http` or the like for serving files is allowed, said in the Report. No SQLite crate.

Not in this item: any call to T3 Code (atelier-e05n), any act or write to a repository (atelier-h3hl), the app's pages (atelier-94pv), the history route, installing a service (atelier-va99). A view the server needs and `atelier-app` lacks is added to `atelier-app`, not written in this crate; if one beyond atelier-l5ai's six is needed, add it there, keep the CLI's output unchanged and say so in the Report. The server reads no working tree, runs no `atelier` binary and reads no command's output.

Expected size: 800 to 1,100 lines of Rust with tests, plus generated types, in one crate.

## Context

- The contract: `docs/architecture/desk.md`. The strategy's Design, Access and Authority: `docs/strategies/desk.md`. Decisions: ADR 0032.
- A `Repo` in `atelier-app` is not `Send`: make the `Skip` and open the repository inside the blocking task that uses them.
- An item or mission ID from a URL is checked with `atelier_records::validate_id` before use. A caller's or thread's identity is an opaque string that can hold `/`, `:` and `%`: it is never split and never part of a path.
- The old site's server (`crates/atelier-site/src`, on master until the switch) shows how `axum`, `ts-rs` and the generated-types test were set up before. It is a reference for those, not code to carry over.
- The switch (atelier-u1ko) moves `next/crates/` to `crates/`. The redesign's planner set the rule on 2026-11-11: once atelier-u1ko is started, nothing merges into `next/`, the root `Cargo.toml` or the CI and release workflows until atelier-wd1i has run; work may go on in the branch. The orchestrator checks both before the merge. If the cutover runs while this branch is open, stop and report to the orchestrator: the old `atelier merge --sync` no longer exists then, and the branch is brought up to date as the cutover's runbook says for an open branch.
- Build with `cargo build -j 4` and a target directory inside the item's own worktree. Converted clones live under `~/.cache/atelier-desk/`; never run against a live repository, and put nothing in `/tmp`.
- Sizes: complexity medium, value high, risk low. Reads that are wrong show as a difference from `show`, which the fourth line tests.
