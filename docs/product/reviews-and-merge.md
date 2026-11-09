# Reviews and Merge

Reviews are Markdown events under `.atelier/reviews/`. Each command
adds a new file with an explicit identity, target, full commit, and explanation;
a [base exemption](#trivial-changes-on-the-base) names its base branch instead
of a target, and a record on the [light path](#a-change-with-no-issue-the-light-path)
names the branch it is about. Until a finding reaches the base branch it can be
[corrected or deleted](#correcting-review-records), while an approval or
trivial exemption stays as first committed; on the base, the one thing that
changes in place is a finding's [disposition](#dispositions), a field of the
finding
([ADR 0025](../adr/0025-findings-carry-their-disposition.md)).
Agent Factory owns review and finding assessment; Atelier guards the recorded
invariants. There is no external review backend or review room; a
[forge](#ci-on-a-forge) only carries branches and reports CI.

## Review commands

Record decisions against the branch owner, a parent issue or a standalone
issue. The commit defaults to current HEAD; `--commit` supplies an ancestor
commit on the work branch, as a full ID or a unique abbreviation.

```text
atelier review record <owner-id> --identity "Reviewer" "What was reviewed and its limits"
atelier review approve <owner-id> --identity "Reviewer" "Approval summary"
atelier review finding <owner-id> --identity "Reviewer" --recommendation fix_now "Problem"
atelier review trivial <owner-id> --identity "Author" "Why correctness is evident"
atelier review trivial --base <commit> --identity "Author" "Why correctness is evident"
atelier review finding --branch <name> --identity "Reviewer" --recommendation fix_now "Problem"
atelier review approve --branch <name> --identity "Reviewer" "Approval summary"
atelier review list <owner-id>
atelier review show <review-id>
```

`--branch <name>` in place of the owner records an event about a branch no
issue owns (see [the light path](#a-change-with-no-issue-the-light-path)).

`record` stores a review account; `approve` stores approval. Identity is
self-declared and recorded with each event; it is not compared with the work's
Git authors (ADR 0028 decision 5, atelier-md0v). Independent review is Agent
Factory's practice, and Atelier requires an approval that covers the current
code. `trivial`
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
confirms the event in one line (see
[mutation confirmation](human-cli-output.md#mutation-confirmation)). A new
record's title is generated, as in `finding on atelier-nqgr @fed2d571`, so
its text is stored once; earlier records keep the text's first line.
`--commit`, `--resolution-commit` and `review trivial --base` take a full
commit ID or a unique abbreviation of at least four hexadecimal digits, which
is resolved on input and stored in full; anything else is an invalid value
(exit 2), except that `review trivial --base` also takes a revision such as
`HEAD`. A prefix no commit has, or one several commits share, is refused,
naming the commits it matches, and a commit the branch does not hold is
refused; both give `git log --format=%H` as the recovery step.
Every command that records an event also takes `--commit-records` and
`--stage-records`, which commit the records it wrote (alone or grouped with the
rest of a step); see [Committing records](cli-surface.md#committing-records).

### Dispositions

A finding recommended `fix_now` needs no disposition: an approval pinned to a
later commit than the finding, one that holds the finding's record, and
recorded after it resolves it, so the fix's re-review is what settles it, and
an approval that never saw the finding cannot, such as a parallel reviewer's
approval of a head that did not hold it. A finding recommended `defer`,
`no_action` or `wont_fix` needs a recorded decision, even if it was fixed
instead. Recommendations and decisions use one of four values:

- `fix_now`: fix it on this branch.
- `defer`: fix it later, in the follow-up issue the decision names.
- `no_action`: the finding is wrong: mistaken, a duplicate, or without
  consequence.
- `wont_fix`: the finding is real, but not worth fixing (atelier-stnv). It
  needs a reason and no follow-up issue, so a real no costs nothing to
  record. A finding that recommends or decides `wont_fix` is written at
  review schema version 4, so an Atelier that predates `wont_fix` reports it
  as needing a newer Atelier, with the reinstall command, rather than as an
  invalid record (atelier-xlxn). Every other finding keeps its version.

```text
atelier review disposition <finding-id> --identity "Reviewer" --decision defer --follow-up <issue-id> "Follow-up accepted"
atelier review disposition <finding-id> --identity "Reviewer" --decision no_action "Why the finding is wrong"
atelier review disposition <finding-id> --identity "Reviewer" --decision wont_fix "Why it is not worth fixing"
atelier review defer <id> --identity "Reviewer" [--dry-run] [--commit-records]
```

`review defer <id>` defers, in one step, every finding on the item and its
children that was recommended `defer` and has no decision yet. Each gets a
`backlog` task discovered from the finding, carrying its text, with the
finding's leading severity (`high:` and so on) as its priority, and a
`defer` disposition naming that task. The tasks go under one parent per
item, a `backlog` task labelled `deferred-findings` and discovered from the
item: the first run creates it, and later runs find it again by that label
and link, while it is open. A finding already decided is left alone, so a
second run files nothing twice. `--dry-run` lists the findings and the
follow-ups it would file and writes nothing; `--commit-records` commits
everything it wrote in one commit.

The disposition is a field of the finding's own record: the decision, its
follow-up for a deferral, the identity that set it, the commit HEAD was
at, the time and the reason. Setting it again replaces it, so a finding is one
file for its whole life. The record keeps only the current disposition;
earlier ones, such as an author's proposal the reviewer replaced or a
`fix_now` later changed to `defer`, stay in the Git history of the finding's
file, which `review show` names (`git log -p -- .atelier/reviews/<id>.md`).

The re-review that resolves a `fix_now` finding may cover only the fix's diff
(`review request` lists the commits since the last review or approval), and
the reviewer who raised the finding may do it. A trivial exemption resolves
no finding. `--decision fix_now` stays accepted and is never asked for; an
older Atelier's FIX NOW disposition with a resolution commit is resolved, as
before, by an approval at or after that commit, and one without by the rule
above. DEFER requires an existing follow-up issue. Any identity may record a
DEFER, NO ACTION or WON'T FIX; the identity is recorded, not compared with
the work's Git authors. Who ought to decide is Agent Factory's rule, not a
gate. A deferral's follow-up may say it was discovered elsewhere.
A defer, no_action or wont_fix finding without a decision, or a fix_now
finding no current approval has resolved, blocks merge. `review list`,
`review show`, the review brief and the gates show a won't-fix decision as
`wont_fix`, and the [debrief](mission-reviews.md#debrief) counts it as
`won't fix`, apart from no_action's `left as is`.

On the base, every part of a review record but a finding's disposition is
append-only: editing it, or deleting a record, is refused, and merge examines
review history from the base, including edits subsequently reverted.

## Correcting review records

A finding, or a review record that is not an approval or trivial exemption,
can be edited or deleted by hand, by any agent on the branch, until it
reaches the base; commit the change with the work. Each change is
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
commit that made it (`land`, `doctor` and `merge` apply the same rule).
Since `merge` always creates a merge commit, this holds under every policy
(atelier-jnrh); a commit an earlier fast-forward put on the base's
first-parent line is judged at its own first parent, as above. When the base
branch cannot be read, every commit is judged at its own first parent and a
review record is final once committed. A change must leave a regular file,
and a removed separate disposition event is never added back.

Approvals and trivial exemptions stay append-only from when they are first
committed, on a branch as on the base: re-pinning an approval's commit, or
changing its target or identity, would move it onto code its reviewer never
saw. `land` restores an uncommitted edit to one, and refuses a committed edit
with `review_append_only`; record a new approval instead.

Identity is self-declared, so no rule says who may correct a finding: an
implementer could delete a reviewer's finding before merge, leaving no
decision on it. That cannot let unreviewed code merge, since approvals cannot
be edited and one still has to cover the code.

Repositories whose dispositions are separate events, from before ADR 0025,
keep working unmigrated: Atelier reads each finding's latest disposition
event as its disposition, with `migrated_from` naming the event, so check,
land, merge and doctor give the same results as after migration. `review disposition` on such a finding
writes the field and removes its events. `atelier migrate dispositions
--apply` is the optional cleanup: it writes that fold into every finding and
removes the events, on the base or a work branch, and keeps approvals
current; Git history keeps the events. Events that continue a folded
disposition, as a branch recorded with an older Atelier brings in, replace
it. Such a branch takes a base that has migrated with a plain merge: the
base never held the branch's later event, so the branch may still correct it
(atelier-ilba), and `atelier migrate dispositions --apply` on the branch then
folds it. Under `fast_forward_only`, before every merge was a merge commit
(atelier-jnrh), a review record was final once committed, so `land` refused
the plain merge with `review_append_only` and named a merge without
committing, migrated before the commit (atelier-j5oq); a branch whose
records are final, as when the base branch cannot be read, still gets that
advice. The read path for separate events goes once every repository has
migrated (atelier-42th). Separate events stay append-only as
before: one may be removed only in a commit whose finding carries its own
disposition and keeps none of its events, as `review disposition` and the
migration write it, and a removed event is never added back (atelier-pfbp).

Terminal targets and their eligible review clusters can leave the active tree
together through [protected prune](retention-and-prune.md); retained events on
the base remain immutable.

## Requesting a review

`atelier review request [<id>] [--identity <name>] [--from <rev>]` prints the
brief to give an independent reviewer, as dense Markdown (ADR 0022), so the
orchestrator does not hand-write it. `<id>` is the branch owner or one of its
children; run it from the owner's worktree. With no ID it reviews the
checked-out branch's owner, or, on a branch no issue owns, the branch itself
([the light path](#a-change-with-no-issue-the-light-path)); on the base it
refuses with `review_request_base`. The brief holds only what comes
from records, Git and policy:

- the owner's Goal and Done when, and each child's Done when;
- each Done when edit made after its item was started, as `merge --check`
  shows it (see [below](#a-done-when-edited-after-the-start));
- the range to review: the commits since the last `review` or `approve` on the
  owner (or after `--from`), and the whole branch since it left the base, with
  the full head commit to pin records to;
- every finding with its disposition, open ones first (a `fix_now` finding
  reads `open until an approval after its fix`, or `approved since by
  <approval>` once one resolves it, judged as `review list` judges it; a
  defer or no_action one `open, no decision`), earlier reviews and approvals,
  and the newest recorded proof;
- the working rules: the built-in ones (read-only on code, run the proof,
  approve only without `fix_now` findings, and that the approval of a later
  commit resolves the `fix_now` findings recorded before it), the base
  policy's `merge_checks` commands, and the text of the
  file named by `[review] rules_file` in `.atelier/config.toml`, a
  repository-relative Markdown path where a repository keeps rules the
  reviewer needs, such as where to put scratch files;
- the `review finding` and `review approve` commands, pinned to the head and
  using `--commit-records`, with `--identity` filled in when given.

It never includes the author's notes or Report: a reviewer judges the
change against the requirements, not the author's account of it, and reads
the Report in the record when it helps.

The request refuses first, with `review_request_unfinished`, while an open
work item it covers has an unticked `- [ ]` Done when line or an empty Report,
and lists each gap: asking for review means the work is implemented and every
line holds, to the best of the author's knowledge ([ADR
0028](../adr/0028-simpler-lifecycle.md), decision 2). It covers the item it
names; naming the owner covers the owner and its started children, so a
parent's children can be reviewed one at a time while an unstarted child,
not yet claimed implemented, waits (finding atelier-2ygf). A line without a
box is not asked for. A line with the old After merge marker
([ADR 0027](../adr/0027-proof-has-a-time.md)) need not be ticked, and the
refusal names `atelier migrate after-merge`, which turns such lines into
loose ends (see [work after the merge](validation.md#work-after-the-merge)).

## Approval freshness

An approval covers code (atelier-3jdm). It is stale when code changed since
the commit it pins, not counting a clean merge from the base. Code is
everything except the tracker records, the Markdown files under
`.atelier/issues`, `.atelier/reviews` and `.atelier/evidence`, so the workflow
policy (`.atelier/workflow.yaml`) and the project config
(`.atelier/config.toml`) count as code. Record edits never make an approval
stale, whatever record they touch: ticking the Done when, adding notes or
links, writing the Report, a status change, another issue's record, a
finding's disposition, a new receipt, a new child or follow-up. Each commit after the pin is examined, so changing
code and then restoring it does not recover an old approval, and a fix made
after a `fix_now` finding makes the approval stale, so the fix is reviewed
again. Review records on the base stay append-only apart from a finding's
disposition, but that is a rule of its own (`review_append_only`), not part
of freshness.

A Done when changed after the item was started is still shown to whoever
judges the work (see [A Done when edited after the
start](#a-done-when-edited-after-the-start)), since the approval no longer
covers it.

This is the one rule for what an approval or trivial exemption covers, and
`merge` (its `review.current` gate), `land` on the work branch, `land` on the
base and `status` apply it alike, through `review_gate::approval_is_current_to`.
Code committed after an approval makes it stale, and `merge` refuses before
integrating until a fresh approval is recorded at or after that commit
(atelier-n642, atelier-uw4x). The merge commit `merge` creates matches Git's
own merge of its parents outside the records, so its done marks keep the
approval current, and `land` on the base accepts them. The approved scope
still decides which done claims an approval covers. The scope is the owner
and its descendants; the owner's parent is outside it. `merge` also runs
`land`'s own judgement of the done claims the branch's commits make, as its
`claims.reviewed` gate: a done claim outside every current approval's scope,
such as an issue created already done after the approval, or another item
set done on the branch, fails it, naming the commit and the fix.

Merging the base into a reviewed branch keeps its approval when the merge is
clean: every other parent is already on the base, and the merge commit matches
the tree `git merge-tree` produces from its parents everywhere outside the
records, with no code conflict resolved and no code edited while merging
(atelier-n7g1). The base commits it brings in were reviewed on their own
branches. Record conflicts may be resolved any way, since records are not what
an approval covers; the gates then read the records as resolved, so a blocker
the resolution keeps still holds the merge. A merge that resolved or edited
code by hand, or one that brings in code from a branch not yet on the base,
makes the approval stale. Proof is different: the policy's `shell.check`s run
again on the merged result and prove what their commands run, a
[`ci.passed`](#what-cipassed-proves) run tested the head merged with an
earlier base, and nothing is recorded (see
[`--sync`](#catching-up-with---sync)).

Git's own merge here is `git merge-tree` with the repository's configuration
and attributes, so a custom merge driver (for example `merge=ours` on a
changelog) counts as part of "Git's own merge": a merge it produces is clean
even if it drops one side's change. Review merge drivers like other policy
(atelier-7hh4). An approval recorded before the work branch left the base, such
as one on the base itself, never counts for the branch.

When `review.current` fails, it names the approval it considered and why it
does not count (atelier-cisa): the newest stale approval or exemption. One
function,
`review_gate::approval_standing`, judges and explains it for `land`, `merge`,
`merge --check`, `merge --sync` and `review list`:

```text
⚠ atelier-7l79 review.current: approval atelier-x16l @e2562bd6 is stale: 1a2b3c4d "Tweak after approval" changed proof-sample.txt. Next: atelier review request atelier-7l79
⚠ atelier-bw0b review.current: approval atelier-x16l @e2562bd6 is stale: base merge 4f1e2a3b resolved or edited src/gate.rs by hand. Next: atelier review request atelier-bw0b
```

A stale line names the first commit that made the approval stale, with up to
three of the code paths it changed and `(and <n> later commits)`; records it
changed are not listed. A merge reads `base merge <commit>` (or `merge
<commit>` when it brings in a branch not on the base) with the code it
resolved or edited by hand, or the code it changed when it did neither. An approval that predates the point where the branch left the base
says so. With no approval at all, the line reads `no independent approval or
trivial exemption`.

## Integration

Commit the implementation, any receipts, branch ownership, the ticked Done
when and the Report on the work branch. Merge marks the owner and its open
children done; none of them may have an open transitive blocker, and the done
marks must pass the `work` workflow's done validators. No gate reads evidence:
a failing receipt is a note and blocks nothing (ADR 0028 decision 5). Review
must have a current independent approval or an explicit trivial exemption, a
decision on each deferred or dismissed finding, and an approval since each
`fix_now` finding.

```text
atelier merge <owner-or-child-id> --check
atelier merge <owner-or-child-id> --check --json
atelier merge <owner-or-child-id>
atelier merge <owner-or-child-id> --sync
atelier merge <owner-or-child-id> --push [--wait [--timeout <minutes>]]
atelier merge <owner-or-child-id> --wait [--timeout <minutes>]
```

`--push` and `--wait` need a forge in the policy; see [CI on a
forge](#ci-on-a-forge). Run these from the recorded work branch. With no ID, `merge` acts on the
checked-out branch's owner, or, on a branch no issue owns, merges the branch
itself on an approval of its head ([the light
path](#a-change-with-no-issue-the-light-path)); on the base it refuses with
`merge_base_branch`. From anywhere else, `merge` refuses
with `merge_branch_mismatch`, naming the work branch and `git switch
<branch>`; on the base, where the records learn of the branch only when the
work merges, it finds the branch from the local branches' own records. An item
no branch owns refuses with `merge_owner_unstarted` and `atelier start <id>`.
Once the work is on the base, `merge` and `merge --check` print `<id> is
already merged into <base> (<commit>).` and pass (atelier-yrlj). Source and any
separate base checkout must have no uncommitted changes to tracked files. An
untracked file is no commit's, so merge leaves it where it is, unless
integrating would write a file at its path, or a file in place of a directory
holding it: that refuses with `merge_untracked_overwritten`, naming the paths,
before anything changes (atelier-l457). Canonical record and policy paths, bytes,
and modes must match committed HEAD, including ignored canonical files; Git
index flags such as assume-unchanged or skip-worktree cannot hide changed
tracked files. Other files follow Git status: with `core.filemode=false`, a
local execute-bit change is not an uncommitted change. The gates
cannot authorize a different branch, HEAD, base, or canonical snapshot while
they run. `--check` evaluates the gates without integrating.

A failed precondition does not hide the gates: uncommitted records
(`canonical_snapshot_uncommitted`, every record named), uncommitted changes
to tracked code (`merge_checkout_dirty`, every path named), an untracked file
merging would overwrite (`merge_untracked_overwritten`) and an invalid candidate policy
(its `workflow_*` code) are each one more failed gate with its fix, and the
gates still run, against HEAD's committed records when the checkout's differ
and from where the branch left the base. A real merge still refuses before
integrating. A precondition the gates cannot run past, such as the wrong
branch or a merge conflict, is reported after the ones found before it, with
a `Not checked:` line naming the gates it skipped (atelier-kiry).

Merge integrates into the `branch_policy.base_branch` of the branch's
committed policy, which must match the base branch's own committed policy
(`merge_base_policy_unsupported` otherwise); a `branch_base` an owner record
carries is not read (ADR 0028 decision 5). It uses the base branch's
committed policy, and always creates a merge
commit, so a base that moved on needs no catch-up merge on the branch first
([ADR 0028](../adr/0028-simpler-lifecycle.md), decision 2). An older policy's
`merge_strategy: fast_forward_only` or `squash` is read as `merge_commit` (see
[workflow configuration](workflow-configuration.md)). The merge records no
new proof: the policy's `shell.check`s run on the merged tree and prove what
their commands run (see [what gets a receipt](validation.md#what-gets-a-receipt)),
a [`ci.passed`](#what-cipassed-proves) run tested the head merged with an
earlier base, and a repository with neither merges a tree nobody ran.

The merge commit marks the owner's open work done: every item of the `work`
workflow in the owner's scope that is not yet in a done status takes the
workflow's first done status, with nothing else changed, so the Report stays
as the reviewer read it. Its message names them (`Marks <ids> done.`), as
does the success line. An item an older Atelier marked done on its branch is left
as it is, and so is a superseded child. For each item it marks done that has
an unticked box (After merge lines aside) or an empty Report, `merge` and
`merge --check` print a note after the gate lines, and refuse nothing, as
for an edited Done when:

```
Note: merge marks atelier-2xx5 done with 1 Done when line unticked: "It passes on Linux."; Report empty.
```

Review asked for both, but a record edit after approval does not make the
approval stale, and a child reviewed on its own leaves its siblings unasked,
so this note is how such work is seen before it merges (finding
atelier-nrq4).

### A Done when edited after the start

`review request` and `merge` and `merge --check` show each change to the
text of the Done when of the owner or a scoped child made after the item was
started, whatever the verdict (atelier-r9hb, atelier-jnrh): the brief under
`Done when edited after start`, merge after the gate lines:

```
Note: atelier-2xx5 Done when edited at 4adb9ff6 after the item was started, after finding atelier-h7l3.
```

The trigger is the start, not a finding: in the usage review that motivated
it, most narrowed criteria had no finding recorded before them. A change
counts when HEAD holds it and the commit that moved the item into an `active`
status, such as `atelier start` or the `start` transition, does not, so an
edit made on the base after the start and merged into the branch is shown
like one made on the branch (finding atelier-l5fb); clock times are not
compared. Ticking or unticking boxes changes no text and is not shown. A
merge counts only where its Done when differs from every parent's, as when a
conflict resolution writes new text; a clean merge brings in commits that are
listed themselves. The line names the newest finding or `fail` receipt on the
item pinned to an ancestor of the edit, when there is one, so the reader can
judge whether the edit answers it or loosens the criterion. The approval does
not cover the edit, since record edits never make it stale, so this line is
how the edit is seen, including one made after approval.

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
branch still merges with `atelier merge`. It cannot be combined with `--check`
or `--json`.

- **Conflicts.** `git merge` stops, nothing is committed, and the output lists
  the conflicted files (`sync_conflict`). Resolve and commit them, or
  `git merge --abort`. Running `--sync` again after the resolution commit
  finds nothing to merge and reports the approval, judged from where the
  branch stood before the merge.
- **No receipts.** The sync runs and records no evidence. At merge, the
  policy's `shell.check`s run on the merged tree and prove what their commands
  run, and [`ci.passed`](#what-cipassed-proves) judges a pull request run of
  the head merged with an earlier base. A repository whose `shell.check`s run
  no suite runs nothing on the merged tree until it adds one to
  `merge_checks` ([ADR 0026](../adr/0026-evidence-records-only-what-nothing-else-records.md)).
  Before atelier-s66u the sync reran the command-backed receipts that were
  current before it and committed them.
- **Approval.** The output says `Approval kept`, judged as `review.current`
  judges it (`review_gate::approval_standing`), or `Approval needs fresh
  review` with the first commit that made it stale and the code paths of
  every stale commit to review, for a merge the code it resolved or edited by
  hand. A stale approval does
  not refuse the sync, which exits zero, but the verdict reads `merge: synced
  — approval needs fresh review` instead of `passed`, because `merge --check`
  will refuse until a fresh approval is recorded.
- **When it matters.** Merge integrates a clean branch on a moved base itself, so
  `--sync` matters for a conflicting branch or when merge refuses with
  `merge_base_records`. Under [`ci.passed`](#what-cipassed-proves) a moved
  base needs no sync and no new CI run; a sync that brings in code changes the
  head's code, so it needs a new run.

A branch that is behind the base merges without catching up. Merge builds
Git's merge of the work branch into the current base with `git merge-tree
--write-tree` and refuses with `merge_conflict`, naming the files, if any path
conflicts; resolve those on the work branch as above. It then writes the done
marks into that tree, and the merge checks run in a temporary detached checkout of
the resulting merge commit, never in the work checkout. Evidence at the branch
head does not prove the merged tree; only a `shell.check` run there does, and
record validators such as `lint.none_blocking` read records and prove nothing
about the tree. With at least one `shell.check`, the gate output adds
`merge.merged_tree`, which passes only when every shell check passed there and
names the tree, base commit, and commands, and the merge commit's message
lists those commands. A policy without one merges a tree nothing ran, and the
merge commit's message says so. This merged-tree rule covers the local
`shell.check`s. [`ci.passed`](#what-cipassed-proves) is not a shell check and
is not part of `merge.merged_tree`: it accepts a pull request run, which
tested the head merged with an earlier base, and its line says so.

Each checkout lives in its own `atelier-merge-*` directory in the Git common
directory, and the merge holds a lock on that directory's `lock` file while its
merge checks run. Normal exit removes the checkout. If merge is killed during
them, the operating system drops the lock, and the next merge removes every
`atelier-merge-*` checkout whose lock is free before it makes its own; one a
concurrent merge still holds stays (atelier-rcnk).

Merge checks do not reuse a build cache. A `shell.check` that builds does so
in the checkout's own target directory (for cargo, by setting
`CARGO_TARGET_DIR=target` in its command), which goes away with the checkout,
so it proves what the merged tree builds and never what an earlier tree left
behind.
A shared target directory is the one cache Atelier does not allow here: it has
run another branch's binary in tests, and cargo's freshness rules cannot tell
two checkouts apart. A policy that wants faster merge checks may put a
compiler cache keyed by its inputs, such as `sccache`, in the command itself.

The gates read the records of the merge commit, which is what the base would
get, so a blocker the base added or reopened holds the merge. Review and
evidence pins are judged only along the work branch, though, so merge refuses
with `merge_base_records`, naming the files, when the base changed an in-scope
issue, or a review or evidence record about one, since the branch left it:
merge the base into the work branch so the gates judge them, then retry. Other
base records do not touch the approval, which is judged on the work branch's
history: it carries over, because a clean merge combines two reviewed changes
and nobody edited the result. Merge also judges the done claims outside the
item that the merge commit can affect, as `land` does, trusting the base
head; no checkout holds that commit before it reaches the base
(atelier-bu66). For example, if the base
closed a mission after the fork point and the branch reopens or adds work that
mission depends on, merge refuses, naming the claim (`merge: refused — 1
check (<mission> work.terminal)`, atelier-m4gv). These claims are read at that commit, so a
claim the base completed and reviewed after the fork point is judged against
the base commits its reviews pin (atelier-goom). The merge commit reaches the
base only when every gate passes and the base has not moved during the merge
checks. When the base moved during them only by commits that change nothing but
issue, review and evidence records, merge reports `merge_base_moved` and runs
the gates again against the new base, up to three attempts in all; a record the
item itself depends on still refuses as `merge_base_records`. If any of those
commits changes anything else, or the base was rewritten, merge refuses with
`merge_snapshot_changed`, naming the first such commit and file. A lockfile that merges cleanly but is
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

Merge creates the merge commit and no other commit; the only records it writes
are the done marks in that commit; and without `--push` it pushes nothing. Git hooks
are disabled during gated integration and cleanup; when the repository has
hooks of its own, merge says they did not run. Put required read-only commands
in the reviewed base policy's `merge_checks`. `merge` and `merge --check`
print failed gates first, each with its fix, and end with a verdict (see
[gate lines](human-cli-output.md#gate-lines)).

Successful integration deletes the local work branch. If the base has a
separate checkout, the source checkout becomes detached and the command prints
its worktree cleanup instruction. Leave that checkout before removing it from
the base checkout. If cleanup fails after integration, inspect the named Git
state before retrying cleanup. Use [prune](retention-and-prune.md) for remaining
eligible artifacts.

## CI on a forge

A repository whose policy names a [forge](workflow-configuration.md#forge)
lets `merge` push branches, open pull requests and read CI through GitHub's
`gh` CLI ([ADR 0031](../adr/0031-a-forge-carries-branches-and-ci.md)). Review
stays here: the pull request carries the branch and runs CI, and nothing on it
but a CI result for a commit feeds a gate. Without a `forge` block, Atelier
makes none of these forge calls and none of this applies. What the policy and
the agent ask for explicitly keeps its behaviour: a `shell.check` that reaches
the network, such as this repository's `scripts/merge-ci-passed.sh`, still
does, and so do `land --push` and the update check.

### What `ci.passed` proves

`ci.passed` is a merge check that passes when the code that merges passed the
trusted CI workflow (`forge.ci_workflow`) on the branch's pull request. Code
is everything outside `.atelier/issues`, `.atelier/reviews`,
`.atelier/evidence` and `.atelier/runtime`; the workflow policy and config
count as code. It passes in two cases:

- **The branch changes no code** since it left the base. It needs no CI run,
  reads nothing from the forge and needs no `gh`:

  ```text
  ✓ atelier-2xx5 ci.passed: the branch changes only records, so it needs no CI run.
  ```

- **A pull request run passed for the head's code.** Walking back from the
  branch head while each commit's code matches the head's, some commit has a
  run of the trusted workflow, for a `pull_request` event, that completed
  successfully. Commits after that run that change only records, such as an
  approval, need no new run. A `push` run does not count, since it tested the
  head alone.

A pull request run tested GitHub's merge of that commit with the base as it
was when the run started (`refs/pull/<n>/merge`), not the tree this merge
makes. Atelier accepts it as the CI gate, so a base that moved since the run
needs no new run, and the pass line says what the run tested. When the base
has gained no code since the branch left it, the run tested the code that
merges:

```text
✓ atelier-2xx5 ci.passed: ci.yml passed on pull request #42 for 1a2b3c4d, which has the branch head's code, and master has gained no code since the branch left it: https://github.com/<owner>/<repo>/actions/runs/<run-id>
```

When the base has gained code since, it says that the merged combination is
untested until CI runs on the base after the push:

```text
✓ atelier-2xx5 ci.passed: ci.yml passed on pull request #42 for 1a2b3c4d, which has the branch head's code: https://github.com/<owner>/<repo>/actions/runs/<run-id>. That run tested the head merged with master as it was when the run started, not the tree this merge makes. master has gained code since the branch left it, and CI on master after the push covers the combination.
```

Other passes collapse into one line ([gate
lines](human-cli-output.md#gate-lines)), but `ci.passed` prints its pass on a
line of its own, because it says what the run tested, and the merge commit's
message repeats it. The local `shell.check`s still run on the merged tree and
catch what their commands check there, and nothing more: a policy whose
checks build or test catches a combination that breaks those, while one that
runs only whitespace and formatting checks, as this repository's does, leaves
the build of the combination to CI on the base after the push.

Each refusal is a gate line with its Next step. `<id>` is left out of a Next
command on [the light path](#a-change-with-no-issue-the-light-path). A Next
command that reruns `merge` keeps `--push` whenever the refused run had it,
shown below as `[--push]`, since `merge --wait` alone merges without
publishing the base and deletes the local branch `--push` would have pushed
from.

| Code | When | Next |
| --- | --- | --- |
| `forge_gh_missing` | `gh` is not on `PATH`. | Install the GitHub CLI (https://cli.github.com), then `gh auth login`. |
| `forge_gh_signed_out` | `gh` is not signed in to github.com. | `gh auth login` |
| `forge_unreachable` | `gh` failed for another reason, such as the network; the line quotes its message. | Retry; `gh auth status` shows the sign-in. |
| `ci_no_run` | The branch has no pull request into the base, or no `pull_request` run of the trusted workflow exists for a commit with the head's code. The line says which. | `atelier merge <id> --push --wait` |
| `ci_pending` | The newest such run is queued or in progress, and none passed, or `--wait` timed out. The line names its state and link, and how long it waited. | `atelier merge <id> [--push] --wait` |
| `ci_failed` | The newest such run failed, and none passed. The line names its conclusion and link. | `gh run view <run-id> --log-failed`; fix it on the branch, commit, then `atelier merge <id> --push --wait`. A cancelled or timed-out run instead says `gh run rerun <run-id>`. |

For example, after `atelier merge atelier-2xx5 --push`:

```text
⚠ atelier-2xx5 ci.passed: ci.yml is in progress on pull request #42 for 1a2b3c4d: https://github.com/<owner>/<repo>/actions/runs/<run-id>. Next: atelier merge atelier-2xx5 --push --wait
```

`merge`, `merge --check` and `merge --push` also show the trusted workflow's
newest completed run for the remote base's head when it failed, as a note
after the gate lines. It refuses nothing, since the merge may be the fix:

```text
Note: ci.yml failed on master at 5e6f7a8b: https://github.com/<owner>/<repo>/actions/runs/<run-id>. It refuses nothing.
```

`ci.passed` trusts the workflow file the base policy names, but a pull request
run uses the branch's copy of that file. A branch that edits it changes code,
and its review covers the edit.

### `merge --push`, step by step

```text
atelier merge <id> --push
atelier merge <id> --push --wait
atelier merge <id> --push --wait --timeout 45
```

`--push` takes an approved branch to the published base in one command. Run
it from the work branch, as `merge`. In a policy without a `forge` block it
refuses with `merge_push_no_forge` and pushes nothing. It does this, in order:

1. **Checks `gh`.** If `gh` is missing or signed out, it refuses with
   `forge_gh_missing` or `forge_gh_signed_out` and their Next lines above,
   before pushing anything. A refusal that comes before any gate, such as the
   wrong branch, also pushes nothing.
2. **Pushes the branch.** It pushes the committed branch head to
   `forge.remote` under the branch's name, never forcing, and sets the
   branch's upstream when it has none and the remote is a name. A rejected
   push refuses with `merge_push_branch_rejected` and git's message: the
   remote branch has commits this one lacks. Next: `git fetch <remote>
   <branch> && git merge --no-edit FETCH_HEAD`, then `atelier merge <id>
   --push`, with `<remote>` the value of `forge.remote` as written, a name or
   a URL. Fetching into `FETCH_HEAD` needs no remote-tracking branch, so it
   works for a URL remote too.
3. **Ensures a pull request.** It reuses an open pull request from the branch
   into the base, or opens one titled `<id>: <title>` (on the light path, the
   branch's newest commit subject), whose body names the item and says review
   is recorded in `.atelier/reviews`, not on the pull request. It prints
   `Pull request #42: <url>`, saying whether it opened it.

   Steps 2 and 3 happen even when a later gate refuses. Pushing a work branch
   is always allowed, and it starts CI early, so `merge --push` can run before
   the approval is in.
4. **Fetches the base.** It fetches the base from `forge.remote` into
   `FETCH_HEAD`, with no remote-tracking branch needed. If the local base is
   behind, it fast-forwards it. If the two have diverged, it refuses with
   `merge_push_base_diverged`, naming both heads. Next: in the base checkout,
   `git fetch <remote> <base> && git merge --no-edit FETCH_HEAD` and `atelier
   land`, then `git push <remote> <base>`, then back on the work branch
   `atelier merge <id> --push`.
5. **Judges the gates,** as `merge` does, `ci.passed` included, and refuses as
   `merge` does when one fails; the branch and pull request stay. Right after
   step 2 pushed a new head, a run that has not started yet reads as pending,
   since GitHub starts runs a few seconds after a push.
6. **Merges,** as `merge` does: the merge commit, the `shell.check`s on the
   merged tree, the done marks, and the local branch deleted. It never deletes
   the remote branch.
7. **Pushes the base.** It judges the unpushed base commits as `atelier land`
   does on the base, then pushes the base to `forge.remote`, never forcing. A
   rejected push, when the remote base moved during the merge, refuses with
   `merge_push_base_rejected` and git's message. The merge commit stays on the
   local base and nothing is undone. Next: in the base checkout, `git fetch
   <remote> <base> && git merge --no-edit FETCH_HEAD`, then `atelier land`,
   then `git push <remote> <base>`. `atelier land --push` is not the fix: it
   pushes to the base's configured upstream, which need not be
   `forge.remote`.
8. **Confirms the pull request merged.** GitHub marks a pull request merged
   when its head commits become reachable from the base, which the pushed
   merge commit does. Atelier reads its state for up to a minute. The success
   line reads `merge: passed — pushed master to origin, pull request #42
   merged`. If GitHub has not marked it merged, `merge` still exits zero,
   since the base is published, and says so:

   ```text
   Note: pushed master to origin at 9c0d1e2f; pull request #42 is still open because GitHub has not marked it merged. Atelier does not close it, since closed is not merged. Next: gh pr view 42
   ```

### Waiting for CI with `--wait`

Without `--wait`, `merge` and `merge --push` refuse at once when CI is
pending, naming its state and the run's link, so an agent decides how to
wait. With `--wait`, they judge the other gates first and refuse without
waiting when one of those fails. Otherwise they poll CI every 30 seconds,
printing a line when its state changes, until the run finishes or
`--timeout` minutes pass (default 30). A run that passes continues the merge;
a failed run refuses with `ci_failed`; a timeout refuses with `ci_pending`,
naming how long it waited, and its Next keeps `--push` when it was given. A
run that has not started counts as pending for its first 5 minutes after the
push, then refuses with `ci_no_run`.

`--wait` without `--push` waits for a run on a branch already pushed, then
merges without pushing. `--push` and `--wait` do not combine with `--check`,
`--json` or `--sync`, and `--timeout` needs `--wait`.

## Planning changes on the base

Filing, grooming, re-prioritising, recording decisions (including deciding a
`decision` record), and editing mission strategies are commits too. Commits that change only records
(`.atelier/issues`, `.atelier/evidence`, `.atelier/reviews`) and the strategy
files linked from missions may be committed on the base branch directly
(atelier-3mjv, decided by Sumeet 2026-10-15). Any other change goes through a
work branch, review, and `atelier merge`, or, when its correctness is evident
from the diff, is exempted on the base as a [trivial
change](#trivial-changes-on-the-base).

`atelier land` on the base branch enforces this before you push. It looks at
the base commits that are not yet on the base's upstream. A commit counts as
reviewed when a commit pinned by an approval or trivial exemption contains it,
for completed started owners that are not already done upstream, and only an
approval `merge` would accept counts (atelier-n90q): one as first committed,
whose identity is recorded and not compared with any author (atelier-md0v).
Every finding pinned at or before it must be settled by the rules merge
applies: a defer or no_action finding has a decision; a `fix_now` finding is
resolved by this approval when it pins a later commit holding the finding's
record and was recorded after the finding, or, for an older Atelier's FIX NOW
disposition, when its
resolution commit is at or before the approval's pin. That is what `atelier merge` integrates. A commit a [base
exemption](#trivial-changes-on-the-base) pins is covered too, and so are the
commits an approval of a [light-path branch](#a-change-with-no-issue-the-light-path)
covers, though their done claims are still judged. Every other commit may touch only records and the
strategies linked from missions, or `land` refuses with
`land_base_unreviewed`. Those record-only commits also may not take a work
item to done: work is done by the merge that integrates it, so `land` refuses
a done claim made outside review with `land_base_done_claim`. The merge
commit's done marks are covered by the approval `merge` accepted, which is
current at that commit by the rule in [Approval
freshness](#approval-freshness), as is a claim an older Atelier made after an
approval's pin. Revert an outside claim (a claim no longer done at the tip is
withdrawn), then `atelier start` the item and merge it (atelier-fj1t).
Closing a mission, deciding a `decision` record, and superseding or
[declining](work-model.md#completion) work stay allowed on the base. A merge commit, such as a non-rebasing pull or the one
`atelier merge` creates, is judged by the paths that differ from all of its
parents and from Git's own merge of them, so a line Git merged cleanly is not
counted, while a conflict resolved by hand is (atelier-jnrh). The install predates the gates and is exempt: commits
made before the policy existed, the first commit on the base's first-parent
line that adds it, and everything that commit merges in. Without an upstream, `land`
notes that it cannot judge base commits.

`atelier start` applies the same rule on the local base branch, using the
base's own records, whether it creates the work branch or switches to an
existing one. A branch started while the base holds an unreviewed commit
would carry it in, and a review of `<base>...<branch>` would never show it.
`start` skips it when the base has no upstream or no workflow policy
yet, as when Atelier is being adopted on a branch. To
recover, move the commit to a work branch (`git switch -c <id>`, then
`git branch -f <base> <base>@{upstream}`) and run `atelier start <id>` there.
`land` guards a handoff; it is not an access control. Review identities are
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
and it covers no done claim, which only a merge of the work makes
(`land_base_done_claim`). `review list` and `review show` show it, and a
mission's debrief shows each base exemption recorded between the mission's
start and its done claim on its timeline, because it names no mission.

Use it for one commit whose correctness is evident from its diff, already on
the base. Anything that deserves a review but has no issue, such as a
release, a docs change or a mockup, goes on a branch and takes [the light
path](#a-change-with-no-issue-the-light-path) instead.

## A change with no issue: the light path

A release, a docs edit or a mockup change gets an independent review without
an issue around it: no create, `start`, Done when or Report ([ADR
0028](../adr/0028-simpler-lifecycle.md), decision 6, atelier-4nsl). Commit it
on a branch no issue owns, then, from that branch:

```text
atelier review request
atelier review finding --branch <name> --identity "Reviewer" --recommendation fix_now "Problem" --commit-records
atelier review approve --branch <name> --identity "Reviewer" "What was reviewed" --commit-records
atelier merge --check
atelier merge
```

- **Which branches.** A branch is owned while an issue that is not done on
  the base records it as its work branch; an owner already done on the base
  no longer owns the name, so a branch that reuses it is on the light path.
  The base is never on it. On an owned branch, `review request` and `merge`
  with no ID act on the owner, and records about the branch count for
  nothing.
- **The brief.** `review request` with no ID prints the branch's commits since
  it left the base: the range, each commit's subject, the diff stat, earlier
  findings and reviews of the branch, the merge checks, the working rules,
  and the `--branch` commands to record findings and an approval. It has no
  Goal or Done when, so the reviewer judges each change from its commits. A
  branch with no commits since the base refuses with `review_request_empty`.
- **Records.** A record about a branch is an ordinary review record whose
  front matter names `branch: <name>` instead of a `target`, at review schema
  version 3; an Atelier that predates the light path reports such a record as
  needing a newer Atelier and refuses to load the repository's records until
  it is upgraded, so a repository whose CI pins an older binary waits for the
  pin to move. Records about issues are still written at version 2. It is written only from a checkout of that branch, unowned
  (`review_branch_owned` names the owner otherwise), and pinned after the
  branch left the base (`review_branch_pin`). `review disposition` decides a
  finding about a branch by its ID, and `review list` and `review show` show
  `on branch <name>`. There is no trivial exemption of a branch: an approval
  is the only cover. Records about a branch are never pruned, and a deferred
  finding's follow-up issue is kept while they are.
- **The current run.** A record counts only when it is pinned to a commit the
  branch holds and the point where it left the base does not, so a later
  branch named `release` inherits none of an earlier `release` branch's
  findings or approvals. Approvals stay as first committed and go stale only
  on code changes, as for an issue.
- **Merge.** `atelier merge` with no ID requires a current independent
  approval of the branch head, judges the branch's findings as
  `findings.disposed` judges an issue's, runs the base policy's
  `merge_checks` on the merged tree (a check that reads an issue passes,
  since there is none) and the `claims.reviewed` gate, and writes no issue
  state: the merge commit's tree is Git's own merge of the base and the
  branch head. `atelier merge --sync` with no ID catches the branch up and
  reports whether its approval survived.
- **No done claim.** An approval of a branch covers its code and never a done
  claim: a light-path branch whose commits mark an issue done is refused by
  `claims.reviewed`, and on the base by `land_base_done_claim`. When
  `atelier start` takes a branch that holds records about itself, it notes
  that they stop counting, and the branch follows the issue path.
- **On the base.** `land` counts the branch's commits as reviewed when its
  approval is one merge would have accepted: it reached the base through a
  merge, no issue owned the branch when it merged, the approval is as first
  committed and every finding of its run pinned at or before it is settled.
  `atelier check` and `doctor` accept the records.
- **A release.** File the tag as a [loose end](validation.md#work-after-the-merge)
  on the release branch with no blocker: it reaches the base with the merge,
  where it is ready, and cannot be forgotten. It also shows as ready on the
  branch, which is harmless.

`atelier merge` is the documented path. A plain `git merge` of an approved
light-path branch passes `land` without running `merge_checks`, because
`land` cannot tell how a merge commit was made; the approval still covers
the code, and CI runs on the base (an accepted risk, ADR 0028 decision 6).
