---
schema: atelier.review
schema_version: 2
id: atelier-weem
title: approval on atelier-fyay @8250db31
target: atelier-fyay
identity: gpt-6.1-sol
commit: 8250db3163a9ec26821515bd8e114df2001fc91e
created_at: 2026-11-09T00:57:08.582289471Z
event:
  event: approval
---

Verdict: merge. This adds a usable confidence test for missions that should run unattended, without a new mechanism or a requirement to eliminate uncertainty.
Sizes: complexity low (one existing procedure and no new runtime surface); value high (directly serves the requested mission-confidence outcome); risk medium (agents act on this guidance across repositories, but it is visible and reversible).
Contract: pass. All four Done when lines hold: likely strategy-breaking causes must be settled; route-only questions wait for evidence; small spikes are run and larger ones require human approval outside the mission; their results are recorded; readiness includes confidence; remaining risks say what happens if they bite. No score, CLI gate or empty-Unknowns requirement was added.
Fitness: pass. Compared every paragraph of master:skills/agent-factory/procedures/plan.md with 8250db31, and read all four branch commits, the review procedure, sizing rules, SKILL.md, the Atelier reference, orchestrate.md and atelier-qj9d's budget requirement.

Findings: no new findings and no remaining FIX NOW recommendation. R1 / atelier-mt5r (low): the added read-path length is removed and the inaccurate Report is corrected. Fresh counts are master and head both 97 + 100 + 105 = 302 raw lines; counted at 80 columns both are 307, with the excess in unchanged SKILL.md. The initial version was 315 raw / 321 at 80 columns. Returning to the baseline resolves the regression; the existing budget overrun is not a new blocker for this item. This accepts the rebuttal on the remaining baseline excess, rather than claiming the path is within 300 or that further shortening would necessarily remove rules.

Paragraph comparison:
- Intro: planning still owns strategy, increments and first issues. orchestrate.md explicitly owns keeping the plan current and routes strategic problems back through decide and plan, so the shortened intro does not change that division.
- Layers: the removed content list remains in Strategy File; SKILL.md defines strategy and its authority boundary. The linked strategy is still summarized rather than duplicated. Increment shape, evidence boundaries, provisional backlog increments without tasks, initial choices and their records, end-to-end first work and docs-first API planning are retained.
- One theme loses an example, not its test. Finishable retains the separate backlog/loose-end treatment. Swept backlog still requires grooming, using proposals and answering the digest; Ready Work requires these shaping tests before readiness. Sized to finish is unchanged.
- Success criteria retains the whole-mission, attainable, verifiable and non-redundant tests and forbids dropping an existing criterion. Proposing a rewrite does not authorize changing it: SKILL.md's authority rule and Strategic Revision still govern who may accept it. Omitting the alternative backlog proposal does not permit silently removing a criterion.
- Strategy File retains location, size, metadata, section definitions and the boundary excluding mutable execution detail; the residual-risk description is the intended addition.
- Ready Work retains independent review and human approval after the walkthrough. The explicit review approve command with --identity <the human> still requires the approval record under their name, with the Authority exception intact. Issue readiness is unchanged.
- Strategic Revision retains the authority stop, reason, decision, in-place revision, timestamp, commit and dated note, supersession and treatment of affected work. Handoff is unchanged.

Usability: a likely feasibility or contract question that could change the target is settled before declaring readiness, commonly by a spike; an implementation-route choice within the agreed target waits for evidence. A larger exploratory task is filed outside the mission and needs approval to run. Ready Work makes the confidence test mandatory, while 'most likely', 'usually', route-only deferral and explicit remaining Risks and Unknowns preserve judgment rather than demand certainty.

Proof run independently at 8250db3163a9ec26821515bd8e114df2001fc91e: git diff --check HEAD^ HEAD -- . ':!.atelier', the whole-branch equivalent, cargo fmt -- --check, and cargo metadata --locked --format-version 1 > /dev/null all pass. scripts/merge-ci-passed.sh was also run and exits 2 with 'HEAD is not a merge commit; this check runs only inside atelier merge’s merge checkout.' Its documented merge-only precondition is absent in this read-only branch review; this is not a failing test of the prose. This approval does not attest to CI success or waive the merge gate, which must pass in its supported context before integration.
Open questions: none about the change.
Residual risk and tests not run: actual planner behavior was not evaluated; confidence remains a judgment call. The pre-existing read-path overrun remains. CI success is unverified here. No Rust test suite was run for this prose-only change.
