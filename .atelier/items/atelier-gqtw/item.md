---
schema: atelier.item
schema_version: 3
id: atelier-gqtw
title: Drop started items in one backlog branch and clean up merged branches and stale worktrees
status: building
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

- [ ] `atelier drop <started-item> --reason "<why>"` works directly on a `backlog/*` branch, writing `status: dropped` and `Dropped: <reason>` to `.atelier/items/<id>/item.md` (copying front matter plus `Goal`, `Done when`, and `Scope` from the branch record when the item only exists on its branch via `start --new`).
- [ ] When dropping a started item whose unmerged branch added other new `.atelier/items/<other>/item.md` records not on `base`, `drop` names each of them in its output so newly filed items (including under `atelier-dh7a` / `atelier-u6oy`) are not silently lost.
- [ ] Running `atelier drop <started-item> --reason "<why>"` on the item's own branch continues to work as before.
- [ ] When `atelier merge` merges a backlog branch that sets a started item to `dropped` and a forge is configured, it best-effort closes any open PR for that item's branch (warning on failure without failing the merge).
- [ ] Claim safety when branches are deleted (and on single-branch drops):
  - When a remote is configured, `atelier start <id>` (both fresh start and take-over) fetches `<remote>/<base>` first and judges against that freshly fetched base, refusing an item that is `done` or `dropped` there (or failing cleanly if the remote is unreachable).
  - After the push succeeds, it re-checks `<remote>/<base>` to close the fetch-to-push race window if `<remote>/<base>` moved and `<id>` became `done` or `dropped`:
    - On a **fresh `start` claim** (`begin`, before switching the checkout), delete only the local branch (`git::delete_branch(&root, branch, &commit)`), do not switch to it, refuse, and warn about the pushed remote branch (which `Located::record` rule 1 ignores because `done`/`dropped` on `base` wins; never delete a remote branch outside `atelier merge`).
    - On a **take-over** (`take_over`), delete **neither** the remote branch nor the local branch (the branch holds the previous implementer's unmerged work or a late push); refuse, name the branch, and leave the checkout with a valid `HEAD`.
  - Tests prove: (a) a stale clone cannot restart or take over a merged or dropped item whose branch was deleted; (b) a fresh claim racing a merge refuses without switching; and (c) a take-over racing a drop refuses while leaving the earlier implementer's work on `origin` and leaving the checkout on a valid `HEAD`.
  - When a remote is configured, `atelier submit` also checks `<remote>/<base>` (after fetching it) and refuses if the item is already `dropped` or `done` there so a stale clone does not open a review for an item dropped on a backlog branch.
  - `atelier merge` deletes **neither** the remote branch nor the local branch unless this clone's `base_tip` contains `<head>` (so if `catch_up`'s fetch fails, both refs stay intact and `status` does not re-offer the merged item to `start`). A test proves a failed `catch_up` leaves both refs untouched.
- [ ] After `atelier merge` succeeds (for an item, mission, or backlog branch) and `base_tip` contains `<head>`:
  - **Remote:** With a forge configured, it best-effort deletes the merged remote branch using `git push <remote> --force-with-lease=refs/heads/<branch>:<head> --delete <branch>` (never plain `--delete` and never `gh pr merge --delete-branch`), warning rather than failing if the remote already deleted it or refuses. A test proves a late push to the remote branch causes deletion to be refused cleanly without losing the late commit.
  - **Local:** When the current checkout is on `<branch>` at `<head>` with no uncommitted changes (`uncommitted` empty), it detaches `HEAD` at `<head>` (`git switch --detach <head>`), deletes `<branch>` at that exact commit with `git::delete_branch(&root, &branch, &head)`, and prints a one-line pointer to `atelier start --backlog <slug>` / `atelier start <id>` for further changes. If uncommitted changes were skipped via `ATELIER_SKIP`, it leaves `<branch>` attached and prints a one-line hint. Tests prove the local branch is deleted and the worktree lands on detached `HEAD` at `<head>`.
- [ ] Detached `HEAD` safety after merge:
  - Planning commands (`new`, `set`, `ready`, `drop`, `close`, `lead`, `settle`) refuse to run on a detached `HEAD` (telling the caller to run `atelier start --backlog <slug>` or `atelier start <id>`), while `atelier review` and `atelier verdict` continue to work in detached reviewer worktrees (`review.rs::on_base` only refuses the base checkout).
  - If `HEAD` is detached and no branch, remote-tracking branch, or tag contains `HEAD` (`git for-each-ref --contains HEAD refs/heads refs/remotes refs/tags` is empty — explicitly excluding other namespaces such as `refs/t3/*` checkpoint refs), `atelier status` reports the unattached commit(s) and `atelier start` (whenever it would switch away) refuses without `ATELIER_SKIP` so no commit is silently orphaned. A test covers the post-merge sequence with a ref under `refs/t3/` pointing at or past `HEAD` (`merge` -> `atelier new` refused -> manual commit on detached `HEAD` + `refs/t3/` checkpoint ref -> `status` still reports it and `start --backlog next` still refuses).
- [ ] Desk PR links stay intact after branch deletion: in `crates/atelier-desk/src/server.rs`, when `page.row.branch` is `None` on a `done`/`dropped` item (or `page.mission.branch` is `None` on an agreed/done/stopped mission), look up `page.pull_request` by the record's convention branch name (`branch_of(id)`) so `pull_request` stays populated in the Desk and `t3-extension/`. A Rust test proves a `done` item with neither local nor remote branch still gets its `pull_request`.
- [ ] `atelier status` (away from an item branch) prints a **read-only** one-line summary when all merged local branches (`>= 20`) or detached worktrees (`>= 10`) accumulate, naming only inspection commands (`git branch --merged <base>` and `git worktree list`) and never suggesting a destructive deletion command that could remove live reviewer worktrees.
- [ ] `AGENTS.md` (narrowing line 71's remote-branch deletion rule to allow `atelier merge`'s lease-checked deletion while still forbidding manual deletion and force-pushing `master`), `skills/agent-factory/roles/lead.md` (replacing the two-branch drop instructions), `docs/product.md`, and `docs/design.md` (updating the `start` row, reversing the two-branch drop decision, updating `drop`'s stale-clone paragraph, and updating "The branch is kept") are updated.
- [ ] `cargo nextest run -p atelier-core -p atelier-cli -p atelier-desk --locked`, `evals/agent-factory/test.sh` (updating eval fixtures that assumed the merged local branch remained after `merge`), `desk/` tests (`pnpm test`), and `node t3-extension/run-tests.mjs` pass.

## Scope

In: `crates/atelier-core/src/{plan.rs,git.rs,forge.rs,say.rs}`, `crates/atelier-cli/src/{plan.rs,merge.rs,commands.rs,review.rs,output.rs}`, `crates/atelier-desk/src/{server.rs,read.rs}`, `crates/atelier-desk/tests/`, `evals/agent-factory/`, `AGENTS.md`, `skills/agent-factory/roles/lead.md`, `docs/product.md`, `docs/design.md`, and tests.

Out: deleting unmerged branches or automatically removing worktrees, changing `atelier-dh7a`'s `open`-only status model, and `t3-extension/`.
