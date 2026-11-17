---
schema: atelier.item
schema_version: 3
id: atelier-6dm7
title: Give the extension a React root that draws T3 Code's own components, checked against T3's stylesheet
status: approved
mission: atelier-osel
labels: []
priority: medium
blocked_by:
- atelier-3dip
implementers:
- thread:delegated-task:command%3Amcp%3A3eadca8a-0dea-4bef-b682-3d92ff9eca06%3Adelegate-task%3Adce1aefa-68cf-4ba1-94b8-e8e9351f1968
ready:
  by: 'sumeet, in conversation on 2026-11-17: looks good, start up a lead'
  at: 2026-11-17T21:31:50.834606783Z
trivial: null
caused_by: null
created_at: 2026-11-17T19:24:26.113324242Z
---

## Goal

Let the extension draw with T3 Code's own components instead of imitating
them, and prove on every change that they still match T3.

## Done when

- [x] A script copies from a T3 Code checkout, into committed files: the Tailwind theme mapping and variants of its `index.css`, the stylesheet itself for the check below, and the components named in the script. It rewrites T3's `dark`, `light` and `wco` variants to `:host-context(...)` forms and fails with a message when T3's form of a variant has changed. The commit of T3 Code copied from is recorded beside the files.
- [x] The first components copied are `badge`, `kbd`, `tooltip`, `collapsible-section-header` and `button`, with T3's `cn`. Each copied file says it is copied and not to edit it, and T3 Code's MIT licence text is kept beside them.
- [x] One function mounts a React root in a shadow root, adopts the built stylesheet there, registers Tailwind's `@property` rules on the document once, gives Base UI a portal container inside the shadow root, and unmounts when the extension is reloaded or disposed.
- [x] The loader's read gate (`deskReadUrl` in `t3-custom.mjs`) also admits the reads the three native pages need, each a `GET` of exactly this form: `/api/repos/{repo}/missions`, `/api/motion`, `/api/history?repo=` with `days=` or `ids=`, `/api/checks?repo=&pr=`, `/api/roadmap?repo=`, `/api/thread?id=` with or without `full=1`, and `/api/ties?thread=`. A test asks for each through the real loader, and the gate still refuses a Desk not in the configuration, a dot segment, a backslash, a fragment, an unlisted parameter and any write. `README.md` says the loader must be restarted to pick the change up.
- [x] One hook reads a Desk path through `api.request('desk-get', ...)` for a named Desk, again on an interval while the page is visible, and gives a failed read as an error and not as empty data.
- [x] T3's default border and outline colours from its base layer apply inside the shadow root.
- [x] A check, run by `node run-tests.mjs`, draws each copied component in a shadow root and again under T3's stylesheet as last copied, and fails naming the element and property where a computed style differs, in light and in dark. It also fails when a tooltip opened in the shadow root is drawn outside it or unstyled.
- [x] The extension's settings section shows the installed release in a `Badge` drawn this way, as the first use in the running extension.
- [x] `t3-extension/README.md` says how to copy from a newer T3 Code and what the check proves.

## Scope

In: `t3-extension/`, with `t3-custom.mjs` for the read gate only.

Out: any page; copying a T3 component no item in this mission uses; menus, popovers and dialogs, which wait for a page that needs one.

## Context

`t3-extension/spike/` on `spike/extension-react` has a working `sync-t3.mjs`, `verify.mjs`, `src/main.tsx` and `src/styles.css` for `badge` and `kbd`. T3's `tooltip.tsx` imports app internals (`~/lib/windowControlsOverlay`); copy what stands alone or wrap Base UI's tooltip with T3's classes, and say which in the Report. T3 Code's source is MIT, at `~/projects/t3code` on this host.

## Report

The extension now draws with T3 Code's own components. `t3-extension/sync-t3.mjs` copies them, and `test-components.mjs` (run by `run-tests.mjs`) proves on every change that they draw as T3's stylesheet draws them. The settings section's installed release is the first use: T3's `Badge` in its own React root.

