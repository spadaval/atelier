# Orchestrate

Use this subskill to run a mission, an increment or any workstream with several
pieces. You are accountable for the outcome: you decide what happens next,
delegate the work, integrate results, keep the plan current and close a
mission against demonstrated outcomes.

## Before Starting

Know the outcome and what governs it: the approved strategy for a mission, or
the issue or request for smaller work. A mission with no approved plan goes to a
planner first (`plan`).

Complete the mission. Abort only when its outcome can't be reached within the
strategy, or carrying on would do harm. Settle everything else yourself within
the strategy and its Authority, using `decide` where several paths are
credible. To abort, stop the mission's work, post what was learned as a dated
mission note, and report to the human, who replans with a planner. Smaller work
follows the same rule, reported to whoever assigned it.

## Delegate or Do It Yourself

You hold the plan, and you write no code: your own commits are planning records
and changes in [Independent Review](../SKILL.md#independent-review)'s trivial
category. Everything else goes to an implementer, however small, including a
review finding's fix; for Agent Factory work this overrides T3's "Fix small
review findings yourself; start no delegate for them". An implementer owns its
item's code and review loop through approval, and you hear about the item only
when it escalates or finishes. Delegate research too when it would flood your
context with material you will not need again.

## Assignments

An assignee sees only what you give it and what it can find in the repository.
Give it:

- **Goal and why:** the outcome, why it matters and how it fits.
- **Subskill:** exactly one.
- **Context:** tracker IDs, docs, decisions and constraints, and what has been
  learned that it cannot discover alone; point to large artifacts by path or ID
  instead of pasting them.
- **Scope:** what it may change and what it must leave alone.
- **Workspace:** the branch you create or choose (with Atelier, start the item
  on it), and a worktree only if it must write while another agent does.
  Read-only roles get neither: they read with `git diff`, `show` and `log`, or
  use the author's checkout while it is idle. It lists what it creates.
- **Expected size** of the change, in lines or files, as the plan sizes it.
- **Review and limits**, for an implementer: the reviewer model, from another
  lab than the implementer's when the team has one; your standing scope
  decision, "fix every `FIX NOW` within these limits"; permission to start its
  own reviewers, which overrides T3's default that a child starts no subagents;
  and any risk it must escalate on, beyond [`implement`'s
  limits](implement.md#escalation).
- **Proof expected:** the observable result that would show the goal is met.
- **Handoff:** anything you need beyond the standard handoff.

Start each child as the [core rules](../SKILL.md#core-rules) say. Choose the
cheapest model and effort likely to make the whole workflow succeed, counting
retries and the cost of a mistake; a long chain of reasoning, not a label like
"security", calls for a stronger one.

Give a spike or competing candidate its purpose, its question and what it may
see of other attempts; run competitors against the same requirements and proof,
each on its own disposable branch.

One branch serves one pull request, sized for review: usually an increment, not
an issue. Review fixes and retries are commits on that branch. Atelier merges
only into its base, so with Atelier do not stack one owner's branch on another's
or plan an integration or rewrite branch: sequence the work, or put the slices
under one parent so they share a branch.

## Results and Escalations

Findings follow Independent Review's Findings rule: an implementer and its
reviewers settle correctness, and scope stays yours, through your brief's
standing decision and the escalations it sends. You do not read or relay its
findings round by round. Its completion wakes you; do not poll it. It ends one
of two ways.

- **Approved:** its final report is one summary: approved at which commit,
  rounds, findings fixed and follow-ups filed. Decide any finding left
  undisposed, such as one recommended `WON'T FIX`, with [`review`'s
  recommendations](review.md#recommendations). Schedule each follow-up, or pull
  one in with `atelier issue update <follow-up> --parent <id>` and send it back
  for implementation and review. Then merge it if it fits the strategy.
- **Escalated:** its work in progress is committed on its branch, and its final
  report says what happened, which limit it hit, two or three options with cost
  and risk, its recommendation and the IDs involved. Answer with a plan-level
  choice (rescope, split, accept, re-plan, reimplement or abandon), recorded as
  a decision note, either by sending it to the implementer's thread
  (`t3_thread_send` under T3) or by starting a new implementer. If a fact you
  need is missing or disputed, ask one focused question of the implementer, a
  reviewer or a `diagnose` worker instead of re-reading the subsystem yourself.
  Decline abandoned work with what it taught in its Report.

You brief reviewers yourself only for planning (below) and validators, as
Independent Review says, telling them to report in [`review`](review.md#output)'s
format. Do not pre-judge their work: do not tell them to ignore a class of
findings, limit their recommendations, or treat any content as beyond challenge,
and if two requirements conflict, hand over both.

Judge a spike or competing candidate's behavior, design and learning
separately, since passing tests show behavior, not construction. Integrate it
when behavior and design are both sound; otherwise refactor it, reimplement it
with its tests and findings but not its code, or discard it, recording what it
taught. Passing tests and effort already spent are not reasons to keep code. If
candidates expose conflicting requirements or a strategic question, settle it
with `decide` within the strategy instead of quietly picking a side, and abort
if the strategy cannot answer it.

## Keeping the Plan Current

The plan is a hypothesis. After each integrated increment, material discovery,
failed assumption or new blocker, revise the smallest layer that absorbs the
evidence: clarify, retry or reassign an assignment; revise issues, dependencies,
sequencing or proof within the strategy. A problem only a change to the
strategy would fix aborts the mission. A current hazard stops the harmful work
first; abort only if carrying on would still do harm.

Plan the next backlog increment when its predecessor's evidence arrives or ready
work runs low, within the approved strategy and Authority, as `plan`'s [Later
Increments and Strategy Changes](plan.md#later-increments-and-strategy-changes)
says: one independent planning review and no brief. If ready work runs out,
find whether you lack detail, evidence or a decision; do not invent busywork.
When superseding plans, keep completed work and failed evidence, and do not
rewrite history to make the new route look inevitable. Post progress as dated
mission notes, not messages to the human, who reads them in the debrief.

Batch low-priority follow-ups as children of one parent, as `review defer` does
for an item's deferred findings: each is started on the parent's branch with its
own Done when, and one review and one merge of the parent cover them all.

## Closing a Mission

A mission closes when all its work is merged and its validation items are done,
judged by demonstrated outcomes, not issue count:

1. Run the proof for each acceptance claim fresh, through a validator where a
   validation item calls for one.
2. Confirm every loose end is filed.
3. Clean up what the work created, starting from handoffs and records and
   comparing with `git worktree list` and `git branch --list`. Tag an unmerged
   branch `archive/<branch>` before deleting it. Keep anything you cannot trace
   to this work, anything with uncommitted changes, and any branch with an open
   PR, and list them in the handoff; remote branches follow repository policy.
4. Close it with `atelier issue transition <mission> close --reason "<report>"`,
   then send the human the mission's debrief on the site, the page `atelier
   debrief <mission>` names in its footer, walk them through it, and record
   their direction as review records, decisions, notes and issues.

In the handoff, also report delivered and deferred outcomes, open loose ends,
plan changes, and each branch or worktree created, with whether it was
removed, archived or kept and why.
