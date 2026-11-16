---
schema: atelier.review
schema_version: 3
round: 1
commit: a12aef7b279095e204b7760d7c02f53656095a6c
reviewers:
- thread:delegated-task:command%3Amcp%3Aa84d37b3-df57-4187-b9cd-61253ac82eb1%3Adelegate-task%3A0dc6d8d4-8ec2-4764-bc87-0510409a01d9
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-16T23:42:28.973194125Z
---

## Summary

- Complexity: high. The change adds a mission model, CSS ordering and folding of another application's live list, and a capture-phase event guard, with lifecycle state spanning both applications.
- Value: high. Showing missions around T3's real rows is the requested sidebar outcome.
- Risk of merging: high. T3 still performs range selection using its original order, so bulk actions can affect threads outside the visible selection range.

The value warrants the change after the two fix-now findings below. I found no substantially smaller implementation that delivers the requested layout.

Reviewed a12aef7b279095e204b7760d7c02f53656095a6c, including the full implementation diff, the item checks and report, README and verification notes, and the sidebar, motion and selection source in the supplied November 10 T3 checkout. Ran `cargo build -j 4 -p atelier-desk` with a separate CARGO_TARGET_DIR and TMPDIR under HOME, built the Desk app, and ran `node t3-extension/run-tests.mjs` with ATELIER_DESK pointing at that binary: both tests passed, in 42 and 3 seconds. `git diff --check` passed.

The fixture's drawn-order assertions exercise real geometry and are not vacuous. They cover folding, folder close/reopen, a replacement list and named fallback. However, the fixture does not implement T3's range selection, and its fading checks only clone native thread rows; it never simulates T3 fading a removed injected mission header. Focused checks outside the repository used the actual T3 Sidebar.motion.ts implementation in the browser fixture, and the actual threadSelectionStore.ts logic with a synchronous store shim and Sidebar.tsx's Ctrl/Shift click branches. Those checks reproduced findings 1 and 2. Ordinary thread pointerdowns are outside the Settle selector, and the guard's listeners use the extension's abort signal; I found no separate event-swallowing or listener-removal defect. The old Missions block's rendering and hiding code has been removed.

No Windows connection or real T3 mutation was used. I could not verify the few-days-newer host, macOS, or a real successful Settle operation. The README's arrow-key and motion limitations are candid, but its clean-removal claims are too strong for finding 2. A separate startup probe reproduced the previously reported fetch failure and established its cause in finding 3.

## Finding 1: [P1] Keep Shift-click selection from selecting threads outside the drawn range
Recommendation: fix now

`t3-extension/custom.js:954` changes only CSS order, and the only native range-operation guard at lines 966-968 stops a pointerdown on Settle. Shift-click still reaches T3 unchanged. In the supplied host, `Sidebar.tsx:1569-1572` calls `rangeSelectTo(threadKey, orderedThreadKeysRef.current)`, and `threadSelectionStore.ts:47-77` selects the inclusive slice of that original order. It does not read CSS order or filter mission-folded rows.

Concrete failure using this PR's fixture: the original atelier-folder order starts `thread-child, thread-loose, thread-a`, while the rendered mission places `thread-child` immediately above `thread-a` and moves `thread-loose` below the mission. Ctrl-click `thread-child`, then Shift-click the adjacent `thread-a`. The focused check selected `["env:thread-child", "env:thread-loose", "env:thread-a"]`. The loose thread was outside the range the user selected on screen. Folding can also leave selected rows invisible. T3's context menu applies Settle, Snooze and Delete to this selection (`Sidebar.tsx:2281` onward), and dragging a selected row to the composer also uses it (`Sidebar.tsx:1739` onward).

Handle this range gesture while mission layout is active, under the same constraint of preserving T3's nodes, so it cannot silently select an unseen range. Cover it in the fixture. The documented arrow-key navigation limit does not describe or mitigate wrong bulk-action targets.

## Finding 2: [P2] Remove faded copies of injected rows when the layout goes away
Recommendation: fix now

`t3-extension/custom.js:863-866` removes only the original nodes in `listRows`; lines 950 and 915 likewise delete tracked originals and ignore all copies. T3's actual `Sidebar.motion.ts:40-78` clones removed children, preserves `data-t3-list-row` and the mission attributes, appends the clone, and removes it only when its animation finishes. Its position cache includes the extension's rows on an ordinary T3 update.

Concrete failure: let the host motion controller record a list containing mission headers; close that mission's folder, so the extension removes its header and more-missions row; then let another T3 list update run motion. T3 creates faded copies of both injected rows after they have left `listRows`. In the focused fixture using the actual host motion code, both inert rows remained after the views toggle was disabled. A separate run confirmed both remained even after `control('stop')`. Animations were paused to reproduce the covered-window case already described in the item's report. Even with normal animation timing there is a ghost until completion; in the covered-window case it can persist.

Include the injected rows' exit copies in teardown and account for copies produced on a subsequent host update. Preserve native T3 rows. Add a fixture check that fades an injected header, then disables the view by name and stops/reloads the extension without leaving its rows. The current helper and `drawn()`'s unconditional exclusion of inert nodes never exercise this cleanup case, so passing the current tests does not support the README's claim that no extension row remains.

## Finding 3: [P2] Parse a complete startup log line before returning the fixture Desk URL
Recommendation: defer

`t3-extension/fixture-desk.mjs:108-111` matches the startup URL against whatever stderr chunk has arrived and resolves immediately, even if the URL is incomplete. Stderr data events do not preserve `eprintln!` boundaries. This code predates the PR; it deserves a separate fixture fix rather than being blamed on the new layout.

I reproduced the reported `fetch failed` in a focused fixture run. Repeated startup checks then captured `startFixtureDesk().url` as `http://127.`, `http://127.0.`, `http://127.0.0.1`, and `http://127.0.0.1:` while `desk.log()` subsequently contained the full address and ephemeral port. Fetching the truncated value attempted addresses such as `0.0.0.127:80` (UND_ERR_CONNECT_TIMEOUT) and `127.0.0.1:80` (ECONNREFUSED). A retry of the returned value still fails because the promise has already resolved the wrong string.

This is a real stream-chunk parsing race, not evidence that the Desk needs a startup sleep or request retry. Buffer through a complete line before parsing the URL, and test split chunks. The standard suite passed on this review's first run, but rerunning until green conceals this existing defect.
