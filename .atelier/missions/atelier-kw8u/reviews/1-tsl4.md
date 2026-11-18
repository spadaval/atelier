---
schema: atelier.review
schema_version: 3
round: 1
commit: 93d3b59f2b6a7d600123d55bb187df47f907ddd2
reviewers:
- 01a1f078-e1a3-7703-a7b3-5de2644b46f9
model: grok-4.6
verdict: changes
created_at: 2026-11-18T21:04:29.551225789Z
---

## Summary

Checked the mission at 93d3b59, all ten items, and the tree they describe: the four role files and SKILL.md Independent Review; `crates/atelier-core/src/say.rs`, `crates/atelier-cli/src/view_output.rs`, `crates/atelier-cli/tests/skip.rs`; `atelier show` on a mission; `evals/agent-factory/README.md`, `harness/run.py`, `harness/delegate.py`; grader and test line counts; the 2026-11-11 skill results and 2026-11-17 smoke; the 2026-11-11 cut at b22a03359; `docs/architecture/quality/scorecard-baseline/defects.md`; and the item graph against the success criteria.

The Why's claims that hold: graders plus their tests are 3,106 lines; SKILL.md and roles/ are 1,231 words; `roles/lead.md` still says "You write no code" and that a finding is not the lead's to settle; implementer Findings still say to fix a kept finding; every grader fails `ATELIER_SKIP`; `next-settle-within-authority` fails a lead for writing outside `.atelier/`; skip.rs shows `atelier merge` can pass "no approval" with a reason and write `Atelier-Skip` trailers; `show` on a mission lists disputes and plan reviews and no skip; one merge since the cutover carries an `Atelier-Skip` trailer (78f7b3dd3, CI); the smoke run's three wrong-or-null grades match the Why; `next-rebuttal-kept` is 3/3 with no skill on both models and its `escalate` path is gone.

Complexity: high. The mission adds a facts-and-judge grading contract, mixed-agent setups, a skill change to who may skip review, `show` of skips, new lead and review cases, and four extra skill arms.

Value: high. The evals currently grade wording and process, the skill contradicts the 2026-11-18 rule, and the loop is what the human asked to measure.

Risk of merging it: medium. A lead skipping review can land unreviewed code, which they chose; a house run whose judge shares a lab with the thread it grades, or a mixed-agent harness that assigns the wrong child, can mis-count the matrix without a failing test.

Could not check: the live run cache under `~/.cache/af-evals`; whether three after-merge diffs in defects.md are small enough for one reviewer to hold; this host's exact Claude effort for Fable and Opus.

## Finding 1: 1tkg is blocked until xfsn deletes the fixture it copies
Recommendation: fix now
atelier-1tkg's Goal says the first fixture exists: `next-rebuttal-kept` holds a kept finding that is wrong on the facts ([item.md](.atelier/items/atelier-1tkg/item.md) lines 24-26). Its Done when builds the new case from that fixture (lines 30-31). Its `blocked_by` lists atelier-xfsn (lines 10-12). atelier-xfsn's Done when deletes `next-rebuttal-kept` ([item.md](.atelier/items/atelier-xfsn/item.md) lines 45-46). 1tkg therefore starts after the directory is gone, and neither item says to copy it from git first. Drop xfsn from 1tkg's blockers and make xfsn wait on 1tkg, or have xfsn keep the fixture until 1tkg has copied it.

## Finding 2: house names one judge, but its threads are from two labs
Recommendation: fix now
The Outcome grades meaning with "a model from another lab than the run's agent" ([mission.md](.atelier/missions/atelier-kw8u/mission.md) lines 102-105). atelier-b9o6 makes that one judge "named by the setup" and "from another lab than the thread whose work the question is about" ([item.md](.atelier/items/atelier-b9o6/item.md) lines 31-33). atelier-8e8e's `house` setup is Fable as lead, Opus as implementer, and Codex `gpt-6.1-sol` as reviewer ([item.md](.atelier/items/atelier-8e8e/item.md) lines 31-33): Anthropic on two roles, OpenAI on the third. One named judge cannot be from another lab than every house thread. A Sol judge grades Sol's reviews; a Claude judge grades Fable and Opus. `setups` requires house, so this is not an optional cell. Say how house picks a judge: per thread, two judges, or that same-lab grading is accepted for some questions.

## Finding 3: lead-facing dispute text lives outside mtzj's say.rs scope
Recommendation: fix now
atelier-mtzj's Done when requires that every line the binary prints to a lead agrees with the new skill, and that a test shows a lead merging past an open fix-now finding with a reason ([item.md](.atelier/items/atelier-mtzj/item.md) lines 50-53). Its Scope names `crates/atelier-core/src/say.rs` and its tests (lines 59-62). The mission's Scope In says the lead-facing lines live there ([mission.md](.atelier/missions/atelier-kw8u/mission.md) lines 168-171). `status` already tells a lead the old rule in `crates/atelier-cli/src/view_output.rs` lines 624-626: it steps in by moving disputed work out of Scope, and nothing waits on those disputes. That is the current `roles/lead.md` lines 12-16, which mtzj rewrites. The kept-finding line in say.rs (lines 101-103) is in scope; this lead page is not. Expand mtzj's Scope to the CLI strings that address a lead, or move that line into say.rs as part of the item.

## Finding 4: Scope In omits the files oiwy restores
Recommendation: fix now
Scope In allows skill edits only in `roles/lead.md`, `roles/implementer.md`'s Findings, and SKILL.md's Independent Review ([mission.md](.atelier/missions/atelier-kw8u/mission.md) lines 168-170). Scope Out forbids any other change to the skill's text (lines 175-178). atelier-oiwy restores four passages onto the skill as mtzj left it ([item.md](.atelier/items/atelier-oiwy/item.md) lines 29-35): the reviewer's "What to judge" (cut from reviewer.md at b22a03359), the implementer's two Building lines (also cut; implementer.md today is Findings only), the lead's "Close against the Outcome", and a captain sentence in lead.md. Authority allows a restore that oiwy shows (mission.md lines 197-199), and oiwy's own Scope is `SKILL.md` and `roles/` (lines 47-50). An implementer or a later review that follows Scope In/Out will refuse the reviewer.md and Building restores. Name those files in Scope In, and narrow Scope Out to match Authority.
