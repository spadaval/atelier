---
created_at: "2026-11-07T15:35:46.504583439+00:00"
id: "atelier-uqxh"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: atelier-uqxh
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-mmpv"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Clear the contradictions the mission's cold read found"
updated_at: "2026-11-07T16:26:38.336158889+00:00"
---

## Goal

The mission validation (atelier-2bho, receipt atelier-pznw) found that a fresh agent reading only the agent-factory skill, astromech's AGENTS.md and the CLI help gets the lifecycle right but meets contradictions. Remove them so the legible criterion passes.

## Done when

- [x] The skill (references/atelier.md:144-147) says a loose end is planning like any other and is filed straight to the base, even for an item in flight (its `blocked_by` link lives on the loose end, so the in-flight item's record is not touched), matching astromech's AGENTS.md:36-39; one rule, no exception.
- [x] The skill's Words section (references/atelier.md) says that the `close_reason` policy key and `--reason` write the Report on every transition that asks for one, so "nothing else closes" does not contradict them; and that `done` and `superseded` are both terminal, the policy's done category, with `done` meaning merged or done on the base and `superseded` meaning retired.
- [x] The skill says one thing about decision records and ADRs: a decision record holds the question and decision, and an ADR is the document a decision record may produce (`issue update --adr`); tracker-records.md, decide.md and repository-shape.md agree.
- [x] The skill no longer mentions `issue note --one-way` (tracker-records.md:237-238), which does not exist.
- [x] The skill, Atelier's AGENTS.md (:30-32), astromech's AGENTS.md (:21-22, changed through astromech's own review and merge) and `atelier --help` say the same thing about a Done when line rewritten after start: it is shown in the review brief and by `merge --check`, and refuses nothing; record edits never make an approval stale on their own. The skill's lines 65-67 ("gets a fresh approval") go, since lines 91-92 already say this.
- [x] A fresh cold read of the same three sources finds no contradiction: the spadaval/.agents worktree commit that will merge, astromech's AGENTS.md on main after its change merges plus its workflow policy, and the CLI help from this branch's binary; its report is attached as a note.

## Scope

The agent-factory skill in spadaval/.agents (worktree; reviewed at a named commit; merged into main no-ff with this item), Atelier's AGENTS.md and CLI help text, astromech's AGENTS.md lines 21-22 (its own light-path branch or issue, reviewed and merged there).

## Context

Receipt atelier-pznw lists each contradiction with file and line. docs/strategies/simpler-lifecycle.md criterion legible.

## Notes

- 2026-11-07 [result] Cold read, third and final round: no contradictions found. A fresh subagent read only the agent-factory skill at spadaval/.agents atelier-uqxh 658dfe5 (SKILL.md and every file it links), astromech AGENTS.md at atelier-uqxh-agents fb6bed36 (main e6ae38df plus the lines 21-22 change) with origin/main's .atelier/workflow.yaml, and the help of this branch's binary (tree of b8e3baef). Lifecycle: start; implement; tick every Done when line and write the Report; review request, which refuses until both hold; findings, fix_now fixes re-reviewed, approval, dispositions; merge, which marks the owner and open children done; no close step; land at every handoff. The skill, astromech AGENTS.md:17-21 and atelier --help 41-51 give the same order. Stale: only code, meaning anything outside .atelier/issues, reviews and evidence, not counting a clean base merge, with policy, config and strategy files counting as code; record edits never. A Done when line rewritten after start is shown in the brief and by merge --check and refuses nothing (skill, AGENTS.md:21-23, help 44-46); Independent Review still has a non-trivial rewrite reviewed. Loose end: code-free work that can only follow a merge or release, unless it is an independent validation; filed when planning or as soon as it turns up, committed on the base even while the item is in flight, blocked_by it, reviewed as a planning change recorded against the loose end; done on the base with a one-line result; a release tag's loose end goes on the release branch with no blocker. Every flag cited exists; close and land hold one meaning; decision record and ADR agree with issue update --adr. Ambiguities left, none bearing on the three questions: done items in doctor and work missions may include superseded; a decided record replaced by a newer one stays done; whether --adr works after decide (it does, tried on a fixture); agent-made decision records versus astromech's 'Don't do them'; disposition value for a fixed defer finding; --identity for the human's approval; astromech CI not stated as a merge condition; spikes deleted unmerged; where moving a Done when line to a loose end is edited, since the in-flight record should not change on the base. Rounds 1 and 2 also found no contradictions; their ambiguities on loose-end review target, Report location, mission start, --commit-records scope, trivial exemption wording, ADR supersession and work human's help were fixed in 658dfe5 and b8e3baef.
- 2026-11-07 [result] astromech's AGENTS.md change merged on the light path as 7b9188b0 (approval astromech-dyr0) and is pushed to brewer-corp/astromech main; its AGENTS.md matches fb6bed36, the commit the cold read used.

## Report

The skill, Atelier's AGENTS.md and help, and astromech's AGENTS.md now agree, and the last of three fresh cold reads found no contradiction (result note of 2026-11-07). Skill: branch `atelier-uqxh` in spadaval/.agents, head **6bc0d5c** (cc8bbe9, e8bc071, 658dfe5, 6bc0d5c on main 9f60b8e), merged into its main with this item. astromech: branch `atelier-uqxh-agents`, head **fb6bed36** on origin/main e6ae38df, changing only the stale-approval sentence of AGENTS.md:21-22 (one line became two), merged on the light path as 7b9188b0 (approval astromech-dyr0) and pushed; line 5 is ticked.

- Line 1: references/atelier.md Loose Ends files a loose end straight to the base as planning, even while its issue is in flight, with the review recorded against the loose end; the work-branch case and "reviewed with the item it follows" are gone. After finding atelier-7s4p, Loose Ends and implement.md:35-40 also say to leave the in-flight issue's own record alone on the base and rewrite or drop the Done when line the loose end replaces on its work branch, where the review brief shows the edit (6bc0d5c). On a fixture with this branch's binary, a loose end filed and approved on the base while its item was in flight left that item's `merge --check` passing (no `merge_base_records`).
- Line 2: the Words section's done and close entries.
- Line 3: tracker-records.md, decide.md, repository-shape.md, strategic-plans.md, docs.md and plan.md say a decision record holds the question and decision and may produce an ADR (`issue update --adr`, which also works on a decided record, tried on the fixture).
- Line 4: `grep -rn one-way` finds nothing in the skill.
- Line 5: step 5's "gets a fresh approval" is gone; the staleness section, AGENTS.md:30-32, `atelier --help` and astromech fb6bed36 say the rewrite is shown in the brief and by `merge --check` and refuses nothing. `cargo nextest run -j 4 -p atelier-cli` passed (800 tests; setup_guidance asserts the help paragraph) and `cargo fmt -- --check` is clean.
- Line 6: three fresh subagents each read only the three sources; none found a contradiction. The third read 658dfe5, fb6bed36 (standing in for astromech main after merge) with origin/main's workflow.yaml, and the help of b8e3baef's tree. 6bc0d5c came after it and adds only the finding atelier-7s4p sentence.

### Deviations
Beyond the receipt's list, I fixed ambiguities the first two reads raised: where the Report is written, how a mission starts (`atelier start` refuses missions), which commands take `--commit-records`, "repository policy" for trivial exemptions, superseding decisions in docs.md and plan.md, a pointer to decide.md for a rule it does not state, and `work human`'s help, which omitted open decisions (b8e3baef). The light-path release tag still goes on the release branch (ADR 0028, atelier-4nsl), now stated as the case with no issue to wait on.

### Decisions
SKILL.md's "Changes after review" stays: the staleness section calls Atelier's rule its gate, not a waiver of that norm. The rule that the replaced Done when line changes on the work branch went in on the coordinator's direction (finding atelier-7s4p), although astromech AGENTS.md:36-39 still sends planning changes straight to main without that carve-out.

### Open items
The final read's other ambiguities, none bearing on the three questions: astromech AGENTS.md:36-39 has no carve-out for an in-flight item's own record; "done item" in `doctor` and `work missions` help possibly including superseded; agent-made decision records versus astromech's "Don't do them"; the disposition value for a fixed `defer` finding; `--identity` for the human's approval; astromech not stating CI as a merge condition; spikes deleted unmerged.
