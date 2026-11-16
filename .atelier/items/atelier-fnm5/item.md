---
schema: atelier.item
schema_version: 3
id: atelier-fnm5
title: Let a host theme the embedded Desk and open threads for it
status: changes_requested
mission: atelier-61gh
labels: []
priority: medium
blocked_by:
- atelier-vg1l
implementers:
- thread:delegated-task:command%3Amcp%3Abedbfab3-88f2-4387-b923-ba6bd0a0f9b1%3Adelegate-task%3Ad60912f0-5bf6-42db-a388-3bc0bd0b6903
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

- [x] `docs/architecture/desk.md` has a section for the contract: each message, its direction and its fields, and what the Desk does with a message it does not know (ignores it). It names no T3 Code internals.
- [x] The Desk's configuration lists the origins that may frame it for this contract, empty by default, and the app learns the list from the server. The Desk posts only to a parent whose origin is listed, naming that origin as the target, and reads a message only when it comes from its parent window with a listed origin. With an empty list or an unlisted parent it sends and reads nothing. The installed service on this host lists T3 Code's desktop origin, `t3code://app`.
- [x] In embedded mode the Desk tells its parent when it is ready and, on every navigation, its path and page title.
- [x] The host can send colours and a font. The Desk maps them onto its own tokens in `desk/src/styles.css`, derives the tokens the host does not send, and applies a later message without a reload. A value that does not parse as a colour is dropped and the token keeps its value.
- [x] In embedded mode with a listed parent, every thread link (`ThreadTitle`) sends the parent one message with the thread's identity, taken from `Agent.identity` or `LiveThread.id`, and opens no tab or popup. Where a page's thread entry lacks the identity, the server's response gains it. Outside embedded mode, or with no listed parent, the link works as on the base.
- [x] No message makes the Desk write, act or navigate outside its own routes.
- [x] An interface test frames the Desk in a fixture host page and checks: ready and navigation messages arrive; light then dark colours sent by the host become the Desk's background and text colours; a thread link sends one message, no new page opens and the frame's URL does not change; and a fixture host on an origin that is not listed receives nothing and changes nothing.
- [x] `pnpm test` and `pnpm e2e` pass.

## Scope

In: `desk/src`, `desk/e2e`, `crates/atelier-desk` for the configured origins and a thread entry's identity, `docs/architecture/desk.md`, and the installed service's configuration on this host.

Out: the T3 extension's side of the contract, which is written in its own repository; the thread the host is showing (atelier-knx8 adds that message to this contract).

## Context

The Desk's tokens are the custom properties at the top of `desk/src/styles.css`, with a light set under `:root[data-theme="light"]`. A thread's link is `LiveThread.link`.

## Report

A host whose origin the Desk's configuration lists can now theme the
embedded Desk and open its thread links itself. Framed by anything else, or
with the list empty (the default), the Desk sends and reads nothing and
works as on the base.

- **The list.** `embed_origins` in the Desk's configuration
  (`crates/atelier-desk/src/config.rs`), empty by default. `load` refuses a
  value that is not `scheme://host[:port]`, so `*`, `null` and a path never
  reach `postMessage`. The app reads it from the new `GET /api/embed`
  (`{origins}`), and only when it opens under `/embed` inside a frame.
- **Who it talks to** (`desk/src/lib/host.ts`). Its parent window only. It
  learns the parent's origin from `location.ancestorOrigins`, else
  `document.referrer`, else the first message the parent sends from a
  listed origin; it posts with that origin as the target, and reads a
  message only when `event.source` is the parent and `event.origin` is that
  origin.
- **The messages,** whose names I chose: `desk:ready {version: 1}`,
  `desk:navigated {path, title}` (path without `/embed`; sent on every
  navigation and again when a page's title arrives with its data), and
  `desk:open-thread {thread}` from the Desk; `host:theme {colors, font,
  scheme}` from the host. Unknown types and non-objects are ignored.
  `docs/architecture/desk.md`, "The host contract", defines them and names
  no T3 Code internals.
- **The theme** (`desk/src/lib/theme.ts`). Six colours (`background`,
  `foreground`, `surface`, `border`, `muted`, `accent`) and a font map onto
  the tokens in `styles.css`; the rest are derived with `color-mix` from
  `--bg` and `--ink`. A value is kept only if `CSS.supports` parses it and
  it does not refer outside itself (`var()`, `env()`, `url()`, `inherit`,
  or `currentColor` anywhere in it); otherwise it is dropped and the token keeps its value.
  Messages merge, so a later one changes only what it sends. `scheme` picks
  the Desk's light or dark set for its state colours; without it the
  scheme follows the background's luminance, read from the colour painted
  on a canvas. The e2e test caught a bug here
  that I fixed: a `scheme` sent with a light background stuck after the
  host sent dark colours without one. A new background without a scheme now
  clears it.
- **Thread links.** `ThreadTitle` now takes the thread's identity. With a
  listed parent it is a button that sends `desk:open-thread` and opens
  nothing. Every caller already had the identity in its response
  (`Agent.identity`, `LiveThread.id`, `Side.identity`,
  `SessionResponse.thread_id`), so no response changed.
- **Nothing acts.** The only message the Desk handles is `host:theme`, which
  sets custom properties and `data-theme`. No message writes, fetches,
  opens or navigates.

### The installed service

`desk/service/install.sh` writes `embed_origins = ["t3code://app"]` into a
new configuration. On this host the existing
`~/.config/atelier-desk/config.toml` gains the same line when this merges:
the installed binary refuses a key it does not know
(`deny_unknown_fields`), so adding it before the new binary is installed
would stop the service from starting. At merge I run `install.sh` (which
keeps the configuration), then add the line and restart the service.

### Validation

- `cargo nextest run -p atelier-desk`: 44 passed, with a new route test and
  the configuration test's new cases. `cargo fmt --check`, `cargo clippy -p
  atelier-desk --all-targets -D warnings`.
- `pnpm test`: 51 passed, with `src/lib/theme.test.ts`. `pnpm build`.
- `pnpm e2e`: 90 passed across desktop and phone, with
  `desk/e2e/host.spec.ts`. It serves two fixture host pages on loopback
  ports of their own (Chromium refuses to let a page with no loopback
  address frame `127.0.0.1`), and checks: ready and navigation messages
  with the right path and title; light then dark colours becoming the
  frame's background and text, and a font; a bad colour and an unknown
  message changing nothing; a thread link sending exactly one
  `desk:open-thread` with no new page and no change to the frame's URL;
  an unlisted host receiving nothing and its theme ignored, with the link
  still pointing at T3 Code's address; and an empty list drawing the Desk
  and sending nothing.
- I could not check that T3 Code's desktop app reports `t3code://app` as
  its origin to a framed page; that depends on how it registers the scheme,
  and the extension's side is out of scope.


### Responses

- 1-52g0.1: fixed in 763f68fd2: `scheme()` reads the background's sRGB bytes from a 1-pixel canvas, so `oklch()`, `color()` and `color-mix()` set the scheme; `desk/e2e/host.spec.ts` checks `color-scheme` flips light then dark with those forms.
- 1-52g0.2: fixed in 763f68fd2: `currentColor` is refused anywhere in a colour; the unit and interface tests send `color-mix(in srgb, currentColor 50%, white)` and check it is dropped.
