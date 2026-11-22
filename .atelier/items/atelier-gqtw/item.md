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
2. **Automatic cleanup of merged branches and visibility into stale worktrees:** Nothing deletes merged branches on `atelier merge` or warns when merged branches and detached review worktrees pile up (this clone has 285 merged local branches, 266 merged remote branches, and 238 worktrees).

## Done when

- [ ] `atelier drop <started-item> --reason "<why>"` works directly on a `backlog/*` branch, writing `status: dropped` and `Dropped: <reason>` to `.atelier/items/<id>/item.md` (copying front matter plus `Goal`, `Done when`, and `Scope` from the branch record when the item only exists on its branch via `start --new`).
- [ ] When dropping a started item whose unmerged branch added other new `.atelier/items/<other>/item.md` records not on `base`, `drop` names each of them in its output so newly filed items (including under `atelier-dh7a` / `atelier-u6oy`) are not silently lost.
- [ ] Running `atelier drop <started-item> --reason "<why>"` on the item's own branch continues to work as before.
- [ ] When `atelier merge` merges a backlog branch that sets a started item to `dropped` and a forge is configured, it best-effort closes any open PR for that item's branch (warning on failure without failing the merge).
- [ ] After `atelier merge` succeeds (for an item, mission, or backlog branch):
  - With a forge configured, it best-effort deletes the remote branch (`git push <remote> --delete <branch>`, not `gh pr merge --delete-branch`, so linked worktrees never fail trying to switch to `base`), warning rather than failing if the remote already deleted it or refuses.
  - Locally, it deletes the merged local branch (`git branch -d <branch>`) when `git::checkout_of` returns `None`, and otherwise prints a one-line hint to remove the worktree and branch.
- [ ] `atelier status` (away from an item branch) prints a concise notice with a safe cleanup hint when merged local named branches (`>= 10`) or detached worktrees (`>= 5`) accumulate; it deletes nothing automatically.
- [ ] `docs/product.md` and `docs/design.md` reflect the single-branch drop and merge cleanup; CLI tests and `evals/agent-factory/test.sh` pass.

## Scope

In: `crates/atelier-core/src/{plan.rs,git.rs,forge.rs,say.rs}`, `crates/atelier-cli/src/{plan.rs,merge.rs,commands.rs,output.rs}`, `docs/product.md`, `docs/design.md`, and tests.

Out: deleting unmerged branches or automatically removing worktrees, changing `atelier-dh7a`'s `open`-only status model, and `t3-extension/`.
