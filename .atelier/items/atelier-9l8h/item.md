---
schema: atelier.item
schema_version: 3
id: atelier-9l8h
title: Draw This thread natively in T3's panel
status: ready
mission: atelier-osel
labels: []
priority: medium
blocked_by:
- atelier-40dc
implementers: []
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

- [ ] The panel beside a thread shows This thread drawn by the extension in React with no `iframe`: what the thread holds, the work under it, the other threads it started, the records its conversation names, and what of that waits on the human, as `docs/architecture/desk.md` lists them today. A thread that holds, started and names nothing shows its project at a glance.
- [ ] The thread, its children and the IDs its conversation names come from the extension's own state, not from a `host:thread` message. The extension no longer sends `host:thread` to a frame, so a framed page the human opened stays where it is when T3 shows another thread.
- [ ] A link to an item, a mission's record page or the inbox opens that Desk page framed in the panel, with a way back to This thread. The title of a mission that is agreed or done opens the mission page; a proposed mission's title opens its record page framed.
- [ ] A read that fails leaves its part out and says which, and the page then does not claim the thread holds nothing.
- [ ] `docs/product/desk.md` and `docs/architecture/desk.md` say where the page is drawn now. The Desk's page itself stays until atelier-yy4j.
- [ ] The checks in `test-extension.mjs` that the panel opens a frame on `/embed/`, that `host:thread` is sent and that the frame follows the thread to `/embed/t` become checks that the panel shows the native page for the thread T3 shows, with no `iframe` and no `host:thread` sent. The checks that a record link opens its Desk page framed stay.
- [ ] Fixture checks in `t3-extension/` cover a thread that implements an item, one that leads a mission, one with children and mentions only, one that planned a mission still proposed, whose title opens the framed record page, and one with nothing.
- [ ] The page is seen in T3 Code Nightly on the test instance on this host (atelier-diru) for a thread that holds an item and one that holds nothing, in light and dark, with screenshots in the Report.

## Scope

In: `t3-extension/`, the files of `desk/src` the extension imports, `docs/product/desk.md`, `docs/architecture/desk.md`.

Out: a new design for the page. It keeps what it shows today, in T3's components; the Desk's item and inbox pages.

## Context

`desk/src/pages/ThisThread.tsx` is React, so its components move with little change. The extension does not fetch the Desk itself: it asks the loader (`api.request('desk-get', { desk, path })`), for each Desk in its configuration, while the Desk's hooks in `desk/src/lib/api.ts` fetch paths on their own origin. So the components and the pure functions move, and the reads go through atelier-6dm7's hook, which also opened the loader's gate for them. Which thread the framed Desk follows was atelier-knx8's work; once this page is native, the frame is only opened on a page the human asked for.
