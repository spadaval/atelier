---
schema: atelier.item
schema_version: 3
id: atelier-fnm5
title: Let a host theme the embedded Desk and open threads for it
status: ready
mission: atelier-61gh
labels: []
priority: medium
blocked_by:
- atelier-vg1l
implementers: []
ready:
  by: 'sumeet, in conversation on 2026-11-15: get that mission going'
  at: 2026-11-16T02:21:33.165230332Z
trivial: null
caused_by: null
created_at: 2026-11-16T02:20:38.280088356Z
---

## Goal

Let the page that frames the Desk set its look and handle its thread links,
through one documented contract. The embedded Desk keeps its own colours
whatever T3 Code's theme is, and its thread links open a `t3code:` link
though the Desk is already inside the T3 window that can show the thread.

## Done when

- [ ] `docs/architecture/desk.md` has a section for the contract: each message, its direction and its fields, and what the Desk does with a message it does not know (ignores it). It names no T3 Code internals.
- [ ] In embedded mode the Desk tells its parent when it is ready and, on every navigation, its path and page title.
- [ ] The host can send colours and a font. The Desk maps them onto its own tokens in `desk/src/styles.css`, derives the tokens the host does not send, and applies a later message without a reload. A value that does not parse as a colour is dropped and the token keeps its value.
- [ ] In embedded mode a link to a thread sends the parent one message with the thread's identity and does nothing else. Outside embedded mode the link works as on the base.
- [ ] The Desk reads messages only from its parent window, and no message makes it write, act or navigate outside its own routes.
- [ ] An interface test frames the Desk in a fixture host page and checks: ready and navigation messages arrive; light then dark colours sent by the host become the Desk's background and text colours; a thread link sends one message and the frame's URL does not change.
- [ ] `pnpm test` and `pnpm e2e` pass.

## Scope

In: `desk/src`, `desk/e2e`, `docs/architecture/desk.md`.

Out: the T3 extension's side of the contract, which is written in its own repository; the thread the host is showing (atelier-knx8 adds that message to this contract).

## Context

The Desk's tokens are the custom properties at the top of `desk/src/styles.css`, with a light set under `:root[data-theme="light"]`. A thread's link is `LiveThread.link`.
