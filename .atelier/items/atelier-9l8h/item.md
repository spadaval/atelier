---
schema: atelier.item
schema_version: 3
id: atelier-9l8h
title: Draw This thread natively in T3's panel
status: changes_requested
mission: atelier-osel
labels: []
priority: medium
blocked_by:
- atelier-40dc
implementers:
- thread:delegated-task:command%3Amcp%3A57e793d0-a6ba-4a2b-9999-f8cb06fe0316%3Adelegate-task%3A7f50a589-3bae-4641-980f-d09f2b7ca370
ready:
  by: 'sumeet, in conversation on 2026-11-17: looks good, start up a lead'
  at: 2026-11-17T21:31:50.834606783Z
trivial: null
caused_by: null
created_at: 2026-11-17T19:24:26.396490199Z
---

## Goal

Draw This thread with T3 Code's components in T3's panel, so the view a
person sees beside every thread no longer looks like a web page in a frame.

## Done when

- [x] The panel beside a thread shows This thread drawn by the extension in React with no `iframe`: what the thread holds, the work under it, the other threads it started, the records its conversation names, and what of that waits on the human, as `docs/architecture/desk.md` lists them today. A thread that holds, started and names nothing shows its project at a glance.
- [x] The thread, its children and the IDs its conversation names come from the extension's own state, not from a `host:thread` message. The extension no longer sends `host:thread` to a frame, so a framed page the human opened stays where it is when T3 shows another thread.
- [x] A link to an item, a mission's record page or the inbox opens that Desk page framed in the panel, with a way back to This thread. The title of a mission that is agreed or done opens the mission page; a proposed mission's title opens its record page framed.
- [x] A read that fails leaves its part out and says which, and the page then does not claim the thread holds nothing.
- [x] `docs/product/desk.md` and `docs/architecture/desk.md` say where the page is drawn now. The Desk's page itself stays until atelier-yy4j.
- [x] The checks in `test-extension.mjs` that the panel opens a frame on `/embed/`, that `host:thread` is sent and that the frame follows the thread to `/embed/t` become checks that the panel shows the native page for the thread T3 shows, with no `iframe` and no `host:thread` sent. The checks that a record link opens its Desk page framed stay.
- [x] Fixture checks in `t3-extension/` cover a thread that implements an item, one that leads a mission, one with children and mentions only, one that planned a mission still proposed, whose title opens the framed record page, and one with nothing.
- [x] The page is seen in T3 Code Nightly on the test instance on this host (atelier-diru) for a thread that holds an item and one that holds nothing, in light and dark, with screenshots in the Report.

## Scope

In: `t3-extension/`, the files of `desk/src` the extension imports, `docs/product/desk.md`, `docs/architecture/desk.md`.

Out: a new design for the page. It keeps what it shows today, in T3's components; the Desk's item and inbox pages.

## Context

`desk/src/pages/ThisThread.tsx` is React, so its components move with little change. The extension does not fetch the Desk itself: it asks the loader (`api.request('desk-get', { desk, path })`), for each Desk in its configuration, while the Desk's hooks in `desk/src/lib/api.ts` fetch paths on their own origin. So the components and the pure functions move, and the reads go through atelier-6dm7's hook, which also opened the loader's gate for them. Which thread the framed Desk follows was atelier-knx8's work; once this page is native, the frame is only opened on a page the human asked for.

## Report

The panel beside a thread now shows This thread drawn by the extension in
React with T3's components (`t3-extension/src/thread/`), with no frame. The
Desk page that used to fill the panel is gone from it: a frame is opened only
on a page the human asks for, and it stays there across thread switches.

What changed:

- `src/thread/model.ts` restates the Desk page's rules with no React: which
  records the page is about, what waits on the human, which children are
  listed apart, and `holdsNothing`, which claims the thread holds nothing only
  when every read that could show something answered and none did.
  `ThisThread.tsx`, `ProjectGlance.tsx` and `parts.tsx` draw Waiting on you,
  Holds, Work under this thread, Other threads it started, Mentioned and,
  for a thread with none, the project at a glance, as
  `docs/architecture/desk.md` lists them.
