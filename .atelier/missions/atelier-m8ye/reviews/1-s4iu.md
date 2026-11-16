---
schema: atelier.review
schema_version: 3
round: 1
commit: cf2cf2f32c0f2e03b7368cdb79c5c1d1b7a77db4
reviewers:
- thread:delegated-task:command%3Amcp%3A932557e7-853d-4352-845a-e9951a7775d5%3Adelegate-task%3A76c0f36d-f2ed-476c-8a62-0c105097fd0c
model: gpt-6-astra
verdict: changes
created_at: 2026-11-16T19:26:48.432495403Z
---

## Summary

Complexity: high. The plan introduces a release channel and an updater, imports history, and adds a test spanning the browser and Rust server.
Value: high. Keeping the Desk and its extension together with a contract test directly addresses their documented coordination failures.
Risk of merging it: medium. These are planning records, but contradictory acceptance criteria would send unattended implementers into incompatible work.

The value warrants the work. I read the mission and all four items at cf2cf2f, the current CI selector and workflow, dist configuration, the Desk's dependency manifest and ADR 0032 decisions. I also read the source extension's README, installer and agent instructions at a242410 from the existing local clone. I compared acceptance criteria, dependencies and scope across the records. This is a plan review; I did not run future workflows, measure CI timing, or validate installation on Windows or macOS.

## Finding 1: Allow the documentation changes in the import's tree comparison
Recommendation: fix now

`.atelier/items/atelier-n4g7/item.md:30` and `.atelier/missions/atelier-m8ye/mission.md:56` require the entire imported tree to equal a242410, allowing only whitespace changes. But the item's line 33 requires its README to stop describing a separate repository, and permits folding AGENTS.md into it. The README at a242410 explicitly tells the user to clone spadaval/t3-code-atelier and refers to the Desk documentation in another repository. Meeting line 33 therefore fails line 30 and the mission's same-tree criterion. Define equality for the import commit before the documented adaptation, or explicitly allow the named documentation changes in the final-tree comparison. Preserve a separate comparison of executable files so the import remains auditable.

## Finding 2: Decide whether extension releases may build the Desk for testing
Recommendation: fix now

`.atelier/missions/atelier-m8ye/mission.md:43` says the extension release builds no Rust. `.atelier/items/atelier-451s/item.md:30` requires that workflow to run the fixture test as atelier-7tg6 does, whose first Done when criterion requires an atelier-desk built from the same commit. That server is a Rust binary, and the plan supplies no existing same-commit artifact or policy for using one. A cold release run cannot meet both instructions as written. Either permit building only the Desk for release verification, while keeping the atelier distribution build excluded, or specify how the release obtains and verifies a same-commit tested artifact without a Rust build. Align the mission and release item on that choice.

## Finding 3: Make the mission's exclusions match the updater item
Recommendation: fix now

`.atelier/missions/atelier-m8ye/mission.md:96` excludes any change to what the extension shows or does, and line 100 excludes macOS altogether. `.atelier/items/atelier-wlyn/item.md:30-33` nevertheless requires Windows and macOS installation, startup and daily update checks, an installed-version and update display, and optional automatic application of updates. These are explicit behavior and UI changes, and the macOS requirement is explicit too. An implementer cannot tell which scope governs. Narrow the behavior exclusion to unrelated Desk/extension features and state whether macOS means preserving existing support without a runner or implementing and validating bundle updates there. Make the item's platform acceptance criteria agree with that decision.
