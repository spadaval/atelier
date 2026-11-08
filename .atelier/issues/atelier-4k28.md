---
created_at: "2026-11-08T05:35:32.055259750+00:00"
id: "atelier-4k28"
issue_type: "task"
labels:
- "tooling"
workflow_branch:
  branch_base: master
  work_branch: atelier-4k28
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-08T06:57:50.100257596+00:00"
status: "done"
title: "Smoke-test a new atelier binary against every local Atelier repository before installing it"
updated_at: "2026-11-08T06:57:50.100257596+00:00"
---

## Goal

Review of atelier changes caught cross-repository regressions by hand (atelier-2ygf, atelier-2j35, atelier-8ljp): the new binary refused astromech's per-child reviews and other repositories' records. A script should catch these before review and before install.

## Done when

- [x] A script in this repository builds the current tree's binary, makes scratch clones of each local Atelier repository (discovered from a list in the script or a directory scan, including atelier, astromech, life-on-rails, cosplay-studio, demiurge and worldcrafter), and runs read-only commands with both the new and the installed binary: at least `check`, `doctor`, `status`, `work missions` and `review request` for an open item where one exists.
- [x] It prints a per-repository summary and exits non-zero when the new binary errors or refuses where the installed one does not. It never writes to the repositories it clones; build output in this repository's `target/` is fine.
- [x] Run once on the current base: the Report shows its output.
- [x] This repository's AGENTS.md says to run it before requesting review of a change to atelier's commands or records, and before installing a new binary.

## Scope

A script and the doc line. No change to atelier's commands.

## Context

From the 2026-11-08 review audit.

## Notes

- 2026-11-08 [note] Handoff: implemented on atelier-4k28 (draft PR #47). All Done when lines ticked; proof and the base run are in the Report. Independent review outstanding.

## Report

All four Done when lines hold. `scripts/smoke-repos.sh` builds `target/debug/atelier`, finds every directory under the scan root (default: the parent of this repository's main checkout) with a `.atelier` folder, keeps one per git repository, and mirror-clones each twice into a `mktemp` scratch directory, one clone per binary, with the push URL disabled. In each clone it runs `check`, `doctor`, `status`, `work missions`, and `review request <id>` on every local branch that an unfinished item owns (checked out in the clone). It prints one line per command with both exit codes and exits 1 when the new binary fails where the installed one passes. Proof:

- Discovery: the run below covers atelier, astromech, life-on-rails, cosplay-studio and demiurge. worldcrafter is a symlink to demiurge (`worldcrafter -> demiurge`, one `.git`), so it is that repository and is reported as "also linked as worldcrafter". The other `.atelier` directories under `~/projects` are worktrees and were skipped.
- Non-zero on refusal: a scratch build whose `check` bails when `.atelier/config.toml` mentions astromech (built to `target/smoke-broken`, source reverted with `git checkout`, never committed) gave `check installed 0 new 1 REGRESSION`, printed the new binary's error, and exited 1 (`scripts/smoke-repos.sh --new target/smoke-broken/debug/atelier ../astromech`).
- Read-only: the script snapshots each source repository's refs, HEAD and `git status` before and after, with `GIT_OPTIONAL_LOCKS=0` so reading does not refresh an index. astromech, cosplay-studio, demiurge and life-on-rails were unchanged. atelier changed in both full runs; the reflog shows other agents committing and pushing `atelier-8bj8` (01:10:20) and `atelier-edaj` (01:12:43 to 01:13:04) during the runs, and the script has no command that writes to a source.
- AGENTS.md's Command References now says to run it before requesting review of a change to atelier's commands or records, and before installing a new binary.
- `cargo fmt -- --check` and `git diff --check` pass.

Run on the current base (branch tip a259e464 is master 1c3b9145 plus this script, with no Rust change), against installed 0.3.1+e03d6b71:

```
Building the new binary: cargo build --locked -p atelier-cli
New:       /home/supadava/projects/atelier-wt-4k28/target/debug/atelier (atelier 0.3.1+a259e464)
Installed: /home/supadava/.cargo/bin/atelier (atelier 0.3.1+e03d6b71)

== astromech @ 5b8e3b07 (1 uncommitted paths not cloned)
  check                                    installed   0  new   0  same
  doctor                                   installed   0  new   0  same
  status                                   installed   0  new   0  same
  work missions                            installed   0  new   0  same
  review request astromech-y6d6            installed   1  new   1  same
  review request astromech-v6r3            installed   1  new   1  same
  ok: 6 commands, no regression

== atelier @ b81f650d
  check                                    installed   0  new   0  same
  doctor                                   installed   0  new   0  same
  status                                   installed   0  new   0  same
  work missions                            installed   0  new   0  same
  review request atelier-4k28              installed   1  new   1  same
  review request atelier-8bj8              installed   0  new   0  same
  review request atelier-edaj              installed   0  new   0  same
  review request atelier-pmib              installed   0  new   0  same
  ok: 8 commands, no regression
  warning: /home/supadava/projects/atelier changed during the run (refs, HEAD or working tree); was someone working there?

== cosplay-studio @ 1034f62
  check                                    installed   0  new   0  same
  doctor                                   installed   0  new   0  same
  status                                   installed   0  new   0  same
  work missions                            installed   0  new   0  same
  ok: 4 commands, no regression

== demiurge @ c8a096a (also linked as worldcrafter)
  check                                    installed   0  new   0  same
  doctor                                   installed   0  new   0  same
  status                                   installed   0  new   0  same
  work missions                            installed   0  new   0  same
  review request worldcrafter-efis         installed   1  new   1  same
  ok: 5 commands, no regression

== life-on-rails @ 6a5db78
  check                                    installed   0  new   0  same
  doctor                                   installed   0  new   0  same
  status                                   installed   0  new   0  same
  work missions                            installed   0  new   0  same
  review request lor-clgw                  installed   1  new   1  same
  review request lor-f3vn                  installed   1  new   1  same
  review request lor-sfh9                  installed   1  new   1  same
  ok: 7 commands, no regression

smoke-repos: no regression across 5 repositories.
exit 0
```

The `review request` lines that exit 1 on both sides are refusals for unticked Done when lines or an empty Report, as expected for work in progress.

### Decisions

- Exit codes only decide a regression. Output text differs legitimately between versions, so comparing it would be noise.
- Each binary gets its own clone, so `review request`'s branch checkouts and anything one binary writes cannot affect the other's results.
- Uncommitted changes in a source repository are not cloned; the summary says how many paths were left out (astromech had one).

### Open items

- shellcheck is not installed on this host, so the script has not been linted.
- The read-only snapshot only warns; it cannot tell another agent's commit from a write by the script. The reflog was checked by hand for this run.
