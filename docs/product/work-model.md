# Work Model

Atelier parses tracked Markdown records on every command. Records are the source
of truth for current work; Git is the source of truth for branch ancestry,
integration, and history. See [storage](../architecture/markdown-first-record-store.md)
for the on-disk contract and [Agent Factory record templates](https://github.com/spadaval/.agents/blob/main/skills/agent-factory/references/tracker-records.md)
for authoring guidance.

## Records and relationships

Missions and work items are issue records with distinct `issue_type` values. A
mission is a themed container: its Outcome says why its items belong
together, and it may link a strategy file. Success criteria are optional, and
any criterion must be something the mission's own work can make true and an
agent can check when the mission closes
([ADR 0024](../adr/0024-missions-are-themed-containers.md); Agent Factory's
[Shaping a Mission](https://github.com/spadaval/.agents/blob/main/skills/agent-factory/procedures/plan.md#shaping-a-mission)). Work
items (task, feature, bug, spike, validation, loose end, or any type the
policy declares) state Goal and Done when, with optional Scope, Context,
Unknowns, and Design. A loose end and a validation are done on the base by
their `done` transition, with no branch, review or merge (see [work after
the merge](validation.md#work-after-the-merge)).
Missions and work items may also carry an optional Background section: the
research, references, and motivation that a mission or every child of a parent
shares, so it lives in the record instead of a side document.
Notes carry dated handoffs. The Report says what happened
([ADR 0028](../adr/0028-simpler-lifecycle.md), decision 2): on a work item,
the implementer report its author writes before asking for review, so the
reviewer can read it; for `supersede`, a mission's `close` and a decision's
`decide`, the reason the transition records. A transition that requires the
policy's `close_reason` field needs a non-empty Report, and `land` checks it
on the done claims it judges. Its first paragraph is the Outcome: one or two
sentences giving the observable result against Done when, or for `supersede`
why the work will not happen. Test names, measurements, and review status
belong in evidence and review records, not here.

The implementer report's other parts are `### Deviations` (where the work
departed from the plan), `### Decisions` (the choices that shaped it, usually
pointing at its decision notes), `### Surprises` (what the implementer did not
expect), and `### Open items` (what is left, with follow-up IDs). They are
Agent Factory's guidance; Atelier requires only that the Report is not empty,
and reads an older policy's `report.deviations`, `report.decisions`,
`report.surprises` and `report.open_items` required fields as `close_reason`.
Like every record, the Report can be written or corrected after approval
without making the approval stale. A transition reads the Report from the
record body when it is already written there, and then prints the Outcome it
kept, or from `--reason "…"` or `--reason-file <file>`, which replace the
whole Report, so a report written earlier does not linger under a supersede
reason (atelier-ypks). No transition leaves a done status, so an item reopens
only by an edit to its status, which keeps its old Report. Records written
before atelier-jnrh call the section `## Close Reason`; it is still read, and
written back as `## Report`.

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
`--rejected "<alternative>"` (repeatable). A choice that is costly to reverse
is a decision record, not a note (decision atelier-6306). The note stores the
choice on its first line and indented `Reason:` and `Rejected:` lines, tagged
`[decision]`. `check` refuses a decision note without a choice or a reason,
with any other tag, or with a line after the first that starts with a
misspelled label (`Rejectd:`, `reason:`), which would otherwise fold silently
into the text before it. `work decisions <mission>` lists the decisions across
a mission's scope: current decision records (decided and not superseded)
first, then decision notes, then open decision records, which are proposals
and not yet in force, then superseded records.

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
moves draft epics to `backlog`, keeps their links, and removes `epic` from the
policy. Review the Done when sections and commit.

Work that only the human can do, such as a step at a particular machine, an
account approval, or a judgement call, carries the reserved `needs-human`
label. Ready work with that label is classified "needs a human" instead of
ready: `atelier work ready`, mission and parent views, and `status` count and
list it separately, and `--quiet` ready lists leave it out, so agents choosing
ready work do not start it. `atelier work human` lists it, and `atelier start`
notes the label when someone starts one.

Do not make a `needs-human` check a child of a branch owner when the check runs
after the merge, such as a game, headset or Windows check on the released
build. Review asks every open child of the `work` workflow for ticked Done
when lines and a Report before `atelier merge` marks it done, so the owner's
branch could not integrate until the human acts. Make the check a [loose
end](validation.md#work-after-the-merge) `blocked_by` the parent instead,
outside any mission. `atelier start` and `atelier status` warn when a branch
owner has an open `needs-human` child that its merge would mark done, and give
the two commands that make it a loose end, or a standalone item `blocked_by`
the parent under a policy without `loose_end` (atelier-1jll, atelier-8bh4). A
`needs-human` child that must finish before the merge, such as an account
approval the code depends on, can stay a child.

`atelier work ready` lists work an agent can take up now: work to start, and
items done on the base, such as loose ends and validations, which are finished
by their `done` transition on the base rather than started (atelier-8bh4).
Missions, and parents with open children, are worked through their children,
so ready views show them in a separate "Parents and Missions" context panel
instead, and `--quiet` ready lists leave them out. A parent whose children are
all terminal is ready again, to be started and closed.

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
have no open blockers and expose a direct transition into active work, or,
for an item done on the base, its transition to done. A child
also waits on its parent's blockers, since it works on the parent's branch:
`work ready` leaves it out, `work blocked`, mission and parent views list it as
blocked by them, `issue show` lists them under "Blocked via parent", and
`atelier start` refuses it with the path child -> parent -> blocker. Only the
child itself inherits: a blocker's own parent does not matter, and a parent
blocked by one of its own children is sequencing its children, which does not
block them.

## Ownership and transitions

A standalone work item owns its branch, and a parent owns its children's
branch. The merge that integrates the branch marks the parent and its open
children done together (see [Completion](#completion)), so no rule orders a
parent after its children. Superseding a parent retires the parent with any
open children, which then show as context only. Superseding needs no open blockers resolved, its own or a
parent's, and `land` does not hold a superseded item to them: the work will
not happen (atelier-lq42). Branch names carry no issue type: on the base,
`start` names a new branch with the owner's bare ID. Typed `workflow_branch`
metadata records the owner's `work_branch`, and a `branch_base` that `start`
sets to the policy's base branch and nothing reads: the base is always the
policy's `base_branch`, and the field stays only so an older Atelier can parse
the record, until atelier-ml3r drops it (ADR 0028 decision 5). Missions
coordinate work rather than own a work branch.

`atelier start <id>` resolves the owner, checks the target branch's records,
checks out or creates its work branch, records branch metadata, and applies the
available `start` transition. Commit those record changes with the work. An
item whose workflow has no `start` transition, such as a decision, loose end
or validation, takes no branch: `start` and `merge` refuse it with
`start_done_on_base` or `merge_done_on_base` and name the transition that
marks it done on the base (atelier-8bh4).

Which branch `start` uses depends on where it runs (atelier-md0v):

- An owner with a recorded branch, in this checkout's records or in a local
  branch's own record of it (as on the base before the work merges), switches
  to that branch, checking and changing the records as they are there; a
  branch without the issue's committed record is refused with
  `start_branch_missing_record`.
- On the base, `start` creates a branch named with the owner's bare ID from
  the base, or switches to an existing branch of that name.
- On a branch another item owns, it refuses with `start_branch_owned`, naming
  that item, with `git switch <base> && atelier start <id>` as the step.
- From a detached HEAD it refuses with `start_detached_head`.
- On any other branch, such as one created by hand, it takes the current
  branch as the owner's work branch.

Switching branches needs no uncommitted changes to tracked files and no
untracked records (`start_checkout_dirty`, naming each path, with the stash
that sets them aside). Any other untracked file is no commit's, so it comes
along unless the existing branch `start` switches to holds a file at its
path, which refuses with `start_untracked_overwritten` before anything
changes (atelier-l457).

`start` runs from where it is. An older `--base <branch>` is accepted,
hidden and ignored, with a note that the base comes from policy, so
guidance that still passes it does not fail.

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
no owner. While such work is active, `atelier land` reports it with
`land_work_untraceable` (on the base, a refusal); if it is set done on the base
branch without a branch, `land` there refuses the done claim (atelier-fj1t). Use the bare
transition when the owner branch already exists and is checked out, for
example to start a further child on its parent's branch.

Statuses and permitted transitions come from `.atelier/workflow.yaml`.
`atelier issue transition <id>` shows options and blockers. Supplying a
transition changes the record after read-only checks. Transitions do not branch,
commit, push, open reviews, or merge.

## Completion

Work is done when its change is merged
([ADR 0028](../adr/0028-simpler-lifecycle.md), decision 2). The order is:
start, implement, tick every Done when line and write the implementer report,
ask for review, fix and re-review, approve, merge. A `fix_now` finding needs
no disposition: the approval of the fix resolves it. There is no close step for
work: the `work` workflow has `supersede` for work that will not happen, and
no `close`.

A Done when line with a box (`- [ ]`) is ticked (`- [x]`) once it holds.
`atelier review request <id>` refuses with `review_request_unfinished`, and
names each gap, while an open work item it covers has an unticked box or an
empty Report: asking for review means the work is implemented and done to the
best of the author's knowledge. It covers the item it names; naming the owner
covers the owner and its started children, so a parent's children can be
reviewed one at a time while unstarted siblings wait. A line without a box is
not checked, and a line with the old After merge marker need not be ticked;
the refusal names `atelier migrate after-merge` for such lines
(atelier-8bh4).

`atelier merge <id>` checks the base branch's policy and review gates, then
lands one merge commit whose tree passed `merge_checks`. That commit sets
every open item of the `work` workflow in the owner's scope, the owner and
its non-superseded children, to the workflow's first done status, and
changes nothing else the branch holds, the Report included. Merge refuses
while one of those items has an open blocker, and notes, without refusing,
each one it marks done that has an unticked box or an empty Report. It creates no other commit, so
the base shows the item done together with its code, and the work branch
never says done for work the base lacks. An item an older Atelier closed on
its branch merges as it is. A repository whose policy still gives work a
`close` keeps it until it updates its policy, and the done marks merge
writes answer to that transition's required fields and validators.
See [reviews and merge](reviews-and-merge.md) for approval freshness and
findings.

A mission closes when all its work is merged and its validation, if it has
one, is done; its `close` transition's description says so, and nothing else
enforces it. A loose end is never part of a mission, so it never holds one
open: mission scope leaves loose ends out, and a link that would put one there
is refused when written (atelier-8bh4).

Branch-local state can differ until integration. Use labeled cross-branch
status for orientation; it does not rewrite the current checkout's graph.
Terminal records eventually leave the active tree through
[prune](retention-and-prune.md), with their history retained in Git.
