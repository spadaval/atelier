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
owner edits the owner's record, so it does make approval stale. A link is
stored only on the record that declares it (atelier-ihss), so new work that
links itself `blocked_by` the owner leaves the owner's record and its approval
alone; a link the owner declares, or one written into an existing record
outside the scope, is a relationship edit and makes approval stale. Each commit is checked; changing and then restoring code
does not recover an old approval. New material edits require renewed review.

Merging the base into a reviewed branch keeps its approval when the merge is
clean: every other parent is already on the base, and the merge commit is
exactly the tree `git merge-tree` produces from its parents, with no conflict
resolution or edits made while merging (atelier-n7g1). The base commits it
brings in were reviewed on their own branches. A merge whose conflicts are
only in records, such as both sides linking work from a mission's record, also
keeps the approval when it matches Git's own merge everywhere else and each
conflicted record is a three-way merge of its sides: every field takes the
side that changed it, relationship and label lists keep each side's additions
and removals, and a field both sides changed differently needs renewed
review. So a resolution cannot drop a blocker or child the base added
(atelier-1isk). In either kind of merge, a record in the owner's scope may
still change only in status: any other base edit to it, such as a new child,
a blocker the record declares, or new acceptance, needs renewed review. New
work on the base linked `blocked_by` the owner is stored in the new record
only, so it leaves the approval current. A conflicted or hand-edited
merge, or one that brings in a branch not yet on the base, needs renewed
review. Proof is different: tests must run again on the merged result, so
record fresh evidence after merging the base.

Git's own merge here is `git merge-tree` with the repository's configuration
and attributes, so a custom merge driver (for example `merge=ours` on a
changelog) counts as part of "Git's own merge": a merge it produces is clean
even if it drops one side's change. Review merge drivers like other policy
(atelier-7hh4). An approval recorded before the work branch left the base, such
as one on the base itself, never counts for the branch.

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

Merge uses the base branch's committed policy and its
`branch_policy.merge_strategy`. Under `fast_forward_only`, the default, merge
fast-forwards only. If the base has moved on, merge it into the work branch,
record fresh proof, and renew review only if the merge needed resolving (see
approval freshness).

Under `merge_commit`, a branch that is behind the base can land without
catching up. Merge builds Git's merge of the work branch into the current base
with `git merge-tree --write-tree` and refuses with `merge_conflict`, naming
the files, if any path conflicts; resolve those on the work branch as above.
Evidence at the branch head does not prove the merged tree, so `merge_checks`
must include a `shell.check` (otherwise `merge_unproven`); record checks such
as `lint.none_blocking` read records and prove nothing about the tree. The
checks run in a temporary detached checkout of the merge commit instead of the
work checkout. The gate output adds `merge.merged_tree`, which passes only when
the shell checks passed there and names the tree, base commit, and commands,
and the merge commit's message lists those commands.

The gates read the records of the merge commit, which is what the base would
get, so a blocker the base added or reopened holds the merge. Review and
evidence pins are judged only along the work branch, though, so merge refuses
with `merge_base_records`, naming the files, when the base changed an in-scope
issue, or a review or evidence record about one, since the branch left it:
merge the base into the work branch so the gates judge them, then retry. Other
base records do not touch the approval, which is judged on the work branch's
history: it carries over, because a clean merge combines two reviewed changes
and nobody edited the result. No checkout holds the merged records before they
reach the base, so merge also checks the done claims outside the item that
they affect, as `land` does, trusting the base head: a branch that reopens work
a mission the base closed since the fork point depends on refuses, naming the
claim, such as `merge blocked: <mission> work.terminal` (atelier-m4gv). The
merge commit reaches the base only when every
gate passes and the base has not moved during the checks
(`merge_snapshot_changed`). A lockfile that merges cleanly but is
inconsistent is caught only by a `merge_checks` command that validates it, such
as `cargo metadata --locked`. Rebase only
before any review is recorded: rebasing rewrites the commits that findings and
approvals pin, and `merge` then refuses with "pins a commit outside the work
branch". To recover: reset the branch to its pre-rebase tip (from `git
reflog`); cherry-pick the code and planning commits made since, leaving out the
review and evidence records they add, which pin rewritten commits; merge the
base in; then record proof and review again (atelier-j48s, atelier-v9ve). For a
commit that mixes code and such records, run `git cherry-pick -n <commit>`,
`git rm -f` each file that `git diff --cached --name-only --diff-filter=A --
.atelier/reviews .atelier/evidence` lists, and commit the rest (atelier-imbo).
Bootstrap a valid committed policy on the base before starting governed work;
a work branch cannot weaken its own merge policy. See
[workflow configuration](workflow-configuration.md).

Merge creates the merge commit under `merge_commit` and no other commit, writes
no tracker state, and does not push. Git hooks
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
