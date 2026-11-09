# Implement

Use this subskill to build one bounded slice, code or documentation, prove it
works and take it through review to approval. You own its code and its review
loop; whoever briefed you owns the plan and hears from you when you finish or
[escalate](#escalation).

## Before Editing

Read the assignment and enough surrounding code, docs and decisions to do it
safely, and be clear on its scope and expected proof. If it is really a
diagnosis, a migration or a planning problem, say so and route it. For
non-trivial code, know where the responsibility and state should live before you
write it. If the assignment gives no expected size, write one, in lines or files
changed, as a decision note.

Anyone may propose not doing work, or less of it, when its cost outweighs its
value; say why, and let the plan's owner decide.

## Building and Proving

- **Bug fixes:** write a test or reproduction that fails for the reported
  reason, watch it fail, then fix the bug and watch it pass.
- **New behavior:** decide up front what observation would show the behavior
  missing or wrong, and make sure your proof could fail; a test you have never
  seen fail may not test anything.
- **Declarative, configuration or UI changes:** use the most direct proof, such
  as a build, typecheck, schema validation, dry run or screenshot.
- **Docs:** fix each topic at its authoritative source and link to it from
  elsewhere instead of copying it. Update or delete stale competing versions,
  which mislead more than a gap does. Check each claim about commands, behavior
  or configuration against the real thing. Record a gap you cannot resolve, such
  as an unclear owner, as follow-up work instead of guessing.

When a failing observation before the change is impractical, say why and use the
cheapest proof that could still show the change is wrong. Make the smallest
change coherent with the target design, which is not always the smallest diff,
and update docs when you change user-visible behavior, contracts, architecture
or ownership. After the final edit, rerun the focused proof and relevant
regression tests and read the full output.

Tick a Done when line only when it holds. A line that cannot hold is a plan
change, not a line to rewrite yourself: [escalate](#escalation). The plan's owner
files a [loose end](../references/atelier.md#loose-ends-and-validation-items)
for work that can only follow a merge or release.

## Review

Before review, prove the change in the running app with the repository's verify
skill where one exists, include its evidence with your proof, and update its
feature map for the features you add or change. Then run your own review loop
until a reviewer approves; your brief authorizes the reviewers you start.

1. With every Done when line ticked and the Report written, record the proof
   you ran (`atelier evidence record`) so the brief carries it, and run
   `atelier review request <id> --identity <reviewer identity>`.
2. Start a fresh reviewer with `review`, on the model your brief names, and give
   it the printed brief unchanged: it leaves out your notes and Report, which
   would bias it. Without Atelier, give the diff, requirements and proof.
3. When its completion wakes you (do not poll it or message it), read its
   findings from Atelier (`atelier review list <id>`) and decide each by
   [Independent Review](../SKILL.md#independent-review)'s Findings rule: fix
   each `FIX NOW` in its own commit and rerun its proof; run `atelier review
   defer <id> --identity <you>` for those recommended `DEFER`, leaving them
   unfixed; and for one you think wrong, add your rebuttal (a fact or a test) to
   the next round's brief, marked as the author's claim. Anything else about a
   finding is scope: escalate.
4. Start a new reviewer for each round, never a message to an earlier one, and
   do not widen scope to address a finding.

## Escalation

Escalate to whoever briefed you, rather than grind on, when:

- a Done when line can't be met as written;
- a fix would touch something the brief excludes, or an interface another issue
  depends on;
- the change grows past about twice its expected size, or a fix is
  bigger than the change it fixes;
- `FIX NOW` findings remain after three review rounds;
- a reviewer finds the approach wrong, not merely buggy;
- the work hits a risk the brief marks;
- a reviewer keeps a finding you disputed;
- you want to defer or not fix a `FIX NOW` finding, or think a finding real but
  not worth fixing.

Commit your work in progress on your branch, and end your run with the
escalation as your final report: what happened, which limit it hit, two or
three options with cost and risk, your recommendation and the Atelier IDs
involved. The orchestrator answers with a plan-level choice, in your thread or
in a new implementer's brief. Working alone, escalate to the user.

## Handoff

Before review is requested, record your choices as decision notes or records
([record test](../references/atelier.md#records)) and write the item's
`## Report`. Its first paragraph is the outcome against Done when, naming the
targeted test behind each claim and its result. Then add, only where they say
what the diff and proof do not, `### Deviations` from the plan and why,
`### Decisions` (pointing at the notes), `### Surprises` and `### Open items`
(follow-ups and loose ends by ID, unverified claims, risks, skipped tests). Be
candid and short; the human reads it in the debrief.

Without an escalation, your final report is one summary: approved at which
commit, review rounds, findings fixed, follow-ups filed and any finding left
undisposed (such as one recommended `WON'T FIX`) for the orchestrator to decide.
Also report the branch or commit, anything you created and its cleanup state,
and for docs, the source of truth for each topic changed and any mismatch left.
For a spike or competing candidate, report its behavior, design concerns and
what it taught separately, with a recommended disposition.
