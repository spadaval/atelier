---
schema: atelier.review
schema_version: 3
round: 2
commit: a07e9d09344a52a7a67b2428eff587a1c5cb3f24
reviewers:
- thread:delegated-task:command%3Amcp%3A932557e7-853d-4352-845a-e9951a7775d5%3Adelegate-task%3A9f10b4fb-3db3-49f7-82a6-50d572089648
model: gpt-6-astra
verdict: approve
created_at: 2026-11-16T19:30:38.506951177Z
---

## Summary

Complexity: high. The mission coordinates a history import, a real Desk fixture job, a separate release workflow and an installed updater across four items.

Value: high. Testing both sides of the Desk contract in one commit addresses the stated coordination failures, and bundles meet the requested device installation model.

Risk of merging it: high. Executing the plan publishes formerly private history and adds downloaded code updates; a missed private blob cannot be recalled by reverting the import.

The value warrants this scope. The ordered items, explicit pre-push history scan, same-commit CI gate and checksum tests give implementers concrete boundaries. I found no smaller change that meets the requested outcomes and no remaining fix-now finding.

I read the mission, all four items and all seven carried findings at a07e9d09344a52a7a67b2428eff587a1c5cb3f24, compared the revision with its parent, and checked the existing CI workflow, job selector, release configuration, merge checks and ADR 0032 decisions. I also inspected the available source clone's installer and fixture test and listed the source tree at a242410. This is a plan review, not verification of implementation: I did not run a release, scan the full private history, prove the six-minute CI target or test a real Windows or macOS installation. The items require that evidence where applicable.

## Finding 1: The extension release must run the fixture test, but the mission says that release builds no Rust
Recommendation: fix now
Was: 1-ler1.1
Now: fixed
Fixed. atelier-451s now requires a successful CI run containing the extension job and explicitly builds no Rust or tests in the publishing workflow, matching the mission.

## Finding 2: The installer can't run on Linux, but three criteria need it to install into an empty home in CI
Recommendation: fix now
Was: 1-ler1.2
Now: fixed
Fixed. atelier-451s owns and documents an installer option or variable for a stand-in app on Linux. atelier-wlyn depends on it and uses that mechanism; the platform criteria now preserve paths without claiming real-machine validation.

## Finding 3: Importing the history publishes the private repository's screenshots of other projects
Recommendation: fix now
Was: 1-ler1.3
Now: fixed
Fixed. atelier-n4g7 now requires filtering artifacts from every imported commit in a scratch clone before any push, scanning the rewritten history, reporting the scan and stopping on other personal or secret material.

## Finding 4: The extension job's rule leaves out its own workflow and the Desk's dependencies
Recommendation: fix now
Was: 1-ler1.4
Now: fixed
Fixed. atelier-7tg6 now names the CI workflow, selector, workspace manifest, lockfile and Rust toolchain as triggers and requires tests of each. Its deliberate core-only exclusion is explicit.

## Finding 5: Allow the documentation changes in the import's tree comparison
Recommendation: fix now
Was: 1-s4iu.1
Now: fixed
Fixed. Both the mission and atelier-n4g7 compare the last imported commit before adaptation byte for byte with the source tree. Documentation and whitespace changes must be later commits.

## Finding 6: Decide whether extension releases may build the Desk for testing
Recommendation: fix now
Was: 1-s4iu.2
Now: fixed
Fixed. The mission and release item agree that publishing consumes a successful same-commit CI result including the extension job, without rebuilding the Desk.

## Finding 7: Make the mission's exclusions match the updater item
Recommendation: fix now
Was: 1-s4iu.3
Now: fixed
Fixed. The mission permits the installed-version and update controls, and excludes macOS validation rather than preservation of existing paths. atelier-wlyn matches those boundaries.
