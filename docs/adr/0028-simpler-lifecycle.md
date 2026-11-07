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
settled.

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
- The author writes the implementer report before review, so the reviewer
  reads it.

Mechanics, as atelier-jnrh built them:

- **The report.** One section, `Report`, says what happened for every
  transition that requires the policy's `close_reason` field: the implementer
  report on a work item, a supersede reason, a mission's close report and a
  decision's `decide`. The author writes the implementer report in the item's
  `## Report` before requesting review, so the reviewer can read it. The
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
  second kind of commit Atelier writes. atelier-ml3r tells those
  repositories.
- **"Atelier never creates commits"** now reads: Atelier creates one commit,
  the merge commit `atelier merge` integrates reviewed work with, which also
  marks that work done. Every other commit, records included, is the
  agent's.
- **Work closed by an older Atelier.** An item already done on an unmerged
  branch merges as it is: merge marks only open work. A repository whose
  policy still has a work `close` keeps working until atelier-ml3r updates
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
`blocked_by` the issue or release it waits on and is never part of a mission,
so it never holds one open.

The After merge marker, stand-ins, `carried to`, the awaiting-validation
views and the mission-close warning go. The `validation` type keeps its
meaning, an independent assessment of a mission's criteria before it closes,
and loses its own branch and review round.

Open, settled by atelier-8bh4: whether `validation` and `loose_end` share one
workflow, and the shape of the migration that turns open after-merge lines
into loose ends. Settled by atelier-8bh4 with atelier-4nsl: what a loose end
waits on when the change it follows had no issue, such as a release on the
light path (decision 6).

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
  source rule (`review_follow_up_source_mismatch`) gets no write-time
  version, since atelier-md0v removes it.
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

Open, settled by atelier-md0v: what replaces the recorded base for a branch
that targets a mission branch rather than the base, and whether the rule that
a `defer` or `no_action` be set by an identity other than the work's Git
authors ([ADR 0025](0025-findings-carry-their-disposition.md) decision 4) goes
with the reviewer rule, since it makes the same comparison.

### 6. A light path

A change with no issue, such as a release, docs or a mockup, is reviewed and
merged on an independent approval of its branch head, under the same merge
checks, with no start or close.

Open, settled by atelier-4nsl: how a review targets a branch rather than an
issue, and whether this replaces or reuses `review trivial --base`.

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
with the recorded base; whether a target is recomputed from the hierarchy
instead is open under decision 5. Stands: `base_branch`, work branches,
opt-in mission integration branches, mission scope as direct `advances` links
plus descendants, and domain names for starter workflows.

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
- Decision 3: `start` no longer records a branch base (atelier-md0v); `merge`
  also marks the issue done in the merge commit it always creates
  (atelier-jnrh); a change with no issue merges without `start`
  (atelier-4nsl). "Atelier never creates commits" now reads as decision 2
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
- Decision 4: whether its comparison with the work's Git authors goes is open
  (decision 5 above, atelier-md0v).
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
  also marks the issue done, and a change with no issue merges without
  `start` (atelier-jnrh, atelier-4nsl).
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
  atelier-ilba (4), atelier-md0v (5), atelier-4nsl (6), atelier-l457 (7) and
  atelier-ml3r (8, with the skill, AGENTS.md and every repository on this
  host). Each must name what it deletes before it starts (decision 9).
- The six draft missions hold items planned under the old model, some
  contradicting this one. Which are superseded is Sumeet's call
  (atelier-mzpn), and none of those missions starts before he decides.
- Agent Factory's text changes with atelier-ml3r, upstream per
  [ADR 0023](0023-agent-factory-is-not-vendored.md).
