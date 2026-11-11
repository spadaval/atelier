---
schema: atelier.item
schema_version: 3
id: atelier-2mlh
title: Run the first real items under the new tool, and fix what first use breaks
status: done
mission: atelier-qr5s
labels:
- redesign
priority: medium
blocked_by:
- atelier-wd1i
implementers:
- thread:delegated-task:command%3Amcp%3A2074a88e-920b-49f9-bd17-caf5903cc60f%3Adelegate-task%3A77050191-cad3-4b48-9372-f665e6d5a28e
ready:
  by: sumeet
  at: 2026-11-11T15:43:38.001833021Z
trivial: null
caused_by: null
created_at: 2026-11-10T00:56:10.176751966Z
---

## Goal

Take the first real item through the new tool in this repository, and with it remove the last piece of the old one from CI. CI's `check` step still has a branch that downloads the old 0.4.0 release for a repository that holds `.atelier/issues/`; no repository does any more. This item is also the first time the new binary pushes a branch, opens a pull request, waits for CI and pushes `master` on real GitHub, so what it shows about the loop is half its result.

## Done when

- [x] `.github/workflows/ci.yml`'s `check` step builds and runs the tree's binary only: it downloads no release, holds no checksum, and does not look for `.atelier/issues/`. The step's commands, run by hand in a throwaway clone of this branch, pass, and the Report gives them with their output.
- [x] Nothing else in `.github/` or `scripts/` outside `scripts/cutover/` still serves the fallback: `git grep -n -e 'v0\.4\.0' -e '\.atelier/issues' -- .github scripts ':!scripts/cutover'` prints nothing, or the Report explains each line it prints. `scripts/cutover/` is left alone: its rollback and its `adopt` command still need the fallback until the work host has adopted.
- [x] The Report keeps the record of this first use as far as it can be known before `submit`: each `atelier` command run so far, in order, with its exit status, and every refused or errored call, with what it printed. What happens from `submit` on (the push, the pull request, CI on it, the reviews, `merge` and the push of `master`) cannot be ticked here, since `submit` checks these lines first. The implementer reports it to the mission's lead when the item is done, and the lead writes it into the mission's Result. Throughout, the branch is not pushed, the pull request not opened and `master` not pushed by hand around the tool.
- [x] Each defect this first use showed in the binary, the skill or `AGENTS.md` before `submit` is in the Report with what was seen. None is fixed here, since each is outside this item's Scope: one that stops the item is put to the lead with `atelier escalate`, and the rest are left for the lead to file.
- [x] The Report gives the non-test Rust line count of each crate under `crates/` (`scripts/count_non_test_rust.py`), beside the 48,988 lines the old workspace had before the redesign.

## Scope

`.github/workflows/ci.yml`, a script it calls where that script serves only the fallback, and this item's record. Not `scripts/cutover/`, not the docs (atelier-6w8d), and not what `status` prints (atelier-ivbq).

## Context

Mission atelier-qr5s, step H. The cutover ran on 2026-11-11: this repository's records are in the new layout, the installed `atelier` is 0.5.0 built from this tree, and `AGENTS.md` describes the loop. The specifications are `docs/strategies/redesign-product.md` and `docs/strategies/redesign-architecture.md`.

The same step asks for one item in a product repository. That is worldcrafter-s9lp in demiurge, planned there with the same care: the cutover moved that repository's open mission's strategy into the mission's directory, and tracked files outside `.atelier/` still name the old path. Its Done when is that a search for the old path finds nothing outside `.atelier/`, each changed reference resolves, and the change touches nothing else. demiurge configures no forge, so its merge is local and its base is pushed with Git, as its `AGENTS.md` says.

This host runs a service its owner depends on: one cargo command at a time, with `-j 4`; set `TMPDIR` to a directory under `$HOME` outside any repository, since `/tmp` is a small quota'd tmpfs.

## Report

CI's `check` step now runs one command, `target/debug/atelier check`, on the binary the step before it built. The 0.4.0 download, its two SHA-256 sums and the `.atelier/issues` test are gone, with the comment that explained them. The workflow-level `ATELIER_NO_UPDATE_CHECK: 1` went too: its comment said it kept the fallback release's `check` off the network, and nothing in `crates/` reads it (`git grep NO_UPDATE_CHECK -- crates` prints nothing). The change is commit d0f75b44a, `.github/workflows/ci.yml` only.

### The check step, run by hand

In a throwaway clone of this branch at d0f75b44a (`git clone --branch atelier-2mlh <this worktree> ~/.cache/atelier-2mlh/clone`, with `TMPDIR` under `$HOME` and `RUST_BACKTRACE=0` as CI sets it), the `test` job's commands up to and including the check step:

