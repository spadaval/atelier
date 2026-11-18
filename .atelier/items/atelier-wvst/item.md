---
schema: atelier.item
schema_version: 3
id: atelier-wvst
title: Draw the project home natively over T3's main area
status: building
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

- [ ] Each of the three ways to the project home (the project's name in T3's thread breadcrumb, the project's name in the strip under the composer, and the "more missions" row in the sidebar) opens it drawn by the extension in React with no `iframe`: Now, Next and Later, with a mission folding open to its items and their holders, and the strip of the last seven days, as `docs/architecture/desk.md` lists them today.
- [ ] The title of a mission that is agreed or done opens the mission page. A proposed mission's title, under Later, opens its record page framed. An item opens its Desk page framed. Each framed page has a way back.
- [ ] `docs/product/desk.md` and `docs/architecture/desk.md` say where the page is drawn now. The Desk's page itself stays until atelier-yy4j.
- [ ] The checks in `test-extension.mjs` that the breadcrumb, the strip's project name and the "more missions" row each open a frame on `/embed/home/...` become checks that each opens the native page with no `iframe`. No code path of the extension asks for `/embed/home` afterwards.
- [ ] Fixture checks in `t3-extension/` cover a repository with a mission in each of Now, Next and Later, and one with nothing open, and that the proposed mission under Later opens the framed record page.
- [ ] The page is seen in T3 Code Nightly on the test instance on this host (atelier-diru), in light and dark, with screenshots in the Report.

## Scope

In: `t3-extension/`, the files of `desk/src` the extension imports, `docs/product/desk.md`, `docs/architecture/desk.md`.

Out: a new design for the page; the inbox, which stays framed, in columns when wide.

## Context

`desk/src/pages/ProjectHome.tsx` is React, and its reads go through atelier-6dm7's hook. `GET /api/roadmap` and `GET /api/history?days=7` give its data.
