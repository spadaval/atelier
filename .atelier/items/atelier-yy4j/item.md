---
schema: atelier.item
schema_version: 3
id: atelier-yy4j
title: Delete the Desk's three T3-only pages once the release that draws them natively is installed
status: draft
mission: atelier-osel
labels:
- needs-human
priority: medium
blocked_by:
- atelier-ut89
- atelier-9l8h
- atelier-wvst
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-17T19:28:10.550247610Z
---

## Goal

Leave one drawing of each view. Once the extension draws the mission page,
This thread and the project home itself, the Desk's copies are dead code and
the messages only they used are dead contract.

## Done when

- [ ] An extension release that holds the three native pages exists, and the human has said their installs run it. The Customizations section shows the installed release. This line is why the item is labelled needs-human.
- [ ] `desk/src/pages/MissionControl.tsx`, `ThisThread.tsx` and `ProjectHome.tsx` are deleted with their routes, styles and tests, and with every hook, read and route of `crates/atelier-desk` that only they used.
- [ ] The host contract loses what only those pages used: `desk:resized`, and the fields of `host:thread` that fed This thread, or the whole message if nothing else reads it. The extension stops sending them.
- [ ] The Desk answers `/embed/mission/...`, `/embed/t` and `/embed/home/...` as it answers an unknown page.
- [ ] An item's page and the inbox still open framed in T3's panel, with the host's theme and with thread links that open in T3. `pnpm e2e` and `node run-tests.mjs` pass.
- [ ] `docs/product/desk.md` and `docs/architecture/desk.md` say that no page of the Desk is made only for T3 Code, and list the contract's messages that remain.

## Scope

In: `desk/`, `crates/atelier-desk`, `t3-extension/`, `docs/product/desk.md`, `docs/architecture/desk.md`.

Out: the Desk's other embedded pages and the rest of the host contract.

## Context

An installed extension older than the native pages frames these three Desk pages, and This thread is beside every thread. Deleting them before the installs are updated would blank that panel, so the lead cuts a release once atelier-ut89, atelier-9l8h and atelier-wvst have merged (the extension's `README.md`, "Updates", says how), and this item waits for the human to install it.
