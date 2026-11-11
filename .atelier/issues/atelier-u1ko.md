---
created_at: "2026-11-10T01:29:41.330899323+00:00"
id: "atelier-u1ko"
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
    id: "atelier-1oel"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-5zre"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-mlwx"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-nafc"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-oci2"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-psjw"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-qr5s"
    type: "advances"
  - kind: "issue"
    id: "atelier-t747"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-yudj"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Delete the old workspace and put the new one in its place, with the cutover runbook"
updated_at: "2026-11-11T04:46:58.250378721+00:00"
---

## Goal

Make the one reviewed change after which the tree holds only the new tool, and put everything the cutover will run under review before it runs. After this change merges, the tree's binary cannot read this repository's records, so the cutover (atelier-wd1i) follows at once. That is why this item stops at an approval on its branch: it merges only once Sumeet has named the time.

The change has two parts on one branch. The switch deletes the old workspace and moves the new one from `next/` to the repository root, with CI and the release build following. The cutover kit, under `scripts/cutover/`, is the runbook, its rollback, the text each repository's `AGENTS.md` gets, and the one workflow another repository needs replaced. The rollback is tried on scratch clones at every stopping point before anyone relies on it.

## Done when

### The switch

- [ ] `crates/` holds the new workspace and nothing of the old one: the old `crates/`, with all its tests and the site crate and app, is deleted, and every tracked file under `next/` is moved to the same path without `next/`, with no change to a moved source file. `git diff -M --stat master...HEAD` shows the moves as renames, and the Report lists every moved file whose content changed and why. The root `Cargo.toml` and `Cargo.lock` are the new workspace's. No `next/` directory remains, and no tracked file names a path under it.
- [ ] Every tracked file outside `crates/` that existed only for the old workspace is deleted or brought into line, and the Report lists each with what happened to it: `scripts/smoke-repos.sh`, `.github/build-setup.yml`, `scripts/ci-jobs.sh` and its test, `scripts/count_non_test_rust.py`, `scripts/bench_read_commands.py`, the root `package.json` and `package-lock.json`, `proptest-regressions/`, `.cargo/`, `.config/`, `.gitignore`, `.gitattributes`, `dist-workspace.toml` and the dependabot config among them. The docs are not rewritten here; step I (atelier-6w8d) replaces them. The eval harness and `scripts/cutover/rehearse.sh` find the binary at its new path.
- [ ] The root workspace is the new tool and is releasable: `cargo build --locked`, `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings` and `cargo nextest run --locked` pass at the root, each with `-j 4`; `cargo install --path crates/atelier-cli --locked --root <scratch directory>` installs a binary named `atelier`; and that release build is the new binary's first: it runs `check` on a converted scratch clone of each of the five repositories, and the Report gives the build's wall time on this host and the binary's size. The workspace version is `0.5.0`, `atelier --version` prints it, and no build step needs Node or pnpm.
- [ ] `.github/workflows/ci.yml` has one set of Rust jobs, for the root workspace, and no job or step for the old site's app. Its `check` step runs the fallback release while the repository holds `.atelier/issues/`, and the tree's binary once it does not. The fallback is fetched as atelier-mlwx's Report gives it: the `v0.4.0` archive for the runner's target, with its SHA-256 checked against a value written in the workflow. Both branches of that step are shown to work: the first by this branch's own CI run, the second by running the step's commands in a scratch clone whose records are converted. The workflow keeps the name `ci.yml`, which the old tool's merge gate and the converted `config.toml` both name.
- [ ] `.github/workflows/release.yml` and `dist-workspace.toml` build the new tool with no Node or pnpm step, and the release workflow's plan job passes on this branch's pull request. Nothing is tagged or released.
- [ ] `AGENTS.md` describes the tree between the switch and the cutover: the installed binary (0.4.0) stays the tracker, nobody installs from the tree or runs `cargo install`, the tree's binary is run only in scratch clones, and the next step is the runbook. Its command references name what exists in the tree. This text lives only until the cutover replaces it.
- [ ] This branch's CI passes on its latest code before the final review round. The branch reaches GitHub the one way the merge policy allows, `atelier merge atelier-u1ko --push`, run only while no approval of this item exists, so that a gate refuses and nothing merges; it is never run once a reviewer has approved (Context, "Not merging").

### The cutover kit

