# ADR 0028: The Simpler Lifecycle

## Status

Accepted (2026-11-07, within the authority of mission atelier-mmpv's strategy,
docs/strategies/simpler-lifecycle.md, on Sumeet's direction: he made these
decisions on 2026-11-06 and 2026-11-07; atelier-4yqd). Sumeet reads it after
merge.

Supersedes [ADR 0027](0027-proof-has-a-time.md), and the parts of
[ADR 0016](0016-canonical-work-branches-and-mission-integration.md),
[ADR 0019](0019-verified-repo-native-tracker.md),
[ADR 0021](0021-parent-issues-replace-epics.md),
[ADR 0025](0025-findings-carry-their-disposition.md),
[ADR 0026](0026-evidence-records-only-what-nothing-else-records.md) and
[PRODUCT_INTENT.md](../../PRODUCT_INTENT.md) that [Supersession](#supersession)
names. The rest of each stands.

Decisions 1 to 8 take effect when the mission item named for each merges.
Until then the code, the product docs and the PRODUCT_INTENT.md principles
they supersede describe today's behaviour, and that item updates them.
Decision 9 takes effect with this ADR, and decision 10 binds every item. The
lifecycle item, atelier-jnrh, amended decision 2 with the mechanics it
settled; atelier-ml3r corrected decision 2 on what the review brief leaves
out and named atelier-nn67 for the work left in the other repositories.

## Governing Principle

Atelier exists so that agents produce high-quality, correct code that solves
real problems, with as little red tape and overhead as possible. Nothing else
about Atelier is sacred, and this purpose governs every tradeoff below and
after. When qualities conflict, fewer concepts win over finer guarantees, and
refusing before a write wins over refusing after it.

## Context

An analysis of every Claude session on this host that used Atelier, about 110
sessions and 1,100 subagent transcripts across five repositories from
2026-10-15 to 2026-11-07, found two parts that earn their keep: independent
review, which found real bugs in nearly every round, and issue and mission
records, which work as shared memory between agents.

Nearly everything else that cost agents traced to one design. An approval was
pinned to the bytes of a history that also held the tracker's own records, so
any record edit could make it stale. Atelier grew a list of allowed edits,
ordering rules around it, and a new exception each time a rule hit an edge.
The results:

- Bookkeeping made approvals stale dozens of times, still on 2026-11-06 in
  four repositories, and the re-reviews almost always found nothing changed.
- Three sources gave three different orders of steps. An agent reading only
  the skill, AGENTS.md and the CLI help could not state the order, and every
  November orchestrator closed after approval although the skill said to
  close before review.
- Invalid records were accepted and then blocked every command, while
  append-only rules stopped their authors fixing them: a 5-hour astromech
  stall and a force-push of main on 2026-11-05.
- Releases, docs and mockups took the same ceremony as code, and 70–82% of
  commits touched only records. A release tag was lost because it lived as
  prose in a Close Reason.
- A new repository (life-on-rails) hit about ten refusals between `atelier
  init` and its first merge, and "check" alone meant at least six things.

The lighter-workflow mission (atelier-n4mj; ADRs 0025, 0026 and 0027) trimmed
records at the edges. This decision replaces the layer that produces them.

## Decision

### 1. Approvals cover code

An approval is stale when anything except tracker records (`.atelier/issues`,
`.atelier/reviews`, `.atelier/evidence`) changed since the approved commit,
not counting a clean merge from the base. Workflow policy and config
(`.atelier/workflow.yaml`, `.atelier/config.toml`) count as code. Edits to
tracker records never make an approval stale. The list of allowed record
edits, and the rules built on it, go.

A Done when line whose text changed after start is still shown to the
reviewer (atelier-r9hb), including after approval, and is not refused.
Ticking a box alone is not reported.

Built by atelier-3jdm (freshness) and atelier-jnrh (the Done when report).

### 2. One order, enforced where it happens

The order is: start, implement, tick every Done when line, review, fix and
re-review, approve, merge.

- `atelier review request` refuses while any Done when line is unticked.
  Asking for review means the work is implemented and every line is done to
  the best of the author's knowledge.
- `atelier merge` marks the issue done, so no separate close step can end
  up in the wrong place. An issue is done when its change is merged.
- A mission is closed when all its work is merged and its validation, if it
  has one, is done.
- The author writes the implementer report before review. The review brief
  leaves it out, since Agent Factory keeps the author's self-assessment from
  the reviewer; a reviewer of the finished mission reads it in the debrief.

Mechanics, as atelier-jnrh built them:

- **The report.** One section, `Report`, says what happened for every
  transition that requires the policy's `close_reason` field: the implementer
  report on a work item, a supersede reason, a mission's close report and a
  decision's `decide`. The author writes the implementer report in the item's
  `## Report` before requesting review, and the brief leaves it out. The
  policy key keeps its name and now means that section; a record's older
  `## Close Reason` heading is still read, and records are written with
  `## Report`. Atelier requires only a non-empty Report. Its parts
  (Deviations, Decisions, Surprises, Open items) are Agent Factory guidance,
  so a policy's `report.*` required fields are read as `close_reason`.
- **Asking for review.** `atelier review request` refuses while an open item
  of the `work` workflow it covers has an unticked `- [ ]` Done when line or
  an empty Report, and lists each. It covers the item it names; naming the
  owner covers the owner and its started children. An unstarted child is not
  claimed implemented, so a parent's children can still be reviewed one at a
  time (finding atelier-2ygf). A line with no box is not a checklist item and
  is not checked. A line carrying the old After merge marker need not be
  ticked; the refusal names `atelier migrate after-merge` for such lines
  (atelier-8bh4).
- **Merging marks done.** `atelier merge` always creates a merge commit, and
  that commit sets every open item of the `work` workflow in the owner's
  scope, the owner and its non-superseded children, to the workflow's first
  done status. Nothing is committed on the branch, and the Report stays as
  the reviewer read it. Merge refuses while one of those items has an open
  blocker, and still refuses without a current independent approval. One
  merge marks a parent and its children done, so the rule that a parent
  closes after its children goes, and the `work` workflow has no `close`;
  `supersede` remains. `merge` and `merge --check` note, and do not refuse,
  each item they mark done that has an unticked box (After merge lines
  aside) or an empty Report, so work review did not cover, or a record edit
  after approval undid, is still seen (finding atelier-nrq4). The merge commit matches Git's own merge of its
  parents everywhere outside the records, so it keeps an approval current,
  and `land` on the base accepts the done marks it carries.
- **One merge strategy.** `merge_commit` is the only strategy:
  `fast_forward_only`, `squash` and a missing `merge_strategy` are read as
  it, and it no longer needs a shell merge check (`merge_unproven` goes).
  This gives up linear history and the `merge_diverged` refusal in astromech,
  demiurge and cosplay-studio, and a repository without merge checks or CI on
  the merged tree now lands a merged tree nobody ran. In exchange no branch
  needs a catch-up merge before landing; astromech hit `merge_diverged` about
  56 times since 2026-10-15. Rejected: committing the done state on top of
  the branch head and fast-forwarding, which keeps two strategies and adds a
  second kind of commit Atelier writes. atelier-nn67 tells those
  repositories.
- **"Atelier never creates commits"** now reads: Atelier creates one commit,
  the merge commit `atelier merge` integrates reviewed work with, which also
  marks that work done. Every other commit, records included, is the
  agent's.
- **Work closed by an older Atelier.** An item already done on an unmerged
  branch merges as it is: merge marks only open work. A repository whose
  policy still has a work `close` keeps working until atelier-nn67 updates
  it, and the done marks merge writes answer to that close's required fields
  and validators.
- **A changed Done when.** A change to a Done when line's text after start is
  shown in the review request brief and in `merge --check`, including after
  approval and including a change merged in from the base, and refuses
  nothing. Ticking boxes is not a change.
- **Missions.** A mission closes when all its work is merged and its
  validation, if it has one, is done. Its `close` description says so; it is
  not a new refusal.

### 3. Loose ends replace after-merge checks

Work that changes no code and can only happen after a merge or a release,
such as pushing a tag, a play-test on a device or an eval run, is a **loose
end**: an issue of a new `loose_end` type with its own workflow, as `decision`
does. It has no branch, review or merge, and is done on the base with a
one-line result. A bad result files a linked bug. A loose end is
`blocked_by` the issue it waits on, or, after a release on the light path, is
filed on the release branch with no blocker (decision 6); it is never part of
a mission, so it never holds one open.

The After merge marker, stand-ins, `carried to`, the awaiting-validation
views and the mission-close warning go. The `validation` type keeps its
meaning, an independent assessment of a mission's criteria before it closes,
and loses its own branch and review round.

Built by atelier-8bh4:

- **One workflow.** `loose_end` and `validation` share the `on_base`
  workflow, in the starter policy and atelier's own: `todo` to `done` through
  a transition named `done`, which requires the policy's `close_reason`, a
  non-empty Report holding the one-line result, and runs
  `blockers.none_open`; plus `supersede`. It has no `start` transition.
  Atelier does not count the Report's lines; one line is the guidance.
- **Judged by workflow.** An item whose workflow has no `start` transition
  takes no branch, review or merge, whatever its type: `atelier start` and
  `atelier merge` refuse it (`start_done_on_base`, `merge_done_on_base`) and
  name its done transition, and `land` accepts it marked done on the base, in
  place of the hard-coded `decision` exemption from `land_base_done_claim`;
  missions stay exempt by type. A decision falls under the same rule, so its
  `start` refusal now names `decide`. A repository whose policy keeps
  `validation` in `work` starts and merges validations as before.
- **Ready, never started.** A loose end in `todo` is blocked while what it
  waits on is open, then ready, or needs a human when labelled
  `needs-human`. No `status` or `work` hint suggests starting it, and the
  mission views never offer a validation as the item to start.
- **Never part of a mission.** A loose end that is an issue's child or
  advances a mission is an `atelier check` finding, so the write that would
  link it is refused (atelier-ilba's guard); mission scope leaves loose ends
  out, so neither a mission's views nor its close count one even when a hand
  edit links it.
- **The migration.** `atelier migrate after-merge`, a dry run unless
  `--apply`, refuses when the policy has no `loose_end`. It turns each
  unticked After merge line into a loose end `blocked_by` its item, with the
  check as its Done when and the reason in its Goal, labelled `needs-human`
  when the reason's words name the human's judgment or a device or place no
  agent can reach, which the dry run shows. An item that is not done keeps
  the stand-in as an ordinary Done when line, or loses the line when it gave
  none; a done item is left as it is, so the base's done records do not
  change. A line carried to an existing issue is skipped, and a line already
  migrated is found by its loose end, so reruns change nothing. The marker
  is otherwise read only so `review request` need not ask for it to be
  ticked.
- **What went.** Stand-in parsing outside the migration, `carried to`, the
  Awaiting Validation rows in `status`, `work all`, `work mission` and `work
  parent`, the debrief's awaiting note, the mission-close warning, the review
  brief's After merge section, and the rule that ticking or carrying an
  after-merge check changes no claim, which kept a receipt held at its close.
  Such an edit to a done record now changes its acceptance for receipts as
  any edit does; it still lands, since record edits never make an approval
  stale (decision 1), and ticking a box is still not reported as a Done when
  edit.

Settled by atelier-4nsl: a loose end that follows a change with no issue,
such as the tag of a release on the light path (decision 6), is filed on that
change's branch with no blocker. It reaches the base with the merge, so it
cannot be forgotten as prose can (the v0.3.1 tag was lost that way), and it
is ready there; it also shows as ready on the branch, which is harmless.
Whether astromech's per-feature `validation` play-tests become loose ends is
decided when atelier-nn67 migrates it.

### 4. Records can be corrected

Each write is validated against the rules `atelier check` applies, so an
invalid record is refused before it is written. Append-only applies once a
record is on the base; before that, its author may edit or delete it.

Built by atelier-ilba:

- Every record write passes one guard before any file changes: the records
  the write would leave are checked with `atelier check`'s rules, and a
  finding they have that the records before it lack refuses the whole write
  (`record_invalid`), naming each finding. A write is never refused for a
  problem it did not make, and one that fixes a problem passes. The record
  store runs the guard for every write; a command that writes several
  records checks them together, so a refusal leaves all of them unwritten.
  `start`, `migrate epics` and `prune`, which write or remove records
  outside the store, run the same guard first, and `bundle apply` checks
  its staged records as one batch. `merge` checks the
  records it puts on the base against the base's, except a workflow policy
  that does not parse, which it reports as its own check. The deferral
  source rule (`review_follow_up_source_mismatch`) got no write-time
  version, since atelier-md0v removed it.
- A reference to a missing record, in a refusal, `atelier check` or any
  load, says where the record is: committed at HEAD but missing from the
  checkout, deleted in a named commit, or present only on a named branch. A
  command given the full ID of an issue, review or receipt that only another
  branch holds names that branch.
- A finding, or another review record that is not an approval or trivial
  exemption, may be edited or deleted until it reaches the base. Each
  change is judged against the base as it stood where the change joined it:
  a commit a merge brought in, at that merge's first parent; a commit on the
  base's first-parent line, at its own first parent; a commit not on the
  base yet, at the tip of the policy's base branch. That costs a few Git
  lookups per changed review record, run only for a change the old rule
  refused. `land`, `doctor` and `merge` apply it, and `land` advises
  committing an uncommitted correction rather than restoring it. A change
  must leave a regular file. Adding a record is judged as before, so a
  removed separate disposition event is still never added back.
- Approvals and trivial exemptions stay append-only from when they are first
  committed, wherever they are, as before (review finding atelier-oyh0): a
  hand edit of an approval's commit, target or identity would re-point it
  at code its reviewer never saw, and `merge`, `land` and `check` would
  accept it.
- Under `fast_forward_only`, history did not show where a fast-forward
  began, so each commit was judged at its own first parent and a review
  record was final once committed. atelier-jnrh made every merge a merge
  commit, which ended that exception; a commit an earlier fast-forward put on
  the base's first-parent line is still judged at its own first parent. When
  the base branch cannot be read, a review record is final once committed.
- No identity rule: before a finding reaches the base, any agent on the
  branch may correct it. Identity is self-declared, so an author check would
  add a rule without stopping anyone. Accepted risk (planning review finding
  R13): an implementer could delete a reviewer's finding before merge, so no
  decision on it is recorded. That cannot let unreviewed code merge, since
  approvals cannot be edited and one still has to cover the code.

### 5. Fewer records and gates

- The approval that follows a `fix_now` finding resolves it. Only `defer` and
  `no_action` record a decision.
- Evidence receipts are optional notes for what nothing else records, and
  gate nothing.
- `land` and the hooks report, and never refuse work that is still in
  progress.
- These rules go: a reviewer's identity must differ from the Git authors (the
  identity is still recorded); the recorded base branch and the branch-type
  names, which require a work branch to be named `<issue_type>/<id>`; and the
  source rule on deferral follow-ups (`review_follow_up_source_mismatch`).

Built by atelier-md0v:

- **A fix_now finding.** One whose reviewer recommended `fix_now` and that
  has no disposition is resolved by an approval that is pinned to a later
  commit than the finding, one that holds the finding's record, and recorded
  after it (`created_at`), so an approval that never saw the finding cannot
  resolve it; an approval of the finding's own commit does not, nor does a
  parallel reviewer's approval of a head without the finding's record
  (review finding atelier-6xiw), and a trivial exemption never does. An
  older Atelier's `fix_now` disposition still counts: with a resolution
  commit it is resolved, as before, by an approval at or after that commit,
  and without one by the rule above. `review disposition --decision fix_now`
  stays accepted and is never asked for; the finding's confirmation says to
  fix it in a later commit, and `review list` shows `approved since by
  <approval>` once one resolves it.
- **A decision for defer and no_action.** A finding recommended `defer` or
  `no_action` still needs a recorded decision, even if it was fixed instead,
  and any identity may record it. Its gap names the `review disposition`
  that settles it.
- **No evidence gate.** The built-in `evidence.no_newer_failure` gate and the
  `evidence.attached` validator go. A policy naming `evidence.attached`, with
  any parameters, still parses and the name is ignored, so no transition,
  merge or land reads a receipt. Receipts remain notes for what nothing else
  records: `evidence list` still marks freshness, and marks a fail with no
  pass after it `⚠ no pass since`.
- **land and the Stop hook report.** On a named branch other than the base,
  as HEAD's committed policy names it (an uncommitted `base_branch` edit does
  not move it; review finding atelier-bo2i), where work is in progress, `land` prints its report, ends with `land: work
  in progress — <n> checks open (<checks>)` and exits successfully; `land
  --push` there pushes the commit HEAD was at when land began to the
  branch's own upstream, and nothing when a check moved HEAD. The Stop hook
  reports the same lines as a `systemMessage` and never blocks the turn. On
  the base and on a detached HEAD a refusal stands, since that is where
  unreviewed code would reach the base: `land` refuses and the hook blocks
  as before, except that uncommitted records alone on the base still block
  no turn. `atelier doctor` still refuses any failure. A Stop hook run from a
  directory outside any Atelier repository says so and exits successfully.
