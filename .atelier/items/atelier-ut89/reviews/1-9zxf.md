---
schema: atelier.review
schema_version: 3
round: 1
commit: 4adeaaded8cdb57ff56040d5cee160c4466b0095
reviewers:
- thread:delegated-task:command%3Amcp%3Ad82b0dfa-fccf-4758-8e2f-e6405235c5b5%3Adelegate-task%3Adab2da34-23c6-46c7-8da1-42aaf915b9d1
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-17T23:23:44.150781758Z
---

## Summary

Complexity: high. The change adds a native React page, its history and thread model, multi-read polling, and a second rendering path in the shared layer.
Value: high. The running and finished mission views are a central outcome of atelier-osel.
Risk of merging: medium. The change reaches the extension's release path and the page people use to decide whether a mission needs attention; it is reversible.

The value warrants the implementation, and I found no substantially smaller change that would deliver the requested running and finished views. It needs the fixes below before it can ship.

I reviewed `git diff master...4adeaaded8cdb57ff56040d5cee160c4466b0095`, the new page and model, the layer integration, the Desk's history/thread contracts and implementations, the fixture checks, and the documentation. I inspected the committed T3 running-light and done-dark screenshots and the 728-pixel fixture screenshot. I did not start or drive a new Nightly instance, so the live visual evidence is the author's screenshots rather than an independent Nightly run.

In `t3-extension/`, `pnpm typecheck`, `pnpm build`, and `node run-tests.mjs` all passed. All five suites passed, including the ten mission checks and the changed integration checks. `git diff --check` passed. I also bundled the pure model and fixtures in memory with Vite and exercised a failed quiet-thread read and an item dropped after review; both expose the problems below without changing source files.

`scripts/t3-extension-release.test.sh` failed locally at the bundle build with the unresolved Desk import. The logs for PR #202's CI run 44198928010 show the same failure in `changes`; its extension, Desk and Rust jobs were skipped. The separate extension-release workflow also failed after producing no bundle. Thus the Report's local extension-test claims hold, but release packaging and PR CI do not pass.

## Finding 1: Preserve the extension's isolated release build when importing Desk code
Recommendation: fix now

`t3-extension/src/mission/model.ts:11` adds a runtime import of `../../../desk/src/lib/mission`. The release builder at `scripts/t3-extension-release.sh:138` archives only `HEAD:t3-extension` into its temporary build directory. That directory has no sibling `desk/`, so the new extension cannot be packaged for release, despite building successfully inside the full repository.

This is reproducible with `scripts/t3-extension-release.test.sh`: Vite exits with `[UNRESOLVED_IMPORT] Could not resolve '../../../desk/src/lib/mission' in src/mission/model.ts`, followed by a missing release ZIP. The PR's `changes` job fails for exactly this reason and prevents the downstream test jobs from running. This is a release-blocking regression caused by this change, not an unrelated CI failure.

Make the runtime inputs available to the isolated bundle build, or keep the runtime helper within its available inputs. If the recipe changes, update the release test's scratch source setup as well. The bundle and its verification must pass in addition to the in-repository build.

## Finding 2: Do not turn an unread quiet thread into an at-work thread and say nothing needs attention
Recommendation: fix now

`t3-extension/src/react/desk.ts:94` replaces a previously successful thread answer with an `Error` when a later poll fails. `MissionPage.tsx:182-188` omits that answer from `infos`, and `model.ts:65-76` treats the resulting missing info as a live worker with no changed time and therefore not quiet. The worker moves from *Needs you* to *At work*. At `MissionPage.tsx:218`, only `mission.t3.reachable` suppresses the reassuring sentence, so an individual full-thread read failure still permits `Nothing needs you.` A successful T3 list poll does not guarantee that each separate full-thread request succeeds.

I reproduced this with the existing `needs-you` fixture, clearing its item's needs-human label to isolate the quiet thread. With all answers, it reports `One thread has been quiet for 47m.` and lists `thread:quiet` under *Needs you*. Removing only that thread's full answer, as the error path does, produces no asks, puts `thread:quiet` under *At work*, and reports `Nothing needs you.` The small failed-read note is present in the component, but contradicts rather than fixes that classification and summary. The initial interval before thread answers arrive has the same unknown-data problem.

Keep the last successful thread data with an explicit stale/error state, and distinguish unread or pending information from evidence that a thread is active and needs no attention. Do not assert that nothing needs the human while the reads needed to decide it are incomplete or failed. Add a fixture check for a quiet-thread read that succeeds and then fails.

## Finding 3: Keep dropped items in the finished mission's time lanes
Recommendation: fix now

`t3-extension/src/mission/model.ts:253` builds lanes from `counted(page)`, which removes every dropped item at line 38. The finished page therefore has no lane for work that was built or reviewed and then dropped. This loses exactly the spent time the retrospective is meant to show and does not meet the Done when requirement of a lane per item. Filtering dropped items from the merged/total progress count does not justify removing their history from the lanes.

Using the done fixture, I changed `atelier-d002` to dropped and replaced its merge event with a drop at the same time. The mission still has two items and the dropped one has start, submission and review events, but `lanes(...).lanes` contains only `atelier-d001`. There is a second part to handling that lifecycle: `lane()` at lines 239-244 ignores `dropped`, so simply removing the filter would incorrectly extend the last span through the mission's close instead of ending it when the item was dropped.

Draw the dropped items' lanes as well and end their open spans at their drop event. They should remain outside the merged list. Cover an item that consumed building/review time before being dropped, rather than only missions whose items all merged.
