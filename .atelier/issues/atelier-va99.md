---
created_at: "2026-11-11T02:17:27.158256540+00:00"
id: "atelier-va99"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: atelier-va99
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-e05n"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-h3hl"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-l2aj"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-z4cr"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Install the Desk as a service on this host over converted clones of the five repositories, with one command for each success criterion"
updated_at: "2026-11-11T08:43:54.496229724+00:00"
---

## Goal

Run the Desk as a service on this host, reached over the tailnet, serving converted scratch clones of the five repositories, with everything needed to install it, update it and check it kept in the repository. After this item Sumeet can open the Desk on his phone, and the mission's two success criteria can each be checked by one command. It does not wait for the cutover: pointing the service at the live repositories is the loose end atelier-qxmr.

## Done when

- [x] A systemd user unit for `atelier-desk` is in the repository, with limits like those of `atelier-serve.service` on this host (one core, 1 GiB, a low priority), restarting on failure and started at boot. `docs/architecture/desk.md` gives the steps to install and to update the service: building the binary, building the app, where the binary, the app's files and the configuration go, and how the configuration names the `t3` command so that a T3 Code update does not leave it pointing at a removed version.
- [x] The server stops cleanly on SIGTERM as on SIGINT, revoking its T3 Code sessions, shown by a test (deferred finding atelier-mg2j, which this item settles).
- [x] One script in the repository makes or refreshes the converted scratch clones under a directory it is given: for each of atelier, astromech, demiurge, life-on-rails and cosplay-studio it clones the local repository's base branch, disables pushes, converts with `scripts/cutover/convert_records.py`, verifies the conversion, commits it, and runs the new `atelier check`. It never writes to a live repository, and the Report gives its output on this host.
- [x] **desk-matches-cli as a command.** One command in the Desk's suite takes the Desk's address and the directory of clones. For every open mission in every clone it loads the mission's page in a headless browser, fails on a page error or a console error, and compares the title, status and per-status item counts on the page with the new `atelier show <mission>` in that clone; it compares the inbox's needs-human items with `atelier list --needs-human` in each clone. It prints one line per mission and per inbox and exits non-zero on any difference.
- [x] **act-is-a-record as a command.** One command in the Desk's suite takes a converted clone, copies it to a scratch directory, adds a draft mission that may be made ready there with the new `atelier` commands and commits it, and clones that commit twice. It starts `atelier-desk` with the built app on the first, says go from the mission's page in a headless browser, and runs the new `atelier ready <mission> --by <name>` in the second. It checks that each has exactly one new commit, and compares the two commits' messages and changed paths, and the two mission records with the recorded time set aside; it then checks the page shows the mission ready. It exits non-zero on any difference, and it is shown to fail when the two are made to differ.
- [x] The two commands above are not part of the `desk` CI job, which builds no Rust; `desk/README.md` says how to run each.
- [x] The server says which commit it was built from, in a response the app shows in its footer, so anyone can tell what the service runs.
- [x] Before the merge, the service is installed and running on this host from the branch's last code commit, by the document's steps: `systemctl --user status atelier-desk` shows it active and enabled, it answers on loopback and on the tailnet address at the host's tailnet name, it refuses a request with another `Host`, and it comes back after its process is killed with SIGKILL (`Restart=on-failure` does not restart after a clean stop on SIGTERM). It serves the five converted clones made by the script. The Report gives the address. After the merge the orchestrator runs the document's update steps once from `master`, and the validation item atelier-oc9k checks the commit the service reports.
- [x] Both commands pass against the installed service or, for act-is-a-record, on this host; the Report gives their output. The independent run of both is the validation item atelier-oc9k, not this item.
- [x] The existing `atelier-serve.service` and its `tailscale serve` publication are left as they are.

## Scope

