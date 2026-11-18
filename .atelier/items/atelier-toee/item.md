---
schema: atelier.item
schema_version: 3
id: atelier-toee
title: Rewrite the panel, the Desk tab, record links and the settings section as typed modules
status: in_review
mission: atelier-osel
labels: []
priority: medium
blocked_by:
- atelier-ut89
implementers:
- thread:delegated-task:command%3Amcp%3A30c2d36e-4497-4d4b-a81a-7ea55361424d%3Adelegate-task%3A3276c8f6-31a6-4cee-bf40-6a84a5785d84
ready:
  by: 'sumeet, in conversation on 2026-11-17: looks good, start up a lead'
  at: 2026-11-17T21:31:50.834606783Z
trivial: null
caused_by: null
created_at: 2026-11-17T19:24:26.258637572Z
---

## Goal

Turn the first half of the old script into typed modules, so a mistake in a
T3 hook or a Desk field is a type error and not a blank panel.

## Done when

- [x] The host adapter is TypeScript, with a type for everything it gives the extension: the project and thread context, the panel's state, the probes and selectors.
- [x] These are TypeScript modules under `t3-extension/src`, no longer in the old script: the repositories and their mapping to T3 projects; the Desk panel with its frame and what remains of the host contract; the Desk tab and its controls; record links in a conversation with their hover card; and the Customizations section.
- [x] The hover card and the Customizations section are React, drawn with the copied T3 components. The Desk tab and the record links, which sit inside DOM that T3 owns, stay direct DOM code.
- [x] Each fact about T3's behaviour in the old code's comments for these parts is kept in a comment beside the code that handles it, or the Report says why it was dropped.
- [x] The Desk's API types come from `desk/src/api`, not from hand-written shapes.
- [x] The checks in `test-extension.mjs` for these parts pass, unchanged in what they assert.
- [x] These parts are seen working in T3 Code Nightly on the test instance on this host (atelier-diru), with what was checked in the Report.

## Scope

In: `t3-extension/`.

Out: the sidebar, the Inbox row, the project strip and the layer (atelier-40dc); any change to what a user sees beyond the hover card and settings taking T3's components.

## Report

The Desk panel and its tab, record links and their card, and the
Customizations section are now typed modules under `t3-extension/src`, and
`src/host-adapter.js` is gone. `pnpm typecheck` passes with `strict`, and
`node run-tests.mjs` passed all 5 tests. In T3 Code Nightly on this host, the
contract check passed every hook and each part worked in light and dark.

**What changed.**

- `src/host/adapter.ts` is the adapter in TypeScript. It has a type for each
  part of T3 it reads (the panel store, the snapshot's projects and threads,
  the draft store), for what it gives (`ThreadContext`, `DraftContext`,
  `ThreadEntry`, `HostPanel`, `HostAdapter`), and for every probe name
  (`ProbeName`) and selector (`HostSelector`). `src/host/api.ts` types the
  loader's `api`, the settings, and each injector request with its answer.
- `src/core/extension.ts` holds what the parts share: settings, the thread T3
  shows, the hooks once found, one DOM pass each part adds its sync to, and
  one MutationObserver. `src/core/dom.ts` has `element` and the icons.
- `src/repositories.ts` reads the Desks' repositories and maps T3 projects to
  them (`Repositories.mapped`, `selected`, `override`).
- `src/panel/panel.ts` is the panel, its frame and the host contract.
  `src/panel/tab.ts` is the Desk tab, its controls and the launch button.
  Both stay DOM code inside T3's tab strip.
- `src/references/references.tsx` scans the conversation and lays links over
  the IDs, still DOM code over T3's text. `HoverCard.tsx` is the card, in
  T3's `PreviewCard`, with T3's `Button`.
- `src/settings/` is the Customizations section in React. It is drawn as T3
  draws its own settings (a titled group of rows, as in
  `settingsLayout.tsx`), with T3's `Button`, `Badge`, `Switch` and `Select`.
- `src/views.ts` holds the view table, which Customizations and `custom.js`
  both read. `src/custom.js` keeps the sidebar, the Inbox row, the layer, the
  strip and the project home for atelier-40dc, as `startViews(ext, { panel })`
  run by the typed core. Its body is unchanged except where it reads the
  shared state.
- The Desk's reads use `desk/src/api` types: `ReposResponse`, `RecordName`,
  `ItemPage` and `MissionPage`. The host contract's message types are
  restated from `desk/src/lib/host.ts` in `panel.ts`, since that file imports
  React and the Desk's own modules, which the release bundle does not have.

**Decisions within the mission's Authority.**

- Three more T3 components are copied: `switch`, `select` and
  `preview-card`, each used by the section or the card. `sync-t3.mjs` points
  their portals into the shadow root, as for the tooltip, and gives
  `PreviewCardPopup` one prop, `anchor`, which T3's `SelectPopup` already
  has. The card is placed at an ID in T3's conversation, a Range React does
  not draw. `check/Probe.tsx` draws all three, and `test-components.mjs`
  passes in light and dark (142 elements).
- `mountReact` can draw into an existing shadow root. The card is drawn in
  the extension's root, beside the links, so `#atelier-hovercard` is where
  it was.
- `main.ts` exports the adapter as `T3Custom.adapter`. `test-contract.mjs`
  now runs the built `dist/custom.js` adapter instead of evaluating
  `src/host-adapter.js` as text, so it checks the code that ships.