- `src/panel/panel.ts` draws the page from the thread T3 shows, its children
  (T3's lineage, read every 3 seconds) and the IDs its conversation names (the
  reference scanner); no `host:thread` is sent anywhere. Links on the page
  frame the item's page, the inbox or a proposed plan's record page, with a
  bar whose "This thread" goes back; an agreed or done mission's title opens
  the mission page in the layer. The page is unmounted, so reads stop, while
  the panel is closed or framed. Diagnostics gain `thread` (what the page
  follows) and `thisThread` (what it is drawn with now).
- A read that fails is named in a line with its path and error, and its part
  is left out; a named record the Desk answers 404 for is simply not named.
- `test-thread.mjs` with `check/thread.tsx` and `check/thread-fixtures.ts`
  draws the page for the five fixture threads the item names and checks what
  each lists and opens, and failed reads of the ties, the inbox, an item's
  page and a named record (9 checks). `test-extension.mjs` now checks the
  native page in the panel with no `iframe` and no `/embed` load, records
  what it follows instead of `host:thread` messages (the same mention,
  child, references-off and scan-crediting checks), checks that the framed
  record stays unreloaded across thread switches and that the built script
  holds no `host:thread`; the frame checks (theme, origins,
  `desk:open-thread`, Refresh) run against an item framed from the page.
- Docs: `docs/product/desk.md`, `docs/architecture/desk.md` and
  `t3-extension/README.md` say the extension draws the page and the Desk's
  own page and `host:thread` stay for other hosts until atelier-yy4j.

Verification: `pnpm typecheck` passes; `node run-tests.mjs` passes all six
test files (components, extension, fixture-desk, mission, thread, update)
against a Desk built from this checkout.

Seen in T3 Code Nightly v0.1.11-nightly.20261117.4851 on the test instance
(`t3-instance.mjs start`, own root and ports): the thread waiting on an
approval, which implements `atelier-fi01`, and a new thread sent from the
project's draft that holds nothing and shows the project at a glance, each in
light and dark; and the item framed from This thread with its Back bar.
Screenshots in `artifacts/`: `t3-holds-item-light.png`,
`t3-holds-item-dark.png`, `t3-holds-nothing-light.png`,
`t3-holds-nothing-dark.png`, `t3-item-framed.png`; from the fixtures,
`fixture-leads-mission-dark.png`, `fixture-planned-proposed-light.png` and
`fixture-children-and-mentions-light.png`.

Seen and not changed: the implementer thread in the test instance is stopped
on an approval, yet This thread does not list it under Waiting on you. The
inbox's thread list does not carry `pending_requests`, so T3 reports it as
`running`; the Desk's page has the same gap, and atelier-3zh2 covers it.

`/tmp` ran out of quota mid-way (other sessions' files); the lead freed
Atelier's own stale folders and nothing else was deleted. My test runs used
`TMPDIR` under `~/.cache`; the two `/tmp/atelier-thread-test-*` folders from my
first runs are removed.

In round 1 the review deferred its third finding, which is inherited from the Desk's own glance; once this item is done `atelier list --deferred` lists it for `atelier new --from`. The fixes for round 1 also make `test-extension.mjs` wait until its recorder has logged thread-b before switching back, which made the switch check fail about one run in four; `node run-tests.mjs` passes all six files, and `test-extension.mjs` passed three more runs in a row.

### Responses

- 1-wi75.1: fixed in 12bd1fced. Each thread-info read (the thread's own title and status, each child, each thread on an item of its missions, and each holder in the project at a glance) now names a failure with its path and error. Only the Desk's 404 means T3 Code does not know the thread; a child whose read failed says "Could not be read" and a chip says "not read", and a failed read keeps its last answer beside the note. Four new checks in `test-thread.mjs` inject a 502 into the child, chip, own-title and holder reads; the fixture's unknown child now answers 404, as the Desk does, and is still said to be unknown with no notice.
- 1-wi75.2: fixed in 12bd1fced. The README's "The sidebar and the layer" section is restored from `master`, unchanged; nothing in it describes the panel.
