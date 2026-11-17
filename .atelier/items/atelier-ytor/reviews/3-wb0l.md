---
schema: atelier.review
schema_version: 3
round: 3
commit: 77090dd64ad1035c8036c79b2a5b09d6ad79980e
reviewers:
- thread:delegated-task:command%3Amcp%3Aa84d37b3-df57-4187-b9cd-61253ac82eb1%3Adelegate-task%3Ac5cfb6ee-cc03-4f8f-89cc-367268cf4395
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-17T00:18:37.359467892Z
---

## Summary

- Complexity: high. The extension shares a live list with T3's renderer, motion controller and event handlers, using private DOM hooks and CSS ordering.
- Value: high. Missions around T3's real thread rows are the requested outcome, and this preserves those rows.
- Risk of merging: medium. This reaches the sidebar's daily interactions, but the two original-order range gestures are now blocked, and the remaining observed limitations are visible and reversible by removing the extension.

The value warrants the change. I found no substantially smaller implementation that delivers the requested layout, and no open fix-now finding.

Reviewed head 77090dd64ad1035c8036c79b2a5b09d6ad79980e, the whole diff against origin/master and the changes since round 2, both previous reviews, the item and Responses, README and verification notes. Compared the host claims with Sidebar.tsx, Sidebar.logic.ts, Sidebar.motion.ts, the folder and thread row components, and threadSelectionStore.ts in the supplied T3 checkout, head 532a7aaf424d78876c93924d2ebbfa4be9acfba3. T3's Shift range branch and retained-route-row behavior corroborate both earlier findings. Native reorder returns early in boring mode; Un-settle and Wake sweeps stay within their unchanged shelves. I found no additional original-order operation that silently adds an unintended mutation target. The navigation and retained-selection limitations remain documented.

Built atelier-desk with `cargo build -j 4 -p atelier-desk`, my own CARGO_TARGET_DIR and a TMPDIR under HOME, and built the Desk app. `node t3-extension/run-tests.mjs` passed all three tests: test-extension.mjs (42 seconds), test-fixture-desk.mjs and test-update.mjs. `git diff --check` passed. No production or test source in this checkout was changed.

In disposable copies outside the repository, restoring the Ctrl/Meta exemption failed the modifier assertion, with one range reaching the Windows-shaped handler. Removing the closed-button branch failed the closed-folder check, which drew the mission header, indented thread-a and more-missions row around T3's retained route row. Both checks therefore detect the round 2 bugs independently.

The observer addition at custom.js:1151 is acceptable for this merge. Its committed polling assertion does not isolate the observer, as the Report acknowledges. A focused browser check instead waits for an animation frame and a microtask to drain existing work, changes only the button's aria-expanded, then reads injected-row count after the mutation microtask, before any scheduled task can run. It finds two rows with this head and zero with only attribute observation removed. This supplies direct evidence for the added hook; the durable regression assertion should still be improved as deferred finding 3.

An additional active-layout stop check passed: every native thread row kept its identity and stayed connected, each inline order was cleared, all injected rows and layout rules disappeared, and Settle pointerdown and Shift+Meta-click reached the fixture again after stop. The normal suite also covers disabled views, named fallback, faded copies, replacement lists, plain clicks and Ctrl-clicks. The range listeners use the abort signal, which the loader aborts before cleanup.

No Windows test PC was contacted. I could not verify the newer targeted T3 build, physical macOS modifier behavior, a successful real Settle, or motion in the real app. Copies T3 creates only after extension teardown remain the documented host-animation limitation.

## Finding 1: [P1] Keep Shift-click selection from selecting threads outside the drawn range
Recommendation: fix now
Was: 2-7ulu.1
Response: fixed in c43d460cb (every click with Shift held on a thread row stopped, whatever other key is held)
Now: fixed

custom.js:979-981 now stops every click with Shift on a thread row while a mission is placed, with no Ctrl/Meta exemption. The Windows and macOS host branches therefore cannot receive a Shift range through this handler. Ctrl- or Cmd-click without Shift stays outside the guard. test-extension.mjs:574-578 covers Shift alone, with Ctrl, Meta, Alt, and all three; restoring the old exemption fails that assertion. In particular, Shift+Meta on Windows can no longer select the loose thread between thread-child and thread-a in T3's order while those mission rows appear adjacent on screen.

## Finding 2: [P2] Honor a closed project folder even when T3 retains its open thread
Recommendation: fix now
Was: 2-7ulu.3
Response: fixed in c43d460cb (a closed folder decided by its button's aria-expanded; the open thread's kept row stays plain)
Now: fixed

custom.js:931 now bypasses mission grouping for a button that is not expanded, retaining the native thread rows in sequence without indent, role or folding rules. The fixture now preserves thread-a when its route is open and its folder closes, as T3's routeThreadOnly does. test-extension.mjs:586-589 passes with the fix and fails without it, showing the mission header, indented thread-a and "1 more mission" in the deliberately broken copy. The current head leaves only the plain route row under that closed folder.

## Finding 3: [P3] Make the attribute-only reopen check independent of unrelated layout passes
Recommendation: defer

At t3-extension/test-extension.mjs:592-593, changing only aria-expanded and polling for 1500 ms allows unrelated scheduled layout passes to satisfy the assertion. Removing attribute observation from custom.js:1151 can therefore leave this regression check green, although an otherwise quiet window would wait for another layout pass before showing the reopened folder's mission rows. The Report and verification notes disclose this gap.

Keep a regression check that observes the mutation result before a scheduled task can mask the missing observer. The focused microtask check described above passes on this head and fails with attribute observation removed (injected rows remain zero instead of two). This is deferred because the implementation has now been independently exercised, the regression would visibly delay layout rather than add an unintended bulk-action target, and the submitted test's limitation is documented. It does not block approval.
