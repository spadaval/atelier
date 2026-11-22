---
schema: atelier.item
schema_version: 3
id: atelier-gqtw
title: Drop started items in one backlog branch and clean up merged branches and stale worktrees
status: changes_requested
mission: atelier-ww7w
labels: []
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A337c6277-e094-40d2-a1c8-de83cd3a5839%3Adelegate-task%3Ada024f49-25bc-4592-abd3-1c35332545ef
ready:
  by: Sumeet, in conversation on 2026-11-22
  at: 2026-11-22T05:33:31.467134267Z
trivial: null
caused_by: null
created_at: 2026-11-22T05:30:53.682169742Z
---

## Goal

Cut branch ceremony and ref/worktree sprawl in two places:
1. **Single-branch drop for started items:** Today dropping a started item requires dropping, committing, and pushing on `<id>`, then creating a *second* `backlog/drop-<id>` branch to carry the drop to `base`. Because `Located::record` rule 1 already makes a `dropped` record on `base` win over any unmerged branch, a single `backlog/*` branch drop is enough.
2. **Safe cleanup of merged branches on `atelier merge` and read-only hygiene visibility in `status`:** Nothing deletes merged branches on `atelier merge` or reports when merged branches and detached worktrees pile up (this clone holds 285 merged local branches including `t3/*`, 266 merged remote branches, and 238 worktrees). Sumeet explicitly approved deleting merged remote branches on `atelier merge` in conversation on 2026-11-22.

## Done when

- [x] `atelier drop <started-item> --reason "<why>"` works directly on a `backlog/*` branch, writing `status: dropped` and `Dropped: <reason>` to `.atelier/items/<id>/item.md` (copying front matter plus `Goal`, `Done when`, and `Scope` from the branch record when the item only exists on its branch via `start --new`).
- [x] When dropping a started item whose unmerged branch added other new `.atelier/items/<other>/item.md` records not on `base`, `drop` names each of them in its output so newly filed items (including under `atelier-dh7a` / `atelier-u6oy`) are not silently lost.
- [x] Running `atelier drop <started-item> --reason "<why>"` on the item's own branch continues to work as before.
- [x] When `atelier merge` merges a backlog branch that sets a started item to `dropped` and a forge is configured, it best-effort closes any open PR for that item's branch (warning on failure without failing the merge).
- [x] Claim safety when branches are deleted (and on single-branch drops):
  - When a remote is configured, `atelier start <id>` (both fresh start and take-over) fetches `<remote>/<base>` first and judges against that freshly fetched base, refusing an item that is `done` or `dropped` there (or failing cleanly if the remote is unreachable).
  - After the push succeeds, it re-checks `<remote>/<base>` to close the fetch-to-push race window if `<remote>/<base>` moved and `<id>` became `done` or `dropped`:
    - On a **fresh `start` claim** (`begin`, before switching the checkout), delete only the local branch (`git::delete_branch(&root, branch, &commit)`), do not switch to it, refuse, and warn about the pushed remote branch (which `Located::record` rule 1 ignores because `done`/`dropped` on `base` wins; never delete a remote branch outside `atelier merge`).
    - On a **take-over** (`take_over`), delete **neither** the remote branch nor the local branch (the branch holds the previous implementer's unmerged work or a late push); refuse, name the branch, and leave the checkout with a valid `HEAD`.
  - Tests prove: (a) a stale clone cannot restart or take over a merged or dropped item whose branch was deleted; (b) a fresh claim racing a merge refuses without switching; and (c) a take-over racing a drop refuses while leaving the earlier implementer's work on `origin` and leaving the checkout on a valid `HEAD`.
  - When a remote is configured, `atelier submit` also checks `<remote>/<base>` (after fetching it) and refuses if the item is already `dropped` or `done` there so a stale clone does not open a review for an item dropped on a backlog branch.
  - `atelier merge` deletes **neither** the remote branch nor the local branch unless this clone's `base_tip` contains `<head>` (so if `catch_up`'s fetch fails, both refs stay intact and `status` does not re-offer the merged item to `start`). A test proves a failed `catch_up` leaves both refs untouched.
