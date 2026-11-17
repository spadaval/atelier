---
schema: atelier.mission
schema_version: 3
id: atelier-osel
title: 'Draw the T3 extension''s pages natively: a TypeScript and React rewrite, and a mission page that says what needs you'
status: open
labels: []
lead: null
planners:
- 420b2fcd-ed0d-4cda-89ab-12f168f7808e
created_at: 2026-11-17T19:23:17.709726942Z
---

## Why

Sumeet reads Atelier inside T3 Code through the extension in `t3-extension/`.
On 2026-11-17, in conversation, he said three things about it and directed
this mission.

- **Mission control does not fit T3 Code and wastes its space.** It is a
  terminal-style screen of bordered panes after Factory's Mission Control,
  in monospace, inside an app that has its own design system. Its progress
  log is "a generalized firehose" that is "not all that useful". A mission
  with 26 items gives the live ones a pane 240 pixels tall, and a finished
  mission shows an "Active Item" that merged days ago and an empty threads
  pane.
- **Embedding the Desk has run its course.** A framed page can copy T3's
  colours but not use its components, needs a message contract for its
  size, keys and theme, and reads back through the Desk what the T3 window
  already knows.
- **The extension has outgrown one script.** `custom.js` is 1,177 lines and
  89 KB of hand-built DOM with no types and no build. "No serious project
  has ever been done in one giant vanilla JS file."

He settled what a person opening a mission wants. While it runs: does it
need me, and is it getting where I wanted. Once it is done: what did I get,
and what is worth a look. A proposed plan is not shown in T3's sidebar and is
reviewed beside its planning thread, so the page does not serve that case.

Two things were built with him before this plan.

- **Mockups**, in `t3-extension/spike/mockups/` on the branch
  `spike/extension-react`. `mission-concepts.html` holds three concepts for
  a running mission and a retro. The planner recommended concept A ("the
  briefing") with concept B's timeline as the main part of the retro. Asked
  which the plan should build, he chose that on 2026-11-17.
- **A build spike**, `t3-extension/spike/` on the same branch, which never
  merges. It showed that Vite 8 builds a React 19 root with T3 Code's own
  `Badge` and `Kbd` (copied from its MIT source) into one classic script
  with its stylesheet inside, as the loader injects today, and that in a
  shadow root each element's computed style equals the same element drawn
  by T3's `index.css`, in light and dark. It found three things an
  implementer needs: T3's `dark:` variant must become `:host-context(.dark)`
  inside a shadow root; T3's default border colour comes from its base
  layer and must be copied; and Tailwind's `@property` rules do nothing in
  a shadow root, so they are also registered on the document. A Base UI
  tooltip portalled into the shadow root is drawn and themed there. It did
  not run inside T3 Code, and tried no menu, popover or dialog.

## Outcome

The extension is a TypeScript project built by Vite into the one script the
loader injects. Its pages are React, drawn with T3 Code's own components and
theme, and its changes to T3's own DOM are typed modules. `custom.js` and
`host-adapter.js` are gone as hand-written files. The loader
(`t3-custom.mjs`) stays plain JavaScript.

Three views are drawn by the extension itself, with no frame: the mission
page, This thread and the project home. The Desk no longer has those three
pages or the messages only they used. The Desk's other pages, such as an
item's page and the inbox, still open in a frame.

The mission page replaces mission control. For a running mission it says in
one sentence how the mission stands, lists what needs the human with the
button that deals with each, shows the threads at work with their latest
line, and folds everything else. For a finished mission it shows the result,
a lane for each item over time, and the merged items with the most
reviewed first. It has no progress log.

## Success criteria

- one-built-script: `pnpm build` in `t3-extension/` writes the script the
  loader injects, `pnpm typecheck` passes with `strict` on, and the
  repository holds no hand-written `custom.js` or `host-adapter.js`.
- behaviour-kept: the checks in `t3-extension/test-extension.mjs` that are
  about a view this mission does not replace pass against the built script,
  unchanged in what they assert.
- drawn-as-t3-draws: a check in CI draws each T3 component the extension
  copies in a shadow root and beside it under T3's stylesheet as last
  copied, and their computed styles are equal in light and dark.
- needs-you-first: for a fixture mission with an item labelled needs-human,
  a thread that began waiting on a person a minute ago, a live thread that
  has not changed in 30 minutes and two live threads that have, the mission page
  shows a sentence naming those facts, a row for each of the first three
  with a button that opens the item or the thread, and a line per live
  thread; its drafts and merged items are each one folded line; and it has
  no list of the mission's steps.
- retro: for a fixture mission that is done, the page shows its Result, one
  lane per item with its building and review spans, and its merged items
  ordered by review rounds, each opening its pull request where the Desk
  gives one and its Desk page where it gives none.
- no-frame: the mission page, This thread and the project home are drawn
  without an `iframe`, and the Desk answers its `/embed` routes for mission
  control, This thread and the project home as it answers an unknown page.
- frames-kept: an item's page and the inbox still open framed in T3's panel
  from a record link and from the Inbox row.
- seen-in-t3: each of the three pages is seen in T3 Code Nightly on Legend's
  test instance, in light and dark, with a screenshot in its item's Report.
- installs-without-node: a release bundle built by
  `scripts/t3-extension-release.sh` holds the built script, installs with
  `install.py` on a machine with no Node, and reports its version.

## Scope

In: `t3-extension/`, `scripts/t3-extension-release.sh` and its test, the
extension's jobs in `.github/workflows/`, `scripts/ci-jobs.sh`; in `desk/`
the three pages that move and what only they use, and the files the
extension imports from it; `crates/atelier-desk` where a route or a read
only those pages used is removed; `docs/product/desk.md`,
`docs/architecture/desk.md` and the extension's `README.md` and `AGENTS.md`.

Out:

- The Desk's other embedded pages (an item, a mission's record page, the
  inbox, proposed, disputed, in motion, agents). They stay framed.
