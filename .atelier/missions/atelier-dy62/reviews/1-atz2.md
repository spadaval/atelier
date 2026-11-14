---
schema: atelier.review
schema_version: 3
round: 1
commit: c22b1377b1cbc8ed1ddd9d4f675dea268c5eafc7
reviewers:
- thread:delegated-task:command%3Amcp%3A7528a691-d081-4f17-89b8-d45bea97b6e3%3Adelegate-task%3Aae4d0ff6-cfcd-4ac2-ae71-2c61c72b6420
model: gpt-6-astra
verdict: changes
created_at: 2026-11-14T05:58:48.290808620Z
---

## Summary

Complexity: medium. Three mechanical changes cross the library, CLI, Desk and documentation, with a required item from another mission between them.

Value: medium. The merge removes misleading crate names and an actual obstacle to sharing Git helpers in code used by both programs.

Risk of merging it: medium. Moving shared record and process code can affect both callers, but the existing contract tests and a restriction to mechanical changes make regressions checkable and reversible.

The value warrants the scoped refactor. I found no smaller change that achieves the agreed single-crate outcome; the optional terminology rename is already identified as the part to drop if necessary.

I reviewed commit c22b1377b1cbc8ed1ddd9d4f675dea268c5eafc7: the mission, all three items, and the dependency on atelier-yclc. I checked the current crate manifests and module layout, Git and forge call sites, qualified paths in tests, the Desk's action route and generated types, and the relevant design and architecture documentation. This is a plan review; no implementation exists to build or test. The two findings concern acceptance criteria that the current sources show cannot both be met as written.

## Finding 1: Compare behavior against each item's base, allowing the required open-mission stop feature
Recommendation: fix now

`.atelier/missions/atelier-dy62/mission.md:69-72` requires every test from before atelier-ni3d to pass after the mission with only import and terminology changes. However, lines 103-106 require atelier-yclc to merge between atelier-ni3d and atelier-x5al. That item explicitly adds the Desk's ability to stop an open mission (`.atelier/items/atelier-yclc/item.md:34-36`).

Existing tests assert the opposite behavior: `crates/atelier-desk/tests/acts.rs:344-360` asserts `can_stop == false` and a refused stop request for an open mission, and `desk/e2e/acts.spec.ts:125-138` expects only manual commands. Those assertions must change for atelier-yclc, so the mission's original test baseline cannot remain unchanged through its required dependency. This is an intended feature from the other mission, not a regression to prevent.

Scope the preservation check to the changes made by this mission's items against their respective bases, with atelier-x5al preserving the behavior after atelier-yclc. Explicitly allow and account for atelier-yclc's test changes in any mission-wide comparison. Keep the unchanged assertions requirement for the refactor itself.

## Finding 2: Allow mechanical crate-path changes inside test bodies
Recommendation: fix now

`.atelier/items/atelier-ni3d/item.md:31` permits only changes to `use` lines in each test file, and the mission repeats this restriction at `.atelier/missions/atelier-dy62/mission.md:69-72`. Yet the item also removes both old crates and forbids their names outside records and historical text (lines 27 and 34).

The tests use those crate names outside imports. For example, `crates/atelier-records/tests/records.rs:649` and `:658` call `atelier_records::put_first`, `crates/atelier-records/tests/load.rs:65` names `atelier_records::Problem` in a closure parameter, and `crates/atelier-desk/tests/acts.rs:367` calls `atelier_app::plan::drop_mission`. Removing the crates requires editing those lines too. Import aliases would retain the expressly forbidden old names.

Allow mechanical crate/module path substitutions wherever they occur in tests, while keeping test inputs, assertions and behavior unchanged. Apply the same wording to the mission's success criterion so a correct implementation can satisfy both checks.
