---
schema: atelier.item
schema_version: 3
id: atelier-6dm7
title: Give the extension a React root that draws T3 Code's own components, checked against T3's stylesheet
status: draft
mission: atelier-osel
labels: []
priority: medium
blocked_by:
- atelier-3dip
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-17T19:24:26.113324242Z
---

## Goal

Let the extension draw with T3 Code's own components instead of imitating
them, and prove on every change that they still match T3.

## Done when

- [ ] A script copies from a T3 Code checkout, into committed files: the Tailwind theme mapping and variants of its `index.css`, the stylesheet itself for the check below, and the components named in the script. It rewrites T3's `dark`, `light` and `wco` variants to `:host-context(...)` forms and fails with a message when T3's form of a variant has changed. The commit of T3 Code copied from is recorded beside the files.
- [ ] The first components copied are `badge`, `kbd`, `tooltip`, `collapsible-section-header` and `button`, with T3's `cn`. Each copied file says it is copied and not to edit it, and T3 Code's MIT licence text is kept beside them.
- [ ] One function mounts a React root in a shadow root, adopts the built stylesheet there, registers Tailwind's `@property` rules on the document once, gives Base UI a portal container inside the shadow root, and unmounts when the extension is reloaded or disposed.
- [ ] The loader's read gate (`deskReadUrl` in `t3-custom.mjs`) also admits the reads the three native pages need, each a `GET` of exactly this form: `/api/repos/{repo}/missions`, `/api/motion`, `/api/history?repo=` with `days=` or `ids=`, `/api/checks?repo=&pr=`, `/api/roadmap?repo=`, `/api/thread?id=` with or without `full=1`, and `/api/ties?thread=`. A test asks for each through the real loader, and the gate still refuses a Desk not in the configuration, a dot segment, a backslash, a fragment, an unlisted parameter and any write. `README.md` says the loader must be restarted to pick the change up.
- [ ] One hook reads a Desk path through `api.request('desk-get', ...)` for a named Desk, again on an interval while the page is visible, and gives a failed read as an error and not as empty data.
- [ ] T3's default border and outline colours from its base layer apply inside the shadow root.
- [ ] A check, run by `node run-tests.mjs`, draws each copied component in a shadow root and again under T3's stylesheet as last copied, and fails naming the element and property where a computed style differs, in light and in dark. It also fails when a tooltip opened in the shadow root is drawn outside it or unstyled.
- [ ] The extension's settings section shows the installed release in a `Badge` drawn this way, as the first use in the running extension.
- [ ] `t3-extension/README.md` says how to copy from a newer T3 Code and what the check proves.

## Scope

In: `t3-extension/`, with `t3-custom.mjs` for the read gate only.

Out: any page; copying a T3 component no item in this mission uses; menus, popovers and dialogs, which wait for a page that needs one.

## Context

`t3-extension/spike/` on `spike/extension-react` has a working `sync-t3.mjs`, `verify.mjs`, `src/main.tsx` and `src/styles.css` for `badge` and `kbd`. T3's `tooltip.tsx` imports app internals (`~/lib/windowControlsOverlay`); copy what stands alone or wrap Base UI's tooltip with T3's classes, and say which in the Report. T3 Code's source is MIT, at `~/projects/t3code` on this host.
