---
schema: atelier.item
schema_version: 3
id: atelier-40dc
title: Rewrite the sidebar missions, the Inbox row, the project strip and the layer as typed modules, and delete the old script
status: in_review
mission: atelier-osel
labels: []
priority: medium
blocked_by:
- atelier-toee
implementers:
- thread:delegated-task:command%3Amcp%3A57e793d0-a6ba-4a2b-9999-f8cb06fe0316%3Adelegate-task%3A28060ad7-31da-41e0-a0d3-63a0480b28fe
ready:
  by: 'sumeet, in conversation on 2026-11-17: looks good, start up a lead'
  at: 2026-11-17T21:31:50.834606783Z
trivial: null
caused_by: null
created_at: 2026-11-17T19:24:26.326415300Z
---

## Goal

Finish the rewrite, so the extension is one typed codebase and the old
script is deleted.

## Done when

- [x] These are TypeScript modules under `t3-extension/src`: missions in T3's thread list with their folding and their rows; the Inbox row and its count; the project strip under the new-thread composer; and the layer over T3's main area that holds a native page or a framed one.
- [x] After the layer is rewritten, a mission's title still opens atelier-ut89's native page with no `iframe`, and the Inbox row still opens the inbox framed. A separate check covers each.
- [x] The project strip is React, drawn with the copied T3 components. Rows placed inside T3's own list stay direct DOM code.
- [x] The repository holds no hand-written `custom.js` or `host-adapter.js`, and `pnpm typecheck` passes with `strict` on and no file excluded.
- [x] Each fact about T3's behaviour in the old code's comments for these parts is kept beside the code that handles it: the copy T3 appends when it fades a removed row, the range gestures and their platform rule, the folder's `aria-expanded`, and timers throttled in a covered window. The Report lists them.
- [x] The checks in `test-extension.mjs` for these parts pass, unchanged in what they assert.
- [x] `test-contract.mjs` passes with every hook against the test instance on this host (atelier-diru), and the sidebar, the strip and the layer are seen working there.

## Scope

In: `t3-extension/`.

Out: This thread and the project home, which stay framed until atelier-9l8h and atelier-wvst.

## Report

The extension is now one typed codebase. `src/custom.js` is deleted, and the
sidebar, the layer and the project strip are TypeScript modules under
`t3-extension/src`. `pnpm typecheck` passes with `strict` on and nothing
excluded, and `node run-tests.mjs` passes all 5 tests. In T3 Code Nightly on
this host, `test-contract.mjs` passed all 35 hooks, and the sidebar, the
layer and the strip were seen working.

**What changed.**

- `src/layer/layer.ts` is the layer over T3's main area. A `LayerTarget`
  names what it shows: a framed Desk page, or the native mission page when
  `mission` is set. `layerTargets` builds the inbox, project home, item and
  mission targets that the sidebar and the strip open. The frame, the Close
  and Back buttons, and the host contract are DOM code inside T3's `main`.
- `src/layer/home.ts` is the breadcrumb's project home. It is still framed,
  as the item's scope says.
- `src/sidebar/sidebar.ts` is the Inbox row and its count, placed under T3's
  search. `src/sidebar/missions.ts` places missions in T3's thread list, with
  their folding, rows and gestures. Both are DOM code among T3's own nodes.
- `src/strip/ProjectStrip.tsx` draws the strip's cards in React with T3's
  `Button`, `InlineButton` and `Badge`. `src/strip/strip.tsx` places the
  strip under the composer on each DOM pass and redraws it only when its
  repository, draft, width class or composer availability changes. It reads
  `/api/launch` through `useDeskRead`, every 60 seconds while the window is
  shown and again when it is shown.
- `main.ts` wires the views. The DOM pass runs, in order: the strip, the
  project home, the sidebar with its missions, then the layer, which marks
  the row of what it shows.
- `tsconfig.json` loses `allowJs`, since no JavaScript is left in `src/`.
- `test-extension.mjs` reaches the strip's cards through
  `[data-t3-strip]`'s `shadowRoot`, since they are drawn in one. Those 8
  lines change only the query. Every assertion is unchanged.

**Facts about T3 kept beside the code that handles them.**

- The copy T3 appends when it fades a removed row: `missions.ts`, at
  `removeListRows` (the extension's rows are copied too, so every row with
  its mark goes) and in `sync` (the inert, absolutely placed copies are left
  out of the walk; `Sidebar.motion.ts`).
- The range gestures and their platform rule: `missions.ts`, above the
  `pointerdown` and `click` listeners. A drag from Settle and a Shift-click
  act on T3's own order. Every click with Shift held is stopped, because
  T3's rule for which modifiers make a range depends on the platform.
- The folder's `aria-expanded`: `missions.ts` in `sync`. A closed folder
  keeps only the open thread's row, and T3 can reopen a folder by its
  button's `aria-expanded` alone, which `Extension.onMutation` watches
  (`core/extension.ts`).
- Timers throttled in a covered window: `layer.ts` in `open` (the layer
  syncs at once rather than waiting on the DOM pass's timer), and `main.ts`
  at `visibilitychange` (the sidebar reads again when the window is shown;
  the strip's and the mission page's reads follow visibility themselves).
- Also kept: moving an iframe reloads it (`layer.ts`); a list with threads
  and no folders, while not searching, means the boring sidebar is off; the
  composer's width changes without a window resize (`strip.tsx`).

**What a user sees differently.**

- The strip is drawn with T3's components. Counts are T3 badges, the rows
  are T3 ghost buttons, "Plan a mission" is an outline button and the
  project name is an inline button. The text and layout are as before.
- When a later `/api/launch` read fails, the strip keeps the last cards and
  shows "Not read again: <error>" in its head. Before, the error replaced the
  cards.
- An In motion holder whose title T3 does not know shows its identity, not
  "null".

**In the real T3.** I started the test instance (`node t3-instance.mjs
start`, T3 Code Nightly `v0.1.11-nightly.20261117.4851`) and drove it over
its debugging port:

- `test-contract.mjs --thread … --draft …` exits 0 with 35 `ok` lines.
- The Inbox row read "2". The fixture mission's header read `1/3` with the
  instance's thread under it, labelled `lead` and indented, and "1 more
  mission" had its dot. No probe was missing.
- The mission's title opened the mission page in the layer: `running`, h1
  "Fixture mission", no `iframe`, its row pressed.
- The Inbox row opened `/embed/inbox` framed, with `IFRAME,BUTTON` in the
  layer and no native page. The Desk connected, drew the inbox, and the row
  was pressed.
- On the draft, the strip sat 12px under the composer, 736px wide and 176px
  high, with Waiting on you 2, In motion 1 and Next up 1. No error was
  reported.

Screenshots are in `~/tmp/atelier-40dc/look/` on buildbot.

**A separate fix.** The test instance's fixture Desk had no `embed_origins`.
So no Desk page framed in T3 Code (`t3code://app`), in the panel or the
layer, ever sent `desk:ready`: it loaded but was never themed or told the
thread. `writeFixtureDesk` now takes the origins to allow, and
`t3-instance.mjs` passes `t3code://app`. This was its own commit, dae1c5420.
Master made the same fix meanwhile, with more, so the merge of master keeps
master's version of both files.

**Not done.** The project home and This thread stay framed, as the scope
says. I looked at the views in light mode only. `test-components.mjs` checks
T3's components as the extension draws them in light and dark, but nothing
looked at the strip in dark.

### Responses

1-tto1.1: fixed in 59b7942b6: rows are 34px (no border, a centred line, 13px text) under a 20px head, so three fit; a check with three held items asserts none passes the card.
