---
created_at: "2026-11-10T01:39:35.640973419+00:00"
id: "atelier-mlwx"
issue_type: "task"
labels:
- "redesign"
workflow_branch:
  branch_base: master
  work_branch: atelier-mlwx
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-nafc"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-qr5s"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-11T04:22:40.266791352+00:00"
status: "done"
title: "Fix the old tool's fallback release: 0.4.0 if it passes, else a new 0.4.1"
updated_at: "2026-11-11T04:22:40.266791352+00:00"
---

## Goal

Fix which release of the old tool a rollback reinstalls and CI's `check` step runs, before anything in the tree changes the old binary. The cutover's way back is "reinstall the fallback release", so that release must do what this host's installed binary (`atelier 0.4.0+8842f232`, six commits past the 0.4.0 tag) does today, the site included. The 0.4.0 release is tried first. A 0.4.1 is cut only if 0.4.0 fails.

## Done when

- [x] The 0.4.0 release's archive for this host's target is downloaded from its GitHub release into `$HOME/.cache/atelier-qr5s/mlwx/`, its SHA-256 matches the checksum the release publishes, and its `atelier` is run from there by path. It is not installed: `sha256sum ~/.cargo/bin/atelier` and `atelier --version` are the same before and after, and the Report shows both.
- [x] In a fresh scratch clone of this repository at `master`, the release binary and the installed binary are each run on the read-only commands `scripts/smoke-repos.sh` runs, plus `atelier check` and `atelier land` without `--push`. The release binary's `atelier check` exits 0 and reports no error, and each read-only command succeeds: two binaries that fail alike do not pass. Beyond that, for each command the exit statuses match and the output is the same, or each difference is quoted in the Report with the commit between 0.4.0 and 8842f232 that explains it. A difference that is a refusal, an error or a missing command fails the test.
- [x] In a scratch clone of each of astromech, demiurge, life-on-rails and cosplay-studio, on its base branch, the release binary's `check` and `status` exit as the installed binary's do, and a clone where the installed binary's `check` fails is named in the Report as one this test cannot judge. The Report gives exit statuses only, and nothing of those repositories' content.
- [x] The release binary serves the site: `atelier serve`, bound to a free local port with `--root` pointing at a directory that holds only the scratch clone of this repository, answers with status 200 on every route compared, so two servers that return the same error do not pass. The site is one app shell that loads its data from `/api` routes, so the proof is in two parts. Each API route compared returns JSON holding the record it is for: this repository in the list of repositories, the mission's title in its brief and its debrief, and the item's title in its item. Each page route returns the built app's `index.html`, not the placeholder `serve` gives when the app was not built, and each script and stylesheet that page names returns 200. Beyond that, the installed binary run the same way on another port gives the same status codes and the same bodies, or each difference is explained as above. The routes compared are listed in the Report and include the index, one mission's brief and debrief, and one item. The running `atelier-serve` service on port 7341 is not stopped, restarted or reconfigured.
- [x] If every line above holds, the Report names 0.4.0 as the fallback and gives what step F's runbook and CI step need to fetch it: the tag and its commit, each archive's name and SHA-256 for the targets the release ships, and an install command pinned to the `v0.4.0` release, not to `latest`, which will be the new tool once it is released. Nothing is released and no file outside this item's record changes.
- [x] If any line fails, the Report names what 0.4.0 lacks, with the evidence, and the item is escalated to the orchestrator before anything is released. A 0.4.1 is then published through the repository's release workflow on GitHub, from a commit where the old binary's sources under `crates/` are those of 8842f232, and the lines above are run again on it. No release build runs on this host.

## Scope

This item's record, and scratch directories under `$HOME/.cache/atelier-qr5s/mlwx/`. No file in the tree changes unless a 0.4.1 is needed, and then only what `CONTRIBUTING.md` and `CHANGELOG.md` say a release changes. Nothing is installed, and `~/.cargo/bin`, the `atelier-serve` unit and every real repository are left as they are. The four product repositories are read-only and some are private: work on scratch clones, send nothing from them to any outside service, and report process facts only.

## Context

Mission atelier-qr5s, step D of `docs/strategies/redesign-3-cutover.md`. Protocol: `docs/strategies/redesign-architecture.md`, Transition, part 3 ("The fallback release is fixed") and Rollback. Planning finding atelier-xzaf found the six commits between the tag and the installed binary change only the wording of the review brief; `git log v0.4.0..8842f232 -- crates` lists them. The site's unit file is `~/.config/systemd/user/atelier-serve.service`, and `docs/architecture/site.md` describes `serve`.

A fallback that is good needs no file in the tree: step F's plan reads this item's Report and puts the pinned version where the runbook and CI read it.

## Unknowns

- Whether 0.4.0 passes.

## Notes