- [x] After `atelier merge` succeeds (for an item, mission, or backlog branch) and `base_tip` contains `<head>`:
  - **Remote:** With a forge configured, it best-effort deletes the merged remote branch using `git push <remote> --force-with-lease=refs/heads/<branch>:<head> --delete <branch>` (never plain `--delete` and never `gh pr merge --delete-branch`), warning rather than failing if the remote already deleted it or refuses. A test proves a late push to the remote branch causes deletion to be refused cleanly without losing the late commit.
  - **Local:** When the current checkout is on `<branch>` at `<head>` with no uncommitted changes (`uncommitted` empty), it detaches `HEAD` at `<head>` (`git switch --detach <head>`), deletes `<branch>` at that exact commit with `git::delete_branch(&root, &branch, &head)`, and prints a one-line pointer to `atelier start --backlog <slug>` / `atelier start <id>` for further changes. If uncommitted changes were skipped via `ATELIER_SKIP`, it leaves `<branch>` attached and prints a one-line hint. Tests prove the local branch is deleted and the worktree lands on detached `HEAD` at `<head>`.
- [x] Detached `HEAD` safety after merge:
  - Planning commands (`new`, `set`, `ready`, `drop`, `close`, `lead`, `settle`) refuse to run on a detached `HEAD` (telling the caller to run `atelier start --backlog <slug>` or `atelier start <id>`), while `atelier review` and `atelier verdict` continue to work in detached reviewer worktrees (`review.rs::on_base` only refuses the base checkout).
  - If `HEAD` is detached and no branch, remote-tracking branch, or tag contains `HEAD` (`git for-each-ref --contains HEAD refs/heads refs/remotes refs/tags` is empty — explicitly excluding other namespaces such as `refs/t3/*` checkpoint refs), `atelier status` reports the unattached commit(s) and `atelier start` (whenever it would switch away) refuses without `ATELIER_SKIP` so no commit is silently orphaned. A test covers the post-merge sequence with a ref under `refs/t3/` pointing at or past `HEAD` (`merge` -> `atelier new` refused -> manual commit on detached `HEAD` + `refs/t3/` checkpoint ref -> `status` still reports it and `start --backlog next` still refuses).
