---
schema: atelier.item
schema_version: 3
id: atelier-gqtw
title: Drop started items in one backlog branch and clean up merged branches and stale worktrees
status: ready
mission: atelier-ww7w
labels: []
priority: medium
blocked_by: []
implementers: []
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
- [ ] After `atelier merge` succeeds (for an item, mission, or backlog branch):
  - **Remote:** With a forge configured, it best-effort deletes the merged remote branch using `git push <remote> --force-with-lease=refs/heads/<branch>:<head> --delete <branch>` (never plain `--delete` and never `gh pr merge --delete-branch`), warning rather than failing if the remote already deleted it or refuses. A test proves a late push to the remote branch causes deletion to be refused cleanly without losing the late commit.
  - **Local:** When the current checkout is on `<branch>` at `<head>` with no uncommitted changes (`uncommitted` empty), it detaches `HEAD` at `<head>` (`git switch --detach <head>`) and deletes `<branch>` at that exact commit with `git::delete_branch(&root, &branch, &head)`. If uncommitted changes were skipped via `ATELIER_SKIP`, it leaves `<branch>` attached and prints a one-line hint. Tests prove the local branch is actually deleted after `merge` and the worktree lands on detached `HEAD` at `<head>`.
- [ ] `atelier status` (away from an item branch) prints a **read-only** one-line summary when all merged local branches (`>= 20`) or detached worktrees (`>= 10`) accumulate, naming only inspection commands (`git branch --merged <base>` and `git worktree list`) and never suggesting a destructive deletion command that could remove live reviewer worktrees.
- [ ] `AGENTS.md` (narrowing line 71's remote-branch deletion rule to allow `atelier merge`'s lease-checked deletion while still forbidding manual deletion and force-pushing `master`), `skills/agent-factory/roles/lead.md` (replacing the two-branch drop instructions), `docs/product.md`, and `docs/design.md` (reversing the two-branch drop decision and updating "The branch is kept") are updated.
- [ ] CLI tests and `evals/agent-factory/test.sh` pass.

## Scope

In: `crates/atelier-core/src/{plan.rs,git.rs,forge.rs,say.rs}`, `crates/atelier-cli/src/{plan.rs,merge.rs,commands.rs,output.rs}`, `AGENTS.md`, `skills/agent-factory/roles/lead.md`, `docs/product.md`, `docs/design.md`, and tests.

Out: deleting unmerged branches or automatically removing worktrees, changing `atelier-dh7a`'s `open`-only status model, and `t3-extension/`.
