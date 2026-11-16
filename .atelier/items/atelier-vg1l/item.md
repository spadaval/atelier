---
schema: atelier.item
schema_version: 3
id: atelier-vg1l
title: Lay the Desk out for a panel when it is embedded
status: in_review
mission: atelier-61gh
labels: []
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3Abedbfab3-88f2-4387-b923-ba6bd0a0f9b1%3Adelegate-task%3A198d7fda-8b3f-4b54-bc0c-d7e7cec75961
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

- [x] One switch in the URL turns the embedded mode on, and it stays on as the user follows links inside the Desk. Without it every page renders as it does on the base.
- [x] In embedded mode there is no sidebar, brand, theme toggle or footer. The breadcrumb stays as the only way up, and the inbox is reachable from every page.
- [x] At 420 pixels and at 720 pixels wide, the inbox, repositories, mission, item, proposed, in motion and disputed pages have no sideways scroll, and no control is cut off. An interface test checks each page at both widths.
- [x] `docs/architecture/desk.md` says how the mode is turned on and what it removes, and `docs/product/desk.md` says the Desk can be shown inside T3 Code and that T3 supplies the frame.
- [x] `pnpm test` and `pnpm e2e` pass, with the interface tests that exist on the base unchanged.

## Scope

In: `desk/src`, `desk/e2e`, the Desk's two documents.

Out: colours and fonts from the host and any message to it (atelier-fnm5); choosing a page from the host's thread (atelier-knx8); new pages.

## Context

`desk/src/components/Shell.tsx` draws the chrome and `desk/src/App.tsx` wraps every route in it. `pnpm e2e` answers `/api` from `desk/e2e/fixtures.ts`.

## Report

All five Done when lines hold. `/embed` in the path turns the embedded mode
on, and the Desk then draws only the page, its breadcrumb and an inbox link.

- **The switch.** A path under `/embed` (`/embed/inbox`,
  `/embed/r/{repo}/item/{id}`) turns the mode on. `desk/src/main.tsx` gives
  the router `/embed` as its base when the page opens there
  (`desk/src/lib/embed.ts`), so every `Link` keeps the prefix and a reload
  stays in the mode. I chose a path prefix over a query parameter for that
  reason: a query parameter would have needed every link rewritten. The
  server already answers any path outside `/api` with the app's
  `index.html`, so no Rust changed.
- **What it removes.** `Shell` draws `EmbeddedShell` in the mode: no
  sidebar (brand, page links, repository list), no theme toggle, no footer,
  no menu button. The top bar keeps the breadcrumb and gains an inbox link
  with the waiting count. The theme still follows the saved choice or the
  system until the host sends one (atelier-fnm5).
- **Panel widths.** The panel is the iframe's viewport, so the existing
  narrow media queries already apply; the embedded CSS only drops the
  sidebar column. `desk/e2e/embedded.spec.ts` loads the inbox,
  repositories, mission, item, proposed, in motion and disputed pages at 420
  and 720 pixels, and checks for no chrome, no sideways scroll, and that
  every visible link, button, input, summary and tab lies inside the
  window's width. It also follows links from the inbox to a mission, up the
  breadcrumb and back to the inbox, and reloads, staying under `/embed`.
- **Docs.** `docs/architecture/desk.md` has "The embedded mode" under The
  app; `docs/product/desk.md` says the Desk can be shown in T3 Code's panel
  and that T3 supplies the frame.

### A change outside the embedded mode

The width test found the disputed page broken on the base at every width,
desktop included: each row's text sat in a column two characters wide. Two
causes, both fixed:

- Its rows used `.qline`, whose first grid column is an 18-pixel glyph, but
  had no glyph, so the text went into that column. Each row now has one: a
  scale for a rebuttal, a target for a split plan round, as the proposed
  page has.
- The side of a dispute was a `span.side`, which took the sidebar's styles
  (sticky, full height, and off-screen on a phone). It is now
  `span.dispute-side`. `.t-link` also gets `max-width: 100%`, so a long
  thread name is cut short with its ellipsis instead of widening the page.

The mission's Authority asks before a change a user would see outside the
mode. I made this one without asking because it repairs a page that did not
read at all, and nothing else on it changed; a reviewer who disagrees can
ask for it as its own item.

### Validation

- `pnpm typecheck`, `pnpm test` (43 tests, including the new
  `src/lib/embed.test.ts`), `pnpm build`.
- `pnpm e2e`: 82 passed across the desktop and phone projects. The
  existing `pages.spec.ts` and `actions.spec.ts` are unchanged and pass.
- Screenshots at 420 and 720 pixels of the disputed, item and mission
  pages, and of the disputed page at 1440 outside the mode, looked right.
