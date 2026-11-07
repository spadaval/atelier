# Reviews and Merge

Reviews are Markdown events under `.atelier/reviews/`. Each command
adds a new file with an explicit identity, target, full commit, and explanation;
a [base exemption](#trivial-changes-on-the-base) names its base branch instead
of a target. Until a review record reaches the base branch it can be
[corrected or deleted](#correcting-review-records); once it is there, the one
thing that changes in place is a finding's [disposition](#dispositions), a
field of the finding
([ADR 0025](../adr/0025-findings-carry-their-disposition.md)).
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
atelier review trivial --base <commit> --identity "Author" "Why correctness is evident"
atelier review list <owner-id>
atelier review show <review-id>
```

`record` stores a review account; `approve` stores approval. Approval must come
from an identity other than the substantive Git authors. Identity is
self-declared and compared with author names and email addresses. `trivial`
is an explicit visible exemption with a reason, never an implicit fallback.
It is for a change whose correctness is evident from the diff, code or prose
alike; it is the only route that skips independent review, and there is no
lighter standard for some paths or kinds of text. `review list <id>` shows
the events of the issue's branch owner (a child's parent) one row each,
oldest first, then the owner's approval state and its findings with their
dispositions; `review show <id>` is a brief view of one event (see
[review rows](human-cli-output.md#review-rows)). `review list` accepts
`--json`, which keeps the stored records of the issue alone, and can omit the
owner to list all events, base exemptions included. Each recording command
confirms the event in one line and warns when an approval's, or a defer or
no_action disposition's, identity is a Git author the gates will refuse (see
[mutation confirmation](human-cli-output.md#mutation-confirmation)). A new
record's title is generated, as in `finding on atelier-nqgr @fed2d571`, so
its text is stored once; earlier records keep the text's first line.
`--commit` and `--resolution-commit` take a full commit ID: anything else is
an invalid value (exit 2), and a commit the branch does not hold is refused
with `git log --format=%H` as the recovery step.
Every command that records an event also takes `--commit-records` and
`--stage-records`, which commit the records it wrote (alone or grouped with the
rest of a step); see [Committing records](cli-surface.md#committing-records).

### Dispositions

Every finding requires a disposition. Recommendations and decisions use
`fix_now`, `defer`, or `no_action`:

```text
atelier review disposition <finding-id> --identity "Reviewer" --decision fix_now --resolution-commit <full-commit> "Fix verified"
atelier review disposition <finding-id> --identity "Reviewer" --decision defer --follow-up <issue-id> "Follow-up accepted"
atelier review disposition <finding-id> --identity "Reviewer" --decision no_action "Reason accepted"
```

The disposition is a field of the finding's own record: the decision, its
resolution commit or follow-up, the identity that set it, the commit HEAD was
at, the time and the reason. Setting it again replaces it, so a finding is one
file for its whole life. The record keeps only the current disposition;
earlier ones, such as an author's proposal the reviewer replaced or a
`fix_now` later changed to `defer`, stay in the Git history of the finding's
file, which `review show` names (`git log -p -- .atelier/reviews/<id>.md`).

A FIX NOW disposition needs a resolution commit later than the finding's pin,
followed by fresh independent approval. That re-review may cover only the
fix's diff (`review request` lists the commits since the last review or
approval), and the reviewer who raised the finding may do it. DEFER requires
an existing follow-up issue. DEFER and NO ACTION must be set by a non-author
identity: the author may propose one, and the reviewer accepts it by setting
it again. All findings without accepted dispositions block merge.

Who set a disposition is judged at the commit that set the field: the oldest
commit since which the finding's file has held this disposition, not the
commit or time the disposition names. Setting it again, or restoring earlier
bytes by hand, is judged at that later commit. On the base, every other part
of a review record is append-only: editing it, or deleting a record, is
refused, and merge checks review history from the base, including edits
subsequently reverted.

## Correcting review records

A review record can be edited or deleted by hand, by any agent on the branch,
until it reaches the base; commit the change with the work. Each change is
judged against the base as it stood where the change joined it, so a
correction is not refused after the branch merges (ADR 0028 decision 4,
atelier-ilba):

- a commit not on the base yet, by whether the tip of the policy's base
  branch holds the record;
- a commit a merge brought into the base, by whether the merge's first
  parent held it;
- a commit on the base's first-parent line, such as one made on the base
  itself, by whether its own first parent held it.

A change the base already holds is refused with `review_append_only`, at the
commit that made it (`land`, `doctor` and `merge` apply the same check). Under
`merge_strategy: fast_forward_only`, history cannot show where a fast-forward
began, so every commit is judged at its own first parent and a review record
is final once committed, as it is when the base branch cannot be read. A
corrected approval counts as it reads now, with its identity judged at the
commit that last set it. A change must leave a regular file, and a removed
separate disposition event is never added back.

Identity is self-declared, so no rule says who may correct a record: an
implementer could delete a reviewer's finding before merge, leaving no
decision on it. That cannot let unreviewed code merge, since an approval
still has to cover the code.

Repositories whose dispositions are separate events, from before ADR 0025,
keep working unmigrated: Atelier reads each finding's latest disposition
event as its disposition, with `migrated_from` naming the event, and judges
it at the commit that added the event, so check, land, merge and doctor give
the same results as after migration. `review disposition` on such a finding
writes the field and removes its events. `atelier migrate dispositions
--apply` is the optional cleanup: it writes that fold into every finding and
removes the events, on the base or a work branch, and keeps approvals
current; Git history keeps the events. Events that continue a folded
disposition, as a branch recorded with an older Atelier brings in, replace
it. To bring such a branch up to date with a base that has migrated, merge
without committing and migrate before the commit: `git merge --no-commit
<base>`, `atelier migrate dispositions --apply`, `git add .atelier`, `git
commit --no-edit`. A plain merge removes the base's migrated event while the
branch's later event remains, and `land` refuses it with
`review_append_only`, naming that sequence after the step that leaves the
merge out (atelier-j5oq). At the branch tip that step drops only the merge;
below it, it rewrites the commits after the merge, which then need review
again, since review and evidence records pinned to them no longer count. The read path for separate events goes once every repository has
migrated (atelier-42th). A disposition read from an event, or written by the
migration commit that removed it, is judged at the commit that added the
event; once the finding's file has held another disposition, the later
setting counts, even of the same bytes. Separate events stay append-only as
before: one may be removed only in a commit whose finding carries its own
disposition and keeps none of its events, as `review disposition` and the
migration write it, and a removed event is never added back (atelier-pfbp).

Terminal targets and their eligible review clusters can leave the active tree
together through [protected prune](retention-and-prune.md); retained events on
the base remain immutable.

## Requesting a review

`atelier review request <id> [--identity <name>] [--from <rev>]` prints the
brief to give an independent reviewer, as dense Markdown (ADR 0022), so the
orchestrator does not hand-write it. `<id>` is the branch owner or one of its
children; run it from the owner's worktree. The brief holds only what comes
from records, Git and policy:

- the owner's Goal and Done when, and each child's Done when;
- each Done when edit made after its item was started, as `merge --check`
  shows it (see [below](#a-done-when-edited-after-the-start));
- under After merge, each Done when line marked to run after the merge
  ([ADR 0027](../adr/0027-proof-has-a-time.md)), with its item, reason and
  stand-in, `reason missing` or `stand-in missing` where the line gives none,
  `ticked before merge` for a check already ticked, and the rule that the
  reviewer records a finding for any check that could run before merge or
  lacks a reason or a stand-in;
- the range to review: the commits since the last `review` or `approve` on the
  owner (or after `--from`), and the whole branch since it left the base, with
  the full head commit to pin records to;
- every finding with its disposition, open ones first, earlier reviews
  and approvals, and the newest recorded proof;
- the working rules: the built-in ones (read-only on code, run the proof,
  approve only without `fix_now` findings, an identity that differs from the
  authors), the base policy's `merge_checks` commands, and the text of the
  file named by `[review] rules_file` in `.atelier/config.toml`, a
  repository-relative Markdown path where a repository keeps rules the
  reviewer needs, such as where to put scratch files;
- the `review finding` and `review approve` commands, pinned to the head and
  using `--commit-records`, with `--identity` filled in when given.

It never includes the author's notes or Close Reason: a reviewer judges the
change against the requirements, not the author's account of it.

## Approval freshness

An approval covers code (atelier-3jdm). It is stale when code changed since
the commit it pins, not counting a clean merge from the base. Code is
everything except the tracker records, the Markdown files under
`.atelier/issues`, `.atelier/reviews` and `.atelier/evidence`, so the workflow
policy (`.atelier/workflow.yaml`) and the project config
(`.atelier/config.toml`) count as code. Record edits never make an approval
stale, whatever record they touch: ticking the Done when, adding notes or
links, the status and Close Reason of a close (with the implementer report it
carries), another issue's record, a finding's disposition, a new receipt, a
new child or follow-up. Each commit after the pin is checked, so changing
code and then restoring it does not recover an old approval, and a fix made
after a `fix_now` finding makes the approval stale, so the fix is reviewed
again. Review records on the base stay append-only apart from a finding's
disposition, but that is a check of its own (`review_append_only`), not part
of freshness.

A Done when changed after the item was started is still shown to whoever
judges the work (see [A Done when edited after the
start](#a-done-when-edited-after-the-start)), since the approval no longer
covers it.

This is the one rule for what an approval or trivial exemption covers, and
`merge` (its `review.current` gate), `land` on the work branch, `land` on the
base and `status` apply it alike, through `review_gate::approval_is_current_to`.
So the done claims of the owner and its children may be committed before or
after approval: a close that changes no code keeps the approval current, while
code committed with it or after it makes the approval stale, and `merge`
refuses before integrating until a fresh approval is recorded at or after
that commit (atelier-n642, atelier-uw4x). The approved scope still decides
which done claims an approval covers. The scope is the owner and its
descendants; the owner's parent is outside it. `merge` also runs `land`'s own
check of done claims on the commit it would put on the base, as its
`claims.reviewed` gate: a done claim outside every current approval's scope,
such as an issue created already done after the approval or a follow-up
closed on the branch, fails it, naming the commit and the fix.

Merging the base into a reviewed branch keeps its approval when the merge is
clean: every other parent is already on the base, and the merge commit matches
the tree `git merge-tree` produces from its parents everywhere outside the
records, with no code conflict resolved and no code edited while merging
(atelier-n7g1). The base commits it brings in were reviewed on their own
branches. Record conflicts may be resolved any way, since records are not what
an approval covers; the gates then read the records as resolved, so a blocker
the resolution keeps still holds the merge. A merge that resolved or edited
code by hand, or one that brings in code from a branch not yet on the base,
makes the approval stale. Proof is different: tests must run again on the merged
result, so record fresh evidence after merging the base.

Git's own merge here is `git merge-tree` with the repository's configuration
and attributes, so a custom merge driver (for example `merge=ours` on a
changelog) counts as part of "Git's own merge": a merge it produces is clean
even if it drops one side's change. Review merge drivers like other policy
(atelier-7hh4). An approval recorded before the work branch left the base, such
as one on the base itself, never counts for the branch.

When `review.current` fails, it names the approval it considered and why it
does not count (atelier-cisa): the newest stale independent approval or
exemption, or else the newest approval whose identity matches a Git author of
the branch. One function,
`review_gate::approval_standing`, judges and explains it for `land`, `merge`,
`merge --check`, `merge --sync` and `review list`:

```text
⚠ atelier-7l79 review.current: approval atelier-x16l @e2562bd6 is stale: 1a2b3c4d "Tweak after approval" changed proof-sample.txt. Next: atelier review request atelier-7l79
⚠ atelier-bw0b review.current: approval atelier-x16l @e2562bd6 is stale: base merge 4f1e2a3b resolved or edited src/gate.rs by hand. Next: atelier review request atelier-bw0b
⚠ atelier-i3ay review.current: approval atelier-q2m8 @4c79d30b by author@example.com is not independent: the identity matches a Git author of the branch (authors: author a <author@example.com>). Next: atelier review request atelier-i3ay
```

A stale line names the first commit that made the approval stale, with up to
three of the code paths it changed and `(and <n> later commits)`; records it
changed are not listed. A merge reads `base merge <commit>` (or `merge
<commit>` when it brings in a branch not on the base) with the code it
resolved or edited by hand, or the code it changed when it did neither. An approval that predates the point where the branch left the base
says so. With no approval at all, the line reads `no independent approval or
trivial exemption; authors: ...`.

## Integration

Commit the implementation, any receipts, branch ownership, and done claim on
the work branch. The owner and its scoped children must be terminal with no
open transitive blockers. Evidence must satisfy the base policy's completion
checks; a failing receipt with no pass recorded after it blocks success. Review must have current independent approval
or an explicit trivial exemption and accepted finding dispositions.

The implementer report parts a done claim needs come from the base's policy
where the claim's commit last met the base, so a claim made before the policy
required a part stays valid (see [the work model](work-model.md)). That rule
reads only history: a claim made on a side branch that left the base before
the policy changed, then merged into the work branch, passes without the
parts, and nothing in the history tells it from a child closed honestly
before the change (atelier-u0i3). A reviewer who sees a done claim without a
report the base's policy now requires should ask where it was made.

```text
atelier merge <owner-or-child-id> --check
atelier merge <owner-or-child-id> --check --json
atelier merge <owner-or-child-id>
atelier merge <owner-or-child-id> --sync
```

Run these from the recorded work branch. From anywhere else, `merge` refuses
with `merge_branch_mismatch`, naming the work branch and `git switch
<branch>`; on the base, where the records learn of the branch only when the
work merges, it finds the branch from the local branches' own records. An item
no branch owns refuses with `merge_owner_unstarted` and `atelier start <id>`.
Once the work is on the base, `merge` and `merge --check` print `<id> is
already merged into <base> (<commit>).` and pass (atelier-yrlj). Source and any
separate base checkout must be clean. Canonical record and policy paths, bytes,
and modes must match committed HEAD, including ignored canonical files; Git
index flags such as assume-unchanged or skip-worktree cannot hide changed
tracked files. Other files follow Git status: with `core.filemode=false`, a
local execute-bit change is not an uncommitted change. Checks
cannot authorize a different branch, HEAD, base, or canonical snapshot while
they run. `--check` evaluates the gates without integrating.

A failed precondition does not hide the gates: uncommitted records
(`canonical_snapshot_uncommitted`, every record named), uncommitted code
(`merge_checkout_dirty`, every path named), a base the branch has not merged in
under `fast_forward_only` (`merge_diverged`) and an invalid candidate policy
(its `workflow_*` code) are each one more failed check with its fix, and the
gates still run, against HEAD's committed records when the checkout's differ
and from where the branch left the base. A real merge still refuses before
integrating. A precondition the gates cannot run past, such as the wrong
branch or a merge conflict, is reported after the ones found before it, with
a `Not checked:` line naming the gates it skipped (atelier-kiry).

Merge uses the base branch's committed policy and its
`branch_policy.merge_strategy`; omitted, it is `merge_commit` when `merge_checks`
include a `shell.check` and `fast_forward_only` otherwise (see [workflow
configuration](workflow-configuration.md)). Under `fast_forward_only`, merge
fast-forwards only. If the base has moved on, merge it into the work branch,
and renew review only if the merge resolved or edited code by hand (see
approval freshness). The merge
records no new proof: CI or the merge checks test the merged tree where the
repository has them (see [what gets a receipt](validation.md#what-gets-a-receipt)).

### A Done when edited after the start

`merge` and `merge --check` show each edit to the Done when of the owner or a
scoped child made after the item was started, whatever the verdict, after the
gate lines (atelier-r9hb):

```
Note: atelier-2xx5 Done when edited at 4adb9ff6 after the item was started, after finding atelier-h7l3.
```

The trigger is the start, not a finding: in the usage review that motivated
it, most narrowed criteria had no finding recorded before them. An edit
counts when the commit that made it descends in Git from one that moved the
item into an `active` status, such as `atelier start` or the `start`
transition; clock times are not compared, so an edit made on the base while
a branch worked on the item is not shown. The line names the newest finding
or `fail` receipt on the item pinned to an ancestor of the edit, when there
is one, so the reader can check whether the edit answers it or loosens the
criterion to let the item close. Merge commits are not read: the commit that
wrote an edit is in the history itself. The approval does not cover the edit,
since record edits never make it stale, so this line is how the edit is seen,
including one made after approval.

The line refuses nothing and changes neither the verdict nor the exit status.
A Done when is sometimes rightly corrected after a finding, and refusing it
would need an override or a human-only record, which [ADR
0018](../adr/0018-strategy-draft-epics-and-adaptive-expansion.md) keeps
Atelier from becoming. Under `-q` the line goes to stderr, so stdout keeps
its token; `--json` leaves it out. The [debrief](mission-reviews.md#debrief)
lists the same edit under the item. Mission success criteria are not
covered.

### Catching up with `--sync`

```text
atelier merge <owner-or-child-id> --sync
```

`--sync` is the catch-up step as one command, run from the clean work branch.
It merges the base into the branch with `git merge <base>`, never a rebase,
which would orphan the commits reviews pin; reports whether the approval
survived; and ends in a verdict line, `merge:
passed`, `merge: synced — approval needs fresh review`, or `merge: refused —
1 check (<code>)` (see [verdict line](human-cli-output.md#verdict-line)). It integrates nothing, so the
branch still lands with `atelier merge`. It cannot be combined with `--check`
or `--json`.

- **Conflicts.** `git merge` stops, nothing is committed, and the output lists
  the conflicted files (`sync_conflict`). Resolve and commit them, or
  `git merge --abort`. Running `--sync` again after the resolution commit
  finds nothing to merge and reports the approval, judged from where the
  branch stood before the merge.
- **No receipts.** The sync runs and records no evidence. CI or the policy's
  `merge_checks` test the merged tree where the repository has them; a
  repository with neither has no merged-tree check until it adds a suite to
  `merge_checks` ([ADR 0026](../adr/0026-evidence-records-only-what-nothing-else-records.md)).
  Before atelier-s66u the sync reran the command-backed receipts that were
  current before it and committed them.
- **Approval.** The output says `Approval kept`, judged as `review.current`
  judges it (`review_gate::approval_standing`), or `Approval needs fresh
  review` with the first commit that made it stale and the code paths of
  every stale commit to review, for a merge the code it resolved or edited by
  hand. An approval whose identity matches a Git
  author of the branch is named as not independent. A stale approval does
  not refuse the sync, which exits zero, but the verdict reads `merge: synced
  — approval needs fresh review` instead of `passed`, because `merge --check`
  will refuse until a fresh approval is recorded.
- **Strategies.** The sync is the same under both. Under `fast_forward_only`
  it is required whenever the base moved, since merge creates no merge commit.
  Under `merge_commit`, merge lands a clean branch on a moved base itself, so
  `--sync` matters for a conflicting branch or when merge refuses with
  `merge_base_records`.

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

Each checkout lives in its own `atelier-merge-*` directory in the Git common
directory, and the merge holds a lock on that directory's `lock` file while its
checks run. Normal exit removes the checkout. If merge is killed during its
checks, the operating system drops the lock, and the next merge removes every
`atelier-merge-*` checkout whose lock is free before it makes its own; one a
concurrent merge still holds stays (atelier-rcnk).

Merged-tree checks do not reuse a build cache. Each run builds in the
checkout's own target directory (the repository's `merge_checks` set
`CARGO_TARGET_DIR=target`), which goes away with the checkout, so a check
proves what the merged tree builds and never what an earlier tree left behind.
A shared target directory is the one cache Atelier does not allow here: it has
run another branch's binary in tests, and cargo's freshness checks cannot tell
two checkouts apart. A policy that wants faster checks may put a compiler cache
keyed by its inputs, such as `sccache`, in the check command itself.

The gates read the records of the merge commit, which is what the base would
get, so a blocker the base added or reopened holds the merge. Review and
evidence pins are judged only along the work branch, though, so merge refuses
with `merge_base_records`, naming the files, when the base changed an in-scope
issue, or a review or evidence record about one, since the branch left it:
merge the base into the work branch so the gates judge them, then retry. Other
base records do not touch the approval, which is judged on the work branch's
history: it carries over, because a clean merge combines two reviewed changes
and nobody edited the result. Merge also checks the done claims outside the
item that the commit it would put on the base can affect, as `land` does,
trusting the base head, under either strategy: the merge commit under
`merge_commit`, which no checkout holds before it reaches the base, or the
branch head under `fast_forward_only` (atelier-bu66). For example, if the base
closed a mission after the fork point and the branch reopens or adds work that
mission depends on, merge refuses, naming the claim (`merge: refused — 1
check (<mission> work.terminal)`, atelier-m4gv). These claims are read at that commit, so a
claim the base completed and reviewed after the fork point is judged against
the base commits its reviews pin (atelier-goom). The merge commit reaches the
base only when every gate passes and the base has not moved during the checks.
When the base moved during the checks only by commits that change nothing but
issue, review and evidence records, merge reports `merge_base_moved` and runs
the gates again against the new base, up to three attempts in all; a record the
item itself depends on still refuses as `merge_base_records`. If any of those
commits changes anything else, or the base was rewritten, merge refuses with
`merge_snapshot_changed`, naming the first such commit and file. Under
`fast_forward_only` merge cannot land on a base that moved at all, since it
creates no merge commit; `merge_diverged` then says whether the base gained only
records-only commits or names the code commit, and the recovery is still `git
merge <base>` on the work branch (atelier-p189). A lockfile that merges cleanly but is
inconsistent is caught only by a `merge_checks` command that validates it, such
as `cargo metadata --locked`. Rebase only
before any review is recorded: rebasing rewrites the commits that findings and
approvals pin, and `merge` then refuses with "pins a commit outside the work
branch". To recover: reset the branch to its pre-rebase tip (from `git
reflog`); cherry-pick the code and planning commits made since, leaving out the
review and evidence records they add, which pin rewritten commits; merge the
base in; then get the branch reviewed again (atelier-j48s, atelier-v9ve). For a
commit that mixes code and such records, run `git cherry-pick -n <commit>`,
`git rm -f` each file that `git diff --cached --name-only --diff-filter=A --
.atelier/reviews .atelier/evidence` lists, and commit the rest (atelier-imbo).
Bootstrap a valid committed policy on the base before starting governed work;
a work branch cannot weaken its own merge policy. See
[workflow configuration](workflow-configuration.md).

Merge creates the merge commit under `merge_commit` and no other commit, writes
no tracker state, and does not push. Git hooks
are disabled during gated integration and cleanup; when the repository has
hooks of its own, merge says they did not run. Put required read-only checks
in the reviewed base policy's `merge_checks`. `merge` and `merge --check`
print failed gates first, each with its fix, and end with a verdict (see
[gate lines](human-cli-output.md#gate-lines)).

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
work branch, review, and `atelier merge`, or, when its correctness is evident
from the diff, is exempted on the base as a [trivial
change](#trivial-changes-on-the-base).

`atelier land` on the base branch checks this before you push. It looks at
the base commits that are not yet on the base's upstream. A commit counts as
reviewed when a commit pinned by an approval or trivial exemption contains it,
for completed owners that branched from this base and are not already done
upstream, and only an approval `merge` would accept counts (atelier-n90q). Its
identity must be independent of the branch's own authors: authors of base
commits the branch merged in do not count, as in `review.current`
(atelier-0cj6). Judging from history, `land` cannot tell a side branch that
was never on the base from one that was, so it leaves out the authors of every
branch merged into the work branch, where `review.current` counts a side
branch not on the base. `merge` may therefore refuse an approval that `land`
would accept, never the reverse (atelier-p5qp). Every finding pinned at or before it needs an
accepted disposition, and a FIX NOW's resolution commit must be at or before
the approval's pin; the disposition itself may be recorded later. That is what `atelier merge` integrates. A commit a [base
exemption](#trivial-changes-on-the-base) pins is covered too. Every other commit may touch only records and the
strategies linked from missions, or `land` refuses with
`land_base_unreviewed`. Those record-only commits also may not take a work
item to done: a done claim travels on the work branch with the work, so `land`
refuses one made outside review with `land_base_done_claim`. A claim made after
an approval's pin is reviewed while that approval is still current at the
claim's commit, by the rule in [Approval freshness](#approval-freshness), so a
close-only commit that `merge` accepted lands too. Revert the claim (a
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

### Trivial changes on the base

A trivial change, such as a one-line policy fix or a typo in an ADR, can be
committed on the base directly and exempted there, with no owner issue or work
branch (atelier-mbrf). Commit it, then from the base checkout record:

```text
atelier review trivial --base <commit> --identity "Author" "Why correctness is evident" --commit-records
```

The exemption is the same trivial exemption as an owner's, under the same
standard: correctness evident from the diff. It is stored like other review
events but names `base: <branch>` instead of a target, with the full commit,
the identity and the reason. It is recorded only from a checkout of the base
branch, for a commit the base already holds. `land` on the base, and `start`,
count exactly that commit as reviewed and report it as exempted as trivial:
the exemption never covers an earlier or later commit, even one on top of it,
and it covers no done claim, which still travels on a work branch
(`land_base_done_claim`). `review list` and `review show` show it, and a
mission's debrief shows each base exemption recorded between the mission's
start and its done claim on its timeline, because it names no mission.