**Copy (`sync-t3.mjs` → `src/t3/`).** It copies `badge`, `kbd`, `tooltip`, `collapsible-section-header` and `button`, T3's `cn` (cut out of `lib/utils.ts`), T3's whole `index.css` for the check, its MIT `LICENSE`, and `SOURCE` naming the commit (532a7aaf424d). `theme.css` holds T3's `@custom-variant`, `@theme` and `@utility` blocks and three base-layer rules. The rewrites stop with a message naming the file and the form they expected: the `dark`/`light`/`wco` variants become `:host-context(...)`, app imports are mapped, and the script refuses a checkout with uncommitted changes to what it copies. I went beyond the item in three places, each because the check failed without it or T3 does it at runtime:
- `@utility` blocks: the `glass` button and tooltip variants use `surface-glass` and `dropdown-glass`.
- The lucide `stroke-width: 1.75` base rule: the check flagged the section header's chevron at 2px.
- T3's plain `@theme` (its fonts) is copied as `@theme reference`. Settings → Appearance sets `--font-sans` on `<html>`, and redeclaring it on the shadow root would hide the user's font. The check sets a custom font on `<html>` to cover this.

**Tooltip: copied, not wrapped.** `tooltip.tsx` is T3's file with two rewrites by the script. `TooltipPrimitive.Portal` gets `container={usePortalContainer()}` (`src/react/portal.ts`), because Base UI 1.9 exports no portal context. `~/lib/windowControlsOverlay` points at `src/react/windowControlsOverlay.ts`, which restates T3's `useNativeTitlebarHeight` (52px on macOS Electron, else the overlay's rect) without T3's `env` and `utils`.

**Mount (`src/react/mount.tsx`).** `mountReact(host, api)` returns `{ render, unmount }`. It attaches a shadow root, adopts the built stylesheet (one shared sheet), registers Tailwind's `@property` rules on the document once and removes them on cleanup, and adds a portal container inside the shadow root. It unmounts on `api.onCleanup`. `test-extension.mjs` checks that the old root is empty after a reload and that `document.adoptedStyleSheets` does not grow.

**Hook (`src/react/desk.ts`).** `useDeskRead(desk, path, intervalMs = 15000)` reads through `desk-get` with one read in flight at a time. It reads again on the interval while the page is visible and at once when the page becomes visible. A failure sets `error` and keeps the last data. The check page tests it against a stand-in injector.

**Read gate (`deskReadUrl`).** Paths without a query are a regex as before, plus `/api/motion` and `/api/repos/{repo}/missions`. Paths with a query each list their exact forms (key order fixed, no repeats), and each value has a shape (`repo` name characters, `days`/`pr` digits, `ids` record IDs, `full=1`, `id`/`thread` non-empty). A refusal names the forms the path takes. `test-extension.mjs` asks for all 9 page reads through the real loader and checks each reached the Desk exactly as asked. It also checks that 13 refused forms plus another Desk never reach it. README says the injector must be restarted.

**Build.** `vite.config.ts` runs Tailwind with `optimize: false`. Lightning CSS does not know `:host-context` and printed 17 misleading "is this a typo?" warnings on every build. Chromium reads the nesting it would have lowered, and the check builds T3's stylesheet optimised, as T3 does. `dist/custom.js` grows from 90 kB to 853 kB unminified (166 kB gzip), mostly React DOM.

**Proving the check can fail.** I broke the copy one thing at a time and the check named each break. The dark variant left in T3's form gave the dark backgrounds. Dropping the border/outline rule gave outline colours. Plain `@theme` gave font families. Dropping the stroke rule gave the chevron at `strokeWidth 2px`. Portalling to `<body>` gave "tooltip-*: not drawn in the shadow root" and "tooltip-hover: drawn outside the shadow root".

**Separate commit: a flaky wait in `test-extension.mjs`.** "Explicit project mapping" waited on `!mount?.shadowRoot.querySelector("iframe").hidden`, which is true when the mount does not exist yet. With the larger bundle the test failed 3 runs in 5 (master 0 in 3). After waiting for a mount that exists, it passed 4 runs in 4.

**Verified:** `pnpm typecheck`, `pnpm build`, `node run-tests.mjs` (components, extension, fixture-desk, update) and `scripts/t3-extension-release.test.sh` all pass. I have not seen it in a real T3 Code; `test-contract.mjs` is still the by-hand check after a T3 update.
