---
schema: atelier.item
schema_version: 3
id: atelier-vg1l
title: Lay the Desk out for a panel when it is embedded
status: ready
mission: atelier-61gh
labels: []
priority: medium
blocked_by: []
implementers: []
ready:
  by: 'sumeet, in conversation on 2026-11-15: get that mission going'
  at: 2026-11-16T02:21:33.165230332Z
trivial: null
caused_by: null
created_at: 2026-11-16T02:20:38.208572902Z
---

## Goal

Give the Desk an embedded mode that draws only content, sized for a panel.
Inside T3 Code's right panel the Desk shows its own sidebar, brand and theme
toggle under T3's tab, and its pages overflow a panel 400 to 750 pixels wide.

## Done when

- [ ] One switch in the URL turns the embedded mode on, and it stays on as the user follows links inside the Desk. Without it every page renders as it does on the base.
- [ ] In embedded mode there is no sidebar, brand, theme toggle or footer. The breadcrumb stays as the only way up, and the inbox is reachable from every page.
- [ ] At 420 pixels and at 720 pixels wide, the inbox, repositories, mission, item, proposed, in motion and disputed pages have no sideways scroll, and no control is cut off. An interface test checks each page at both widths.
- [ ] `docs/architecture/desk.md` says how the mode is turned on and what it removes, and `docs/product/desk.md` says the Desk can be shown inside T3 Code and that T3 supplies the frame.
- [ ] `pnpm test` and `pnpm e2e` pass, with the interface tests that exist on the base unchanged.

## Scope

In: `desk/src`, `desk/e2e`, the Desk's two documents.

Out: colours and fonts from the host and any message to it (atelier-fnm5); choosing a page from the host's thread (atelier-knx8); new pages.

## Context

`desk/src/components/Shell.tsx` draws the chrome and `desk/src/App.tsx` wraps every route in it. `pnpm e2e` answers `/api` from `desk/e2e/fixtures.ts`.
