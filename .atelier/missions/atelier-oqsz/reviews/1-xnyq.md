---
schema: atelier.review
schema_version: 3
round: 1
commit: d1de63be53787c524d5f19275ca9ca412c8e3509
reviewers:
- 01a21be2-67b9-7a61-ad8e-4aa41f119054
model: grok-4.6
verdict: changes
created_at: 2026-11-27T07:23:10.847177271Z
---

## Summary

Reviewed the plan as commit d1de63be53787c524d5f19275ca9ca412c8e3509 holds it: `mission.md`, `notes.md`, and the four items (`atelier-f0sn`, `atelier-7ci6`, `atelier-qut2`, `atelier-hota`). Checked the Why against the Hub lead's saved `status` outputs (41 files, largest 17,870 bytes at `hook-055e1695-…-stdout.txt`), merge commits `122403ea` and `acf8a7f4` in `/home/supadava/projects/industrial-experience-hub`, and today's binary: `status` prints `Can start now` before `You lead mission` (`crates/atelier-cli/src/output.rs` lines 223 and 267), `merge_message` always lists every configured check as passed (lines 1816–1820), and `Skip::refuse` passes every rule once a reason is set (`crates/atelier-core/src/skip.rs` lines 102–116). Read `atelier-ww7w` (done; five plan-review rounds), `atelier-oiwy` (open; freeze on `roles/reviewer.md` and the first four parts of `SKILL.md`), `atelier-5asm`, `atelier-3sym`, `atelier-tuln`, `atelier-itqc`, `atelier-b0vb`, `atelier-7w9s`, and `atelier-pv58`. Read `roles/planner.md`, `roles/reviewer.md`, `roles/lead.md`, `SKILL.md`, and `docs/product.md` on `status` and skip.

Could not check: a timed hand count of `hub-v2-bg14` review rounds (pace-shown's Report check); `atelier log` size on the Hub checkout (the path in the item is a saved-output directory, not the git worktree). Did not read another reviewer's file.

- Complexity: medium. Four items, three surfaces in the binary plus the skill, and a measurement that depends on this review.
- Value: high. The Hub lead's `status` hid the missions it led behind the backlog; two merges recorded a failed check as passed; `atelier-ww7w`'s plan review ran five rounds.
- Risk of merging it: medium. A skip that no longer covers a failed merge check will show; a bad `status` order or a vacuous skill re-review will also show. Silent wrong merge records are what `atelier-7ci6` removes.

The four outcomes match what the Why asked for, and `atelier-f0sn`, `atelier-7ci6`, and `atelier-hota` have checks an implementer can run. `skill-measured` does not: it names no plan to re-review, and the skill change it cites is off the path those reviewers are told to read. No smaller mission gets the Hub `status` and the dishonest skip. Two contract gaps need closing before this plan starts.

## Finding 1: Name the plan that skill-measured re-reviews
Recommendation: fix now

`mission.md` lines 63–66 (`skill-measured`) and `.atelier/items/atelier-qut2/item.md` lines 53–56 require that "one plan that was reviewed under the old text is reviewed again under the new by two fresh reviewers", with verdicts and outcome/mechanism counts side by side. Neither names the plan, nor a rule for picking it.

The Why's comparison is `atelier-ww7w`: five rounds, a 2,400-word plan, 45 findings by the Hub lead's count. This plan is the other candidate: one page, reviewed under today's skill, with `notes.md` already set up to mark each finding. Those are different measurements. A third pick, a small already-approved plan, makes the criterion pass without testing the claim.

Name the plan in `skill-measured` and in `atelier-qut2`'s Done when: this mission's plan, `atelier-ww7w`, or a stated rule (for example the last plan that split). Until that is written, two implementers can both tick the box and not have measured the same thing.

## Finding 2: The skill change skill-measured cites is off the path a plan reviewer reads
Recommendation: fix now

`skill-measured` (`mission.md` lines 63–66) is met only if the re-review runs under the new skill text. `atelier-qut2` also requires that "a plan's reviewer is told what a plan review is for, where a finding about a mechanism goes, and when plan review stops" (item.md lines 47–49).

The path those reviewers follow today is:

- `skills/agent-factory/SKILL.md` lines 16–21 (Your Role): a reviewer of a plan reads `roles/reviewer.md`.
- `roles/reviewer.md` line 26: "Report every other finding you can support as well, whatever your verdict."
- The brief `atelier review <mission>` prints, from `crates/atelier-cli/src/output.rs` line 1501: "write down each problem you find."

`atelier-qut2`'s Scope (`item.md` lines 61–66) is `skills/agent-factory/` and the documents that repeat it. While `atelier-oiwy` is open it may change neither `roles/reviewer.md` nor the parts of `SKILL.md` before Starting Another Thread. The brief lives in `crates/`, which that item does not name. There is no unfrozen file on the required read path.

The item says to "put text for a plan's reviewer where a plan's reviewer will read it without them, and file what can only go in those two files as an item blocked by `atelier-oiwy`." A follow-up blocked by `atelier-oiwy` lands after `atelier-qut2`'s re-review if that re-review is in `atelier-qut2`. The measurement then still loads `roles/reviewer.md` and the old brief.

Pick one in the plan, and put it in `atelier-qut2`'s Done when:

- Add the plan-review brief in `crates/atelier-cli/src/output.rs` to this item's Scope, so the text `atelier review <mission>` prints is the new rule, and the re-review uses that binary; or
- Make `skill-measured` wait on the item blocked by `atelier-oiwy` that edits `roles/reviewer.md`, and say the re-review runs only after that edit is on the branch the reviewers read.

The later-round code-review bullet (`item.md` lines 49–50) has an escape ("or the Report says why the evidence does not support that"). The plan-review measurement does not. The brief already tells a later *code* round to stay on what changed (`output.rs` lines 1358–1370); `roles/reviewer.md` line 26 still tells that reviewer to report every finding. That is the same read-path hole, and the Report escape covers it. Plan review has none.
