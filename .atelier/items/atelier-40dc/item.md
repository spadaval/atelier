---
schema: atelier.item
schema_version: 3
id: atelier-40dc
title: Rewrite the sidebar missions, the Inbox row, the project strip and the layer as typed modules, and delete the old script
status: draft
mission: atelier-osel
labels: []
priority: medium
blocked_by:
- atelier-toee
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-17T19:24:26.326415300Z
---

## Goal

Finish the rewrite, so the extension is one typed codebase and the old
script is deleted.

## Done when

- [ ] These are TypeScript modules under `t3-extension/src`: missions in T3's thread list with their folding and their rows; the Inbox row and its count; the project strip under the new-thread composer; and the layer over T3's main area that holds a native page or a framed one.
- [ ] The project strip is React, drawn with the copied T3 components. Rows placed inside T3's own list stay direct DOM code.
- [ ] The repository holds no hand-written `custom.js` or `host-adapter.js`, and `pnpm typecheck` passes with `strict` on and no file excluded.
- [ ] Each fact about T3's behaviour in the old code's comments for these parts is kept beside the code that handles it: the copy T3 appends when it fades a removed row, the range gestures and their platform rule, the folder's `aria-expanded`, and timers throttled in a covered window. The Report lists them.
- [ ] The checks in `test-extension.mjs` for these parts pass, unchanged in what they assert.
- [ ] `test-contract.mjs` passes against T3 Code Nightly on Legend's test instance, and the sidebar, the strip and the layer are seen working there.

## Scope

In: `t3-extension/`.

Out: This thread and the project home, which stay framed until atelier-9l8h and atelier-wvst.
