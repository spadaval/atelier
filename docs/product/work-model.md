# Work Model

Atelier parses tracked Markdown records on every command. Records are the source
of truth for current work; Git is the source of truth for branch ancestry,
integration, and history. See [storage](../architecture/markdown-first-record-store.md)
for the on-disk contract and [Agent Factory record templates](../../.agents/skills/agent-factory/references/tracker-records.md)
for authoring guidance.

## Records and relationships

Missions and work items are issue records with distinct `issue_type` values. A
mission links a strategy file and states an Outcome and Success criteria. Work
items (task, feature, bug, spike, validation, or any type the policy declares)
state Goal and Done when, with optional Scope, Context, Unknowns, and Design.
Missions and work items may also carry an optional Background section: the
research, references, and motivation that a mission or every child of a parent
shares, so it lives in the record instead of a side document.
Notes carry dated handoffs. Close Reason is required by done-category
transitions and checked by `land`: one or two sentences giving the observable
result against Done when, or for `supersede` why the work will not happen.
Test names, measurements, and review status belong in evidence and review
records, not here.

Close Reason can also carry the implementer report (atelier-sfsq). Its first
paragraph stays the Outcome described above. The report's other parts are
`### Deviations` (where the work departed from the plan), `### Decisions`
(the choices that shaped it, usually pointing at its decision notes),
`### Surprises` (what the implementer did not expect), and `### Open items`
(what is left, with follow-up IDs). Policy chooses which parts a transition
requires with the `report.deviations`, `report.decisions`,
`report.surprises` and `report.open_items` required fields. A required part
must be present and non-empty; write `None.` when there is nothing to report.
The close transition reads Close Reason from the record body when it is
already written there, or from `--reason-file <file>`; `--reason` replaces
the Outcome and keeps the parts. `land` and `merge` check a required part
only on a done claim committed beside a policy that required it, so done
records from before the policy required a part stay valid, including a
child closed on its parent's branch before the base's policy changed. Like the rest of Close Reason, the report can be written after
approval without making the approval stale. This repository's policy
requires all four parts on the work `close` transition; `supersede` needs
only the Outcome.

Priority has two spellings for four values: records store `P0`, `P1`, `P2`
and `P3`, which views print as `critical`, `high`, `medium` and `low`. The
`-p`/`--priority` flags on `issue create`, `issue update` and `issue list`
accept either spelling; bundles take the word. `check` rejects a record that
stores the word, and names the stored token it needs.

Any work item can be a parent: it may own direct children, one level deep, so a
child has no children of its own (atelier-9vq3, decided by Sumeet 2026-10-15;
this replaced the epic type, because elsewhere "epic" usually means a theme and
adopters got branches that never merged). Themes are labels, and missions keep
the why. Missions have no parent or children; work links to missions through
`advances`. Native blockers represent sequencing, not membership or proof.
Evidence attachments represent proof; semantic links remain separate.

