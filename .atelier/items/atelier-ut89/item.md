---
schema: atelier.item
schema_version: 3
id: atelier-ut89
title: 'Draw the mission page natively: what needs you while it runs, and the result and where the time went once it is done'
status: changes_requested
mission: atelier-osel
labels: []
priority: medium
blocked_by:
- atelier-6dm7
implementers:
- thread:delegated-task:command%3Amcp%3A3eadca8a-0dea-4bef-b682-3d92ff9eca06%3Adelegate-task%3Aa07e5f96-8b1b-4155-a803-a72e033fa7ee
ready:
  by: 'sumeet, in conversation on 2026-11-17: looks good, start up a lead'
  at: 2026-11-17T21:31:50.834606783Z
trivial: null
caused_by: null
created_at: 2026-11-17T19:24:26.186003324Z
---

## Goal

Replace mission control with a page that answers what a person opening a
mission wants. While it runs: does it need me, and is it getting where I
wanted. Once it is done: what did I get, and what is worth a look.

## Done when

- [x] The title of a mission that is agreed or done, in T3's sidebar and on a page, opens the mission page over T3's main area, drawn by the extension in React with no `iframe`, from the Desk's existing reads.
- [x] Running, the page follows concept A of `mission-concepts.html`: the title, a state badge, items merged of total and a button and key for the lead's thread; one sentence that says how many items are being built, merging, waiting on the human and quiet; *Needs you*, a row for each item labelled needs-human, each thread T3 Code says is waiting on a person, and each quiet thread, with a button that opens the item's page or the thread; *At work*, a line for each live thread not already under *Needs you*, with its item, latest line and running time; and *Not started*, *Merged* and *Outcome and success criteria*, each one folded line that opens in place.
- [x] A thread is quiet when T3 Code says it is live and its `updated_at` has not changed in 30 minutes, the rule This thread uses today. An item that has been building or in review for longer with a thread that is still changing is not quiet. A waiting thread is listed at once, whatever its age, as the inbox and the sidebar's count do today, and a thread both waiting and quiet is one row.
- [x] A proposed mission's title does not open this page. It opens the mission's record page, framed, as atelier-9l8h and atelier-wvst say.
- [x] With nothing needing the human the page shows no *Needs you* section and the sentence says so.
- [x] Done, the page shows the state and when it closed; the mission's Result as written; a lane per item from the plan's merge to the close, with its building, review and merging spans and its review rounds and duration at the end, with a legend; and the merged items, most review rounds first, each with its review rounds. An item opens its pull request on the forge where the Desk gives one. The Desk gives none for an item whose branch is deleted or whose pull request is older than the forge's recent list, and that item is still listed, says its pull request is not known, and opens its Desk page framed.
- [x] The page has no list of the mission's steps. The lanes and the stage ages say they come from commit subjects, as `docs/architecture/desk.md` says of the progress log today.
- [x] `L` opens the lead's thread, else the planner's, `Enter` acts on the focused row, and `Esc` closes the page. Each key is shown on the button it presses, and no footer lists keys.
- [x] The page holds at 728 pixels wide and stacks below 560. At 1,100 pixels and wider *Needs you* and *At work* sit side by side, the lanes take the full width, and no line of text is longer than 75 characters.
- [x] `docs/product/desk.md` and `docs/architecture/desk.md` describe the new page and say the Desk's mission control page is no longer opened by the extension. The Desk's page itself stays until atelier-yy4j.
- [x] The check in `test-extension.mjs` that a mission's title opens a frame on `/embed/mission/...` becomes a check that it opens the native page with no `iframe`; the checks that the layer frames the inbox stay as they are.
- [x] Fixture checks in `t3-extension/` cover the running page with a needs-human item, a quiet thread and two live threads; the running page with no needs-human item and one thread that began waiting a minute ago, which is listed; the running page with nothing needing the human; and the done page, with one merged item whose `pull_request` is null.
- [x] The page is seen in T3 Code Nightly on the test instance on this host (atelier-diru), running and done, in light and dark, with screenshots in the Report.

