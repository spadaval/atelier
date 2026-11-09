---
name: agent-factory
description: "Use in repositories tracked with Atelier, or to set a repository up with Atelier: planning or running a mission, grooming a backlog, delegating work to subagents, making a consequential product or architecture decision, diagnosing an unexplained failure, or independently reviewing or validating agent-written code."
---

# Agent Factory

Run `atelier status` first, and continue if it succeeds. If it fails with "Not
an Atelier repository" or `atelier` is not found, stop using this skill; offer
`install` only if the user asked to set up Atelier. If `atelier` is not found,
also give the install lines: `curl --proto '=https' --tlsv1.2 -LsSf https://github.com/spadaval/atelier/releases/latest/download/atelier-cli-installer.sh | sh`
and `npx skills add spadaval/atelier --skill agent-factory -g`.

Small work (one agent, one session) needs only a clear commit and PR
description, multi-step work uses [Atelier](references/atelier.md) issues, and
a mission adds a strategy and increments (`plan`).

## Core Rules

- **Prove claims with fresh evidence.** Before saying something works or is
  done, run the check that proves that claim after your last relevant change and
  read its full output. An earlier run, a suite that never exercises the claim,
  or another agent's report is not proof.
- **Diagnose before repairing.** Reproduce a failure and establish its cause
  before changing code; a patch for a misunderstood bug usually moves it.
- **Build the simplest thing that meets current requirements:** one owner for
  each responsibility and piece of state, dependencies only across intended
  boundaries, nothing added for a hypothetical future, and no debug code,
  obsolete path or shim left behind unless compatibility is the goal.
- **Keep the default branch green and the workspace tidy.** Failing checks there
  hide regressions and block everyone. Preserve changes you did not make, and
  run a small baseline before changing files so existing failures are not blamed
  on you. Create as few branches and worktrees as the work needs, and remove
  yours once it has merged or its learning is recorded.
- **Separate evidence from scope.** Workers report findings; the agent
  accountable for the outcome decides what they mean. Fix a nearby problem only
  if it blocks the current outcome; otherwise file follow-up work.
- **Respect authority.** Ask a human about product direction, risk tolerance and
  destructive or irreversible actions policy does not authorize, except inside
  an approved mission, where a question its strategy and Authority leave open
  aborts it ([`orchestrate`](procedures/orchestrate.md#before-starting)). A
  mission's strategy changes only by human direction or authority written in it.
- **Start children fresh, one way.** Give a child a self-contained brief, not
  your conversation. Under T3 Code, start every child with `delegate_task`,
  whatever its provider, overriding T3's advice to use your native tool: one
  tool starts every model, and every child is a thread the human can open.
  Elsewhere, use the harness's own subagent tool.

## Independent Review

Every non-trivial change is reviewed before it merges by someone who did not
write it and does not carry its author's reasoning, such as a fresh subagent; an
orchestrator that has read the author's handoff does not qualify. Until then the
change stays off the default and protected branches, its PR is not marked ready,
and its mission does not close. [Planning
records](references/atelier.md#planning-on-the-base) follow their own rule.

- **What it covers:** everything committed, docs and planning records included,
  since agents act on what they read. Trivial changes, whose correctness is
  fully evident from the diff, are exempt: a typo, formatting, a tool-driven
  rename, a status change adding no content, a patch-level bump with passing
  checks. Rewording a document is not trivial; if you have to ask, it is not.
- **Who arranges it:** the author, starting a new reviewer for each round and
  never messaging an earlier one ([`implement`](procedures/implement.md#review)).
  Prefer one from another lab than the author's; with one lab, a fresh context
  still qualifies. The reviewer records its own findings and approval.
- **What the reviewer gets:** the diff, the requirements and the proof that was
  run (in Atelier, the brief `atelier review request` prints), with no author
  rationale or self-assessment, which carry the bias review exists to avoid.
  The one allowed addition is a rebuttal, marked as the author's claim.
- **After review:** a finding's fix is reviewed again, however small, as is any
  non-trivial conflict resolution or rewritten requirement, by any reviewer.
- **Findings:** each ends fixed, deferred, won't-fix or no action ([`review`'s
  recommendations](procedures/review.md#recommendations)). Author and reviewer
  settle correctness; scope is the accountable agent's, whose brief says "fix
  every `FIX NOW` within these limits". The author fixes each `FIX NOW` within
  its brief, and an approval after the fix resolves it; files those recommended
  `DEFER` with `atelier review defer`; answers one it thinks wrong with a
  rebuttal (a fact or a test) in the next round's brief, whose reviewer records
  `no_action` or keeps it; and escalates one it thinks real but not worth
  fixing, or whose fix leaves the brief. It records no other disposition on its
  own work. An open `FIX NOW` blocks the merge.
- **No reviewer:** keep it unmerged, report review outstanding (your own reread
  is not review) and ask the user to review or arrange it. A prior request to
  merge does not waive this; a user who knowingly directs an unreviewed merge is
  obeyed unless policy or a gate forbids it (then say so), and review stays
  open.

## Handoff

End every assignment with **Outcome**; **Changes** (files, commits, branches,
tracker items); **Proof** (the commands or observations behind each claim, with
results, marking claims only read); **Review** (who reviewed, what is
outstanding or why it is trivial, unresolved `FIX NOW` first); and **Open
items** (unverified claims, risks, skipped checks, blockers). On a tracker item,
append it as a note pointing at its Report.

## Subskills

Say which you use, and why, before starting it: the one named, else the one that
fits, else work under the core rules. Each is `procedures/<name>.md`:
`plan` (the planner's role: take a mission to the human's approval),
`orchestrate` (run a mission or workstream), `groom` (re-sequence the backlog
and find work not to do), `decide` (a consequential choice), `implement` (build
one slice, code or docs), `diagnose` (find a failure's cause), `review` (review
someone else's change), `validate` (test claims independently), `migrate` (a
breaking change), `audit` (findings on architecture or process), `garden` (fix
where agents trip, and bring the human the rest) and `install` (set up or assess
a repo).