- [ ] `scripts/cutover/cutover.sh` runs the cutover's steps as `docs/strategies/redesign-architecture.md`, Transition, part 4, gives them, stops at the first failure, and says at each stop which step failed and how to undo it. It can be run again after a stop and picks up where the state it recorded says. Everything outside the script that it touches comes from one settings block at its top or the environment, with this host's values as defaults: the five repositories' paths and base branches, the Git remote, the install root, the skill checkout, the `systemctl` and `gh` commands, and the directory where it records state. The rollback trials run the same script with those pointed at a sandbox.
  1. **Preflight.** It stops unless: each repository's checkout is clean and its base branch is at its remote's tip; no item is in progress in any of the five, as the installed old binary reports; and no T3 thread is running in any of them, read from T3's database with one read-only, bounded query (or, if that cannot be read, it stops and says how to check by hand). It lists each repository's unmerged local branches, whose reviews the conversion will not carry, as information. It records each repository's base commit, copies each clone's Git hooks directory aside, and records the installed binary's version and hash, the skill checkout's commit, and whether `atelier-serve` is enabled and running.
  2. **Convert atelier**, on `master`: run the converter, replace `AGENTS.md` with the reviewed text, move `skills/agent-factory-next/` to `skills/agent-factory/`, write the harness hook settings the new `setup` writes, and commit. Run `--verify` and the tree's `atelier check`.
  3. **Install.** `cargo install --path crates/atelier-cli --locked -j 4` from `master`; `atelier setup` for the Git hooks and for each harness; move the skill checkout forward to `master`; stop and disable `atelier-serve`, since the new binary has no `serve`.
  4. **Push `master`** with `ATELIER_SKIP="cutover conversion"`, by plain `git push`.
  5. **Each of the other four repositories**, converted in a fresh clone of its base branch, never in a checkout that may sit on a feature branch: convert; replace the process section of `AGENTS.md` and, in astromech, the workflow that runs Atelier; write the hook settings; commit; run `--verify` and `atelier check`; push with the same skip; then bring the real checkout's base branch up to the pushed commit without touching what it has checked out, and run `atelier setup` there.
  6. **Close** atelier-wd1i with `atelier close`, the first act the new tool records, and push.
  At the end it prints, for each repository, the conversion commit, the count of items and missions, `--verify`'s result and the count of relative links that still resolve.
- [ ] `scripts/cutover/rollback.sh` undoes whatever `cutover.sh` recorded as done, in the order the Transition's Rollback gives: hooks directories back first; each repository reset to its recorded base commit, or its conversion commit reverted and pushed if it was pushed; then the fallback release reinstalled from the pinned `v0.4.0` archive with its checksum checked, its `setup` run, the skill checkout moved back, and `atelier-serve` enabled, started and checked to answer. It is safe to run twice.
- [ ] **The rollback is tried on scratch clones at each stopping point** the Transition lists, and the Report says for each what was stopped, what `rollback.sh` did, and what was compared afterwards: (1) a conversion that fails `--verify` before its commit; (2) after step 2's commit; (3) part-way through step 3, with the binary installed and the hooks not; (4) after step 3; (5) after step 4; (6) a second repository's conversion in step 5 committed and not pushed. Each trial runs in a sandbox: clones of all five repositories with their remotes pointed at local bare repositories and Git limited to local files, as `rehearse.sh` does it; a scratch install root; a copy of the skill checkout; and doubles for `systemctl` and `gh`. After each rollback, every clone's base, working tree and hooks directory equal what they were before the trial, the sandbox's installed binary is the fallback and passes `check` on the old records, and the skill copy is at its old commit. One clone has a hook of its own beforehand, and it is back afterwards. One full run with no stop is made too, and it ends with `check` passing in all five under the new binary and atelier-wd1i's sandbox copy closed.
- [ ] **Text for each repository's `AGENTS.md`**, under `scripts/cutover/agents/`. For atelier, the whole file: the new commands, the merge policy as the new tool applies it, and how a mission is read until the Desk has its page (`atelier show <mission>`). For the four product repositories, one process section each, written here and holding nothing of theirs: how work starts (`atelier status`), and how the base reaches the remote, since none of them configures a forge: after `atelier merge`, `git push` of the base, which the pre-push hook checks. The runbook finds the section to replace by rule, not by quoting the repository: the sections of its `AGENTS.md` that name a removed Atelier command. It stops if a repository has none or more than the number recorded for it. The Report gives, per repository, the count of sections and lines replaced and nothing else. No text sends anyone to a site page, names a removed command or record path, or tells an agent to commit after `ready`, `drop`, `close` or `lead`.
- [ ] **A replacement for astromech's workflow that runs Atelier**, under `scripts/cutover/`: it installs Atelier from the commit of `master` the cutover converted, which the runbook writes into it, and runs `atelier check`. It runs no `doctor` and no `land`. The runbook replaces the old workflow with it in astromech's conversion commit. No other repository's workflow names Atelier.
- [ ] **Three first uses that the cutover would otherwise meet live** each happen here first, and the Report says what each showed:
  - *A release build*, in the switch's lines above.
  - *Claude Code's hooks in a live session*: in a converted scratch clone where the new `atelier setup claude` wrote the hooks, one real Claude Code session on a cheap model starts with `atelier status` in its context and ends its turn through the Stop hook's `atelier check`, shown from its transcript.
  - *Real GitHub, read only*: from a scratch clone whose fetch URL is this repository's and whose pushes go to a local bare repository, the new binary asks the forge the questions `review` and `merge` ask, for a branch that has a pull request and a finished CI run, and gets the pull request and the run. Nothing is pushed or opened by the new binary before the cutover; its first push and pull request are step H's first item, and the Report says what that item should do if either fails.
