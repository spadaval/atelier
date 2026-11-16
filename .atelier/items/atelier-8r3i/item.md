---
schema: atelier.item
schema_version: 3
id: atelier-8r3i
title: Let the extension's updater find its release past the first 100 releases
status: draft
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-16T21:00:22.042004153Z
---

## Goal

Keep a device able to find a newer extension release however many `atelier` releases come between. `t3-custom update` reads one page of 100 releases from GitHub, so once 100 other releases follow the newest `t3-extension-v*` one, a device on an older extension sees none. A reviewer of atelier-wlyn deferred this.

## Done when

- [ ] The updater follows GitHub's pages until it has seen an extension release newer than the installed one or has read every release, with a bound on the pages it reads.
- [ ] A test in `t3-extension/test-update.mjs` serves more than 100 releases with the newest extension release on the second page, and the updater finds it.

## Scope

In: `t3-extension/`.