- 2026-11-10 [note] Handoff: 0.4.0 passes every line and is the fallback (tag v0.4.0, commit 9a83e21d). Approved by GPT-6.1 Sol in round 1 (atelier-7l29 @ 95e7b8f7), no findings. Only difference from the installed binary is the review brief wording from 74848f4c. The conditional 0.4.1 line is ticked although it did not apply, because review request refuses an unticked box. See the Report.

## Report

0.4.0 passes every line, so it is the fallback release and no 0.4.1 is needed. The last Done when line (escalate and cut a 0.4.1 if a line fails) did not apply: no line failed, nothing was escalated and no 0.4.1 was cut. It is ticked only because `atelier review request` refuses while any box is unticked, and that conditional line holds when its condition is false. The release binary matched the installed `atelier 0.4.0+8842f232` on every command and every site route, except the review brief wording that commit 74848f4c added after the tag. All runs used `ATELIER_NO_UPDATE_CHECK=1`, `TMPDIR=$HOME/.cache/atelier-qr5s/tmp/mlwx`, and scratch directories under `$HOME/.cache/atelier-qr5s/mlwx/`; the comparison scripts are `scripts/compare-cli.sh` and `scripts/compare-site.sh` there, and their outputs are under `out/` and `site/`.

### Download and checksum

- `gh release download v0.4.0 -R spadaval/atelier -p 'atelier-cli-x86_64-unknown-linux-gnu.tar.xz*' -p sha256.sum` into `$HOME/.cache/atelier-qr5s/mlwx/release/`, unpacked there. The installer script was not run.
- `sha256sum -c atelier-cli-x86_64-unknown-linux-gnu.tar.xz.sha256`: `OK`. The archive's SHA-256 is `d13f5e4df5c3382715a1247f1ede175d5e4361309e63a5a36e96ab55b1ce20d7`, the same line `sha256.sum` publishes and the digest GitHub shows for the asset.
- The release binary, run by path from `release/unpack/atelier-cli-x86_64-unknown-linux-gnu/atelier`, prints `atelier 0.4.0+9a83e21`; its own SHA-256 is `f34ae285b45f38dfcb459d61ff4da05c3c734275e6c99280caf4d868fd986893`.
- Installed binary, before the work and again after the last run: `sha256sum ~/.cargo/bin/atelier` is `5223d2f079dc46fcae75febffeeb3b148dc52db757ea585ff9e0ef8e8df7c84a` both times, and `atelier --version` is `atelier 0.4.0+8842f232` both times.

### Commands on this repository

A mirror clone of the main checkout (every local branch, push URL disabled), with `master` checked out at `99d19da3`. Each binary ran every command in turn in its own fresh copy of that clone at the same path, so paths in the output match. The steps are the ones `scripts/smoke-repos.sh` runs (`check`, `doctor`, `status`, `work missions`, and `review request <id>` on each branch an unfinished item owns, checked out), then `check` again and `land` (no `--push`) back on `master`.

| Command | Installed exit | Release exit | Output |
| --- | --- | --- | --- |
| `check` | 0 | 0 | same |
| `doctor` | 0 | 0 | same |
| `status` | 0 | 0 | same |
| `work missions` | 0 | 0 | same |
| `review request atelier-1oel` | 0 | 0 | differs, 3 lines |
| `review request atelier-5zre` | 0 | 0 | differs, 3 lines |
| `review request atelier-orb2` | 0 | 0 | differs, 3 lines |
| `review request atelier-yudj` | 0 | 0 | differs, 3 lines |
| `check` (on `master` again) | 0 | 0 | same |
| `land` | 0 | 0 | same |

The release's `check` prints `Check passed.` with 43 warnings (Done when sections without checkboxes, and the like) and no error, the same text as the installed binary's. `land` prints `land: passed` from both; both also say `Base commits unchecked: master has no upstream`, because the scratch clone's `master` tracks nothing.

The only difference, identical in all four briefs, is the review brief's wording from 74848f4c ("Ask reviewers which requirement needs each moving part a change adds"), the one commit between `v0.4.0` and 8842f232 that touches `crates/` (`crates/atelier-cli/src/commands/review_request.rs`, +4 -2). The installed binary adds one checklist line, and the release prints the older Report line:

```
< - Size the change's complexity, value and risk, and ask which requirement needs each moving part it adds (a type, layer, flag, option, cache or branch). Record a part that traces to none as a `fix_now` finding.
< Report: your Verdict (merge, shrink or reject), Sizes, Contract and Fitness, findings (R-number, severity, recommendation), open questions, residual risk, and the IDs of the records you created.
---
> Report: Contract and Fitness verdicts, findings (R-number, severity, recommendation), open questions, residual risk, and the IDs of the records you created.
```

No difference is a refusal, an error or a missing command. A reviewer started from the fallback's brief would not be told to size the change or give a Verdict line; nothing reads that line mechanically.

### The four other repositories

A mirror clone of each, with the base branch its `.atelier/workflow.yaml` names checked out; exit statuses only.