Design questions are `decision` records (atelier-ox63). A decision states a
Question, optionally Options, Context, and Background, and closes with the
`decide` transition once its Decision section says what was decided and why.
Decisions are the human's by default: an open decision is listed under
"Needs a Human", never as agent-ready work. They need no branch, so they are
filed, decided, and closed on the base branch as planning changes (see
[planning changes on the base](reviews-and-merge.md#planning-changes-on-the-base)).
A decision record may name the ADR it produced (`issue update --adr <path>`;
`check` refuses a path that does not exist). A newer decision record replaces
an older one with a `supersedes` link; once the newer one is decided, views
mark the older one superseded and no longer count it as current.

Work found while doing other work records where, with a `discovered_from`
link stored on the new issue (`issue create --discovered-from <id>`; like other
context links it is declared by the new record, so the source is unchanged).
The source is the issue being worked, a review event such as a finding, or an
evidence receipt. `issue show` prints it, `check` refuses a source that does
not exist and a deferred disposition whose follow-up declares a source other
than the item the finding is about, and the
[debrief](mission-reviews.md#debrief) lists the work by that source. It marks
work as found along the way, not as scope: the work needs no `advances` link
to appear in the debrief, and takes one only if the mission must finish it.

Decisions made inside a work item are decision notes, not records:
`issue note <id> --kind decision "<choice>" --reason "<why>"`, with optional
`--rejected "<alternative>"` (repeatable) and `--one-way` for a decision that
cannot be reversed cheaply. The note stores the choice on its first line and
indented `Reason:` and `Rejected:` lines, tagged `[decision]` or
`[decision one-way]`. `check` refuses a decision note without a choice or a
reason. `work decisions <mission>` lists the decisions across a mission's
scope: one-way notes first, then current decision records (decided and not
superseded), then reversible notes, then open decision records, which are
proposals and not yet in force, then superseded records.

Backlog conventions (decision atelier-ermg, under atelier-b7hb):

- **Size** is the reserved label `size-s`, `size-m`, `size-l`, or `size-xl`.
  `check` rejects other values and more than one size, `issue show --full`
  displays it, and `issue update -l size-<x>` replaces the previous size.
- **Unverified assumptions** go in a work item's Unknowns section.
- **Won't-do** items are `decision` records whose Decision says no and why;
  prune keeps them.
- **Already-built** work is described in the parent's Background or Context,
  not filed as done records without a done claim.

A done record's body is part of its completion claim: its proof and review
were judged against that text. `atelier issue update --body` and bundle
updates refuse to change it (`issue_body_done`), and editing the file by hand
makes `land` treat the claim as new, so its gates run again as a current
claim and its approvals must still be current. Record corrections to a done
item, including its Design or Background, in a follow-up record instead
(atelier-jy9t).

Repositories with records from before this change run
`atelier migrate epics --apply` once: it retypes each epic as a feature (or
`--to <type>`), turns Outcome into Goal and Proof and Delivers into Done when,
moves draft epics to `backlog`, keeps their links, removes `epic` from the
policy, and adds `work.terminal` to the work `close` when it has the starter
shape (otherwise it says to add it). Review the Done when sections and commit.

Work that only the human can do, such as a step at a particular machine, an
account approval, or a judgement call, carries the reserved `needs-human`
label. Ready work with that label is classified "needs a human" instead of
ready: `atelier work ready`, mission and parent views, and `status` count and
list it separately, and `--quiet` ready lists leave it out, so agents choosing
ready work do not start it. `atelier work human` lists it, and `atelier start`
notes the label when someone starts one.

Do not make a `needs-human` check a child of a branch owner when the check runs
after the merge, such as a game, headset or Windows check on the released
build. `atelier merge` needs every child closed, so the owner's branch could
not integrate until the human acts. Model the check as a standalone
validation `blocked_by` the parent instead (and `advances` the mission, if
any). `atelier start` and `atelier status` warn when a branch owner has an open
`needs-human` child and give the two commands that detach it (atelier-1jll).
A `needs-human` child that must finish before the merge, such as an account
approval the code depends on, can stay a child.

`atelier work ready` lists only work an agent can start. Missions, and parents
with open children, are worked through their children, so ready views show
them in a separate "Parents and Missions" context panel instead, and `--quiet`
ready lists leave them out. A parent whose children are all terminal is ready
again, to be started and closed.

Records are branch-local, so work another session has started on its own
branch still reads as ready on the base. `work ready`, the ready section of
`work all`, the Ready Work of `work mission` and `work parent`, and the ready
and unassigned items `status` lists stay classified from the
checkout's records, but an item that owns another local branch carries its
committed state there on a `↳` line under its row, such as `↳ in_progress on
bug/atelier-j48s` or `↳ ✓ done on bug/atelier-fj1t, not merged` (see
[Other branches](sessions-and-hooks.md#other-branches)). Choose unmarked work,
or check with the branch's session before taking marked work.

Work that is recorded but not near-term sits in `backlog` until it is planned;
parked statuses such as `backlog` and `draft` stay in backlog views even when
they have open blockers. A parked parent parks its unstarted children too
(atelier-28mz, decided by Sumeet 2026-10-15): they show as backlog, and
`atelier start` and the `start` transition refuse them until the parent is
planned out of its parked status (for `backlog`, with `plan`). Children
already in flight keep their state and transitions, and a parent that can
never start, such as an open decision, parks nothing.
A todo-category status alone does not establish readiness: ready work must
expose a direct transition into active work and have no open blockers. A child
also waits on its parent's blockers, since it works on the parent's branch:
`work ready` leaves it out, `work blocked`, mission and parent views list it as
blocked by them, `issue show` lists them under "Blocked via parent", and
`atelier start` refuses it with the path child -> parent -> blocker. Only the
child itself inherits: a blocker's own parent does not matter, and a parent
blocked by one of its own children is sequencing its children, which does not
block them.

## Ownership and transitions

A standalone work item owns its branch, and a parent owns its children's
branch. A parent closes only after every child is terminal (the starter
policy's `close` runs the `work.terminal` check). Superseding a parent is
different: it retires the parent with any open children, which then show as
context only. Superseding needs no open blockers resolved, its own or a
parent's, and `land` does not hold a superseded item to them: the work will
not happen (atelier-lq42). The default branch name is
`<issue_type>/<id>`; typed `workflow_branch`
metadata records an explicit `work_branch` and `branch_base` on the owner.
Missions coordinate work rather than own a work branch.

`atelier start <id>` resolves the owner, checks the target branch's records,
checks out or creates its work branch, records branch metadata, and applies the
available `start` transition. Commit those record changes with the work.

A new work branch is created from the current branch, which `start` records
as the branch base without checking it; `land` later refuses with
`land_base_mismatch` unless that base is the policy's base branch, the one it
trusts. `atelier start <id> --base <branch>` creates the branch from that local
branch instead, from any checkout including a detached HEAD, resolves the issue and reads and
checks the records as they are on that base (the checkout need not hold them), and records it. `--base` names
the policy's base branch only (`start_base_mismatch` otherwise; a branch that
does not exist is `start_base_missing`), and a base without the issue's
committed record is refused with `start_branch_missing_record`. Without
`--base`, a detached HEAD refuses with `start_detached_head`, and a new branch
is not created from a current branch named `<prefix>/<owner id>` whose prefix
is not the owner's type, such as `task/<id>` for a feature: `start` refuses
with `start_branch_prefix_mismatch` and names the right branch to create, since
it would record the wrong branch as the base and `land` would later fail with
`land_base_mismatch` (atelier-mhx0, atelier-yc4d).

`start` also leaves a runtime lease for the owner, so a second session can
learn the item is taken (atelier-81e5). The lease is a small file under
`.atelier/runtime/leases/` of the repository's primary checkout, shared by
all its worktrees and never committed. It names the worktree, branch and a
heartbeat that `start` sets and `status` renews from the holding worktree.
Another worktree's lease is live while that worktree exists and its heartbeat
is under 4 hours old; it expires at once when the worktree is deleted or starts
a different owner. `start` refuses a live lease with `start_item_held` unless
given `--take-over`, notes a take-over, and replaces a stale lease with a note.
`status` shows `Held: ⚠ also held by <worktree> ...` in Local State when
another live worktree holds the owner. The lease is a warning, not a lock, and
two sessions in one worktree cannot be told apart.

Begin work with `atelier start`, not `atelier issue transition <id> start`.
The bare transition only changes the record: it creates no branch and records
no owner. While such work is active, `atelier land` refuses it with
`land_work_untraceable`; if it is closed on the base branch without a branch,
it never passes the review and merge gates (atelier-fj1t tracks refusing
that). Use the bare transition when the owner branch already exists and is
checked out, for example to start a further child on its parent's branch.

Statuses and permitted transitions come from `.atelier/workflow.yaml`.
`atelier issue transition <id>` shows options and blockers. Supplying a
transition changes the record after read-only checks. Transitions do not branch,
commit, push, open reviews, or merge.

## Completion

A done claim is a terminal status written and committed on the work branch.
Open dependencies prevent completion. Evidence must state what was checked,
against which commit, and whether it passed. Independent review examines the
implementation and the done claim together.

`atelier merge <id>` checks the base branch's policy and review gates, then
fast-forwards the base to the committed work, or under `merge_commit` adds one
merge commit whose tree passed `merge_checks`. It creates no other commit and
changes no records. The base therefore shows the item done together with its code.
See [reviews and merge](reviews-and-merge.md) for approval freshness and findings.

The done claim comes before review on purpose (atelier-9k7o, decided by
Sumeet 2026-10-15): the reviewer sees the claim together with the code. A
review that finds problems leaves the claim in place: the author fixes the
findings on the work branch, and the merge waits for fresh approval. Until the
merge, though, that branch says done for work the base does not have. So
`status`'s owner row shows a done owner as
`✓ done on branch <work-branch>, not merged into <base>`, or as
`✓ done, merged into <base>` once the base branch's own copy of the record is
done. Its `Branches` rows say the same in their own form, `✓ done, not merged`
(see [human-cli-output.md](human-cli-output.md#branches)).

Branch-local state can differ until integration. Use labeled cross-branch
status for orientation; it does not rewrite the current checkout's graph.
Terminal records eventually leave the active tree through
[prune](retention-and-prune.md), with their history retained in Git.
