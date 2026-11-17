---
schema: atelier.item
schema_version: 3
id: atelier-3dip
title: Build the T3 extension from TypeScript sources with Vite, with its behaviour unchanged
status: draft
mission: atelier-osel
labels: []
priority: high
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-17T19:24:26.038773289Z
---

## Goal

Give the extension a build, so the items after this one can write TypeScript
modules instead of adding to one 89 KB script. Nothing a user sees changes.

## Done when

- [ ] `t3-extension/` has a `package.json`, a strict `tsconfig.json` and a Vite 8 configuration that builds `src/` into one classic script. The entry imports today's `custom.js` and `host-adapter.js`, moved under `src/` with their behaviour unchanged; they may stay JavaScript in this item.
- [ ] The loader injects the built script. `t3-custom.mjs` stays plain `.mjs` and still reloads when the built script, `custom.css` or `config.json` changes, so `vite build --watch` gives the edit-and-see loop `README.md` describes.
- [ ] `install.py` installs the built files from a checkout after `pnpm build`, and refuses with the command to run when there is no build.
- [ ] `scripts/t3-extension-release.sh bundle` builds before it zips, the bundle holds the built script and not `src/` or `node_modules/`, and `verify` installs it with no Node on the path but the stand-in for T3's executable. `scripts/t3-extension-release.test.sh` covers both.
- [ ] CI's extension job installs with a frozen lockfile, runs `pnpm typecheck` and `pnpm build`, then `node run-tests.mjs` against the built script. `scripts/ci-jobs.sh` still runs the job for the same paths.
- [ ] `node run-tests.mjs` passes with no change to what any test asserts.
- [ ] `t3-extension/README.md` and `AGENTS.md` say how to build, watch and install from a checkout.

## Scope

In: `t3-extension/`, `scripts/t3-extension-release.sh` and its test, `.github/workflows/ci.yml`, `.github/workflows/t3-extension-release.yml`, `scripts/ci-jobs.sh`.

Out: rewriting any of the moved code; React; any change to the Desk.

## Context

The spike's `vite.config.ts` on `spike/extension-react` builds a library in `iife` format with `process.env.NODE_ENV` defined. Today the release bundle is `t3-extension/` at HEAD and `verify` compares the two file by file, which a built bundle no longer satisfies.