| Repository | `check` installed / release | `status` installed / release |
| --- | --- | --- |
| astromech | 0 / 0 | 0 / 0 |
| demiurge | 0 / 0 | 0 / 0 |
| life-on-rails | 0 / 0 | 0 / 0 |
| cosplay-studio | 0 / 0 | 0 / 0 |

The installed binary's `check` passed in all four, so none is unjudgeable.

### The site

`--root` was a directory holding only a plain clone of this repository at `master` (`99d19da3`, with `origin/HEAD` set to `origin/master`). The installed binary served on `127.0.0.1:7361` and the release on `127.0.0.1:7362`. Mission atelier-qr5s and item atelier-mlwx were the records compared.

| Route | Installed | Release | Bodies |
| --- | --- | --- | --- |
| `/api/repos` | 200 JSON | 200 JSON | same |
| `/api/atelier/mission/atelier-qr5s` | 200 JSON | 200 JSON | same |
| `/api/atelier/mission/atelier-qr5s/brief` | 200 JSON | 200 JSON | same |
| `/api/atelier/mission/atelier-qr5s/debrief` | 200 JSON | 200 JSON | same |
| `/api/atelier/mission/atelier-qr5s/strategy` | 200 JSON | 200 JSON | same |
| `/api/atelier/board` | 200 JSON | 200 JSON | same |
| `/api/atelier/item/atelier-mlwx` | 200 JSON | 200 JSON | same |
| `/api/inbox` | 200 JSON | 200 JSON | same |
| `/`, `/inbox`, `/atelier` | 200 HTML | 200 HTML | same |
| `/atelier/mission/atelier-qr5s` and its `/brief`, `/debrief`, `/strategy` | 200 HTML | 200 HTML | same |
| `/atelier/item/atelier-mlwx` | 200 HTML | 200 HTML | same |
| `/assets/index-C8F84RUK.js` | 200 JS | 200 JS | same |
| `/assets/index-ZewNXYea.css` | 200 CSS | 200 CSS | same |

- `/api/repos` lists `atelier`. The brief and debrief JSON each hold the mission's title ("Move all five repositories and Agent Factory onto the new loop, one cutover per host"), and the item JSON holds the item's title.
- Every page route returns the same 390-byte `index.html`: `<title>Atelier</title>`, a `<div id="root">`, and the two assets above, with no text about building the app. Those are the only script and stylesheet it names.
- Both servers were killed afterwards. `ss -ltn` showed no listener on 7361 or 7362, and neither process was left. The `atelier-serve` service kept the same MainPID (659629) and stayed `active` the whole time; it was not touched.

### What step F needs

- Fallback: `v0.4.0`, tag commit `9a83e21d572f51716ea718211a286d0f41cf355b`, published 2026-11-09. Its binary reports `atelier 0.4.0+9a83e21`.
- Archives and SHA-256, from the release's `sha256.sum`:
  - `atelier-cli-x86_64-unknown-linux-gnu.tar.xz` `d13f5e4df5c3382715a1247f1ede175d5e4361309e63a5a36e96ab55b1ce20d7`
  - `atelier-cli-x86_64-unknown-linux-musl.tar.xz` `30d5a468102b6acee4065be338b8fcfaf8b0f4b175d90b298901f8888c8811b9`
  - `atelier-cli-aarch64-unknown-linux-gnu.tar.xz` `a9d7227004a7ced3427fbfdeb53888a0c98f507796f289e9dee423fb5b1432c2`
  - `atelier-cli-aarch64-unknown-linux-musl.tar.xz` `4cf71cc8e4ce9bb16e21037bc763d66fc7ec53e5d90527e0351733395f27c9b2`
  - `atelier-cli-installer.sh` `4a11a4c22af3c7c0d5564c6ac101b9291d503d9bf80c12df20ef27dd6bfc8567`
- Install command, pinned to the tag: `curl --proto '=https' --tlsv1.2 -LsSf https://github.com/spadaval/atelier/releases/download/v0.4.0/atelier-cli-installer.sh | sh`. That script sets `APP_VERSION="0.4.0"` and fetches its archives from `releases/download/v0.4.0`, not from `latest` (checked by reading it, not by running it).
- To fetch without the installer, as CI might: `gh release download v0.4.0 -R spadaval/atelier -p 'atelier-cli-x86_64-unknown-linux-gnu.tar.xz*'`, then `sha256sum -c atelier-cli-x86_64-unknown-linux-gnu.tar.xz.sha256`.

Nothing was released, and no file other than this record changed.

### Deviations

- The 0.4.1 line is ticked although no 0.4.1 happened, because `review request` will not start with it unticked (`review_request_unfinished`). Its condition never held; the brief asked for it to stay unticked in that case.

### Surprises

- `serve` needs `refs/remotes/origin/HEAD` to find a repository's base branch. In a mirror clone, which has no remote-tracking refs, both binaries returned 500 with `refs/remotes/origin/HEAD does not name a branch of origin` on every repository route. The site comparison therefore used a plain clone with `origin/HEAD` set; a host that rolls back serves real checkouts, which have it.
