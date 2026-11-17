---
schema: atelier.item
schema_version: 3
id: atelier-9l8h
title: Draw This thread natively in T3's panel
status: draft
mission: atelier-osel
labels: []
priority: medium
blocked_by:
- atelier-40dc
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-17T19:24:26.396490199Z
---

## Goal

Draw This thread with T3 Code's components in T3's panel, so the view a
person sees beside every thread no longer looks like a web page in a frame.

## Done when

- [ ] The panel beside a thread shows This thread drawn by the extension in React with no `iframe`: what the thread holds, the work under it, the other threads it started, the records its conversation names, and what of that waits on the human, as `docs/architecture/desk.md` lists them today. A thread that holds, started and names nothing shows its project at a glance.
- [ ] The thread, its children and the IDs its conversation names come from the extension's own state, not from a `host:thread` message.
- [ ] A link to an item, a mission's record page or the inbox opens that Desk page framed in the panel, with a way back to This thread. A mission's title opens the mission page.
- [ ] A read that fails leaves its part out and says which, and the page then does not claim the thread holds nothing.
- [ ] `docs/product/desk.md` and `docs/architecture/desk.md` say where the page is drawn now. The Desk's page itself stays until atelier-yy4j.
- [ ] Fixture checks in `t3-extension/` cover a thread that implements an item, one that leads a mission, one with children and mentions only, and one with nothing.
- [ ] The page is seen in T3 Code Nightly on Legend's test instance for a thread that holds an item and one that holds nothing, in light and dark, with screenshots in the Report.

## Scope

In: `t3-extension/`, the files of `desk/src` the extension imports, `docs/product/desk.md`, `docs/architecture/desk.md`.

Out: a new design for the page. It keeps what it shows today, in T3's components; the Desk's item and inbox pages.

## Context

`desk/src/pages/ThisThread.tsx` is React, so its components move with little change. The extension does not fetch the Desk itself: it asks the loader (`api.request('desk-get', { desk, path })`), for each Desk in its configuration, while the Desk's hooks in `desk/src/lib/api.ts` fetch paths on their own origin. So the components and the pure functions move, and the reads need a hook over `desk-get`. Which thread the framed Desk follows was atelier-knx8's work; once this page is native, the frame is only opened on a page the human asked for.
