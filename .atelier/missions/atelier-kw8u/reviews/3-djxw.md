---
schema: atelier.review
schema_version: 3
round: 3
commit: 183d7cb5544508270d377ea2d8afcadc083f7065
reviewers:
- thread:delegated-task:command%3Amcp%3A899a046c-fc07-4a9b-a0a8-05ac7d8b429d%3Adelegate-task%3Ad5869196-d2bd-40b2-b6bc-4a6b6b989a5d
model: gpt-6-astra
verdict: approve
created_at: 2026-11-18T21:21:28.803994679Z
---

## Summary

Complexity: high. The plan changes lead authority, mixed-provider execution, grading and the evidence for retaining skill passages across ten items.

Value: high. Outcome grading and realistic role setups directly serve the requested measurement of the lead, implementer and reviewer loop.

Risk of merging it: medium. This is a plan, but it governs subsequent changes to the live skill and eval gates; those changes are reviewable and reversible.

The value warrants the work. I read the mission and all ten items at 183d7cb5544508270d377ea2d8afcadc083f7065, compared the revision with round 2, and checked the carried findings against the harness, retained rebuttal fixture, test runner and binary drop guidance. The dependency corrections let the intermediate fixture gate remain intact. The replay requirement now permits either tested prevention or transcript-based exclusion of lookup, rather than assuming removal of a remote confines the reviewer. The drop case now matches the base record that the command actually updates.

`atelier check` passes record validation, with existing empty-Why notices for atelier-4pzj, atelier-icv4 and atelier-uf97. `git diff --check` passes. I did not run model evals or implementation tests for this records-only plan. The proposed replay protection and mixed-provider execution must still be demonstrated by their implementation items. No new fix-now finding.

## Finding 1: Allow the retained rebuttal fixture to follow renamed dependencies
Recommendation: fix now
Was: 2-3vc6.8
Now: fixed

atelier-xfsn explicitly permits updates to sourced and imported dependency paths and calls into the migrated shared grader while preserving the fixture and what it grades for atelier-1tkg. Its gate explicitly includes next-rebuttal-kept. This resolves the dependency conflict in setup.sh, grade.py and test_grade.py without changing the planned conversion order.

## Finding 2: Replay fixtures still leak the later fix
Recommendation: fix now
Was: 2-ap8j.8
Now: fixed

atelier-1z52 now requires a new item ID, excludes old records and the case file from the fixture, and addresses all three lookup routes: fixture history, other local checkouts and network. For each route it requires either prevention demonstrated by an attempted access or transcript-based detection that invalidates the run. The mission's reviews-from-history criterion also excludes lookup runs. Full confinement is not the only way to address the measurement defect; excluding observed lookup is a proportionate alternative for these evals. This approves the requirement, not a claim that the existing harness already enforces it.

## Finding 3: The authorized drop does not write the mission Result the case grades
Recommendation: fix now
Was: 2-ap8j.9
Now: fixed

atelier-mtzj now requires the actual started-item sequence: drop on the item branch, then carry the base copy through a backlog branch. It defers the mission Result account to close. atelier-aq82 grades the dropped record and recorded reason on main, and requires walks for a completed carry and an incomplete item-branch-only drop. This agrees with dropped_started in crates/atelier-cli/src/output.rs and no longer assumes an item drop writes the mission Result.