- [ ] `scripts/cutover/README.md` is the page Sumeet and the cutover's thread read on the day: what must be quiet beforehand, the two commands in order (the merge of this item, then `cutover.sh`), how long each step took in the full sandbox run, what he will see, how to tell it worked, when and how to run `rollback.sh`, and how the work host converts later with the same script.
- [ ] `docs/strategies/redesign-architecture.md`, Transition, matches the runbook as built, in the same change: where the hook settings are written, how a product repository's base is pushed, that the product repositories are converted in fresh clones, and the full list of what the move touched. The `last_changed` line is left alone.

## Scope

In: the move and what follows from it in the tree's build, CI and release files; `AGENTS.md`; `scripts/cutover/` (`cutover.sh`, `rollback.sh`, `README.md`, the `AGENTS.md` texts, the astromech workflow, and `rehearse.sh` where the binary's path or a shared helper changes); the Transition section; this item's record.

Out, and left alone: any behaviour of the new tool (a defect found is reported to the orchestrator, and fixed here only when it is small, inside the specifications and blocks the runbook); `skills/`; the old docs, `CONTEXT.md`, `PRODUCT_INTENT.md` and the ADRs (step I); the Desk's files and mission; the converter's rules (atelier-yudj settled the last of them).

Nothing real is converted, installed or stopped by this item. The runbook and the rollback run only in sandboxes under `$HOME/.cache/atelier-qr5s/u1ko/`. The installed binary, `~/.cargo/bin`, the skill checkout at `/home/supadava/projects/atelier-skill`, the `atelier-serve` service, T3's database (one read-only bounded query to design the preflight, never a write) and the five real repositories stay as they are. astromech, demiurge, life-on-rails and cosplay-studio are read-only and some are private: work on scratch clones, commit nothing from them here, this repository being public, send nothing from them to any outside service, and report counts and process facts only.

A reviewer's finding that asks for behaviour the specifications and this item do not name is escalated to the orchestrator, not built.

## Context

Mission atelier-qr5s, step F of `docs/strategies/redesign-3-cutover.md`: read it in full, with "What mission 2 showed" for step F. The protocol is `docs/strategies/redesign-architecture.md`, Transition, parts 3 and 4 and the Rollback, as atelier-oci2 corrected it. The converter is `scripts/cutover/convert_records.py`; `scripts/cutover/rehearse.sh` already makes the sandboxed clones, the `gh` double and the logging `atelier` this item's trials need. The fallback release is in atelier-mlwx's Report: tag `v0.4.0`, its archives' names and SHA-256, and an install command pinned to the tag. The rehearsal's Report (atelier-nafc, "For the cutover") has the per-repository facts: which files name Atelier, that no base requires a pull request, and that one repository's checkout sits on a feature branch.

Settled in this plan:

- **One branch.** The switch and the kit are one item and one pull request, because the runbook installs from the tree the switch leaves and its trials need both.
- **Version.** The new tool is `0.5.0`. Nothing is released by this item; a release of the new tool is cut after the cutover, when a second host needs it. astromech's CI installs from a commit, as it does today.
- **No forge for the product repositories.** Their converted `config.toml` names none, as today, so `submit` opens no pull request there and `merge` pushes nothing. Their `AGENTS.md` text says to push the base with Git. Giving a repository a forge is a reviewed change to its config, later, if Sumeet wants pull requests there.
- **Hook settings.** `atelier setup` for a harness writes into the repository it runs in, so each conversion commit carries those files, and resetting or reverting that commit is what restores them.
- **`atelier-serve`.** Stopped and disabled at step 3, and on rollback enabled, started and checked.
- **Not merging.** Under the old tool, `atelier merge <id> --push` pushes the branch and opens its pull request even when a gate refuses. Run it, without `--wait`, only while `atelier review list atelier-u1ko` shows no approval: the missing approval is the gate that refuses, the branch gets its pull request, and CI runs. Do that before requesting each review round, never after a reviewer approves. Once an approval exists, no `atelier merge` command is run by anyone but the orchestrator, at the time Sumeet names.

`AGENTS.md` asks for `scripts/smoke-repos.sh` before review of a command change; this change deletes that script, so it is not run.

Expected size: the move is about 50 renames and 290 deletions with a dozen files changed around it; the kit is 600 to 900 lines of shell, 150 of text, and the trials' harness beside it.

## Unknowns

- How T3's database shows a thread running in a repository. If one bounded read-only query cannot answer it, the preflight asks for a hand check instead, and the Report says so.
- Whether the old binary's merge checks and its `check` pass on the switched tree, since they run until the cutover. The branch's own CI run and `atelier merge --check` answer it early.
