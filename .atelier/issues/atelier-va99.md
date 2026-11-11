---
created_at: "2026-11-11T02:17:27.158256540+00:00"
id: "atelier-va99"
issue_type: "task"
labels: []
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
status: "todo"
title: "Install the Desk as a service on this host over converted clones of the five repositories, with one command for each success criterion"
updated_at: "2026-11-11T07:07:35.774232494+00:00"
---

## Goal

Run the Desk as a service on this host, reached over the tailnet, serving converted scratch clones of the five repositories, with everything needed to install it, update it and check it kept in the repository. After this item Sumeet can open the Desk on his phone, and the mission's two success criteria can each be checked by one command. It does not wait for the cutover: pointing the service at the live repositories is the loose end atelier-qxmr.

## Done when

- [ ] A systemd user unit for `atelier-desk` is in the repository, with limits like those of `atelier-serve.service` on this host (one core, 1 GiB, a low priority), restarting on failure and started at boot. `docs/architecture/desk.md` gives the steps to install and to update the service: building the binary, building the app, where the binary, the app's files and the configuration go, and how the configuration names the `t3` command so that a T3 Code update does not leave it pointing at a removed version.
- [ ] The server stops cleanly on SIGTERM as on SIGINT, revoking its T3 Code sessions, shown by a test (deferred finding atelier-mg2j, which this item settles).
- [ ] One script in the repository makes or refreshes the converted scratch clones under a directory it is given: for each of atelier, astromech, demiurge, life-on-rails and cosplay-studio it clones the local repository's base branch, disables pushes, converts with `scripts/cutover/convert_records.py`, verifies the conversion, commits it, and runs the new `atelier check`. It never writes to a live repository, and the Report gives its output on this host.
- [ ] **desk-matches-cli as a command.** One command in the Desk's suite takes the Desk's address and the directory of clones. For every open mission in every clone it loads the mission's page in a headless browser, fails on a page error or a console error, and compares the title, status and per-status item counts on the page with the new `atelier show <mission>` in that clone; it compares the inbox's needs-human items with `atelier list --needs-human` in each clone. It prints one line per mission and per inbox and exits non-zero on any difference.
- [ ] **act-is-a-record as a command.** One command in the Desk's suite takes a converted clone, copies it to a scratch directory, adds a draft mission that may be made ready there with the new `atelier` commands and commits it, and clones that commit twice. It starts `atelier-desk` with the built app on the first, says go from the mission's page in a headless browser, and runs the new `atelier ready <mission> --by <name>` in the second. It checks that each has exactly one new commit, and compares the two commits' messages and changed paths, and the two mission records with the recorded time set aside; it then checks the page shows the mission ready. It exits non-zero on any difference, and it is shown to fail when the two are made to differ.
- [ ] The two commands above are not part of the `desk` CI job, which builds no Rust; `desk/README.md` says how to run each.
- [ ] The server says which commit it was built from, in a response the app shows in its footer, so anyone can tell what the service runs.
- [ ] Before the merge, the service is installed and running on this host from the branch's last code commit, by the document's steps: `systemctl --user status atelier-desk` shows it active and enabled, it answers on loopback and on the tailnet address at the host's tailnet name, it refuses a request with another `Host`, and it comes back after its process is killed with SIGKILL (`Restart=on-failure` does not restart after a clean stop on SIGTERM). It serves the five converted clones made by the script. The Report gives the address. After the merge the orchestrator runs the document's update steps once from `master`, and the validation item atelier-oc9k checks the commit the service reports.
- [ ] Both commands pass against the installed service or, for act-is-a-record, on this host; the Report gives their output. The independent run of both is the validation item atelier-oc9k, not this item.
- [ ] The existing `atelier-serve.service` and its `tailscale serve` publication are left as they are.

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
