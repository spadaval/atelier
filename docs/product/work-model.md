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
Notes and Close Reason carry dated handoffs and terminal reasons.

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

Backlog conventions (decision atelier-ermg, under atelier-b7hb):

- **Size** is the reserved label `size-s`, `size-m`, `size-l`, or `size-xl`.
  `check` rejects other values and more than one size, `issue show` displays
  it, and `issue update -l size-<x>` replaces the previous size.
- **Unverified assumptions** go in a work item's Unknowns section.
- **Won't-do** items are `decision` records whose Decision says no and why;
  prune keeps them.
- **Already-built** work is described in the parent's Background or Context,
  not filed as done records without a done claim.

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
ready work do not start it. `atelier work human` lists it, `atelier prime`
says how many such items are waiting, and `atelier start` notes the label when
someone starts one.

`atelier work ready` lists only work an agent can start. Missions, and parents
with open children, are worked through their children, so ready views show
them in a separate "Parents and Missions" context panel instead, and `--quiet`
ready lists leave them out. A parent whose children are all terminal is ready
again, to be started and closed.

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
fast-forwards the base to the committed work. It creates no commit and changes
no records. The base therefore shows the item done together with its code.
See [reviews and merge](reviews-and-merge.md) for approval freshness and findings.

The done claim comes before review on purpose (atelier-9k7o, decided by
Sumeet 2026-10-15): the reviewer sees the claim together with the code. A
review that finds problems leaves the claim in place: the author fixes the
findings on the work branch, and the merge waits for fresh approval. Until the
merge, though, that branch says done for work the base does not have. So
`prime`, `status`, and the other-branch rows in `status` show a done owner as
`done on branch <work-branch>, not merged into <base>`, or as
`done, merged into <base>` once the base branch's own copy of the record is
done.

Branch-local state can differ until integration. Use labeled cross-branch
status for orientation; it does not rewrite the current checkout's graph.
Terminal records eventually leave the active tree through
[prune](retention-and-prune.md), with their history retained in Git.
