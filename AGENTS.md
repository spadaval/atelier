# Repository Map

This file is a map to the repository's durable guidance. Keep process and
product policy in the linked sources instead of duplicating it here.

## Between the switch and the cutover

The tree now holds only the rewritten tool: `crates/` is the new workspace,
and the old one is gone. This repository's records are still in the old
layout, which only the old tool reads, so until the cutover (atelier-wd1i)
has run:

- The installed binary, 0.4.0, stays the tracker. Every `atelier` command
  below is that binary's.
- Nobody installs from the tree or runs `cargo install`. A binary built
  here cannot read any repository's records yet, and installing it would
  stop work in every repository on the host.
- The tree's binary, `target/debug/atelier`, is run only in scratch clones
  whose records are converted, never in a checkout of a real repository.
- The next step is the runbook: `scripts/cutover/README.md` says how it is
  run. The cutover replaces this file with `scripts/cutover/agents/atelier.md`.

## Missions

Open missions are listed by `atelier work missions`, and `atelier work mission
<id>` shows each one's strategy; this file does not track them. The verified repo-native tracker rebuild (mission
`atelier-kgcr`, `docs/strategies/atelier-rebuild.md`) closed on 2026-10-15.
[ADR 0019](docs/adr/0019-verified-repo-native-tracker.md) remains the
governing decision, and current product and architecture docs describe the
rebuilt model.

## Process Sources

- Agent Factory skill: the installed copy (`~/.agents/skills/agent-factory`,
  loaded as `/agent-factory`). Its one source is `skills/agent-factory` in
  this repository, and its evals are `evals/agent-factory`
  ([ADR 0029](docs/adr/0029-agent-factory-lives-in-atelier.md)).
- Tracker state: `.atelier/` (records are committed with the code they
  describe)
- Start a session with `atelier status`; before handoff commit the records
  and run `atelier land`. Active work may remain open, with unfinished code
  handed off honestly.
- One issue, in order: `atelier start <id>` (an item with a recorded branch
  switches to it; otherwise on `master` it creates a branch named `<id>`, and
  on another branch it takes that branch), implement, tick
  every Done when line and write the `## Report`, `atelier review request
  <id>`, fix and re-review, approve, then `atelier merge <id>` under the merge
  policy below, which marks the work done. Work has no close step. Only a code
  change makes an approval stale; a Done when line rewritten after start is
  shown in the review brief and by `merge --check`, and refuses nothing. Work
  that changes no code and can only happen after a merge or release is a loose
  end, unless it is an independent validation; a change with no issue takes
  the light path. Agent Factory's Atelier reference and
  `docs/product/work-model.md` have the detail; `CONTEXT.md` defines check,
  review, land, done and close.
- `.claude/settings.json` runs these through the installed CLI's Claude Code
  hooks (`atelier setup claude`). Do not point them at the tree's binary.

## Merge Policy

- Agents may merge independently reviewed work into `master` locally without
  asking. Unreviewed non-trivial code never reaches `master`. Tracker records
  and mission strategies may be committed there directly; non-trivial
  planning is reviewed before the work it defines starts.
- Reviewed work reaches `master` through one command, run from the work
  checkout: `atelier merge <id> --push --wait`, or with no id for a change
  on the light path. It pushes the work branch to `origin`, reuses or opens
  its pull request, waits for the CI workflow, merges, judges and pushes
  `master`, and confirms the pull request merged
  ([merge --push](docs/product/reviews-and-merge.md#merge---push-step-by-step)).
  Do not push branches, open pull requests, wait on checks or push `master`
  by hand. `--wait` polls for up to 30 minutes and the CI run takes about 5,
  so where a harness limits foreground commands, run it in the background,
  or set `--timeout` under the limit and rerun when it refuses with
  `ci_pending`.
- A branch that changes anything besides tracker records (`.atelier/issues`,
  `reviews`, `evidence`; the workflow policy and config count as code) merges
  only after CI has passed on its latest code. The `ci.passed` merge check
  enforces this through `gh` ([ADR 0031](docs/adr/0031-a-forge-carries-branches-and-ci.md)),
  so `atelier merge` runs no test suite on this host, and `master` moving
  after the run needs no sync and no new run. Record-only commits after that
  run, such as approvals, need no new run; a commit that changes code does,
  including an `atelier merge --sync` that brings in code from `master` (a
  clean one still keeps the approval). A record-only branch needs no CI run.
- `merge --push` pushes over `origin`, whose URL here is HTTPS. A host
  without HTTPS credentials for GitHub needs one command in a fresh checkout:
  `git config remote.origin.pushurl git@github.com:spadaval/atelier.git`
  (pushes go over SSH, fetches stay HTTPS), or `gh auth setup-git`. `gh` must
  be signed in either way.
- Tracker records and planning go on `master` directly; publish them with
  `atelier land --push` from the base checkout, which pushes only when land
  passes. Agents may push work branches and `master` this way without asking.
  Never force-push `master` or delete remote branches without the human.

## Product Sources

- Product intent: `PRODUCT_INTENT.md`
- Domain language: `CONTEXT.md`
- Documentation map: `docs/index.md`
- Architecture: `docs/architecture/index.md`
- ADRs: `docs/adr/`
- Code standards: `docs/architecture/quality/standards.md`

## Command References

- Installed CLI: `atelier` 0.4.0, the old tool, until the cutover. Do not
  reinstall it from this tree; the release installer in `README.md` installs
  the latest release, and the pinned fallback is in `scripts/cutover/README.md`.
- The tree's binary: `target/debug/atelier` (`cargo build -j 4 -p
  atelier-cli`), for scratch clones only.
- The Desk's app: `desk/` (`pnpm test`, `pnpm e2e`; see `desk/README.md`).
- The cutover kit: `scripts/cutover/` (`README.md` first).
- Formatting: `cargo fmt -- --check`
- Lints: `cargo clippy --all-targets -- -D warnings`
- Rust tests: `cargo nextest run`
- Whitespace: `git diff --check`
- Locally, run formatting and the tests that cover your change
  (`cargo nextest run -p <crate>` or a filter). The full suite runs on Linux
  in CI on the pull request (`.github/workflows/ci.yml`), and that run gates
  the merge. CI skips the Rust jobs when a change touches only tracker
  records, Markdown outside `crates/`, and the Desk's app outside
  `desk/src/api/`; the run still passes, so such a branch still needs its
  pull request (`scripts/ci-jobs.sh` holds the rule, and its test is
  `scripts/ci-jobs.test.sh`).
