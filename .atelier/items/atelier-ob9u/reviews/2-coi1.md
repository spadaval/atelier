---
schema: atelier.review
schema_version: 3
round: 2
commit: 9e2da86d33a450fb097637c0fc3914627774d7c2
reviewers:
- thread:delegated-task:command%3Amcp%3Aa410b15e-b763-48ac-8edb-eb330175253e%3Adelegate-task%3A0620973c-8709-4e90-b418-01eabea0f3ee
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-13T23:21:51.448368028Z
---

## Summary

Complexity: high. This changes the Git contract across record commands,
branch naming, remote claims, hooks, record-only merges and cross-clone drops.
Value: high. Keeping record writes off the base requires this branch model.
Risk of merging it: high. The new base-commit guard can silently pass a
record deletion, and remote-backed records reach writers that require a
checkout path.

The value warrants the scope; I found no substantially smaller change that
delivers the required branch model. All four earlier findings are fixed,
but the five new findings below need fixing before merge.

I read the implementation and tests against master...9e2da86d, the round-2
diff, the Done when lines, Report, act table and command help. All 320 tests
in atelier-cli, atelier-app, atelier-core, atelier-records and atelier-desk
passed locally, including the four regression cases. Formatting, Clippy
with warnings denied and whitespace checks passed. PR #156's test and
changes checks pass. I also ran scratch-repository probes using the built
binary for deleting config.toml through the pre-commit hook, review and
verdict in a checkout behind its fetched remote branch, a leading-dash
branch prefix, filtered lists after a drop, and the drop's printed guidance.
These reproduced the new findings despite the passing suite. Remote probes
used a local bare repository. I did not test live GitHub claim races or
exercise the Desk UI.

## Finding 1: Refuse a takeover before moving a branch checked out elsewhere
Recommendation: fix now
Was: 1-cj3m.1
Response: fixed in 43f940c82
Now: fixed

The worktree check now precedes local branch creation or movement in
`take_over`. The regression test covers a takeover from the base checkout
and a fetched remote tip ahead of a branch checked out elsewhere; both
refuse without moving that branch or dirtying its checkout.

## Finding 2: Make a new remote claim require the branch to be absent
Recommendation: fix now
Was: 1-cj3m.2
Response: fixed in 43f940c82
Now: fixed

New claims use an empty expected-ref lease and require the push's porcelain
output to report a new ref. The regression test releases two claims of the
same backlog branch and commit in sequence: only the first succeeds, and
the second removes its local branch.

## Finding 3: Include the holder's uncommitted record changes in status
Recommendation: fix now
Was: 1-cj3m.3
Response: fixed in 43f940c82
Now: fixed

`unsaved` no longer excludes the holder's item directory. The new holder's
uncommitted-drop test passes and checks the path and Git commit command.

## Finding 4: Show the pending base copy of a drop on the item's branch
Recommendation: fix now
Was: 1-cj3m.4
Response: fixed in 43f940c82
Now: fixed

Item orientation now passes the unreached-drop distinction to its output.
The updated planning test checks status on the dropped item's own branch
and its backlog action while the base still holds the ready item.

## Finding 5: Keep the base-commit guard active when config.toml is deleted
Recommendation: fix now

In crates/atelier-cli/src/check.rs:213-220, failure to read the checkout's
config makes `on_base` None, disabling the new base-record guard. A staged
deletion of config.toml also escapes the earlier validation: staged_changes
excludes deletions, so filtering problems to those paths removes the
missing-config error.

I initialized a valid repository on main, installed a pre-commit hook that
executes the built `atelier check --hook pre-commit`, then ran
`git rm .atelier/config.toml` and `git commit -m "Delete config on base"`.
The commit succeeded with no ATELIER_SKIP and removed the config directly
on main. This violates the required refusal of record changes on the base,
including deletions, and leaves Atelier unable to open the repository.
Read the base policy from a trustworthy available config, or refuse when
it cannot be determined; do not turn a config read failure into permission.
Add a regression test for a staged config deletion on the base, and for an
unreadable unstaged config alongside a staged record change.

## Finding 6: Refuse review writes while the checkout is behind its fetched branch
Recommendation: fix now

In crates/atelier-cli/src/review.rs:51-61 and 90-92, `review` passes the
selected record's ReviewFile to a writer without checking whether that
record came from the remote-tracking branch. The new reader can select a
remote tip ahead of this checkout, whose review path is a diagnostic path
like `<commit>:.atelier/items/<id>/reviews/<name>.md`, not a writable path.
`verdict` has the same problem at lines 320-322. Unlike submit and merge,
neither command refuses this stale checkout before trying to write.

I submitted an item in clone A, checked its branch out in B, then opened,
committed and pushed a review in A. After a fetch in B, without pulling,
`review <id>` as a different reviewer exited 2 with "No such file or
directory". `verdict approve` as the remote review's holder exited 2 with
"cannot write the review file: No such file or directory". Both tried to
write the commit-qualified path. When opening a new round, using the local
`tip` with the remote record also mixes two snapshots. Refuse with the
behind/pull guidance before either review write, or explicitly obtain a
consistent checkout record. Cover both review takeover and verdict.

## Finding 7: Reject branch prefixes that start with a dash
Recommendation: fix now

In crates/atelier-core/src/config.rs:134-142, validate_prefix allows the
first component to start with `-`. Git accepts such a full ref via
update-ref, but it is not a valid branch name for the ordinary branch
commands start uses.

With `branch_prefix = "-/"`, `atelier check` reported every record valid.
From a detached checkout, `atelier start --backlog tidy` created the branch
`-/backlog/tidy` and then exited 2 because `git switch --quiet
-/backlog/tidy` interpreted the branch as options. Its suggested retry
`git switch -/backlog/tidy` fails the same way. Reject this prefix during
config validation so an accepted configuration cannot create branches
that the workflow cannot switch to. Add it to the prefix validation tests.

## Finding 8: Apply list filters to pending drops too
Recommendation: fix now

In crates/atelier-cli/src/views.rs:47-48, the new pending-drop section is
built from every item, after the regular entries have been filtered. It
checks only `filter.done`, so --ready, --blocked, --mine, --mission and
--label do not narrow that section.

After starting and committing a drop of an unlabeled item whose base copy
was still ready, both `atelier list --ready` and `atelier list --label
unrelated` printed "No open work matches" followed by that dropped item.
This breaks the existing contract that list flags narrow together, and
makes scoped lists report unrelated work. Apply the applicable filters to
the pending-drop entries, and test at least --ready and an unmatched label
or mission.

## Finding 9: Give record writes guidance for their actual branch and act
Recommendation: fix now

In crates/atelier-app/src/plan.rs:334-336, save_with treats every non-base
branch name as an item ID when calling say::planned. That text at
crates/atelier-app/src/say.rs:442-443 is for settling another item's finding
on an implementer's branch. Now it is also used for ordinary backlog and
mission writes and for dropping the current item.

In my drop probe, after correctly explaining the backlog step, the command
then said: "Once committed, it reaches the base when demo-ab12 merges. It
is outside demo-ab12's directory" and said merge would request a submit.
The file it just wrote was inside that very directory, and this drop
refuses both submit and merge, so those instructions cannot be followed.
On backlog branches the same code treats `backlog/<slug>` as an item and
refers to its directory and review. Select guidance using the parsed branch
kind and the operation, and keep a started drop's instructions on its
commit/push and backlog path. Test the complete output of an ordinary
backlog write and a started-item drop.
