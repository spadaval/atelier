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
whatever T3 Code's theme is, and its thread links open T3 Code's web address
in a new tab though the Desk is already inside the T3 window that can show
the thread.

## Done when

- [ ] `docs/architecture/desk.md` has a section for the contract: each message, its direction and its fields, and what the Desk does with a message it does not know (ignores it). It names no T3 Code internals.
- [ ] The Desk's configuration lists the origins that may frame it for this contract, empty by default, and the app learns the list from the server. The Desk posts only to a parent whose origin is listed, naming that origin as the target, and reads a message only when it comes from its parent window with a listed origin. With an empty list or an unlisted parent it sends and reads nothing. The installed service on this host lists T3 Code's desktop origin, `t3code://app`.
- [ ] In embedded mode the Desk tells its parent when it is ready and, on every navigation, its path and page title.
- [ ] The host can send colours and a font. The Desk maps them onto its own tokens in `desk/src/styles.css`, derives the tokens the host does not send, and applies a later message without a reload. A value that does not parse as a colour is dropped and the token keeps its value.
- [ ] In embedded mode with a listed parent, every thread link (`ThreadTitle`) sends the parent one message with the thread's identity, taken from `Agent.identity` or `LiveThread.id`, and opens no tab or popup. Where a page's thread entry lacks the identity, the server's response gains it. Outside embedded mode, or with no listed parent, the link works as on the base.
- [ ] No message makes the Desk write, act or navigate outside its own routes.
- [ ] An interface test frames the Desk in a fixture host page and checks: ready and navigation messages arrive; light then dark colours sent by the host become the Desk's background and text colours; a thread link sends one message, no new page opens and the frame's URL does not change; and a fixture host on an origin that is not listed receives nothing and changes nothing.
- [ ] `pnpm test` and `pnpm e2e` pass.

## Scope

In: `desk/src`, `desk/e2e`, `crates/atelier-desk` for the configured origins and a thread entry's identity, `docs/architecture/desk.md`, and the installed service's configuration on this host.

Out: the T3 extension's side of the contract, which is written in its own repository; the thread the host is showing (atelier-knx8 adds that message to this contract).

## Context

The Desk's tokens are the custom properties at the top of `desk/src/styles.css`, with a light set under `:root[data-theme="light"]`. A thread's link is `LiveThread.link`.