**What a user sees differently.** Only the card and the section changed, as
the item allows. The card is T3's preview card, with T3's buttons. The section
is grouped rows with switches and selects in place of checkboxes and native
selects; its text and controls are the same. The release Badge is part of the
section's tree, not a root of its own.

**The checks in `test-extension.mjs`.** What they assert is unchanged. A few
lines changed how they reach a control:

- A switch's state is read from `aria-checked`, since T3's `Switch` is a
  `role="switch"` span, not a checkbox.
- The project mapping is chosen by opening the `Select` (mousedown) and
  clicking the item, not by setting a native select's value.
- The release Badge is found at `#release-badge [data-slot=badge]` in the
  section's shadow root. The reinjection check takes that shadow root to see
  the old tree unmounted.

One check was vacuous, so it is fixed in its own commit (1bcfd3b8f). "Reference
in unmapped project" focused a link that was still focused, so no card opened.
It passed on the old card, which kept the previous record's `h3` and buttons
in the DOM while hidden. I confirmed this by running the old code with the
card's state printed. The new card unmounts when it closes, so the check now
blurs the link first, as the checks around it do.

**T3 facts in the old comments.** All are kept beside the code that handles
them:

- Electron suspends animation frames in a hidden window: `scheduleDOM`.
- A covered window throttles timers: `main.ts`, `views.refresh`.
- T3 shows a cached list first, then the thread's row: the adapter's
  `context`.
- Discovery imports only renderer modules, never workers, and chunk names
  only order the search.
- T3 records the editor's text a few microtasks later, and its composer is
  ProseMirror (`writeHostComposer`).
- T3 switches theme on its root element's class and style attributes.
- A re-rendered panel slot reloads the moved iframe, so each `desk:ready` is a
  fresh Desk.
- Children appear when a thread starts one, read every 3 seconds.
- The controls go after T3's scrolling tab list, so a narrow panel never clips
  them.
- T3 marks assistant messages with their thread, and may show the previous
  thread's conversation after a switch.
- A closed folder's `aria-expanded` changes with no other mutation: the core
  observer.

Nothing was dropped. One comment was reworded: "the injector then reapplies
the extension" became "applies a change of settings alone in place", which
`test-extension.mjs` checks.

**Checks run.**

- `pnpm typecheck` and `pnpm build` pass.
- `node run-tests.mjs` passed `test-components`, `test-extension`,
  `test-fixture-desk`, `test-mission` and `test-update`, with
  `TMPDIR=~/.cache/atelier-tmp` and a Desk built from this branch.

**Seen in T3 Code.** atelier-diru is approved, not merged. I ran its
`t3-instance.mjs` and `fixture-desk.mjs` from `origin/atelier-diru` over this
branch in a scratch worktree, now removed. It used `T3_TEST_ROOT` under
`/var/tmp` and ports 9336 and 47336, in T3 Code Nightly
`v0.1.11-nightly.20261117.4851`.

- `test-contract.mjs` passed every probe with the built adapter: 22 DOM hooks,
  5 modules, both routes, `css:theme`, `layout:threadList`,
  `state:thread-origin`, `state:find-thread`, `state:composer-prompt` and
  `discovery:imports` (88 of 324 modules).
- The launch button opened the Desk in T3's panel, beside T3's own tabs. The
  frame got the theme and `host:thread`, with the thread, `repository:
  atelier` and the mentioned ID, and showed This thread. The tab read "This
  thread · Desk", selected, with its controls, and Open externally pointed at
  `/t`.
- A line naming `atelier-rf01`, added to T3's conversation, became a link.
  Focused, it opened the card with the title, status line, checks, goal and
  read-from line, flipped above the ID near the window's bottom. Open in Desk
  opened the item in the panel, and the tab read "Fixture reference item ·
  Desk".
- The tab's Customizations control went to Settings → General. The section
  showed the injector, the release Badge and the offered update, the Desk
  reachable, all four switches, and "Automatic: atelier". The project Select
  opened its list in the section.
- Screenshots in `artifacts/`: `t3-panel-card-light.png`,
  `t3-panel-card-dark.png`, `t3-panel-record-light.png`,
  `t3-panel-record-dark.png`, `t3-settings-light.png`,
  `t3-settings-dark.png` and `t3-settings-select-light.png`.

**For the lead: atelier-diru's fixture Desk lists no embed origins.** T3 Code
Nightly's page origin is `t3code://app`, and the instance's Desk answers
`GET /api/embed` with `{"origins":[]}`. So no Desk frame in the instance ever
gets `desk:ready`: not the panel, not the layer's inbox or item pages. This
happens with the old code too. To see the panel connect, I ran a second Desk
over the same fixture with `embed_origins = ["t3code://app"]` and pointed the
instance's injector at it. Adding that line to the instance's `desk.toml`
would fix it for every later item.

### Responses

- 1-0ypk.1: fixed in 295604369. Disposal drops the open card without drawing it after its root unmounted, and `Extension` runs each part's cleanup on its own, so one failure no longer skips the rest. `test-extension.mjs` now stops the extension with a card open and checks that everything goes; that check fails on the previous code ("Full extension removal timed out").
- 1-0ypk.2: fixed in 295604369. The card reads the record's address when it draws, after resolution has set its Desk and kind. `test-extension.mjs` opens the card of `atelier-fm02`, a mission only the second Desk serves, before anything resolved it, and checks its Open externally link is `<second Desk>/r/atelier/mission/atelier-fm02`; that check fails on the previous code.
