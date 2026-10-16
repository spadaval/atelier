# Reviews and Merge

Reviews are immutable Markdown events under `.atelier/reviews/`. Each command
adds a new file with an explicit identity, target, full commit, and explanation.
Agent Factory owns review and finding assessment; Atelier checks the recorded
invariants. There is no external review backend or review room.

## Review commands

Record decisions against the branch owner, a parent issue or a standalone
issue. The commit defaults to current HEAD; `--commit` supplies a full ancestor
commit on the work branch.

```text
atelier review record <owner-id> --identity "Reviewer" "Checks and limits"
atelier review approve <owner-id> --identity "Reviewer" "Approval summary"
atelier review finding <owner-id> --identity "Reviewer" --recommendation fix_now "Problem"
atelier review trivial <owner-id> --identity "Author" "Why correctness is evident"
atelier review list <owner-id>
atelier review show <review-id>
```

`record` stores a review account; `approve` stores approval. Approval must come
from an identity other than the substantive Git authors. Identity is
self-declared and compared with author names and email addresses. `trivial`
is an explicit visible exemption with a reason, never an implicit fallback.
`review list` accepts `--json` and can omit the owner to list all events.

Every finding requires a disposition. Recommendations and decisions use
`fix_now`, `defer`, or `no_action`:

```text
atelier review disposition <finding-id> --identity "Reviewer" --decision fix_now --resolution-commit <full-commit> "Fix verified"
atelier review disposition <finding-id> --identity "Reviewer" --decision defer --follow-up <issue-id> "Follow-up accepted"
atelier review disposition <finding-id> --identity "Reviewer" --decision no_action "Reason accepted"
```

A FIX NOW disposition needs a resolution commit later than the finding's pin,
followed by fresh independent approval. DEFER requires an existing follow-up
issue. DEFER and NO ACTION need acceptance from a non-author identity. All
findings without accepted dispositions block merge. A later disposition
automatically links to the preceding disposition; commit the new
file rather than editing or deleting an old event. Merge checks review history
from the base, including edits subsequently reverted.
Terminal targets and their eligible review clusters can leave the active tree
together through [protected prune](retention-and-prune.md); retained events
remain immutable.

## Approval freshness

Code, policy, acceptance, relationship, metadata, or existing proof/review edits
make approval stale. Later commits may add new review/evidence files, create
issues outside the reviewed scope (such as the follow-up a `defer` disposition
names, atelier-cafy), or change in-scope lifecycle status, timestamps, and
Close Reason without changing the approved acceptance. Adding a child to the
owner edits the owner's record, so it does make approval stale. Each commit is checked; changing and then restoring code
does not recover an old approval. New material edits require renewed review.

Merging the base into a reviewed branch keeps its approval when the merge is
clean: every other parent is already on the base, and the merge commit is
exactly the tree `git merge-tree` produces from its parents, with no conflict
resolution or edits made while merging (atelier-n7g1). The base commits it
brings in were reviewed on their own branches. A conflicted or hand-edited
merge, or one that brings in a branch not yet on the base, needs renewed
review. Proof is different: tests must run again on the merged result, so
record fresh evidence after merging the base.

## Integration

Commit the implementation, proof, branch ownership, and done claim on the work
branch. The owner and its scoped children must be terminal with no open
transitive blockers. Evidence must satisfy the base policy's completion checks;
a newer failure blocks success. Review must have current independent approval
or an explicit trivial exemption and accepted finding dispositions.

```text
atelier merge <owner-or-child-id> --check
atelier merge <owner-or-child-id> --check --json
atelier merge <owner-or-child-id>
```

Run these from the recorded work branch. Source and any separate base checkout
must be clean. Canonical record and policy paths, bytes, and modes must match
committed HEAD, including ignored canonical files; Git index flags such as
assume-unchanged or skip-worktree cannot hide changed tracked files. Other files
follow Git status: with `core.filemode=false`, a local execute-bit change is not
an uncommitted change. Checks
cannot authorize a different branch, HEAD, base, or canonical snapshot while
they run. `--check` evaluates the gates without integrating.

Merge uses the base branch's committed policy and fast-forwards only. If the
base has moved on, merge it into the work branch, record fresh proof, and renew
review only if the merge needed resolving (see approval freshness). Rebase only
before any review is recorded: rebasing rewrites the commits that findings and
approvals pin, and `merge` then refuses with "pins a commit outside the work
branch". To recover, reset the branch to its pre-rebase tip (from `git reflog`),
cherry-pick any code or planning commits made after the rebase back onto it,
leaving out review and evidence records (they pin rewritten commits, so proof
and review are recorded again), and merge the base instead (atelier-j48s, atelier-v9ve).
Bootstrap a valid committed policy on the base before starting governed work;
a work branch cannot weaken its own merge policy. See
[workflow configuration](workflow-configuration.md).

Atelier creates no commit, writes no tracker state, and does not push. Git hooks
are disabled during gated integration and cleanup, with an explicit notice.
Put required read-only checks in the reviewed base policy's `merge_checks`.

Successful integration deletes the local work branch. If the base has a
separate checkout, the source checkout becomes detached and the command prints
its worktree cleanup instruction. Leave that checkout before removing it from
the base checkout. If cleanup fails after integration, inspect the named Git
state before retrying cleanup. Use [prune](retention-and-prune.md) for remaining
eligible artifacts.

## Planning changes on the base

Filing, grooming, re-prioritising, recording decisions (including deciding a
`decision` record), and editing mission strategies are commits too. Commits that change only records
(`.atelier/issues`, `.atelier/evidence`, `.atelier/reviews`) and the strategy
files linked from missions may be committed on the base branch directly
(atelier-3mjv, decided by Sumeet 2026-10-15). Any other change goes through a
work branch, review, and `atelier merge`.

`atelier land` on the base branch checks this before you push. It looks at
the base commits that are not yet on the base's upstream. A commit counts as
reviewed when a commit pinned by an approval or trivial exemption contains it,
for completed owners that branched from this base and are not already done
upstream. That is what `atelier merge` integrates. Every other commit may touch only records and the
strategies linked from missions, or `land` refuses with
`land_base_unreviewed`. Those record-only commits also may not take a work
item to done: a done claim travels on the work branch with the work, so `land`
refuses one made on the base with `land_base_done_claim`. Revert the claim (a
claim no longer done at the tip is withdrawn), then `atelier start` the item
and make the claim on its work branch (atelier-fj1t). Closing a mission, deciding a `decision` record
and superseding work stay allowed on the base. A merge commit, such as a
non-rebasing pull, is judged by the paths that differ from all of its parents. The install predates the gates and is exempt: commits
made before the policy existed, the first commit on the base's first-parent
line that adds it, and everything that commit merges in. Without an upstream, `land`
notes that it cannot check base commits.

`atelier start` runs the same check on the local base branch, using the
base's own records, whether it creates the work branch or switches to an
existing one. A branch started while the base holds an unreviewed commit
would carry it in, and a review of `<base>...<branch>` would never show it.
`start` skips the check when the base has no upstream or no workflow policy
yet, as when Atelier is being adopted on a branch. To
recover, move the commit to a work branch (`git switch -c <type>/<id>`, then
`git branch -f <base> <base>@{upstream}`) and run `atelier start <id>` there.
This is a handoff check, not an access control. Review identities are
self-declared, so an exemption recorded on the base for an owner that merged
but is not yet pushed still covers commits after its branch started, and a
direct commit pushed without running `land` is caught only by later review.