- **No identity rule.** An approval's identity, and a `defer` or `no_action`
  decision's, are recorded and no longer compared with the work's Git
  authors, so the `review.current` and `findings.disposed` author checks and
  the recording warnings go. Approvals stay append-only, and `land` and
  `doctor` still accept one only as first committed.
  [ADR 0025](0025-findings-carry-their-disposition.md) decision 4 goes with
  the reviewer rule: atelier-ilba already accepts that a finding can be
  deleted before merge, so a non-author rule on decisions guards nothing. The
  deferral source rule (`review_follow_up_source_mismatch`) goes too.
- **The base is the policy's.** The merge base is the
  `branch_policy.base_branch` of the branch's committed policy, checked
  against the base ref's own policy as merge already did. Merge, land,
  `review request`, status, prune, `start` and the transition guards read no
  `branch_base`, and `land_base_mismatch` goes. `start` still writes
  `branch_base`, set to the policy base and ignored when read, so older
  pinned binaries still parse new records, until atelier-nn67 moves
  astromech's CI pin and drops it. A branch that targets a mission branch
  needs no recorded base: merge integrates only into the policy's base
  branch, and mission integration branches were never built (ADR 0020). A
  review target is not recomputed from the hierarchy either; it is the
  policy's base branch.
- **start uses no type prefix.** An owner with a recorded branch, in this
  checkout's records or in a local branch's own record of it, switches to
  that branch. On the base, `start` creates a branch named with the owner's
  bare ID, or switches to one of that name. On a branch another item owns it
  refuses with `start_branch_owned`, naming that item; from a detached HEAD
  it refuses with `start_detached_head`; on any other branch it takes the
  current branch. `start_base_mismatch` and `start_branch_prefix_mismatch`
  go; `--base` is accepted, hidden and ignored, with a note that the base
  comes from policy, until atelier-ml3r updates the guidance that still
  passes it. A printed fix that starts work outside
  the owner's scope from its branch reads `git switch <base> && atelier
  start <id>`.
- **Short commit IDs.** Wherever a command takes a commit (`--commit`,
  `--resolution-commit`, `review trivial --base`), a unique abbreviation of
  at least four hexadecimal digits is resolved on input and stored as the
  full ID; an unknown prefix, or one several commits share, is refused,
  naming the matches. `review trivial --base` still takes a revision such as
  `HEAD`. Evidence commands take no commit.
- **One judgement.** Merge, `land` on the base and `doctor` judge findings,
  decisions and approvals by these rules: `findings.disposed`,
  `approval_accepted` (which `land`'s base coverage uses) and the historical
  paths.

### 6. A light path

A change with no issue, such as a release, docs or a mockup, is reviewed and
merged on an independent approval of its branch head, under the same merge
checks, with no start or close.

Built by atelier-4nsl:

- **Owned or not.** A branch is owned while an issue that is not done on the
  base records it as its work branch; an owner already done on the base no
  longer owns the name. The base branch is never a light-path branch. A
  branch no issue owns is on the light path. The rule that `merge` and
  `review request` need an owner issue now applies only to owned branches.
- **The brief.** On a light-path branch, `atelier review request` with no ID
  prints a brief for the branch's commits since it left the base: the range,
  the commit messages, the diff stat, earlier findings and reviews of the
  branch, the merge checks, the working rules and the commands that record
  findings and an approval. It has no Goal or Done when, since no issue
  states them. On an owned branch, `review request` and `merge` with no ID act
  on the owner.
- **Records name the branch.** `atelier review finding`, `approve` and
  `record` take `--branch <name>` in place of an issue ID, and `review
  disposition` decides such a finding by its ID. The record is an ordinary
  review record whose front matter names `branch: <name>` instead of a
  `target`, written at review schema version 3, so a binary that predates
  the light path reports "needs a newer Atelier" rather than an invalid
  record; records about an issue are still written at version 2. It is
  written only on that branch, checked out, unowned and pinned after it left
  the base. There is no trivial exemption of a branch: an approval is the
  light path's only cover.
- **The current run.** A record about a branch counts only when it is pinned
  to a commit the branch holds and the point where it left the base does
  not, so a later branch with a reused name (`release`) inherits none of an
  earlier one's findings or approvals, and records pinned to commits a
  rebase left behind count for nothing. Approvals stay append-only and go
  stale only on code changes, as for issues (decision 1).
- **Merge.** On a light-path branch, `atelier merge` with no ID merges it
  after a current independent approval of its head, with its findings
  judged as for an issue (`findings.disposed`), under the same merge checks;
  a merge check that reads an issue passes, since there is none. It writes no
  issue state: its merge commit's tree is Git's own merge of the base and the
  branch head. `merge --sync` with no ID catches the branch up the same way.
- **Never a done claim.** Records about a branch are kept apart from those
  about issues, so nothing that judges an issue reads them, and on an owned
  branch they count for nothing; when `start` takes a branch that holds them,
  it says they stop counting. An approval of a branch covers its code exactly
  as an issue's approval does, but never a done claim: a light-path branch
  whose commits mark an issue done is refused by `claims.reviewed` in merge
  and by `land_base_done_claim` on the base.
- **On the base.** `land` counts a light-path branch's commits as reviewed
  when its approval is one merge would have accepted: it reached the base
  through a merge, the branch was not owned when it merged, the approval is
  as first committed, and every finding of its run pinned at or before it is
  settled. `atelier check` and `doctor` accept the records. `status` shows
  the checked-out branch's light-path state, and each other unowned branch
  with records about itself, with its review state; `land` on such a branch
  reports its review gates once it has records.
- **Not `review trivial --base`.** The light path neither replaces nor reuses
  the base exemption. `review trivial --base` still covers one trivial commit
  made directly on the base, whose correctness is evident from its diff; any
  other change with no issue, including a release, goes on a branch and gets
  an approval.

Accepted risk, for Sumeet's after-merge read: a plain `git merge` of an
approved light-path branch passes `land` without running `merge_checks`,
since `land` cannot tell how a merge commit was made. The approval still
covers the code; `atelier merge` is the documented path, and CI runs on the
base.

What it deletes (decision 9): the issue, `start` and Report a change with no
issue needed, and the review records of a hand-made issue around it. It adds
no record type: a review record names a branch where it named an issue.

### 7. Small repositories work out of the box

`atelier init` writes a policy that can merge as it is, and every schema
change ships a migration (atelier-l457).

### 8. One meaning per word

check, land, close, review and done each mean one thing across the CLI, the
skill and the docs. This ADR uses done for an issue whose change is merged
and closed for a mission, as the strategy's Outcome does. atelier-ml3r
settles each word's meaning and defines it in CONTEXT.md.

### 9. No regrowth

PRODUCT_INTENT.md states Atelier's purpose and a principle: a new gate or
record type names the kind of defect it would have caught in real agent
sessions, and a rule that only patches another rule's edge case is a reason
to remove the first rule. Every item of mission atelier-mmpv names what it
deletes, and an item that would add a concept other than the loose end goes
back to Sumeet.

### 10. One binary serves every repository on this host

Record and policy changes keep reading the old form until every repository
on this host has migrated, as ADR 0025's expand and contract did.

### What does not change

No non-trivial change reaches the base branch without an independent review
of its code, and a fix made after review is reviewed again before merge. That
re-review may cover only the fix's diff. What remains is what review needs:
findings, a decision for each deferred or dismissed finding, an approval, and
a merge gate that requires an independent approval of the current code plus
the repository's merge checks.

## Supersession

Each replacement takes effect when the item that builds its decision merges,
named in parentheses below. Until then the earlier text describes current
behaviour and still applies. Decision 9 applies now. Everything an entry does
not name stands.

**[ADR 0016](0016-canonical-work-branches-and-mission-integration.md).**
Replaced (decision 5 above, atelier-md0v): the recorded branch base, and
canonical `<issue_type>/<issue_id>` work branch names as a rule Atelier
enforces. The conclusion of the rejected alternative "Recompute Review
Targets From Parent Hierarchy", that the recorded base is the target, goes
with the recorded base; the target is not recomputed from the hierarchy
either, but is the policy's `base_branch`. Opt-in mission integration
branches, never built, are no merge target: merge integrates only into the
policy's base branch. Stands: `base_branch`, work branches, mission scope as
direct `advances` links plus descendants, and domain names for starter
workflows.

**[ADR 0019](0019-verified-repo-native-tracker.md).**

- Decision 2: no gate reads evidence receipts (atelier-md0v), though a
  receipt stays tied to a commit with an explicit result and command capture
  still refuses uncommitted code. "A current approval from a non-author"
  becomes a current independent approval, where current means no code changed
  since it (atelier-3jdm) and the reviewer's identity is recorded but not
  compared with the Git authors (atelier-md0v). "A disposition for every
  finding" becomes a decision for each deferred or dismissed finding
  (atelier-md0v). Atelier enforces two steps of one order: `review request`
  refuses while a Done when line is unticked, and `merge` marks the issue
  done (atelier-jnrh). That narrows "Statuses, transitions, graph shape, and
  planning depth are agent-owned" for those two steps. Valid records are
  guarded when written (atelier-ilba).
- Decision 3: no command reads a recorded branch base, which `start` still
  writes only so older binaries parse the record (atelier-md0v); `merge`
  also marks the issue done in the merge commit it always creates
  (atelier-jnrh); a branch no issue owns is reviewed and merged on an
  approval of its head, with no `start`, through review records that name
  the branch, and its merge writes no issue state (atelier-4nsl, decision 6). "Atelier never creates commits" now reads as decision 2
  says: Atelier creates that merge commit and no other.
- Decision 4: the done claim is no longer committed on the work branch and
  reviewed with the code, and merging writes the done state, so "merging
  writes no tracker state" goes (atelier-jnrh). Its freshness rule, under
  which record-only commits keep an approval current by kind and a change to
  acceptance criteria makes it stale, is replaced by decision 1 above
  (atelier-3jdm). Stands: the base shows an item done exactly when its code
  is merged, a clean merge of the base keeps an approval current, and `land`
  examines only what a change can affect, though it no longer refuses work in
  progress (atelier-md0v).
- Decision 6: append-only applies only to records on the base
  (atelier-ilba), and a `fix_now` finding records no disposition
  (atelier-md0v).
- Decisions 1, 5 and 7 stand.

**[ADR 0021](0021-parent-issues-replace-epics.md).** Decision 2: the parent
no longer closes after every child is terminal, and the starter policy's work
workflow has no `close` to run `work.terminal`, since one merge marks the
parent and its open children done (atelier-jnrh, decision 2 above); the
branch name `<issue_type>/<parent-id>` is no longer a rule (atelier-md0v). Stands: no epic type, the parent owns the branch its
children work on, superseding a parent retires its open children, and
decisions 1, 3, 4 and 5.

**[ADR 0025](0025-findings-carry-their-disposition.md).**

- Decision 1: `fix_now` is no longer a disposition with a resolution commit;
  the approval that follows the finding resolves it (atelier-md0v). The field
  stays for `defer` and `no_action`.
- Decision 2: append-only applies only once a review record is on the base
  (atelier-ilba), and approval freshness no longer shares the append-only
  rule, since it reads no records (atelier-3jdm).
- Decision 3: replaced by decision 1 above; no record edit makes an approval
  stale (atelier-3jdm).
- Decision 4: replaced; a `defer` or `no_action` decision's identity is
  recorded and not compared with the work's Git authors (decision 5 above,
  atelier-md0v).
- Decisions 5 and 6 stand.

**[ADR 0026](0026-evidence-records-only-what-nothing-else-records.md).**
Replaced, from What stays: the `evidence.no_newer_failure` block on close,
land and merge, and a policy's own `evidence.attached` gate; no gate reads
receipts (atelier-md0v). Its "Close Reason names the targeted test or check"
means the implementer report in the item's Report (atelier-jnrh). Stands: what gets a
receipt, the rules it removed, no reruns in `merge --sync`, command capture
refusing uncommitted code, and freshness marks in `evidence list`.

**[ADR 0027](0027-proof-has-a-time.md).** Superseded (atelier-8bh4): loose
ends replace its after-merge checks, so its Decision goes in full. Its
rejected options, a status after `done` and a front-matter field, are not
revived; a loose end is an issue.

**[PRODUCT_INTENT.md](../../PRODUCT_INTENT.md).** These lines stay as written
until the item that changes the behaviour lands.

- "Guard valid records, dependency closure, traceable active work, fresh
  honest evidence, independent approval, visible trivial exemptions, and
  dispositions for findings": no gate reads receipts, though a receipt stays
  tied to a commit with an explicit result; findings need a decision only
  when deferred or dismissed; and valid records are guarded at write
  (atelier-md0v, atelier-ilba).
- "Let agents commit records with the code. Atelier never creates commits":
  Atelier creates the merge commit that integrates work and marks it done,
  and no other (atelier-jnrh, decision 2 above).
- "Make transitions record writes with read-only checks. Branch preparation
  and gated integration are explicit `start` and `merge` commands": `merge`
  also marks the issue done (atelier-jnrh), and a branch no issue owns is
  reviewed and merged on an approval of its head with no `start`
  (atelier-4nsl, decision 6).
- "Git carries both the implementation and its done claim through review and
  integration", in the opening, and "Commit done claims on the work branch.
  Merge writes no tracker state": replaced; merge marks the issue done, and
  the base still sees completion together with the implementation
  (atelier-jnrh).
- "Keep review and evidence append-only and tied to full Git commit
  identities", with its exception for a finding's disposition: append-only
  applies only to records on the base (atelier-ilba), and a `fix_now` finding
  records no disposition (atelier-md0v). Records stay tied to commits.
- Under Work and proof, "Reviews record ... their findings, each with its
  disposition": only a deferred or dismissed finding carries a decision; and
  "`land` checks that the session can be handed off with committed records
  and intact invariants": `land` reports, and never refuses work in progress
  (both atelier-md0v).

## Consequences

- Items: atelier-3jdm (decision 1), atelier-jnrh (2), atelier-8bh4 (3),
  atelier-ilba (4), atelier-md0v (5), atelier-4nsl (6), atelier-l457 (7),
  atelier-ml3r (8, with the skill and Atelier's own AGENTS.md and docs) and
  atelier-nn67 (every other repository on this host: its policy and
  AGENTS.md, astromech's CI pin and validation items, and dropping
  `branch_base`). Each must name what it deletes before it starts (decision
  9).
- The six draft missions hold items planned under the old model, some
  contradicting this one. Which are superseded is Sumeet's call
  (atelier-mzpn), and none of those missions starts before he decides.
- Agent Factory's text changes with atelier-ml3r, upstream per
  [ADR 0023](0023-agent-factory-is-not-vendored.md).