- [x] Desk PR links stay intact after branch deletion: in `crates/atelier-desk/src/server.rs`, when `page.row.branch` is `None` on a `done`/`dropped` item (or `page.mission.branch` is `None` on an agreed/done/stopped mission), look up `page.pull_request` by the record's convention branch name (`branch_of(id)`) so `pull_request` stays populated in the Desk and `t3-extension/`. A Rust test proves a `done` item with neither local nor remote branch still gets its `pull_request`.
- [x] `atelier status` (away from an item branch) prints a **read-only** one-line summary when all merged local branches (`>= 20`) or detached worktrees (`>= 10`) accumulate, naming only inspection commands (`git branch --merged <base>` and `git worktree list`) and never suggesting a destructive deletion command that could remove live reviewer worktrees.
- [x] `AGENTS.md` (narrowing line 71's remote-branch deletion rule to allow `atelier merge`'s lease-checked deletion while still forbidding manual deletion and force-pushing `master`), `skills/agent-factory/roles/lead.md` (replacing the two-branch drop instructions), `docs/product.md`, and `docs/design.md` (updating the `start` row, reversing the two-branch drop decision, updating `drop`'s stale-clone paragraph, and updating "The branch is kept") are updated.
- [x] `cargo nextest run -p atelier-core -p atelier-cli -p atelier-desk --locked`, `evals/agent-factory/test.sh` (updating eval fixtures that assumed the merged local branch remained after `merge`), `desk/` tests (`pnpm test`), and `node t3-extension/run-tests.mjs` pass.

## Scope

In: `crates/atelier-core/src/{plan.rs,git.rs,forge.rs,say.rs}`, `crates/atelier-cli/src/{plan.rs,merge.rs,commands.rs,review.rs,output.rs}`, `crates/atelier-desk/src/{server.rs,read.rs}`, `crates/atelier-desk/tests/`, `evals/agent-factory/`, `AGENTS.md`, `skills/agent-factory/roles/lead.md`, `docs/product.md`, `docs/design.md`, and tests.

Out: deleting unmerged branches or automatically removing worktrees, changing `atelier-dh7a`'s `open`-only status model, and `t3-extension/`.

## Report

A started item is now dropped on one backlog branch, and `atelier merge`
deletes the branch it merged, so merged branches stop piling up.

### What changed

- `drop` on a `backlog/*` branch ends a started item and names the items only
  its branch filed, each with the `git checkout` that brings it over. A drop
  on the item's own branch works as before. A backlog merge closes the pull
  request of each item it drops.
- `merge` deletes the remote branch under a lease at the merged head and the
  local one at that commit, leaving the checkout detached there. It deletes
  neither until the base as fetched holds the head.
- `start <id>` fetches the base before judging and again after its push;
  `submit` fetches it before judging. A fresh claim that loses deletes only
  its local branch, and a take-over that loses deletes nothing.
- Planning commands refuse a detached HEAD. `status` lists, and `start`
  refuses to leave, commits no branch, remote-tracking branch or tag holds.
- `status` gives one read-only line when merged branches or detached
  worktrees pile up. On this clone it counts 307 and 72.
- The Desk finds the pull request of an ended item or agreed mission by the
  branch's conventional name.

### Choices the item left open

- `submit` warns and goes on when it cannot fetch the base, since its push
  fails too and `merge` judges again. `start` refuses, as the item says.
- A failed fetch of the base still counts when the tracking ref already holds
  the remote's tip. Two threads starting at once in one clone race on that
  ref. `merge`'s catch-up now uses the same fetch, which also leaves
  `FETCH_HEAD`, the "last fetched" time, alone.
- `drop` refuses an item the base reads done. Without its branch, a stale
  backlog checkout could otherwise drop merged work.
- `merge` run again from a still-attached merged branch finishes the cleanup.
  It keeps both refs where that checkout has uncommitted changes, so a
  `merge` run before a first commit does not delete a branch in use.
- The merge asks the forge about every item it drops, since one started in
  another clone may have no branch here.

### Behaviour that changes for users

- A second `atelier merge` after a clean merge exits 1 ("runs on an item's
  branch"), where it said "already merged", because the branch is gone.
- `start` on an item needs the remote to answer, also for its own holder.
- The CLI test fixture now works on a branch named `scratch`, since planning
  on a detached HEAD is refused. That is most of the edits to existing tests.

### After atelier-zc9b merged

Round 1's first reviewer was cancelled on the lead's word when master
(`f2449baa8`) began to conflict with this branch. It had recorded no finding,
and its review file, the empty template pinned to `7588ec64b`, was never
committed and was taken out, so the next reviewer opens round 1 on the new
head.

- The five conflicts are resolved by hand in `3883bf5b1`. Both items'
  behaviour stays: `status` keeps zc9b's endings and gains this item's two
  lines, `submit` names the checkout and still warns on a failed fetch, and
  the pre-push way out works as printed, since `start --new` does not fetch.
- The test that a failed catch-up never re-offers a merged item would have
  passed vacuously without zc9b's removed `Then:` line. It now reads the
  lists. The one-branch drop is also tested over a branch with a malformed
  Responses line.
- `10b752de9` fixes a bug that predates both items and made
  `evals/agent-factory/test.sh` fail at random (3 of 160 runs of the test on
  master's binary): the harness lost a hook's skip when the second ticked
  over between Git's timestamp and its pre-commit hook. A new test fails
  without the fix, and the flaky test then passed 220 runs of 220.

### Verification

Run on `10b752de9`, with `origin/master` at `f2449baa8` merged in:

- `cargo fmt -- --check`, `git diff --check` and
  `cargo clippy --all-targets --locked -- -D warnings`: clean.
- `cargo nextest run --locked`, the whole workspace: 456 passed, 1 skipped.
  The new behaviour is in `crates/atelier-cli/tests/cleanup.rs`,
  and the Desk's in `crates/atelier-desk/tests/actions.rs`.
- `evals/agent-factory/test.sh target/debug/atelier`: all pass, with two
  fixtures changed for the deleted branch and the harness fix above.
- `pnpm test` in `desk/`: 57 passed.
- `node t3-extension/run-tests.mjs`: all 7 test files passed, unmodified. The
  inotify ENOSPC expected on this host did not occur.
- `atelier check`: every record valid.
- `target/debug/atelier status` in this clone and in a detached worktree of it.

Not checked against GitHub: the lease-checked deletion and the pull request
close ran only against a local bare remote and a fake `gh`. This item merges
with the older installed binary, so the first merge after it is installed is
the first real run.

### Left for the lead

- `atelier-xiiw` will bite more often: `start` and `submit` now move
  `origin/<base>`, and the pre-commit hook refuses a catch-up merge whose
  `MERGE_HEAD` is no longer that tip.
- `scripts/cutover/agents/atelier.md` still has the old blanket ban on
  deleting remote branches. It is outside this item's Scope.
- A `forge.remote` given as a URL has no remote-tracking base, so `start`
  would refuse and `merge` would keep its branches. Every configured
  repository on this host names `origin`.
- The 307 branches already merged stay until someone removes them.
