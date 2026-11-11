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

- [x] `next/crates/atelier-desk` is a member of the `next/` workspace, depending on `atelier-app` and `atelier-records`. A test in the workspace fails if any other crate depends on it.
- [x] The binary serves `GET /api/repos`, `/api/repos/{repo}/missions` (with `?done=true`), `/api/repos/{repo}/missions/{id}`, `/api/repos/{repo}/doc?path=`, `/api/repos/{repo}/items/{id}` and `/api/inbox`, with the fields the document gives for each, except `threads`, `holds`, `pending_answers`, `can_decide` and `decide_at`, which atelier-e05n and atelier-h3hl add. It does not serve `/history`, `/api/threads` or `POST /api/act`.
- [x] Every record it serves is read from a commit through the read-only opening atelier-l5ai adds. A test serves a scratch repository whose checkout has uncommitted record edits and gets the committed values.
- [x] A test builds a scratch repository on the new records with at least two missions and items in four statuses, one of them started on its own branch, and checks that each mission's title, status and per-status item counts from the API equal what `atelier_app`'s view for `show <mission>` gives at the same commit, and that the inbox's `needs_human` items equal those of `list` with `needs_human` set.
- [x] Repositories come from a configuration file the binary is given: one or more roots, each direct child directory holding a repository on the new records being served under its directory name, and the name the human acts under. A repository that does not open, or holds a record that does not read, appears in `/api/repos` with `error` and does not fail the others.
- [x] Responses are cached by the set of commits they were read from (the base's tip and the tip of each branch named for an item), and a test shows a new commit on the base changes the response without a restart.
- [x] Adding the crate changes nothing the tracker ships, checked in the `next/` workspace, which the switch makes the repository's only one. (The old root workspace is not the measure: its CLI already has `axum` and `tokio` through `atelier-site`.) Three checks: a test in `next/` reads `cargo metadata` and fails if `axum`, `tokio` or `ts-rs` is in the dependency tree of `atelier-cli`, `atelier-app`, `atelier-records` or `atelier-core`; `cargo install --path next/crates/atelier-cli --locked --root <a scratch directory>` installs one binary, `atelier`, with its output in the Report; and `atelier-desk`'s manifest opts the crate out of release packaging in the way the root `Cargo.toml`'s or `dist-workspace.toml`'s `dist` configuration honours, so the crate stays out when the switch gives `next/` that configuration, with the Report naming the setting. Checking the switched tree's release itself is atelier-u1ko's, not this item's.
- [x] The response types are exported for the app with `ts-rs` into `desk/src/api/` and committed. A test fails when the committed files differ from what the types generate.
- [x] The binary serves the files of a directory given at start (the built app) with a fallback to its `index.html` for paths outside `/api`, so `cargo build` of the workspace needs no Node. With no directory given it serves the API alone.
- [x] It binds to the addresses it is given, loopback by default, and refuses with 403 a request whose `Host` is not one of the names it is configured to answer to. A test covers an allowed and a refused host.
- [x] `scripts/ci-jobs.sh` does not run the old workspace's `rust` or `web` jobs for a change that touches only `desk/`, and its test covers that. The `next` job builds and tests `atelier-desk` with the workspace.
- [x] Run against the converted clone at `~/.cache/atelier-desk/clones/atelier`, every open mission's route answers 200, and the Report gives, for each, its title, status and counts beside the new `atelier show <mission>` there.
- [x] `docs/architecture/desk.md` says how the server is configured and started, and is corrected wherever the code as built differs from it.

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

## Report

Every Done when line holds at `e676baad`, where `cargo nextest run -j 4` in `next/` runs 266 tests, all passing, with `cargo fmt -- --check`, `cargo clippy --all-targets` (no warnings) and `git diff --check` clean. Line by line:

1. Workspace member depending on `atelier-app` and `atelier-records` (and `atelier-core`, for the enums it names): `dependencies::no_crate_depends_on_the_desk` passes.
2. The six routes, with the document's fields less the five left to atelier-e05n and atelier-h3hl: `desk::each_mission_agrees_with_show_and_the_inbox_with_list` and `desk::the_item_and_document_routes`, which also get 404 for `/history`, `/api/threads` and `/api/act`.
3. Read from commits: `desk::records_are_read_from_commits_not_the_checkouts` serves a repository with an uncommitted `config.toml` naming a missing base, an uncommitted mission title in the base's checkout and an uncommitted record in a checkout of an item's branch, and gets the committed values. With `read::open` changed to `Repo::open` it fails (`500`, "the base branch elsewhere ... does not exist").
4. Agreement with `show` and `list`: `desk::each_mission_agrees_with_show_and_the_inbox_with_list`, over two missions and items draft, ready, building (on its own branch) and done, compares title, status and per-status counts with `atelier_app::mission_view` at the same commit, and the inbox's `needs_human` IDs with `list` and `Filter { needs_human: true }`.
5. Configuration: `desk::a_repository_that_does_not_read_is_listed_with_its_error` (one root with a good repository, one whose item does not read, and a Git repository with no Atelier config, which is skipped; a second root repeating a name) and `config::tests::a_configuration_listens_on_loopback_unless_told_otherwise`.
6. Cache: `desk::a_new_commit_on_the_base_changes_the_response_without_a_restart` shows an unchanged repository answered without building, and a new commit on the base, then on an item's branch, changing the response. With the key frozen it fails ("Alpha" where "Renamed" is wanted).
7. Nothing shipped changes: `dependencies::the_tracker_is_built_without_the_server_crates` (finds axum, tokio, ts-rs and tower-http under `atelier-desk`, and none under the four tracker crates). `cargo install --path next/crates/atelier-cli --locked --root ~/.cache/atelier-desk/install-check` printed `Installed package `atelier-cli v0.0.0 (...next/crates/atelier-cli)` (executable `atelier`)`, and `bin/` holds only `atelier`. The setting is `[package.metadata.dist] dist = false` in `next/crates/atelier-desk/Cargo.toml`, which `dist` honours per package whatever the workspace's `dist` table says.
8. Generated types: `types::the_committed_typescript_types_are_current`, 34 files in `desk/src/api/`; `ATELIER_UPDATE_TYPES=1 cargo nextest run -p atelier-desk --test types` writes them.
9. The app's files: `desk::the_app_is_served_with_its_index_for_its_own_paths` (a file, `/`, and an app path falling back to `index.html`; `/api/nothing` stays a JSON 404; with no directory `/` is 404 and the API answers). Served with `tower-http`'s `ServeDir`, a dependency the Scope allows.
10. Host and addresses: `desk::a_request_for_another_host_is_refused` (allowed `localhost:3780`, refused `attacker.example`, also for `/`); the loopback default in the configuration test; the run below bound to the `listen` address given.
11. CI: `scripts/ci-jobs.test.sh` passes with the new cases `desk-app` (no job) and `desk-api-types` (`next` only, since the types test reads `desk/src/api/`). The `next` job runs `cargo build --all-targets` and `cargo nextest run` over the whole workspace, which now includes `atelier-desk`; `.github/workflows/` is unchanged.
12. The converted clone, served on `127.0.0.1:3781` from `~/.cache/atelier-desk/clones` and compared with the new binary's `atelier show <mission>` there (base tip `2116defd`):

| Mission | Route | API: title / status / counts | `atelier show` | Same |
| --- | --- | --- | --- | --- |
| atelier-295u | 200 | Give the gardener `atelier log` and a readiness checklist, so a run starts from facts / draft / draft 4 | the same | yes |
| atelier-4pzj | 200 | Let agents and the human see where things stand, which work they can start, and what to do next / draft / draft 33 | the same | yes |
| atelier-bnek | 200 | Record which agent did what, and compare models by role / draft / draft 2, ready 4 | the same | yes |
| atelier-g6rp | 200 | Ship Agent Factory from this repository, so a command and its guidance land in one change / ready / none | the same | yes |
| atelier-icv4 | 200 | Let agents run Atelier without working out its commands by trial and error / draft / ready 5 | the same | yes |
| atelier-qr5s | 200 | Move all five repositories and Agent Factory onto the new loop, one cutover per host / ready / draft 2, ready 5 | the same | yes |
| atelier-uf97 | 200 | Let agents write, check and repair tracker records: one at a time, by section or as a bundle / draft / draft 27 | the same | yes |
| atelier-z4cr | 200 | Build the Desk: the human's place to plan missions and read results, on the redesigned records / ready / draft 5, ready 2 | the same | yes |

The inbox's `needs_human` IDs there equal `atelier list --needs-human`'s (atelier-3km4, atelier-7w9s, atelier-8i86, atelier-j86z, atelier-pywh). An item, a document and `?done=true` answered 200, a foreign `Host` 403, and a cached mission page took 15 ms. The clone's `git status` is clean afterwards.

13. `docs/architecture/desk.md` has a Running the server section (command, configuration format, regenerating the types) and is corrected where the build differs (Deviations).

### Deviations

- The API differs from the document as it was, and the document now says so: a mission page nests its row as `mission` and an item page its row as `row`, rather than repeating those fields at the top; `counts` is `[{status, count}]` with empty statuses left out; the rail's `waiting` is the IDs of open items only; `cannot_start` and `needs` use `say::condition_text`, without the checkbox; `changed_since` is `null` when the clone lacks the reviewed commit and on an item's review; `unsettled` is finding names; `/api/repos` has no `commit` but each repository's `base_tip`; inbox repositories carry `error`; status codes are 400, 403, 404 and 500 with `error`.
- Size: the crate is 1,370 lines under `src/` (360 of them type declarations with their doc comments, and 55 of unit tests) and 570 of integration tests, past the expected 800 to 1,100. No tracker rule is computed in it: statuses, holders, conditions, counts' items and the needs-human list come from `mission_view`, `item_view`, `entries`, `items_of` and `list`.

### Decisions

- One view added to `atelier-app`: `MissionView::located`, the records of the mission's items, so item rows get each item's branch, question, Report and drop reason without a second `Repo::everything`. `show`'s output is unchanged (the CLI's tests pass).
- The cache key also holds the checkout opened and its `HEAD`, since the config is read there.
- A root's child is served when it is a Git checkout whose `HEAD` holds `.atelier/config.toml`, looked up with `git cat-file`; other directories are skipped, not listed as errors, so a root such as `~/projects` does not list every non-Atelier repository.
- Default port 3780, next to T3 Code's 3773.

### Open items

- A mission document's title is its first `# ` heading, as the document says; the converted `design.md` files begin under `# Sizing`, so the Documents tab shows "Sizing" for them. The app item (atelier-94pv) may prefer the file name.
- `scripts/smoke-repos.sh` was not run: it builds the old workspace, which this change does not touch.
