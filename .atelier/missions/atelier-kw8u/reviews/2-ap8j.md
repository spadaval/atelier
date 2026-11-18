---
schema: atelier.review
schema_version: 3
round: 2
commit: 163f081ecf1932c725440f370ad5b8c23707214f
reviewers:
- 01a1f081-2ab9-7ad1-9925-f180a079a500
model: grok-4.6
verdict: changes
created_at: 2026-11-18T21:13:26.956408408Z
---

## Summary

Checked the mission and all ten items at 163f081 against the seven round-1 findings and the tree they name: the four role files and SKILL.md Independent Review; `Skip::pass` and `with_skips` (rule and reason only); `log.rs` merge thread from `implementers.holder()`; `say.rs` kept-finding line and `view_output.rs:625` dispute heading; `atelier drop` of started items vs mission Result; evals harness (`run.py` agent has `$HOME` and network); `defects.md` after-merge rows; grader and test line counts (3,106); SKILL.md and `roles/` word count (1,231); the 2026-11-11 cut at b22a03359. Did not read another reviewer of this round.

Complexity: high. The mission still adds a facts-and-judge grading contract, mixed-agent setups, a skill change to who may skip review, `show` of skips, new lead and review cases, and four extra skill arms.
Value: high. The evals currently grade wording and process, the skill contradicts the 2026-11-18 rule, and the loop is what the human asked to measure.
Risk of merging it: medium. A lead skipping review can land unreviewed code, which they chose. A replay whose later fix is reachable from `$HOME` or the network, or a drop path that the binary does not implement as the case grades it, mis-counts the two questions the Outcome asks.

The value warrants the work. Round 1's seven findings are fixed. Two contradictions remain before the mission starts.

Could not check: the live run cache under `~/.cache/af-evals`; whether three after-merge diffs in `defects.md` are small enough for one reviewer to hold; this host's exact Claude effort for Fable and Opus.

## Finding 1: Keep assessments of recorded reasons out of the pass predicate
Recommendation: fix now
Was: 1-fmx8.1
Now: fixed

atelier-b9o6 now splits graded answers from reported ones and requires a harness walk whose reported answer fails while the run still passes. atelier-xfsn asks whether a recorded reason holds as a reported question, which changes no grade.

## Finding 2: Record the actual bypass caller before promising to display it
Recommendation: fix now
Was: 1-fmx8.2
Now: fixed

atelier-nv22 records the invoking thread in the skip trailer, keeps reading trailers that lack one, shows an honest unknown for those, and tests a lead merging an implementer's item past a rule. Scope now includes where a passed rule is written, not only `show`.

## Finding 3: Let the stalled-work case recognize an authorized documented drop
Recommendation: fix now
Was: 1-fmx8.3
Now: fixed

atelier-aq82 now passes a documented drop of the third item, fails a drop with no reason, and walks both. The remaining mismatch between that pass and what `atelier drop` actually writes is Finding 9.

## Finding 4: 1tkg is blocked until xfsn deletes the fixture it copies
Recommendation: fix now
Was: 1-tsl4.1
Now: fixed

atelier-xfsn now leaves `next-rebuttal-kept` untouched for atelier-1tkg, and the README says so. The directory is still there when 1tkg starts.

## Finding 5: house names one judge, but its threads are from two labs
Recommendation: fix now
Was: 1-tsl4.2
Now: fixed

The Outcome, atelier-b9o6 and atelier-8e8e now give `house` two judges, one per lab, and a harness test that the reviewer's question and the implementer's go to different judges.

## Finding 6: lead-facing dispute text lives outside mtzj's say.rs scope
Recommendation: fix now
Was: 1-tsl4.3
Now: fixed

atelier-mtzj names `view_output.rs` and the dispute heading, and mission Scope In covers lead-facing lines wherever they live in `crates/`.

## Finding 7: Scope In omits the files oiwy restores
Recommendation: fix now
Was: 1-tsl4.4
Now: fixed

Scope In now includes the passages atelier-oiwy tries and keeps, in `SKILL.md` and `roles/`. Scope Out and Authority match that.

## Finding 8: Replay fixtures still leak the later fix
Recommendation: fix now

atelier-1z52's isolation is "the fixture's history ends at the change and it has no remote, so the fix cannot be read from it" ([item.md](.atelier/items/atelier-1z52/item.md) lines 33-35). That "so" does not hold on this harness. `evals/agent-factory/harness/run.py` lines 76-77 run the agent as the user with permissions skipped and full access to `$HOME` and the network. The mission's target is this host, and the repository is public ([mission.md](.atelier/missions/atelier-kw8u/mission.md) lines 192-193). The case files name the `defects.md` row, the cause commits and the fix commit (1z52 lines 36-38), and the fixture recreates the original Goal and Done when, so the original item identity is in the tree the reviewer is given.

A reviewer can read `~/projects/atelier`, this clone's other worktrees, or GitHub, and grade as if it found a defect it looked up. `reviews-from-history` then counts lookup, not review. Name a confinement that actually hides the later fix: no network, no `$HOME` source clones, and new item ids so the public history cannot be searched from the records. A local remote is not enough.

## Finding 9: The authorized drop does not write the mission Result the case grades
Recommendation: fix now

atelier-mtzj tells the lead to `atelier drop` an item "and say in the mission's Result what was not done and why" ([item.md](.atelier/items/atelier-mtzj/item.md) lines 42-46). atelier-aq82 passes the stalled-work case when the third, started item "is dropped with the reason in the mission's Result" (lines 38-45).

`atelier drop` does not write that Result for an item. `drop_mission` in `crates/atelier-core/src/plan.rs` line 211 puts `Dropped: {reason}` under the mission Result. Drop of an item puts the same line under the item's Report (lines 237 and 273). A started item, which aq82's third item is ("in changes requested"), is dropped only on its own branch (`drop_started`, lines 259-260 and 289-290). The binary then says the base copy still shows started until a second `atelier drop` on a backlog branch (`crates/atelier-cli/src/output.rs` lines 1352-1353).

A lead that follows the new skill from the mission checkout cannot complete the pass aq82 treats as authorized discretion: drop refuses off the item branch, the base still lists the item as started, and neither write is the mission Result. Say the real sequence in the skill (item branch, then backlog carry, then a Result edit if that is still required), and grade that sequence, including a walk that stops after the first drop.
