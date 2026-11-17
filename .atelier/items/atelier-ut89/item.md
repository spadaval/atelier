---
schema: atelier.item
schema_version: 3
id: atelier-ut89
title: 'Draw the mission page natively: what needs you while it runs, and the result and where the time went once it is done'
status: building
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

- [ ] The title of a mission that is agreed or done, in T3's sidebar and on a page, opens the mission page over T3's main area, drawn by the extension in React with no `iframe`, from the Desk's existing reads.
- [ ] Running, the page follows concept A of `mission-concepts.html`: the title, a state badge, items merged of total and a button and key for the lead's thread; one sentence that says how many items are being built, merging, waiting on the human and quiet; *Needs you*, a row for each item labelled needs-human, each thread T3 Code says is waiting on a person, and each quiet thread, with a button that opens the item's page or the thread; *At work*, a line for each live thread not already under *Needs you*, with its item, latest line and running time; and *Not started*, *Merged* and *Outcome and success criteria*, each one folded line that opens in place.
- [ ] A thread is quiet when T3 Code says it is live and its `updated_at` has not changed in 30 minutes, the rule This thread uses today. An item that has been building or in review for longer with a thread that is still changing is not quiet. A waiting thread is listed at once, whatever its age, as the inbox and the sidebar's count do today, and a thread both waiting and quiet is one row.
- [ ] A proposed mission's title does not open this page. It opens the mission's record page, framed, as atelier-9l8h and atelier-wvst say.
- [ ] With nothing needing the human the page shows no *Needs you* section and the sentence says so.
- [ ] Done, the page shows the state and when it closed; the mission's Result as written; a lane per item from the plan's merge to the close, with its building, review and merging spans and its review rounds and duration at the end, with a legend; and the merged items, most review rounds first, each with its review rounds. An item opens its pull request on the forge where the Desk gives one. The Desk gives none for an item whose branch is deleted or whose pull request is older than the forge's recent list, and that item is still listed, says its pull request is not known, and opens its Desk page framed.
- [ ] The page has no list of the mission's steps. The lanes and the stage ages say they come from commit subjects, as `docs/architecture/desk.md` says of the progress log today.
- [ ] `L` opens the lead's thread, else the planner's, `Enter` acts on the focused row, and `Esc` closes the page. Each key is shown on the button it presses, and no footer lists keys.
- [ ] The page holds at 728 pixels wide and stacks below 560. At 1,100 pixels and wider *Needs you* and *At work* sit side by side, the lanes take the full width, and no line of text is longer than 75 characters.
- [ ] `docs/product/desk.md` and `docs/architecture/desk.md` describe the new page and say the Desk's mission control page is no longer opened by the extension. The Desk's page itself stays until atelier-yy4j.
- [ ] The check in `test-extension.mjs` that a mission's title opens a frame on `/embed/mission/...` becomes a check that it opens the native page with no `iframe`; the checks that the layer frames the inbox stay as they are.
- [ ] Fixture checks in `t3-extension/` cover the running page with a needs-human item, a quiet thread and two live threads; the running page with no needs-human item and one thread that began waiting a minute ago, which is listed; the running page with nothing needing the human; and the done page, with one merged item whose `pull_request` is null.
- [ ] The page is seen in T3 Code Nightly on the test instance on this host (atelier-diru), running and done, in light and dark, with screenshots in the Report.

## Scope

In: `t3-extension/`, the files of `desk/src` the extension imports, `docs/product/desk.md`, `docs/architecture/desk.md`.

Out: a proposed plan, which the page need not serve; the mockup's "Differed from the plan" and "Not seen working" sections; concept B's timeline for a running mission; any write.

## Context

The mockups are `t3-extension/spike/mockups/mission-concepts.html` on `spike/extension-react`; open the file in a browser. The data mission control reads today is enough: `GET /api/repos/{repo}/missions/{id}` (sections, items, threads), each item's page, `GET /api/history` and `GET /api/thread?full=1`. `desk/src/lib/mission.ts` holds the stage and age logic with tests. The extension does not fetch the Desk itself: it asks the loader (`api.request('desk-get', { desk, path })`), for each Desk in its configuration, while the Desk's hooks in `desk/src/lib/api.ts` fetch paths on their own origin. So the components and the pure functions move, and the reads go through atelier-6dm7's hook, which also opened the loader's gate for them. This item mounts into the layer the old script already draws over T3's main area; atelier-40dc rewrites that layer afterwards. This item may start before atelier-diru merges, and its last line waits for it. atelier-diru's command starts T3 Code here with the fixture Desk, whose records hold a running mission and a done one.
