---
schema: atelier.item
schema_version: 3
id: atelier-toee
title: Rewrite the panel, the Desk tab, record links and the settings section as typed modules
status: building
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

- [ ] The host adapter is TypeScript, with a type for everything it gives the extension: the project and thread context, the panel's state, the probes and selectors.
- [ ] These are TypeScript modules under `t3-extension/src`, no longer in the old script: the repositories and their mapping to T3 projects; the Desk panel with its frame and what remains of the host contract; the Desk tab and its controls; record links in a conversation with their hover card; and the Customizations section.
- [ ] The hover card and the Customizations section are React, drawn with the copied T3 components. The Desk tab and the record links, which sit inside DOM that T3 owns, stay direct DOM code.
- [ ] Each fact about T3's behaviour in the old code's comments for these parts is kept in a comment beside the code that handles it, or the Report says why it was dropped.
- [ ] The Desk's API types come from `desk/src/api`, not from hand-written shapes.
- [ ] The checks in `test-extension.mjs` for these parts pass, unchanged in what they assert.
- [ ] These parts are seen working in T3 Code Nightly on the test instance on this host (atelier-diru), with what was checked in the Report.

## Scope

In: `t3-extension/`.

Out: the sidebar, the Inbox row, the project strip and the layer (atelier-40dc); any change to what a user sees beyond the hover card and settings taking T3's components.
