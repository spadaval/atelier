---
created_at: "2026-11-10T01:39:35.640973419+00:00"
id: "atelier-mlwx"
issue_type: "task"
labels:
- "redesign"
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
status: "todo"
title: "Fix the old tool's fallback release: 0.4.0 if it passes, else a new 0.4.1"
updated_at: "2026-11-11T02:46:33.752549873+00:00"
---

## Goal

Fix which release of the old tool a rollback reinstalls and CI's `check` step runs, before anything in the tree changes the old binary. The cutover's way back is "reinstall the fallback release", so that release must do what this host's installed binary (`atelier 0.4.0+8842f232`, six commits past the 0.4.0 tag) does today, the site included. The 0.4.0 release is tried first. A 0.4.1 is cut only if 0.4.0 fails.

## Done when

- [ ] The 0.4.0 release's archive for this host's target is downloaded from its GitHub release into `$HOME/.cache/atelier-qr5s/mlwx/`, its SHA-256 matches the checksum the release publishes, and its `atelier` is run from there by path. It is not installed: `sha256sum ~/.cargo/bin/atelier` and `atelier --version` are the same before and after, and the Report shows both.
- [ ] In a fresh scratch clone of this repository at `master`, the release binary and the installed binary are each run on the read-only commands `scripts/smoke-repos.sh` runs, plus `atelier check` and `atelier land` without `--push`. For each command the exit statuses match and the output is the same, or each difference is quoted in the Report with the commit between 0.4.0 and 8842f232 that explains it. A difference that is a refusal, an error or a missing command fails the test.
- [ ] In a scratch clone of each of astromech, demiurge, life-on-rails and cosplay-studio, on its base branch, the release binary's `check` and `status` exit as the installed binary's do. The Report gives exit statuses only, and nothing of those repositories' content.
- [ ] The release binary serves the site: `atelier serve`, bound to a free local port with `--root` pointing at a directory that holds only the scratch clone of this repository, answers the pages and API routes the installed binary answers when run the same way on another port, with the same status codes and the same bodies, or each difference explained as above. The routes compared are listed in the Report and include the index, one mission's brief and debrief, and one item. The running `atelier-serve` service on port 7341 is not stopped, restarted or reconfigured.
- [ ] If every line above holds, the Report names 0.4.0 as the fallback and gives what step F's runbook and CI step need to fetch it: the tag and its commit, each archive's name and SHA-256 for the targets the release ships, and an install command pinned to the `v0.4.0` release, not to `latest`, which will be the new tool once it is released. Nothing is released and no file outside this item's record changes.
- [ ] If any line fails, the Report names what 0.4.0 lacks, with the evidence, and the item is escalated to the orchestrator before anything is released. A 0.4.1 is then published through the repository's release workflow on GitHub, from a commit where the old binary's sources under `crates/` are those of 8842f232, and the lines above are run again on it. No release build runs on this host.

## Scope

This item's record, and scratch directories under `$HOME/.cache/atelier-qr5s/mlwx/`. No file in the tree changes unless a 0.4.1 is needed, and then only what `CONTRIBUTING.md` and `CHANGELOG.md` say a release changes. Nothing is installed, and `~/.cargo/bin`, the `atelier-serve` unit and every real repository are left as they are. The four product repositories are read-only and some are private: work on scratch clones, send nothing from them to any outside service, and report process facts only.

## Context

Mission atelier-qr5s, step D of `docs/strategies/redesign-3-cutover.md`. Protocol: `docs/strategies/redesign-architecture.md`, Transition, part 3 ("The fallback release is fixed") and Rollback. Planning finding atelier-xzaf found the six commits between the tag and the installed binary change only the wording of the review brief; `git log v0.4.0..8842f232 -- crates` lists them. The site's unit file is `~/.config/systemd/user/atelier-serve.service`, and `docs/architecture/site.md` describes `serve`.

A fallback that is good needs no file in the tree: step F's plan reads this item's Report and puts the pinned version where the runbook and CI read it.

## Unknowns

- Whether 0.4.0 passes.
