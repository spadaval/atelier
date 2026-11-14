---
schema: atelier.item
schema_version: 3
id: atelier-t5am
title: Finish a merge through the forge's pull request, so Atelier works on a protected base
status: building
mission: atelier-to83
labels:
- redesign
priority: medium
blocked_by:
- atelier-ob9u
- atelier-2vdq
implementers:
- thread:delegated-task:command%3Amcp%3A93b1b51f-2095-4d1f-af23-677aa6ba570d%3Adelegate-task%3A7c80863b-f4e9-4f60-b975-1de8a1234a68
ready:
  by: 'sumeet, on 2026-11-13: execute atelier-to83, relayed by the planner thread'
  at: 2026-11-14T00:56:53.839951546Z
trivial: null
caused_by: null
created_at: 2026-11-13T06:14:32.695875418Z
---

## Goal

Make `atelier merge` work where the base is protected. With a forge
configured, `merge` builds the merge commit itself and pushes the base, which
a protected branch refuses. After this item it finishes through the forge's
merge of the pull request, and the pre-push hook's rule becomes the plain one:
the base receives only merges.

## Done when

- [x] With a forge configured, `merge` does its checks, the plan-review gate and the record-only rule of atelier-2vdq and atelier-ob9u among them, and then has the forge merge the pull request, as a merge commit. It never pushes the base. With no forge it moves the base locally as today.
- [x] `done` is recorded as `docs/design.md` settled, and after a merge by the forge `atelier show <id>` in a fresh clone says done.
- [x] The merge checks in `config.toml` still run on the merged tree before the forge is asked, and the Report says what can change between that run and the forge's merge and what catches it.
- [x] Where the forge will not merge yet because a required review or check is missing, `merge` stops with exit status 3 and says which, and run again later it merges.
- [x] A test remote whose hook refuses every push to the base lets the full item loop pass: `start`, `submit`, `review`, `verdict`, `merge`. The forge's side is the existing contract-test double.
- [x] The pre-push hook refuses any push that moves the base, and says to use `atelier merge`. `ATELIER_SKIP` passes it with a reason, as now.
- [x] The merge lock and the checks that the branch and base have not moved are kept, with their tests, or the Report says which the forge now makes unnecessary and why.

## Scope

In: `crates/atelier-cli` (`merge.rs`, `forge.rs`, `check.rs`), their tests, and the Merge, Hooks and Forge sections of `docs/design.md` where the code differs from what atelier-ogpg wrote.

Out: turning on branch protection anywhere; any forge but GitHub; a merge queue; squash or rebase merges.

## Context

`docs/design.md`, Merge, steps 5 to 8: `merge` writes the merge commit with the item set `done`, moves the base and pushes it. The trailer `Atelier-Merge: <id>` is how the pre-push hook knows an Atelier merge today.

## Report

With a forge configured, `atelier merge` now finishes through `gh pr merge <n> --merge --match-head-commit <head>` and never pushes the base, so it works where the base is protected. Without a forge it moves the local base as before, and no longer pushes anything.

**What changed.**

