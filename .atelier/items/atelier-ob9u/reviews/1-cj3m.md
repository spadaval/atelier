---
schema: atelier.review
schema_version: 3
round: 1
commit: 86362072797f6ea24fdfe6e73b838ff69327f563
reviewers:
- thread:delegated-task:command%3Amcp%3Aa410b15e-b763-48ac-8edb-eb330175253e%3Adelegate-task%3A508eaeaf-8346-4974-b9dd-f9932913e58b
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-13T23:15:30.608058973Z
---

## Summary

Complexity: high. This changes the Git contract across record commands,
branch naming, remote claims, record-only merges, hooks and cross-clone drops.
Value: high. The outcome requires records to reach the base through branches,
and the current commands and other clones depend on that contract.
Risk of merging it: high. A takeover can silently change the HEAD of another
checkout without updating its index, leaving an unintended reversal staged.

The value warrants the scope; I found no smaller change that delivers most
of the required branch model. The four findings below need fixing first.

I read the implementation, the Done when lines, command help, act table,
Report and new and updated tests against master...8636207. All 317 tests in
atelier-cli, atelier-app, atelier-core, atelier-records and atelier-desk
passed locally. Formatting, Clippy with warnings denied, and whitespace
checks passed. The PR's test and
changes checks also pass. I ran additional scratch-repository probes for
worktree takeovers, concurrent backlog claims, and status before and after
committing a drop. These reproduce the findings below despite the passing
suite. The remote probes use a local bare Git repository; I did not test
claim races against a live GitHub repository or exercise the Desk UI.

## Finding 1: Refuse a takeover before moving a branch checked out elsewhere
Recommendation: fix now

In crates/atelier-cli/src/commands.rs:224-230, the existing-worktree check
runs only when the caller is off the base. The base path instead moves the
item ref directly at line 267. This changes HEAD under an existing worktree
without updating its index or files. The earlier remote fast-forward at
lines 214-220 can also move that ref before the worktree check runs.

I reproduced this without a remote: from main, start demo-ab12 as t1, add a
worktree for demo-ab12, then run start demo-ab12 as t2 from main. The second
start reports success, but the previously clean item worktree now has
`M  .atelier/items/demo-ab12/item.md`. Its cached diff deletes t2 from
implementers. An ordinary commit there can undo the takeover without the
agent ever editing that record. The printed worktree-add command also
cannot work because the branch already has a checkout. Check for other
worktrees before either ref move and refuse with the existing checkout's
path, as the design's Taking over section requires. Add coverage for both
the base takeover and a remote tip ahead of a checked-out local branch.

## Finding 2: Make a new remote claim require the branch to be absent
Recommendation: fix now

In crates/atelier-cli/src/commands.rs:402-406, a new claim uses the ordinary
push helper from crates/atelier-cli/src/merge.rs:899-900. An ordinary push
succeeds when the remote already has the same commit, so it does not enforce
creation of a previously absent ref. The backlog preflight cannot close
the gap between its remote lookup and this push.

I started `start --backlog same` in two clones of one bare remote and held
both with ATELIER_TEST_HOLD_BEFORE_BRANCH, after both passed their existence
checks. Releasing the first and waiting for its successful push, then
releasing the second, gives exit 0 from both. Both clones retain
backlog/same and report that they made and claimed it. Both branches point
at the same base commit, so the second push is a successful no-op. This
violates the no-existing-remote-branch condition and the required loser
cleanup. Use a create-only remote ref update for new claims, separate from
ordinary publication and takeover pushes, and test this interleaving.

## Finding 3: Include the holder's uncommitted record changes in status
Recommendation: fix now

In crates/atelier-cli/src/commands.rs:70-72, status passes the holder's own
item directory as an exclusion to unsaved, which removes every change in
that directory at lines 128-130. The reason given is that submit commits
it, but submit is unavailable after a drop, and the new contract explicitly
requires status to name every uncommitted record change with its Git command.

I started demo-ab12 as t1, switched to its branch and dropped it as t1,
without committing. Git reports the modified item.md, but `atelier status`
names no uncommitted path and prints no commit command. It ends with
“Nothing is left to do on it.” The drop has not been saved or published,
and another clone can still act on the item. Include these changes in
status and cover a holder's uncommitted drop, as well as ordinary item
record changes, rather than relying on submit to save them.

## Finding 4: Show the pending base copy of a drop on the item's branch
Recommendation: fix now

In crates/atelier-cli/src/commands.rs:73-80, orientation returns the item
view before computing unreached at lines 99-103. The item view receives
only the drop reason, so it cannot distinguish a branch-only drop from one
that has reached the base.

After committing the drop in the preceding probe, with main still holding
the ready item, `atelier status` on demo-ab12 still ends with “Nothing is
left to do on it.” In that same checkout, `atelier list` correctly says
“Dropped on their branch, not yet on the base.” The Done when line and the
drop command's output promise that both status and list report this pending
step until the base copy is dropped. Pass that distinction to the item
view and name the backlog action there too. Test status from the dropped
item's own branch after its drop is committed, not only the overview away
from the branch.