```
$ cargo fmt -- --check
exit 0
$ cargo build --locked --all-targets -j 4
   Compiling ts-rs v12.0.1
   Compiling atelier-cli v0.5.0 (/home/supadava/.cache/atelier-2mlh/clone/crates/atelier-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 15.10s
exit 0
$ target/debug/atelier check
Every record is valid: 133 items, 7 missions and config.toml.
exit 0
```

The build was quick because this host's cargo uses sccache; `target/debug/atelier` in the clone is the clone's own build (`atelier 0.5.0`, written 10:45 that run). `scripts/ci-jobs.test.sh`, which the `changes` job runs, also passes in the clone (exit 0).

### What the grep still prints

`git grep -n -e 'v0\.4\.0' -e '\.atelier/issues' -- .github scripts ':!scripts/cutover'` prints nothing under `.github/` now. It still prints lines in two places, and neither serves the fallback:

- `scripts/ci-jobs.sh:13` and `:30`, and `scripts/ci-jobs.test.sh:51-52`: the path rule that lets a change touching only records skip the Rust job. `.atelier/issues/*`, `.atelier/reviews/*`, `.atelier/evidence/*` and `.atelier/runtime/*` are old-layout record paths in that rule; the new layout's records already skip through the `*.md` case (the `new-layout-records` test). They download and check nothing, and the script serves the path rule, not only the fallback, so it is outside this item's Scope. They are dead patterns now; dropping them is a small cleanup for the lead to file.
- `scripts/scorecard/` (`README.md:93`, `draw_samples.py:136`, `records.py:27`, `test_scorecard.py`): the redesign scorecard reads the frozen 2026-11 baseline snapshot, whose repositories are in the old layout at the manifest commit. That is history the baseline measures, not a fallback, and it must keep reading `.atelier/issues/` to reproduce `counted.json`.

`scripts/cutover/` is untouched.

### Non-test Rust lines

`scripts/count_non_test_rust.py HEAD crates` at d0f75b44a (exit 0):

| Crate | Lines |
| --- | ---: |
| atelier-app | 4,234 |
| atelier-cli | 5,597 |
| atelier-core | 494 |
| atelier-desk | 2,759 |
| atelier-records | 1,695 |
| Total | 14,779 |

The old workspace had 48,988 before the redesign, so the new one is 30% of its size (34,209 fewer lines). The count excluded 23 test files and 9 `cfg(test)` items (692 lines).

### First use, up to `submit`

Every `atelier` command this thread ran, in order:

1. `atelier status`, run by the SessionStart hook on branch `t3/fbd19368`: exit 0 (the hook reported success). It said the thread is on no item's branch, listed eight items ready to start and five waiting on a human, and ended `Then: atelier start atelier-8ep4`.
2. `atelier start atelier-2mlh`: exit 0. It made branch `atelier-2mlh` from `master` at 96d02deb7, not from the thread's branch `t3/fbd19368` (7f499285e), and committed 46b0014e8 `Start atelier-2mlh`, which sets the status to `building` and records this thread as implementer. It printed the five Done when lines and the Report line as a checklist, said `atelier submit` commits only the item's directory, and ended `Then: atelier submit`.
3. `target/debug/atelier check` and `target/debug/atelier --version` in the throwaway clone: exit 0 both, output above.

No `atelier` call was refused or errored. The pre-commit hook passed the commit of `ci.yml`. The branch has not been pushed, no pull request opened and `master` not pushed: all of that is left to `atelier submit` and `atelier merge`.

### Defects seen before `submit`

None in the binary, the skill or `AGENTS.md` stopped or misled this item, so nothing is escalated. Two things for the lead:

- `atelier status` at session start told a thread that had been delegated atelier-2mlh to run `atelier start atelier-8ep4`, the highest-priority ready item. It cannot know the delegation, so this is a property of `status`, not a fault, but a delegated thread that trusted the `Then:` line over its brief would start the wrong item. Whether `status` should say the `Then:` is a suggestion when several items are ready belongs with atelier-ivbq.
- Outside this item's three targets, the docs still describe the update check the 0.4.0 tool had: `docs/product/development-setup.md:31` says `.cargo/config.toml` sets `ATELIER_NO_UPDATE_CHECK=1`, and no `.cargo/config.toml` exists in the tree; `docs/product/sessions-and-hooks.md:233` describes `doctor`'s update check. `evals/agent-factory/harness/run.py:219` still sets the variable, which is harmless. The docs are atelier-6w8d's.