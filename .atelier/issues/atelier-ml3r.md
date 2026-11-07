---
created_at: "2026-11-07T05:01:01.441016682+00:00"
id: "atelier-ml3r"
issue_type: "feature"
labels: []
workflow_branch:
  branch_base: master
  work_branch: atelier-ml3r
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-3jdm"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-4nsl"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-8bh4"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-ilba"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-jnrh"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-l457"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-md0v"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-mmpv"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Say the lifecycle once, plainly, in the skill and Atelier's own docs"
updated_at: "2026-11-07T13:50:06.456180466+00:00"
---

## Goal

Make the agent-factory skill, the CLI help and Atelier's own docs describe the new lifecycle in the same words, with one meaning per word and nothing left of what this mission removed. A fresh agent read about 1,000 lines across eight files and still could not state the order; "check" alone meant at least six things. The rollout to the other repositories is atelier-nn67 and follows this item.

## Done when

- [x] Before the skill change merges, astromech's CI pin (`ATELIER_REV`) is at or past atelier-4nsl's merge, a commit that reads every record form the installed binary writes (including the light path's version-3 review records) and judges them by this mission's rules, moved through astromech's own process (astromech-5sp5 moved it to 55cf6072 on 2026-11-07 as a stopgap), since the skill, which describes the light path, is live in every repository the moment it merges.
- [x] The agent-factory skill (spadaval/.agents) describes the new lifecycle in its atelier reference and its orchestrate, implement, review, plan and validate procedures: start, implement, tick every Done when line and write the Report, review request, fix and re-review, approve, merge (which marks work done); approvals that only code makes stale; loose ends; the light path; records correctable until they reach the base; a `fix_now` finding resolved by the approval after its fix. The change is made in a worktree (`git -C ~/projects/agents-skills worktree add ~/projects/agents-skills-wt/atelier-ml3r -b atelier-ml3r main`; never switch the live checkout), the reviewer reviews that branch at a named commit together with this item's Atelier diff, and exactly that commit merges into its `main` (no-ff) when this item merges.
- [x] Nothing the mission removed is left in the skill or its evals (`evals/agent-factory`): `start --base`, `<issue_type>/<id>` branch names, close before or after review, After merge markers and stand-ins, `carried to`, `fix_now` dispositions with resolution commits, receipts for suite runs, the reviewer-versus-Git-author rule, recorded bases. The skill's total line count falls.
- [x] One meaning per word, in prose; no flag or policy key is renamed (that would need new shims). CONTEXT.md defines: check = `atelier check` (record validation), merge checks (the policy's merged-tree commands) and `merge --check` (merge gates without merging), and prose uses "line" for a Done when line; land = `atelier land`, the handoff and pushed-commit check; close = what a mission does (the `close_reason` key and `--reason` keep their names and write the Report); review = independent review; done = the status work gets when it merges, decisions, loose ends and validations get on the base, and a closed mission has, with superseded as the other terminal status. CLI help and docs use the words that way.
- [x] `start`'s detached-HEAD refusal suggests `git switch -c <id> <base> && atelier start <id>`, which works in a linked worktree, and docs/product/work-model.md no longer says old guidance passing `--base` "does not fail" (finding atelier-ywpf).
- [x] ADR 0028 (decisions 2, 3 and 5, Consequences), docs/product/workflow-configuration.md and docs/product/work-model.md name atelier-nn67, not this item, for astromech's pin, the `branch_base` drop, astromech's validation items and the other repositories' policies.
- [x] Atelier's own AGENTS.md and docs/product describe the lifecycle in the same words as the skill.
- [x] A fresh subagent given only the rewritten skill, Atelier's AGENTS.md and the CLI help states the lifecycle order, what makes an approval stale and when to file a loose end, and reports no contradiction; its report is attached as a note. (The legible criterion itself is judged by the mission's validation after atelier-nn67, on astromech's AGENTS.md.)

## Scope

The agent-factory skill and its evals in spadaval/.agents, Atelier's CLI help text and `start`'s detached-HEAD hint, CONTEXT.md, AGENTS.md, docs/product, ADR 0028. astromech only for its CI pin, if it needs moving again. Not the other repositories' policies or AGENTS.md (atelier-nn67).

## Context

docs/adr/0028-simpler-lifecycle.md (decision 8), docs/strategies/simpler-lifecycle.md. Everything else in the mission except atelier-nn67 has merged before this starts, so the skill describes what is built.

## Notes

- 2026-11-07 [result] Cold read (Done when line 8), fresh general-purpose subagent given only the skill at agents-skills atelier-ml3r (before fc5e397's last two clarifications), this worktree's AGENTS.md and root `atelier --help`. Eighth read; reads 1-7 each found contradictions, all fixed before the next read: trivial and install commits on the base, land on the base vs planning commits, record edits after review, strategy files as code, unreviewed merge on human direction, merged-tree testing, the five words (land called a check, informal done, brief/debrief as review), FIX NOW as a disposition, increment Done when naming review or validation, receipt targets, dispositions on an orchestrator's own work, mission vs increment validation vs loose ends, and who starts, requests review and merges.
  Its answers. Order: planning reviewed first; start (orchestrator or author); implement (author); tick every Done when line, write ## Report, commit (author); review request (orchestrator), which refuses on an unticked line or empty Report and leaves the Report out of the brief; reviewer records findings and approval with --commit-records, author fixes each fix_now finding in a later commit, the fix is re-reviewed (fix diff only, same reviewer allowed), the accountable agent records dispositions for deferred or dismissed findings and ones recommended defer or no_action; in this repo push, draft PR, CI; merge (orchestrator), one merge commit that marks the owner and every open child done, no close step; land, then push only if land exited zero. Stale: only code, anything outside .atelier/issues, reviews and evidence since the approved commit (policy, config and strategy files count), not record edits, not a clean base merge; a non-trivial record change after approval still gets a fresh approval by process. Loose end: work that changes no code and can only happen after a merge or release (independent validation is a validation item instead), filed when planning or when found, `issue create --issue-type loose_end` then `issue link <loose-end> <issue> --role blocked_by`, never a child or in a mission, done on the base with `issue transition <id> done --reason "<result>"`, a bad result files a linked bug.
  Contradictions: none found. Ambiguities it listed, most important first: ticking a failed loose end's line; whether a fix_now finding recorded after approval blocks merge (both clarified in fc5e397 after the read); which record edits need a fresh approval; strategy files counted as code for staleness but committed on the base; whether this repo's CI tests the merged tree once master moves; help and AGENTS.md summaries omit dispositions; review.md does not mention the reviewer recording dispositions; a solo author converting a line to a loose end; where a decision becomes done; review covering a human review of an unmerged change; two homes for the word definitions; trivial exemptions in this repo; where the pull request fits; "no review" for doing a loose end vs filing one; generated files on a base sync; children's Reports.
- 2026-11-07 [decision] Define land without the word check: atelier land, run at every handoff, judges committed records and the commits about to be pushed
  Reason: The Done when's phrase 'the handoff and pushed-commit check' gave check a fourth meaning beside atelier check, merge checks and merge --check; the first cold read flagged it, and the meaning is unchanged
  Rejected: Keep 'handoff check' and list land as a fourth sense of check
- 2026-11-07 [decision] Give the skill's atelier reference its own Words section with the same five definitions as CONTEXT.md
  Reason: The skill is read in every Atelier repository, where CONTEXT.md is Atelier's own and not present; the cold read needed the definitions in the skill itself
  Rejected: Point the skill at Atelier's CONTEXT.md
- 2026-11-07 [decision] Leave command output (verdict lines counting checks, closed for done work, held at close) to follow-up atelier-xz5c
  Reason: Scope is help text and the start hint; hooks, quiet tokens and many tests read those lines
- 2026-11-07 [result] astromech's CI pin moved to atelier 12efdfd9 (atelier-4nsl's merge) through astromech-5sp5, merged as c10c8cbe and pushed to brewer-corp/astromech main on 2026-11-07; its atelier doctor job built 12efdfd9 fresh and passed (run 42369239411).

## Report

The agent-factory skill, Atelier's CLI help, CONTEXT.md, AGENTS.md, docs/product and ADR 0028 now describe one lifecycle in the same words: start, implement, tick every Done when line and write the Report, `review request`, fix and re-review, approve, `merge`, which marks the work done, with no close step; approvals only code makes stale; records correctable until they reach the base; a `fix_now` finding resolved by the approval after its fix; loose ends; the light path. The skill to review is spadaval/.agents branch `atelier-ml3r` at `67151360091c48e0084c3305417ffd4d0f861dd3` (two commits on `main` 6f42764: fc5e397, and 6715136 answering the review), together with this branch's diff from `417e9e0c`. Skill and evals went from 4326 lines to 4321 (`git ls-files skills/agent-factory evals/agent-factory | xargs wc -l`; skill 2662 to 2650, evals 1664 to 1671, the grader gaining a function that ignores box state). Proof: a residue search of the skill and evals finds none of the removed concepts; `cargo fmt -- --check` and `cargo nextest run -j 4 --workspace` pass (1019 tests); the new `the_detached_head_step_works_in_a_linked_worktree` test runs the refusal's step in a linked worktree, and it and the updated detached-HEAD test fail against the old hint; `test_root_help_lists_every_command_and_lifecycle_common_commands` pins the order in root help; all four eval setups run against the installed binary, and done-when-rewrite's grader passes the untouched fixture and one whose line is ticked and fails one whose criterion is loosened (the old grader failed the ticked one). The eighth fresh cold read reported no contradiction (result note).

### Deviations

- land is defined without the word check ("run at every handoff: it judges committed records and the commits about to be pushed") instead of the Done when's "handoff and pushed-commit check", which gave check a fourth meaning (decision note).
- Command output still counts gates as `checks`, calls done work `closed`, says `held at close`, and the `review_request_unfinished` refusal says work is "done" before it merges. Output was out of scope, so it is filed as atelier-xz5c, and CONTEXT.md's Gate row says so.
- Beyond help text and the start hint, three small strings changed for the same words: init's next step reads "Read the generated policy" (two tests follow), and the starter policy's and Atelier's own `decide` description says decisions "are decided", not "may close", on the base.
- ADR 0028 decision 2 said the reviewer reads the implementer report; the strategy and the CLI leave it out of the brief, so the ADR now says so.
- The cold read took eight rounds. Rounds 1 to 7 each found contradictions, fixed in the skill, AGENTS.md and help before the next round (listed in the result note). After the clean eighth read, fc5e397 clarified its two top ambiguities: a failed loose end's line is not ticked, and merge needs an approval recorded after every `fix_now` finding left without a disposition.

- The first review's seven fix_now findings (atelier-8ljp, atelier-iup5, atelier-h2i8, atelier-ggic, atelier-qt4c, atelier-bzic, atelier-ejwl) are fixed in agents-skills 6715136 and in this branch's commit after 5581c979: the eval fixture's boxed line, the remaining help and docs wording, the plain `git merge` rule, ADR 0028's owner of `--base`, the validation item's definition, the narrowed uses of check, and this Report.

### Decisions

See the item's decision notes: land defined without check; the skill keeps its own Words section matching CONTEXT.md; command output left to atelier-xz5c.

### Surprises

- The done-when-rewrite eval's setup ran the `request_review` transition, which the starter policy no longer has, so the scenario could not set up; it now skips it.
- Stale claims outlived their removal: `merge --sync` help still said it reruns receipts, work-model.md still described the removed follow-up source rule, zen.md said approvals come from another identity and every finding needs a disposition, and reviews-and-merge.md said merge writes no tracker state and to record fresh evidence after a base merge.

### Open items

- Done when line 1 holds: astromech-5sp5 moved astromech's CI pin to atelier 12efdfd9, atelier-4nsl's merge (result note).
- atelier-xz5c: command output words.
- `start_branch_owned`'s step, `git switch <base> && atelier start <id>`, fails in a linked worktree just as the detached-HEAD one did; not changed here (scope).
- `start --base` stays accepted and hidden; the skill no longer passes it, and ADR 0028 now leaves dropping it to atelier-nn67.
- The last cold read's ambiguities stand (result note), notably whether this repository's CI tests the merged tree once master moves, and that the short orders in help and AGENTS.md omit dispositions.
- The eval scenarios were not rerun with an agent against the new skill; only their setups and the done-when-rewrite grader on fixtures were run.