## Scope

In: `t3-extension/`, the files of `desk/src` the extension imports, `docs/product/desk.md`, `docs/architecture/desk.md`.

Out: a proposed plan, which the page need not serve; the mockup's "Differed from the plan" and "Not seen working" sections; concept B's timeline for a running mission; any write.

## Context

The mockups are `t3-extension/spike/mockups/mission-concepts.html` on `spike/extension-react`; open the file in a browser. The data mission control reads today is enough: `GET /api/repos/{repo}/missions/{id}` (sections, items, threads), each item's page, `GET /api/history` and `GET /api/thread?full=1`. `desk/src/lib/mission.ts` holds the stage and age logic with tests. The extension does not fetch the Desk itself: it asks the loader (`api.request('desk-get', { desk, path })`), for each Desk in its configuration, while the Desk's hooks in `desk/src/lib/api.ts` fetch paths on their own origin. So the components and the pure functions move, and the reads go through atelier-6dm7's hook, which also opened the loader's gate for them. This item mounts into the layer the old script already draws over T3's main area; atelier-40dc rewrites that layer afterwards. This item may start before atelier-diru merges, and its last line waits for it. atelier-diru's command starts T3 Code here with the fixture Desk, whose records hold a running mission and a done one.

## Report

A mission's title in T3's sidebar now opens a mission page the extension
draws itself, in React with T3's `Badge`, `Button`, `Kbd` and
`CollapsibleSectionHeader`, with no `iframe`. It was seen working in T3 Code
Nightly on this host, and `node run-tests.mjs` passes all five tests,
including the new `test-mission.mjs` (10 checks) and the changed
`test-extension.mjs`.

**What changed.**

- `t3-extension/src/mission/`: `model.ts` works out everything the page
  shows from the Desk's reads (the sentence, *Needs you*, *At work*, the
  folds, the lanes, the merged items), with no React; `MissionPage.tsx`
  draws it; `markdown.tsx` draws a record's sections; `mount.tsx` mounts it
  through `mountReact`. `src/react/desk.ts` gains `useDeskReads`, which reads
  several paths as `useDeskRead` reads one.
- `src/custom.js`: the layer holds either the mission page or the Desk frame,
  never both. A proposed plan's title opens `/embed/r/<repo>/mission/<id>`
  framed. A Desk page opened from the mission page, such as an item, gets a
  **Back** button to it. `__t3Custom.openMission(repo, id, desk)` opens the
  page for any mission, which is how a done mission (never in the sidebar)
  was opened in T3 and how a later page can open it.
