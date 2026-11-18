---
schema: atelier.item
schema_version: 3
id: atelier-wvst
title: Draw the project home natively over T3's main area
status: approved
mission: atelier-osel
labels: []
priority: medium
blocked_by:
- atelier-40dc
implementers:
- thread:delegated-task:command%3Amcp%3A57e793d0-a6ba-4a2b-9999-f8cb06fe0316%3Adelegate-task%3Ac007f218-3e6d-4019-bfa2-9d5b163d4465
ready:
  by: 'sumeet, in conversation on 2026-11-17: looks good, start up a lead'
  at: 2026-11-17T21:31:50.834606783Z
trivial: null
caused_by: null
created_at: 2026-11-17T19:24:26.466783114Z
---

## Goal

Draw the project home with T3 Code's components over T3's main area, the
last of the three views to leave its frame.

## Done when

- [x] Each of the three ways to the project home (the project's name in T3's thread breadcrumb, the project's name in the strip under the composer, and the "more missions" row in the sidebar) opens it drawn by the extension in React with no `iframe`: Now, Next and Later, with a mission folding open to its items and their holders, and the strip of the last seven days, as `docs/architecture/desk.md` lists them today.
- [x] The title of a mission that is agreed or done opens the mission page. A proposed mission's title, under Later, opens its record page framed. An item opens its Desk page framed. Each framed page has a way back.
- [x] `docs/product/desk.md` and `docs/architecture/desk.md` say where the page is drawn now. The Desk's page itself stays until atelier-yy4j.
- [x] The checks in `test-extension.mjs` that the breadcrumb, the strip's project name and the "more missions" row each open a frame on `/embed/home/...` become checks that each opens the native page with no `iframe`. No code path of the extension asks for `/embed/home` afterwards.
- [x] Fixture checks in `t3-extension/` cover a repository with a mission in each of Now, Next and Later, and one with nothing open, and that the proposed mission under Later opens the framed record page.
- [x] The page is seen in T3 Code Nightly on the test instance on this host (atelier-diru), in light and dark, with screenshots in the Report.

## Scope

In: `t3-extension/`, the files of `desk/src` the extension imports, `docs/product/desk.md`, `docs/architecture/desk.md`.

Out: a new design for the page; the inbox, which stays framed, in columns when wide.

## Context

`desk/src/pages/ProjectHome.tsx` is React, and its reads go through atelier-6dm7's hook. `GET /api/roadmap` and `GET /api/history?days=7` give its data.

## Report

The project home is now drawn by the extension, over T3's main area, with no
frame. The project's name in the thread breadcrumb, the project's name in the
strip and "N more missions" all open it, and the extension no longer asks the
Desk for `/embed/home`. Seen in T3 Code Nightly v0.1.11-nightly.20261117.4851 on
the test instance, opened from the breadcrumb:
`artifacts/home-t3-light.png` and `artifacts/home-t3-dark.png`.

**What changed.**

- `t3-extension/src/home/ProjectHome.tsx` draws *Now*, *Next* and *Later*,
  each mission folding open to its items and who holds them, and *This
  week*. `src/home/model.ts` holds the rules, restated from
  `desk/src/pages/ProjectHome.tsx`, since the extension may import only types
  from `desk/`. It reads `GET /api/repos/<repo>/missions`,
  `GET /api/roadmap?repo=<repo>`, `GET /api/history?repo=<repo>&days=7` and,
  as a mission is folded open, its page; the gate already admitted each.
  Holders are named from the mission page's `threads`, so the home makes no
  thread reads.
- An agreed or done mission's title opens the mission page. A proposed plan's
  title, under Later, opens its record page framed, and an item its Desk page
  framed, each with **Back** ("Back to the project home"). An entry of the
  week opens what it names the same way.
- A read of the items in no mission or of the week that fails leaves its part
  out with a note naming the error; a failed missions read says so, naming
  the Desk.
- The layer's target now has `native` (`mission` or `home`) in place of
  `mission`, and `src/mission/mount.tsx` became `src/layer/native.tsx`, one
  React mount that draws either page. `layer.ts` became `layer.tsx` to
  render them.
- `docs/product/desk.md`, `docs/architecture/desk.md` and
  `t3-extension/README.md` say the extension draws the page; the Desk's own
  page stays until atelier-yy4j.

**Checks.** `node t3-extension/run-tests.mjs`: all 6 pass.

- `test-extension.mjs`: "N more missions", the breadcrumb and the strip's
  project name each open the home with no iframe, drawn from the Desk's reads.
  From it an item opens framed with Back to the home, an agreed mission opens
  the mission page, and a proposed plan under Later opens its record page
  framed with Back. The proxy marks a mission proposed in
  `/api/repos/<repo>/missions` too, for that check. It asserts no request
  for `/embed/home` reached the Desk.
- `test-home.mjs` (new), from `check/home-fixtures.ts`: a repository with a
  mission and a loose item in each of Now, Next and Later, and one with
  nothing open. It checks the lanes, a mission's items in order with their
  holders, the week, what each row opens (a proposed plan says it is one),
  `Esc`, the reads it makes, a failed read, and layout at 500, 728, 1,100 and
  1,400 pixels with no sideways scroll.
- `test-mission.mjs` still passes with the shared mount.

**For atelier-yy4j.** With no main area for the layer (the `layer` view off),
the home opens in the browser at the Desk's `/home/<repo>`, as the mission
page opens the Desk's mission route. Deleting that page needs that fallback
changed too: `layerTargets.home` in `t3-extension/src/layer/layer.tsx`.

### Responses

- 1-pz1g.1: fixed in 02981c85b. A failed refresh of the items in no mission, the week or an open mission's items keeps what it gave with a note "could not be read again, so they are as of <time>". With no earlier answer, the first two still say "left out". `test-home.mjs` checks each case: one successful read, then a failure.
- 1-pz1g.2: fixed in 02981c85b. Items in no mission show the Desk's three priority bars, critical in the warning colour. The fixtures now use critical, high and low, and `test-home.mjs` checks each.
