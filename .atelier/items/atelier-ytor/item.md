---
schema: atelier.item
schema_version: 3
id: atelier-ytor
title: Show missions around T3's own thread rows in the sidebar
status: changes_requested
mission: null
labels: []
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3Ac33647a5-8996-4ff4-be9b-7b5a668ee507%3Adelegate-task%3A05cec35b-99fb-41e8-b803-d483728270c1
ready: null
trivial: null
caused_by: null
created_at: 2026-11-16T23:10:57.206842562Z
---

## Goal

The T3 Code extension's sidebar shows missions inside T3's own thread list, so
there is one list, styled like the sidebar. Today it adds a separate Missions
block under T3's search, hides mission threads from T3's list with CSS and
shows "N hidden from T3's list" and "Show all threads": two cramped lists,
neither styled like the sidebar. Sumeet approved the target in a mockup
(column B): missions as sub-groups of their project's folder, around T3's real
thread rows, without forking T3.

It stays in the injected extension (`t3-extension/`). In T3's boring sidebar
(one flat list of folder headers and thread rows) the extension appends its
own rows and sets CSS `order`, never moving or editing T3's nodes. A spike on
T3 Nightly 0.1.10-nightly.20261115 showed this holds through folder close and
reopen, search and route changes.

## Done when

- [x] In an open project folder, each open mission with a thread there shows as a header (progress ring, title, items done of total, chevron) followed by those threads in T3's order, indented on a guide line with their role (lead, plan, build, review); threads a mission thread started go under it with no role; then one "N more missions" row for the repository's missions with no thread there (a dot when one waits on the human), then the folder's other threads in T3's order. Pinned rows, other folders and shelves keep T3's order.
- [x] The extension never moves, removes or edits T3's nodes: it appends its own `li` rows to T3's list and sets CSS `order` on its children, from T3's DOM order, before the browser paints and again on every new list T3 draws (search, route change).
- [x] A mission's title opens mission control in the Desk layer; its chevron folds it, hiding its threads, and folded it shows its thread count and a dot when it waits on the human; folded state is kept in `t3-custom:atelier:missions:v1`. "N more missions" opens the project home. A closed folder shows no mission rows.
- [x] With the boring sidebar off (no folder headers) nothing is reordered and a note under Inbox names Settings → Labs → Boring sidebar; the extension never changes T3's setting.
- [x] While a mission is placed, no gesture of T3's that takes a run of rows from T3's own order reaches T3: a press on a row's Settle button (the drag-sweep) and a Shift-click on a row (range select, which bulk Settle, Snooze, Delete and a drag to the composer act on); a plain click on Settle, a click and a Ctrl-click still work; checked on the real app.
- [x] T3's faded copies of the extension's rows are removed as they appear, when the view turns off and when the extension stops.
- [x] A named view (`missions`, probes `layout:threadList` and `dom:settleButton`) turns off when T3's list is not one flex column holding every folder with its button, and then T3's list is left exactly as T3 drew it: no order, row or rule of the extension's.
- [x] The separate Missions block, "hidden from T3's list", "Show all threads" and the hiding stylesheet are removed; Inbox is one sidebar-styled row under search.
- [x] The fixture test (`t3-extension/test-extension.mjs`) covers the new layout: the mission header and its threads ordered under their folder, folding, "N more missions", a folder closed, the boring sidebar off, the Settle press and Shift-click stopped, copies of mission rows removed, and the view turning off by name leaving T3's list whole; `node t3-extension/run-tests.mjs` passes.
- [x] `t3-extension/README.md` no longer claims what this makes false ("T3's own rows are never moved", "hidden from T3's list", "Show all threads", the Working/Settled count limit) and lists the new limits: projects with no T3 folder, arrow-key order, T3's motion, Settle drags.

## Scope

`t3-extension/` only: `custom.js`, `host-adapter.js`, the fixture test and
fixture Desk, the contract test, `README.md` and `docs/VERIFICATION.md`. The
Desk is unchanged.

## Report