- Docs: `docs/product/desk.md`, `docs/architecture/desk.md` ("The mission
  page") and the extension's `README.md` describe the page and say the
  extension no longer opens mission control, which stays until atelier-yy4j.

**Reads.** The mission every 15 seconds, its history every 30, each thread T3
lists with `full=1` every 15 while it runs, and each merged item's page every
60 once it is done, for its pull request. All are forms the gate already
admitted; the gate is unchanged.

**Decisions within the mission's Authority.**

- The extension reads the Desk's generated types and `span` from
  `desk/src/lib/mission.ts` by relative path, with no pnpm workspace. It does
  not import `desk/src/lib/api.ts`: that file imports React, which would
  resolve to `desk/node_modules` and bundle a second React. The few paths it
  needs are in `model.ts`.
- The sentence counts items in review as well as building and merging
  ("Two items are being built, one is in review and one is merging. One item
  waits on you and one thread has been quiet for 47m.").
- No new dependency. The Desk draws Markdown with `react-markdown`, which is
  not on the mission's list, so `markdown.tsx` reads paragraphs, lists,
  headings, inline code, emphasis and links (as text).
- Widths are of the page, by container queries, since the layer is narrower
  than the window.

**Where it differs from the mockup.** A needs-human row does not say how long
ago the item was labelled, as the mockup's does: no read the page makes gives
that time. In its place it says how many items wait behind it. The done page
has no "Success criteria 7 of 7" line, which the Done when list for the done
page leaves out. Concept A's "Worth a look" is the *Merged* list. When T3
Code's threads cannot be read (`t3.reachable` false), the page says so in red
and does not say "Nothing needs you".

**Quiet and waiting.** A thread counts as live when T3 Code lists it, says it
is live and its status is not an ended one. It is waiting when its status is
`waiting`, whatever its age, and quiet when its `updated_at` is 30 minutes old.
It is one row when it is both. An item's stage age plays no part.

**Checks.**

- `pnpm typecheck` and `pnpm build` pass.
- `node run-tests.mjs` passed `test-components`, `test-extension`,
  `test-fixture-desk`, `test-mission` and `test-update`.
- `test-mission.mjs` draws four fixture missions (`check/mission-fixtures.ts`)
  with a stand-in injector and a fixed clock. The first is running with a
  needs-human item, a reviewer quiet for 47 minutes and two live threads. The
  second is running with a thread that began waiting a minute ago and no
  needs-human item. The third is running with nothing needing the human. The
  fourth is done, with one merged item whose `pull_request` is null and one
  with #172. It checks:
  - the sentence, rows and folds;
  - what each button, `L`, `Enter` and `Esc` opens;
  - the reads the page makes;
  - the lanes' spans, rounds and start;
  - the merged items' order and targets;
  - the layout at 500, 728, 1,100 and 1,400 pixels: no sideways scroll,
    stacked only below 560, side by side from 1,100, and no line of text
    wider than 75 `ch`.
- `test-extension.mjs` checks that a mission's title shows the native page
  with no `iframe` in the layer. It reads the mission through the injector,
  never asks for `/embed/mission`, keeps the row selected and focuses the
  page. An item opens framed and Back returns to the page. A proposed plan's
  title opens its record page framed. The inbox checks are unchanged.
- One full `run-tests.mjs` run failed `test-extension.mjs` after 124 seconds
  without printing a FAIL line. I did not keep that output. The next full run
  passed, and so did four more runs of `test-extension.mjs` alone. I do not
  know the cause.

**Seen in T3 Code.** atelier-diru is in review, not merged. I ran its
`t3-instance.mjs` and `fixture-desk.mjs` from `origin/atelier-diru`, over this
branch, in a scratch worktree that is now removed. It used its own
`T3_TEST_ROOT` and ports 9334 and 47332, because another thread's instance was
on 9333. In T3 Code Nightly `v0.1.11-nightly.20261117.4851`:

- The sidebar's "Fixture mission" opened the running page with no `iframe`.
- *Open item* opened the item framed, Back returned to the page with focus on
  it, and `Esc` closed it.
- `__t3Custom.openMission("atelier", "atelier-fm02")` opened the done page.

The fixture Desk there has no `[t3]` table, so the running page says T3's
threads cannot be read. Its done mission has no items or Result, so that page
is sparse. The fixture screenshots show those cases full. Screenshots are in
`.atelier/items/atelier-ut89/artifacts/`:

- In T3 Code: `t3-running-light.png`, `t3-running-dark.png`,
  `t3-done-light.png`, `t3-done-dark.png` and `t3-item-framed.png`.
- From the fixtures: `fixture-running-light.png`, `fixture-running-dark.png`,
  `fixture-done-light.png`, `fixture-done-dark.png`, and the running page at
  `fixture-running-728.png` and `fixture-running-500.png`.

**Kept from the old code.** The layer still closes on Escape, on a thread row
click and on a route change. The covered-window rule stands: `useDeskRead`
and `useDeskReads` read again when the page becomes visible. The layer moves
the iframe only when it changes between native and framed, since moving it
reloads it. Nothing the old comments recorded was dropped.

### Responses