- `merge.rs`: the merge commit no longer rewrites the item; it is the merged tree as the branch brings it, with the same message and `Atelier-Merge:` trailer. The checks run on it as before, and `atelier check` on the merged tree now runs for an item branch too, as the design asks. With a forge that commit is discarded, and `finish` asks the forge to merge the head checked with its subject and body. When `gh` refuses, `gh pr view --json state,reviewDecision,mergeStateStatus,headRefOid` says why: `MERGED` counts as merged; `BLOCKED`, `REVIEW_REQUIRED`, `CHANGES_REQUESTED` and `BEHIND` exit 3 naming both fields; a moved head or `DIRTY` refuses. After the merge it fetches the base and fast-forwards the local base (checkout or ref), saying what it could not bring up to date.
- Step 3 now pushes the branch whenever the forge's copy is not the head, because the forge merges the pull request's head. Before, it skipped the push when a CI run on the forge's copy counted, so the reviewer's verdict commit never reached the forge. A run on the submit commit still counts for the head.
- `merge --trivial` commits the reason in `item.md` alone on the branch, once the act's conditions hold, before the wait. Run again with the same reason it commits nothing.
- `done` is recorded as the design settled: nothing writes it at a merge. The reader (`atelier-app` `repo.rs`, `on_base`) reads a base copy in a started status as `done`, and so do `locate_planned` and `list --deferred` unless the checkout is that item's own branch. `log` finds a merge where the base copy is started or done. This touches `atelier-app` and `views.rs`, outside the Scope's list of files, because the Done when line on `done` needs it. Before relying on it I scanned the fetched bases of atelier, astromech, demiurge, life-on-rails and cosplay-studio: none holds an item in a started status, so no existing record changes meaning.
- An item branch that brings another item to the base in a started status is now refused, as records branches already were. Without it, that other item would read as done.
- `check.rs` pre-push: with a forge, any push that moves the base is refused, naming `atelier merge`. Without one, each new first-parent commit must be a two-parent commit with the `Atelier-Merge:` trailer. The old exceptions (planning-only commits, the "sets an item done" rule) and the record reading went. `ATELIER_SKIP` passes both rules.
- `docs/design.md`: Merge steps 1, 7 and 8, the gap paragraph, the pre-push notes and The forge now say what the code does.

**Between the checks and the forge's merge.** The checks run on this clone's local base, merged with the head. Three things can differ when the forge merges:

1. The forge's base gains a merge from another clone. No lock here sees it. `--match-head-commit` fixes only the branch side. A textual conflict makes the forge refuse (`DIRTY`, a refusal). A semantic clash between two clean merges is caught by CI on the base after the merge, or, where the protection requires branches to be up to date, by the forge refusing with `BEHIND`, which `merge` reports with exit 3 and the `git merge origin/<base>` that catches up. The test `a_base_that_gains_a_commit_on_the_forge_during_the_wait_is_merged_onto` shows the forge merging onto a base this clone's checks never saw.
2. This clone's base is behind the forge's because it has not fetched. That is the same gap and is caught the same ways. The design's reader rule, that the base tip is the remote-tracking ref where a forge is configured, is not implemented (the reader still reads the local base). `merge` mitigates this by fetching and fast-forwarding the local base after every merge it makes.
3. The branch moves on the forge, for example a push from another clone. `--match-head-commit` makes the forge refuse, and `merge` refuses as an input that moved.

**The lock and the moved checks.** I kept both, with their tests. The lock still orders two merges in one clone. The confirmation that the branch tip, the base tip and the work checkout are unchanged still stops a commit made during the checks from going out unchecked. With a forge, the base's checkout is no longer judged or merged into, because the forge moves the base; it is only fast-forwarded afterwards where it is clean.

**Tests.** `on_a_protected_base_the_item_loop_merges_through_the_forge_and_a_fresh_clone_reads_it_done` runs start, submit, review, verdict and merge against a bare remote whose `pre-receive` refuses every push to `main`. The fake `gh` gained `pr merge` (a merge commit in the bare repository, past its hooks, as the forge makes it) and `pr view`. The test checks that no push to `main` was attempted, that the forge's commit has the trailer and the approved head as its second parent, that `show` in a fresh clone says done, that a rerun says nothing is left to do, and that a plain push of `main` is refused by the pre-push hook. `where_the_forge_waits_on_a_required_review_or_check_merge_exits_not_yet_and_merges_when_run_again` covers exit 3 for a review, a check and `BEHIND`; refusals for `DIRTY` and a moved head; and the merge once nothing is missing. Hooks tests cover both pre-push rules and the skip. `cargo nextest run --workspace`: 349 passed. `cargo clippy --all-targets -- -D warnings` and `cargo fmt -- --check` are clean.

**Left for others.** `AGENTS.md` still says `merge` pushes `master` and describes the old pre-push rule; it is outside this item's Scope (atelier-j5lr owns that file). Once this merges, `cargo install --path crates/atelier-cli --locked` changes the pre-push rule for every repository on the host: a forge-configured repo's base can then be moved only by `atelier merge`. GitHub must allow merge commits on each forge repo.