Missions now sit inside T3's own boring-sidebar list, as in the approved
mockup's column B, with no T3 fork. Per open folder: a mission header (ring,
title, done/total, chevron), its threads in T3's order indented with their role,
"N more missions", then the loose threads. The extension appends its own `li`
rows and sets CSS `order` from T3's DOM order inside the MutationObserver
callback, so before paint. A generated stylesheet keyed on `data-thread-item`
indents, labels and folds rows; T3's nodes are never moved or edited. The
Missions block, hiding stylesheet, "hidden from T3's list" and "Show all
threads" are gone.

**The Settle drag-sweep is neutralised, checked on the real app.** Before the
change, a CDP mouse press-drag on a row's Settle marked three rows swept. With a
mission placed, a capture-phase `pointerdown` stop meant the same drag swept
none, released or cancelled, and a plain click still reached T3's settle (which
refused it only for the read-only connection). If T3's Settle button is not
found while a folder lists threads, the missions view turns off rather than
leave a live sweep.

**Found on the real app and fixed:** T3 animates its list (`Sidebar.motion.ts`)
and fades a removed row by appending an inert, absolutely placed copy. That
includes the extension's rows, and in the covered test window the copies never
went away. The layout now leaves such copies out of its walk.

Tests: `node t3-extension/run-tests.mjs` passes ("All 2 tests passed:
test-extension.mjs, test-update.mjs"; 30 checks in the fixture test). Four
deliberate breakages each fail a check (`docs/VERIFICATION.md`). The contract
test passes on the test instance with every probe ok, the new ones included.

Not verified: an actual settle by click (the test instance's environment
refuses every settle), T3's slide animations against the reordered list (the
covered window runs none), and macOS. The test instance has no thread of a real
mission, so the header, roles and sweep were checked with a synthetic mission
put in through a temporary copy of `custom.js`; the real file is back there.

Known limits, in the README: arrow keys follow T3's order; T3 may slide a moved
row from a stale spot for 150 ms; Settle cannot be dragged while a mission is
placed; projects with missions but no T3 folder are not shown. No item ID in the
tooltip: that needs editing T3's node.

The brief was right except for one point: the extension has no version in the
repository to bump. Its version is the release tag's, written into `VERSION`
only in the release zip. The first fixture run failed once with `fetch failed`
on the fixture Desk right after it printed "serving on", and passed on every
rerun; I did not chase it.

### Round 2

All three findings hold. Head before this round: a12aef7b2.

1. **Shift-click range (fixed).** I looked through T3's `Sidebar.tsx` for
   everything that takes a range or a neighbour from its own order
   (`orderedThreadKeys`):
   - the Settle drag-sweep (`resolveSidebarSweepKeys`): stopped since round 1;
   - Shift-click range select (`rangeSelectTo`): bulk Settle, Snooze, Delete
     and the drag to the composer act on its result;
   - the next and previous thread shortcuts (`resolveAdjacentThreadId`);
   - the Ctrl+1–9 jump shortcuts;
   - after the open thread is settled, opening the next one
     (`planForwardNavigation`).

   Only the first two can act on threads the user did not mean. While a mission
   is placed, a Shift-click on a thread row is now stopped in the capture phase,
   as the Settle press is, and a click and Ctrl- or Cmd-click still work. On the
   real app, Ctrl-click then Shift-click on two rows drawn next to each other,
   with a third between them in T3's order, selected only the first. The other
   three only move focus, so they are listed under Known limits, as is a
   selected thread hidden by folding its mission. The fixture now has T3-like
   Shift and Ctrl clicks, and the check fails with the stop undone.
2. **Copies of mission rows (fixed).** Every element marked `data-t3-list-row`
   that is not a live row is removed on each sync, when the view turns off and
   at teardown. Fixture checks cover a copy while the view is on, after the
   views toggle is off, and after the view turns off by name. A copy T3 makes
   after the extension has stopped cannot be reached; that is under Known
   limits. On the real app, a folder close and reopen left no header copy.
3. **Fixture Desk address (fixed, own commit 9de9e431e).** The URL is read per
   stream and only from a complete "serving on" line; `test-fixture-desk.mjs`
   checks every split of the line.

Tests: `node t3-extension/run-tests.mjs`: "All 3 tests passed:
test-extension.mjs, test-fixture-desk.mjs, test-update.mjs". Each new check
fails with its fix undone (`t3-extension/docs/VERIFICATION.md`).