A unit file and scripts under `desk/` or `scripts/`, the signal handling in `next/crates/atelier-desk`, `desk/` tests and README, `docs/architecture/desk.md`, and the installation on this host (files under `~/.config`, `~/.local` and `~/.cache/atelier-desk`, and one systemd user unit). The service binds only to loopback and the tailnet address; it is not published with `tailscale funnel` or beyond the tailnet. No release artifact, installer or sign-in. No change to T3 Code's service or to `atelier-serve.service`. The release build of `atelier-desk` runs with `-j 4`; if it would hold the machine's cores for more than about ten minutes, install the debug build and say so.

Expected size: about 60 lines of Rust with a test, about 400 lines of scripts and browser tests, and the document.

## Context

- The strategy: `docs/strategies/desk.md`, Success Criteria, Access and Derisking. The service's configuration and the T3 Code keys: `docs/architecture/desk.md`.
- This host's tailnet name is `buildbot.tail47515a.ts.net`. `atelier-serve.service` (`~/.config/systemd/user/`) is the old site's unit and a model for the new one; port 7341 on loopback is the old site's, and port 80 on the tailnet address publishes T3 Code; use neither.
- The rehearsed conversion of the five repositories is `scripts/cutover/rehearse.sh`, which names each repository's base branch. The converted clone of atelier from atelier-1ie1 is at `~/.cache/atelier-desk/clones/atelier`; this item's script replaces it.
- The strategy states act-is-a-record for a converted scratch clone. A converted clone may hold no draft mission that can be made ready, so the command adds one to its own copy of a converted clone and never touches the served clones.
- The Rust change is small and should merge first if the switch is near: once atelier-u1ko is started, nothing merges into `next/` or the CI workflows until atelier-wd1i has run. If the cutover runs while the branch is open, stop and report.
- Sizes: complexity medium, value high, risk low. The risk is to the host: a service that loops or eats memory beside T3 Code, which the unit's limits bound.

## Notes

- 2026-11-11 [note] Handoff: approved by codex-sol-va99-r2 (atelier-2mz6) at 634498ff after two rounds; atelier-vi6k fixed. The service runs 6f8eeabe2 at http://buildbot.tail47515a.ts.net:3780/. Outcome, proof and decisions are in the Report.

## Report

Every Done when line holds. The Desk runs as the systemd user service `atelier-desk` on this host at <http://buildbot.tail47515a.ts.net:3780/> (also `http://100.124.153.80:3780` and `http://127.0.0.1:3780`), built from 6f8eeabe2 by `desk/service/install.sh`, and serves the five converted clones `desk/service/make-clones.sh` made in `~/.cache/atelier-desk/clones/`. Proof, each run after the last code change:

