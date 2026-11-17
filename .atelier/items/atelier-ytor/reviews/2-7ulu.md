---
schema: atelier.review
schema_version: 3
round: 2
commit: c1652fb991fb4d2647e7370e33c1ee8a61017ad3
reviewers:
- thread:delegated-task:command%3Amcp%3Aa84d37b3-df57-4187-b9cd-61253ac82eb1%3Adelegate-task%3A89393630-68a7-4575-b030-44cd0b21128f
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-17T00:00:48.237675284Z
---


## Summary

- Complexity: high. CSS ordering and folding now share a live list with T3, its motion controller and its event handlers.
- Value: high. Missions around T3's real thread rows are the requested outcome.
- Risk of merging: high. One modifier combination still allows a bulk selection to contain a thread outside the range drawn.

The value warrants the implementation after the fix-now findings below. I found no substantially smaller implementation that delivers the requested layout.

Reviewed c1652fb991fb4d2647e7370e33c1ee8a61017ad3, both the full diff and the changes since round 1, the item and Responses, README, verification notes and fixture. Compared the Report's action inventory with Sidebar.tsx, Sidebar.logic.ts and Sidebar.motion.ts in the supplied T3 checkout (532a7aaf424d78876c93924d2ebbfa4be9acfba3, November 10), and followed their selection store, row controls and tree navigation.

Built atelier-desk with `cargo build -j 4 -p atelier-desk`, a separate CARGO_TARGET_DIR and TMPDIR under HOME, and built the Desk app. `node t3-extension/run-tests.mjs` passed all three tests (42 seconds for the browser fixture, 3 seconds for updates). `git diff --check` passed. In disposable copies outside the repository, removing the Shift-click stop failed the new Shift-click assertion; omitting active-view copy cleanup timed out at "A copy of a mission header removed"; omitting cleanup of copies arriving after the views turn off timed out at "A copy of a mission header removed with the views off". Restoring the old URL regex failed the prefix check with the incomplete address `http://1`. Thus all three regression checks detect their respective breakages. The previously deferred finding 1-iclq.3 is fixed: stream-local buffering plus a newline-terminated match prevents the premature resolution, and its new test fails without that requirement.

A focused browser-fixture run used T3's actual handleThreadClick and threadSelectionStore source, stripped of TypeScript, with a synchronous store shim and supplied platform values. It reproduced finding 1 below with both Win32 and MacIntel values. Another check retained the routed row when closing a folder, as T3 does, and reproduced finding 3. No production code was changed.

The Report's inventory omits the Un-settle and Wake drag-sweeps and forward navigation after Snooze. The former use the same original-order range helper, but act only within their own unchanged shelf (`Sidebar.tsx:1803-1810,1835-1843`); the latter only navigates (`2233-2256`). Native thread reordering returns immediately in boring mode (`2086`), while dragging to the composer uses the explicit selected set. Tree arrows/Home/End follow DOM order; Enter/Space open the focused row. I found no additional order-dependent mutating action that selects an unintended target independently of finding 1. The navigation and retained-selection limitations are documented.

For event scope and teardown, the standard fixture verifies plain thread navigation, Settle clicks, Ctrl-clicks, and the two guards becoming inactive without missions. Additional checks verified that ordinary row pointerdowns and Ctrl-clicks arrive, and that stopping with an active layout and an existing faded mission-header copy leaves zero extension rows, orders or layout stylesheet; Settle pointerdown and Shift-click then reach the native row again. Both listeners use the extension's abort signal, which dispose aborts before cleanup. Nested action/link clicks are excluded from the new click guard; the rename input's own click handler only stops propagation, so I found no separate functional event-swallowing defect to report.

No Windows test PC was contacted. The platform checks used browser events and the supplied older host source; they do not establish physical macOS Ctrl-click behavior or compatibility with the few-days-newer targeted build. No real successful Settle, newer-host motion or macOS UI was exercised. Copies T3 creates only after the extension has stopped remain the documented host-motion limitation; existing copies at stop were verified removed.

## Finding 1: [P1] Keep Shift-click selection from selecting threads outside the drawn range
Recommendation: fix now
Was: 1-iclq.1
Response: fixed in d839006d3 (Shift-click on a thread row stopped while a mission is placed; other order-based shortcuts listed under Known limits)
Now: kept

The ordinary Shift-click case is fixed, and its regression assertion fails with the stop removed. However, `t3-extension/custom.js:976` exempts every click carrying either Ctrl or Meta. T3's `Sidebar.tsx:1561-1572` exempts only the platform's selection modifier: Ctrl on Windows, Meta on macOS. A Shift-click with the other modifier still runs `rangeSelectTo` in T3 but bypasses the extension's guard.

Concrete failure on the Windows handler: Ctrl-click `thread-child`, then Shift+Meta-click the visually adjacent `thread-a`. The focused fixture selected `["env:thread-child", "env:thread-loose", "env:thread-a"]`; `thread-loose` is drawn below the mission, outside the selected visible range. The ordinary Shift-click in the same check was blocked. The MacIntel handler likewise ranges on a delivered Ctrl+Shift-click event. Bulk Settle, Snooze and Delete, and dragging the selection to the composer, can consequently include the unintended loose thread.

Match T3's platform-specific modifier decision when allowing selection toggles, and cover a Shift-click carrying the non-selection modifier. The current fixture assumes Ctrl always means toggle (`test-extension.mjs:122`), so its success cannot rule this out. The response resolves the common case, not the whole finding.

## Finding 2: [P2] Remove faded copies of injected rows when the layout goes away
Recommendation: fix now
Was: 1-iclq.2
Response: fixed in d839006d3 (copies of the extension's rows removed on each sync, when the view is off and at teardown)
Now: fixed

`custom.js:865-869,915,954` now removes every marked copy outside the live-row set, including copies arriving after the view turns off. The actual host motion source preserves this mark on its clones. Active-view, disabled-view and named-fallback checks pass, the two independently removed cleanup paths each fail their corresponding check, and a focused stop with an active layout and an existing copy removed it along with the originals. No native row was removed by this cleanup.

## Finding 3: [P2] Honor a closed project folder even when T3 retains its open thread
Recommendation: fix now

`t3-extension/custom.js:924-950` finds the folder button but never reads its `aria-expanded` value. It assumes a closed folder contains no thread rows. T3 deliberately retains the current route's thread when a folder closes (`Sidebar.tsx:534-542`, via `routeThreadOnly`), so that assumption is false in the supplied host.

Concrete failure: open mission thread `thread-a`, then close its project folder. In the fixture with T3's retained-route-row behavior, the folder button read `aria-expanded="false"`, but the extension still drew `Fixture mission 1/3`, the indented `thread-a`, and `1 more mission` beneath it. Both injected rows remained. A folded mission would additionally hide the route row T3 deliberately retained. This violates the Done when requirement that a closed folder show no mission rows.

Honor the folder button's closed state while preserving T3's retained thread exactly as T3 renders it. Update the fixture's close behavior: `test-extension.mjs:107` currently removes every thread, so its "A closed folder has no mission rows" assertion cannot catch the real host case.