- The Desk outside its embedded mode. It looks and works as it does today.
- The loader's language and how it injects. `t3-custom.mjs` stays `.mjs`.
- What a lead writes in a mission's Result. The mockup's "Differed from the
  plan" and "Not seen working" sections need headings the skill does not
  ask for; the page shows the Result as it is written.
- Any new write or action from a page. A button opens a thread, an item's
  page or a pull request.
- Finding a merged item's pull request once its branch is deleted or it has
  left the forge's recent list. The retro links the ones the Desk gives.
- T3 Code on a phone, where the extension does not run.

## Authority

An implementer may decide, and records in its Report: the layout of
`t3-extension/src`; component and hook names; which T3 components to copy,
kept to those a page it builds uses; whether the extension reads the Desk's
types and pure functions through a pnpm workspace or a path; the wording of
the page's one-sentence summary; and how a native page and a framed page
share the panel or the layer, so long as a framed page has a way back.

atelier-6dm7 opens the loader's read gate to seven existing `GET` reads of
the Desk, named in that item, which is the one change this mission makes to
what the loader lets the extension do.

An implementer asks the thread that started it before: opening the gate to
any other read; adding a dependency
other than React, React DOM, Base UI, class-variance-authority,
tailwind-merge, Lucide's icons, Tailwind, Vite and their plugins; changing
how the loader, the installer or the updater talk to the extension; adding
a read or a write to the Desk's API; showing something on the mission page that
its mockup does not, or leaving out something it shows; showing something
on This thread or the project home that the Desk's page does not show
today, or leaving out something it shows; or changing how a view
this mission does not replace behaves.

The extension's code has learned things about T3 Code from the running app
that no CI check covers, written in its comments: a removed row fades by a
copy appended to the list, range selection differs by platform, a covered
window throttles timers. A rewrite carries each of these over on purpose.
An implementer who drops one says why in its Report.

Five items are seen in T3 Code on Legend's test instance. An implementer
who cannot reach Legend, or finds nobody logged on there to start the test
instance, ticks nothing it did not see and asks the thread that started it.

The Desk's three pages stay until the last item, atelier-yy4j, which waits for the
human to install the release that replaces them. Until then both drawings
exist, and the extension opens only its own.

The spike's code is reference. Items write fresh under `t3-extension/src`
and may copy from `t3-extension/spike/` on `spike/extension-react`.

## Result
