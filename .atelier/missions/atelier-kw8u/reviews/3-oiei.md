---
schema: atelier.review
schema_version: 3
round: 3
commit: 183d7cb5544508270d377ea2d8afcadc083f7065
reviewers:
- 01a1f089-14c2-7912-82c2-17b41ba27a10
model: grok-4.6
verdict: changes
created_at: 2026-11-18T21:22:32.479059795Z
---

## Summary

Checked the mission and all ten items at 183d7cb against the three carried findings and the tree they name: `atelier-xfsn` after dc6ac5f8e; `atelier-1z52`, `atelier-mtzj` and `atelier-aq82` as 183d7cb left them; `evals/agent-factory/harness/run.py` (Claude `--dangerously-skip-permissions`, Codex `--dangerously-bypass-approvals-and-sandbox`, Codex `HOME` rewritten to the work directory); `next-rebuttal-kept` `setup.sh:14-15`, `grade.py:35-36` and `test_grade.py:17-18`; `drop` of a started item in `crates/atelier-cli/src/plan.rs` (`drop_started` at 282, `carry_drop` at 311, `Dropped:` under Report at 237, 299 and 334) versus `drop_mission` writing Result in `crates/atelier-core/src/plan.rs:211`; `dropped_started` in `crates/atelier-cli/src/output.rs:1352-1353`; `defects.md` after-merge rows; the 2026-11-11 cut at b22a03359 (What to judge, Close against the Outcome, implementer Building); skill word count 1,231; grader and test line count 3,106. CI `evals` already uses `fetch-depth: 0`. Did not read another reviewer of this round.

Complexity: high. The mission still adds a facts-and-judge grading contract, mixed-agent setups, a skill change to who may skip review, `show` of skips, new lead and review cases, and four extra skill arms.
Value: high. The evals currently grade wording and process, the skill contradicts the 2026-11-18 rule, and the loop is what the human asked to measure.
Risk of merging it: medium. A lead skipping review can land unreviewed code, which they chose. A replay whose later fix is still reachable from this host's other checkouts or from GitHub, including the case files once they merge, still lets a reviewer grade by lookup.

The value warrants the work. Findings 1 and 3 are fixed. Finding 2 still stands: 183d7cb replaced a false isolation claim with a report of leaks, and left confinement optional.

Could not check: the live run cache under `~/.cache/af-evals`; whether three after-merge diffs in `defects.md` are small enough for one reviewer to hold (several rows are one to three blamed lines, so three holdable ones exist to choose from); this host's exact Claude effort for Fable and Opus.

## Finding 1: Allow the retained rebuttal fixture to follow renamed dependencies
Recommendation: fix now
Was: 2-3vc6.8
Now: fixed

`.atelier/items/atelier-xfsn/item.md:47-51` now keeps `next-rebuttal-kept`'s name, fixture and what it grades for atelier-1tkg, and says this item changes the paths it sources and imports and its calls into the shared grader, and nothing else. Lines 56-57 require `test.sh` and CI's `evals` job to pass with every case in them, that fixture included. The fixture still sources `next-loop-changes/setup.sh` and `next-loop/lib.sh` (`setup.sh:14-15`) and imports `next-loop/grade.py` and `next-loop/test_grade.py` (`grade.py:35-36`, `test_grade.py:17-18`), which is what xfsn is now allowed to retarget. 1tkg remains blocked by xfsn, so the rename of the case itself still happens after the intermediate gate.

## Finding 2: Replay fixtures still leak the later fix
Recommendation: fix now
Was: 2-ap8j.8
Now: kept

183d7cb dropped the false "so the fix cannot be read from it", added a new item ID, and required a Report of whether each of three paths (the fixture, this host's other checkouts, the network) is kept from or only found out afterwards from the transcript, with lookup invalid ([item.md](.atelier/items/atelier-1z52/item.md) lines 31-44; mission `reviews-from-history`). That stops a looked-up run from counting as a review when the transcript shows it. It does not hide the later fix.

`evals/agent-factory/harness/run.py` still runs the agent with permissions skipped: Claude `--dangerously-skip-permissions` (line 160), Codex `--dangerously-bypass-approvals-and-sandbox` (line 171), and the file still says the agent has full access to `$HOME` and the network (lines 76-77). Codex rewrites `HOME` to the work directory (line 217); that does not block an absolute path such as `/home/supadava/projects/atelier` or another worktree. The mission's target is this host, and the repository is public ([mission.md](.atelier/missions/atelier-kw8u/mission.md) lines 192-194).

The case files still name the `defects.md` row, the cause commits and the commit that fixed it (1z52 lines 45-47). "The case file stays outside it" (line 36) only keeps those files out of the fixture clone. They live under `evals/agent-factory/scenarios/` in this repository. id3v is blocked by 1z52, so the files merge, and are on GitHub, before the replay reviews run. A new item ID does not hide that answer key. Recreating the original Goal and Done when (line 33) is a second search key on the public history.

1z52's Done when is met by a Report that says every path was only "found out afterwards", with no test that a command trying the later fix fails (the test is required only for paths that are kept from). Authority still says the replayed reviews share "one way of keeping the later fix from a reviewer, which atelier-1z52 builds" (mission.md lines 213-215). Name that way in 1z52 so a run is actually kept from this host's other checkouts and from fetching this repository, including the case files: the model API may use the network; a fetch of the later fix may not. Transcript invalidation is a backstop for a missed leak, not the confinement.

## Finding 3: The authorized drop does not write the mission Result the case grades
Recommendation: fix now
Was: 2-ap8j.9
Now: fixed

atelier-mtzj now gives the steps the binary takes for a started item: drop on its own branch, then the base copy on a backlog branch, with the reason `atelier drop` records, and says the mission's Result names at close what was not done and why ([item.md](.atelier/items/atelier-mtzj/item.md) lines 42-49). atelier-aq82 passes when the third item "reads as dropped on main with the reason `atelier drop` recorded", and the walks cover a drop carried to main and one that stopped on the item's branch, which fails ([item.md](.atelier/items/atelier-aq82/item.md) lines 38-46).

That matches the binary: `drop_started` writes `Dropped: {reason}` under the item's Report on its branch (`crates/atelier-cli/src/plan.rs:282-302`); `carry_drop` writes the same on the base copy (lines 311-337); `dropped_started` still tells the agent the second `atelier drop` on a backlog branch (`crates/atelier-cli/src/output.rs:1352-1353`). `drop_mission` is the only path that writes Result (`crates/atelier-core/src/plan.rs:211`). The case no longer grades a Result line `atelier drop` does not write.
