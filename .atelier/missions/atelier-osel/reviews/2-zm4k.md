---
schema: atelier.review
schema_version: 3
round: 2
commit: bc262e717896c137668715dcb120cb1a8faacd34
reviewers:
- thread:delegated-task:command%3Amcp%3A909059fb-c6b7-42f1-873e-abaee1bead8d%3Adelegate-task%3A9ba1f12b-9073-4b5b-b441-3a10f3df4557
model: gpt-6-astra
verdict: changes
created_at: 2026-11-17T19:38:51.875990780Z
---

## Summary

Complexity: high. This replaces three views, changes the build and release contract, introduces copied T3 components and shadow-root styling, and rewrites the extension across eight dependent items.

Value: high. The native pages and mission briefing directly serve the human's stated direction, and the typed build supports their maintenance.

Risk of merging it: medium. The work can break the installed extension or its navigation, but the changes are reversible and the plan retains the old Desk pages until the replacement release is installed.

The value warrants the work. The sequence separates the build, rendering foundation, first native page, typed modules, remaining pages and removal; a smaller styling change would not deliver the requested native views and typed codebase.

I read the mission, all eight items, the round-one findings, and the changes at bc262e717896c137668715dcb120cb1a8faacd34. I checked the plan against the loader's read gate and injection contract, the test runner, the current Desk pages and host contract, the API types and handlers, the history and PR readers, and the HTML source of the mockups on spike/extension-react. All eight carried findings are addressed. One new fix-now finding remains about the data promised for the retro.

`atelier check` passes record validation. It also reports pre-existing empty Why sections on atelier-4pzj, atelier-icv4 and atelier-uf97. This is a plan review: I did not build the proposed implementation, run its future fixture checks, render the mockups, or verify T3 Code on Legend. The plan correctly leaves that verification to the implementation items.

## Finding 1: Authority's mockup rule covers pages that have no mockup
Recommendation: fix now
Was: 1-0mwt.1
Now: fixed

The mission's Authority now limits the mockup restriction to the mission page and gives This thread and project home their existing Desk pages as the content baseline.

## Finding 2: host:thread still steers the framed Desk to /t after This thread is native
Recommendation: fix now
Was: 1-0mwt.2
Now: fixed

atelier-9l8h explicitly stops sending host:thread to frames and requires a selected framed page to stay put on a T3 thread switch. atelier-yy4j explicitly removes useFollowThread together with that message and /t.

## Finding 3: yy4j can delete the Desk HTTP reads the native pages still call
Recommendation: fix now
Was: 1-0mwt.3
Now: fixed

atelier-yy4j now names the SPA pages and host messages to remove and explicitly retains the JSON reads used through the loader. Its scope no longer directs deletion of the server handlers.

## Finding 4: A quiet live thread would show in both Needs you and At work
Recommendation: fix now
Was: 1-0mwt.4
Now: fixed

atelier-ut89 explicitly excludes every thread already in Needs you from At work and deduplicates a thread that is both waiting and quiet.

## Finding 5: 3dip cannot both bundle host-adapter and keep every test assertion
Recommendation: fix now
Was: 1-0mwt.5
Now: fixed

atelier-3dip chooses one bundled script containing the adapter, removes the loader's separate concatenation, assigns installation/updater changes, and allows installed-file assertions to change while preserving behavioural assertions.

## Finding 6: Plan the loader read permissions the native pages require
Recommendation: fix now
Was: 1-dva4.1
Now: fixed

atelier-6dm7 now owns the seven additional GET forms before the first native page. It requires testing them through the real loader, preserving rejection checks and documenting the loader restart. The mission explicitly authorizes this gate change.

## Finding 7: Show threads awaiting human input immediately in Needs you
Recommendation: fix now
Was: 1-dva4.2
Now: fixed

atelier-ut89 now includes waiting threads immediately, without the quiet threshold, and adds a recently waiting thread fixture with no needs-human item. The mission's success criterion also includes that case.

## Finding 8: Give proposed missions a supported navigation destination
Recommendation: fix now
Was: 1-dva4.3
Now: fixed

atelier-9l8h and atelier-wvst route proposed titles to the retained framed mission record page, with fixtures for a planning thread and Later. atelier-ut89 explicitly keeps those titles out of the native running/done view.

## Finding 9: Specify how the retro handles merged PRs the Desk cannot return
Recommendation: fix now

atelier-ut89:32 requires the finished page to show the merged pull requests, ordered by review rounds, with forge links. Its Context at line 49 says the existing reads are enough, and its scope at line 43 does not include the Desk server. The mission's retro success criterion makes the same promise.

Those reads do not reliably supply historical PR links. `crates/atelier-desk/src/server.rs:596` sets ItemPage.pull_request only through row.branch. `read.rs:712` obtains that branch from an extant located ref, so a done item whose branch has been deleted has no PR lookup. Even with the branch retained, `fetch.rs:175` looks only in the cached open/recent maps; `fetch.rs:247-257` limits the recent list to 200 PRs of any state. A merged item older than that window therefore has a null pull_request too. Neither the history response nor the review records carry a replacement PR URL. This can omit the very item with the most review rounds from an older mission's Worth a look list.

Settle this before implementation: either authorize and assign retrieval of historical merged PRs, or explicitly permit the retro to list the item with its review count and a clear PR-unavailable state when no URL is supplied, linking only known merged PRs. Align the success criterion and mockup authority with the chosen behaviour. Require a done-mission fixture with a null pull_request for a merged item; an all-populated fixture would miss this normal response shape.
