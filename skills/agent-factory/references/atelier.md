# Atelier

Atelier is Agent Factory's tracker. `atelier --help` lists one issue's steps in
order, `atelier <command> --help` each command's record sections, flags and
rules, and `atelier issue transition <id>` an item's transitions; a refusal
names what is missing.

## Words

Tracker words mean only this: **check** is `atelier check`, the merge checks or
`merge --check`; **review** is independent review, or the human's of a mission's
plan (`atelier brief`) and merged work (`atelier debrief`); **land** is
`atelier land`; **done** is the terminal status of merged work, of a loose end,
validation or decision finished on the base, and of a closed mission;
**declined** is that of real work that will not be done (`decline`, with a
Report saying why and no branch, review or merge); only a mission **closes**
([orchestrate](../procedures/orchestrate.md#closing-a-mission)).

## One Issue, in Order

The steps are `atelier --help`'s; this says who takes each. An author working
alone takes the orchestrator's too.

1. **Start** (orchestrator or author): `atelier start <id>`. Children share
   their parent's branch.
2. **Implement** (author), committing code and records together.
3. **Finish the record** (author): tick each Done when line once it holds and
   write the `## Report` ([implement](../procedures/implement.md#handoff)).
4. **Request review** (author): `atelier review request <id> --identity
   <reviewer>` prints the brief, which a fresh reviewer gets unchanged
   ([implement](../procedures/implement.md#review)). The reviewer records its
   findings with the commands the brief prints.
5. **Fix and re-review** (author, then a new reviewer): the author fixes each
   `fix_now` finding that stands in a later commit, and a new reviewer reviews
   the fix. Each other finding gets a disposition from whoever [Independent
   Review](../SKILL.md#independent-review) says: `defer` (with a follow-up),
   `wont_fix` (real, not worth fixing, no follow-up) or `no_action` (wrong).
   `review defer <id>` defers all its undecided findings recommended defer.
6. **Approve** (reviewer), with the command the brief prints.
7. **Merge** (orchestrator): `atelier merge <id>`, or `atelier merge <id>
   --push --wait` where the policy names a forge (below), also marks every
   open child done, started or not, so move out or supersede any that should
   not merge. Never use plain `git merge`, which runs no gate and marks
   nothing done.

A change that needs no issue, such as small work, a release, docs or a mockup,
takes the light path that `atelier --help` and `review approve --help`
(`--branch`) describe instead, with the same gates and nothing marked done.

How merged work gets published depends on the base policy:

- **A policy with a `forge` block** (`.atelier/workflow.yaml`): `atelier merge <id> --push --wait`, from
  the work checkout, is the whole route to the published base. It pushes the
  branch, reuses or opens its pull request, waits for CI, merges, judges and
  pushes the base, and confirms the pull request merged. Do not push the
  branch, open the pull request, wait on checks or push the base by hand.
  `--wait` polls for up to `--timeout` minutes (30 by default), so where a
  harness limits foreground commands, run it in the background, or set
  `--timeout` under the limit and rerun when it refuses with `ci_pending`. A
  refusal names what stayed unpushed and its Next line.
- **A policy with no forge:** `atelier merge <id>`, then `atelier land --push`
  from the base checkout.

Either way, publish records and planning on the base with `atelier land
--push`, and push only through a command that pushes when its gate passes,
never behind a pipe that hides its status. Push without asking: whether work
goes out was decided at review and merge, and land refuses unreviewed code on
the base. Stop only if a gate refuses, or a commit holds secrets or personal
details.

## What Makes an Approval Stale

A code change since the approved commit, as `atelier --help` says. Code is
anything outside `.atelier/issues`, `.atelier/reviews` and `.atelier/evidence`:
policy, config and strategy files count (a strategy edit on the base is still
planning), and a clean merge of the base does not. For an item or mission marked
done on the base, another mission's strategy edit already there does not count
either: that mission's planning review covers it. Its own mission's does.

A base that moved on needs no catch-up: merge it in (`merge --sync`) only when
the branch is next to merge and conflicts, a merge check fails on the merged
tree, or merge refuses and says to. Never rebase after review is recorded, since
reviews pin commits.

## Planning on the Base

Commits that touch only records and the strategy files missions link go on the
base directly, so every branch sees the plan; a started item's own record
changes only on its work branch. A non-trivial planning commit is reviewed
before the work it defines starts: brief the reviewer with the planning commits
(`review request` is for finished work), and it records `review approve` or
`finding` against the issue, the mission (for a strategy) or the decision. A
mission plan gets two passes and a later increment one, as
[plan](../procedures/plan.md#planning-review) says.
Everything else goes through a branch and `atelier merge`, except a trivial
commit exempted with `review trivial --base` where policy allows, and the result
that marks a loose end or validation done, which needs no review (below).

## Records

A choice is a decision record when it is costly to reverse, reaches beyond its
item, changes a public or persisted contract, or needs the human. Other choices
are decision notes on their item (`atelier issue note --kind decision`), written
where a later reader would ask "why is it like this?". A changed decision gets a
new record that supersedes the old one. Label work only a human can do
`needs-human`; agents leave it, and open decisions, to the human, except inside
an approved mission, where one its strategy and Authority leave open aborts the
mission ([orchestrate](../procedures/orchestrate.md#before-starting)).

## Loose Ends and Validation Items

`atelier --help` says what a loose end is. File it
(`issue create --issue-type loose_end`, then `issue link <loose-end> <issue>
--role blocked_by`) when you plan the item, or as soon as such work turns up, on
the base as a planning change reviewed like any other, even while the issue it
waits on is in flight. The link lives on the loose end; whoever owns the plan
rewrites or drops the Done when line it replaces, on the work branch. A
release's tag, with no issue to wait on, is filed on the release branch instead.
A loose end never advances a mission, so it never holds one open.

A validation item, which holds an independent `validate` assignment and advances
its mission, is needed for mission acceptance, an increment that changes a
user-facing or cross-boundary outcome, or where choosing what to test needs an
independent eye; otherwise author-run proof suffices. Both kinds are done on the
base with no branch, review or merge: once what they wait on has merged, do the
work, mark the item done with a one-line result (`atelier issue transition <id>
done --reason "<result>"`) and commit the record. A bad result files a linked
bug; merged work is never reopened.