- Unit and document: `desk/service/atelier-desk.service` (`MemoryMax=1G`, `CPUQuota=100%`, `Nice=10`, `Restart=on-failure`, `WantedBy=default.target`); `docs/architecture/desk.md`, "Running it as a service", gives install, update and check steps, where each file goes, and how the `t3` wrapper finds the running T3 Code version. `systemctl --user status atelier-desk`: active (running), enabled; `Memory: 48.3M (max: 1G)`.
- SIGTERM: `tests/signals.rs` `sigterm_and_sigint_stop_the_server_and_revoke_its_token` failed before the change (`SIGTERM: exit ExitStatus(unix_wait_status(15))`) and passes after it. This settles the deferred finding atelier-mg2j. On the service, `systemctl --user stop` gave `Result=success`, `ExecMainStatus=0`, and it stayed inactive.
- Clones: `make-clones.sh ~/.cache/atelier-desk/clones` printed `ok` for clone, convert, verify and check in all five (atelier 137 items and 8 missions, astromech 240 and 4, demiurge 19 and 1, life-on-rails 47 and 3, cosplay-studio 37 and 7; each `check` said `Every record is valid`), then `All five clones are converted and pass check.` Before its first write it refuses a clones path that, with symbolic links resolved, lies inside `~/projects/` or a source repository (finding atelier-vi6k): `desk/service/make-clones.test.sh` refuses seven such paths in a scratch projects directory and checks nothing was written; against the script before the fix it failed six of them.
- desk-matches-cli: `pnpm desk-matches-cli http://buildbot.tail47515a.ts.net:3780 ~/.cache/atelier-desk/clones` printed `same` for 23 open missions and 5 inboxes, then `All 23 open missions and 5 inboxes match the CLI.`, exit 0. With a copy of the atelier clone whose atelier-z4cr title was changed, it printed `DIFF  atelier atelier-z4cr (ready; 2 ready): title: page "Build the Desk: ...", show "A title the Desk does not show"` and exited 1.
- act-is-a-record: `pnpm act-is-a-record ~/.cache/atelier-desk/clones/atelier` printed `same` for one new commit each, the message `Ready atelier-8v3b`, the changed path, the mission record with the ready time set aside, the page's status `Ready`, no Say go button and no page errors, then exit 0. With `--differ` it printed `DIFF  .atelier/missions/atelier-i6xf/mission.md, with the ready time set aside: line 10: Desk "  by: sumeet", CLI "  by: sumeet-differs"` and exited 1. It removed its scratch directory both times.
- Neither command is in the `desk` CI job: they live in `desk/live/`, which `pnpm test` (Vitest, `src/**/*.test.ts`) and `pnpm e2e` (Playwright, `e2e/`) do not read; `pnpm typecheck` checks them. `desk/README.md`, "The success criteria, against the real server", says how to run each. `.github/workflows/` is unchanged.
- Built from: `build.rs` records `HEAD`; `GET /api/repos` gives `built_from` (`the_repositories_response_names_the_commit_the_server_was_built_from`) and the sidebar's footer shows it (`the sidebar's footer names the commit the server was built from`, both widths). The service answers `6f8eeabe2b21bb65213fd256b43eb0490c17b901`.
- Service checks: `/api/repos` answered 200 with the five clones and no errors on `127.0.0.1:3780`, `100.124.153.80:3780`, `buildbot.tail47515a.ts.net:3780` and `buildbot:3780` (by `--resolve`); `Host: elsewhere.example` got 403 on loopback and on the tailnet name. After `kill -KILL` of the main PID it was active again with a new PID and `NRestarts=1`. `/api/threads` reports T3 Code `reachable: true`.
- Left alone: `atelier-serve.service` active, `tailscale serve status` unchanged (7341, 3847, 6080, 80, 443, 8443), `t3code.service` active throughout.
- Suites at the final code commit: `cargo nextest run -j 4` in `next/` 285 passed; `cargo fmt -- --check` clean; in `desk/`, `pnpm typecheck` clean, `pnpm test` 43 passed, `pnpm build` built, `pnpm e2e` 52 passed.

### Decisions

- Port 3780, the server's default, on loopback and the tailnet address only. A tailnet address that is not up yet at boot makes the bind fail; `Restart=on-failure`, `RestartSec=5` and `StartLimitIntervalSec=0` retry until it is, since a user unit cannot be ordered after `tailscaled.service`.
- The `[t3] command` is a wrapper, `~/.local/share/atelier-desk/t3`, that reads `activeVersion` from `~/.t3/runtime/service-state.json` and runs that version's `t3`, so a T3 Code update needs no step.
- The commit comes from `build.rs` and is not marked when the tree has changes not committed; `install.sh` refuses such a tree instead.
- The clones hold only each repository's base branch, so a started item shows its base record in the Desk and in the CLI alike. Their work files and logs are in `~/.cache/atelier-desk/clones.work/`, outside the served directory, so the Desk never lists a half-made clone.
- The inbox's needs-human groups carry `data-testid="needs-human"` for desk-matches-cli to read.

### Open items

- After the merge the orchestrator runs `desk/service/install.sh` from `master` (the update steps); the service then reports that commit. atelier-oc9k runs both commands independently.
- The release build took 23 s at `-j 4`, so the service runs the release build.
