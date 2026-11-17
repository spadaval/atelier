---
schema: atelier.item
schema_version: 3
id: atelier-3dip
title: Build the T3 extension from TypeScript sources with Vite, with its behaviour unchanged
status: in_review
mission: atelier-osel
labels: []
priority: high
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A899532db-24ca-47d2-b870-5d183ff33c40%3Adelegate-task%3A18f1460f-3960-46d4-b8ca-de072c2398a1
ready:
  by: 'sumeet, in conversation on 2026-11-17: looks good, start up a lead'
  at: 2026-11-17T21:31:50.834606783Z
trivial: null
caused_by: null
created_at: 2026-11-17T19:24:26.038773289Z
---

## Goal

Give the extension a build, so the items after this one can write TypeScript
modules instead of adding to one 89 KB script. Nothing a user sees changes.

## Done when

- [x] `t3-extension/` has a `package.json`, a strict `tsconfig.json` and a Vite 8 configuration that builds `src/` into one classic script. The entry imports today's `custom.js` and `host-adapter.js`, moved under `src/` with their behaviour unchanged; they may stay JavaScript in this item.
- [x] The loader injects the one built script, which holds the adapter, and no longer reads and joins `host-adapter.js` itself. `install.py`, the updater and `test-update.mjs` name the files a build installs. `t3-custom.mjs` stays plain `.mjs` and still reloads when the built script, `custom.css` or `config.json` changes, so `vite build --watch` gives the edit-and-see loop `README.md` describes.
- [x] `install.py` installs the built files from a checkout after `pnpm build`, and refuses with the command to run when there is no build.
- [x] `scripts/t3-extension-release.sh bundle` builds before it zips, the bundle holds the built script and not `src/` or `node_modules/`, and `verify` installs it with no Node on the path but the stand-in for T3's executable. `scripts/t3-extension-release.test.sh` covers both.
- [x] CI's extension job installs with a frozen lockfile, runs `pnpm typecheck` and `pnpm build`, then `node run-tests.mjs` against the built script. `scripts/ci-jobs.sh` still runs the job for the same paths.
- [x] `node run-tests.mjs` passes. The only assertions changed are those that name the installed files.
- [x] `t3-extension/README.md` and `AGENTS.md` say how to build, watch and install from a checkout.

## Scope

In: `t3-extension/`, `scripts/t3-extension-release.sh` and its test, `.github/workflows/ci.yml`, `.github/workflows/t3-extension-release.yml`, `scripts/ci-jobs.sh`.

Out: rewriting any of the moved code; React; any change to the Desk.

## Context

The spike's `vite.config.ts` on `spike/extension-react` builds a library in `iife` format with `process.env.NODE_ENV` defined. Today the release bundle is `t3-extension/` at HEAD and `verify` compares the two file by file, which a built bundle no longer satisfies.

## Report

The extension now builds from `t3-extension/src/` with Vite 8 into one
classic script, `dist/custom.js`, which installs as `custom.js`. Later items
can add TypeScript modules under `src/` instead of growing one script.

**What moved, and the one change it forced.** `custom.js` and
`host-adapter.js` moved to `src/` as modules (git sees both as renames).
`custom.js` became `export async function start(api)` with its body left
unindented so its history follows it; the adapter's top-level declarations
gained `export`. As joined scripts they shared one function scope, and
`setHostProbe` read `api` from it: as a module that would have been a
ReferenceError. The adapter now holds `api` in a module variable set by
`useHostApi(api)`, which `start` calls first. I found it with
`tsc --checkJs`, which reported exactly one undeclared name in the two files;
`--checkJs` also showed no assignment to an import and no `this` at top
level, the other ways strict-mode modules differ.

**The loader** (`t3-custom.mjs`) reads `custom.js` and `custom.css` only,
runs the script inside the same `async function (api)` as before and returns
`T3Custom.start(api)`. A `custom.js` that defines no `T3Custom.start`, such
as an old unbuilt one, fails with an error naming the file and `pnpm build`.
The watcher no longer lists `host-adapter.js`. The build is not minified, so
injected stacks read as written.

**Install and release.** `install.py` installs `dist/custom.js` and refuses,
naming `pnpm install && pnpm build`, when it is missing or, in a checkout,
older than a file in `src/`. It removes a `host-adapter.js` an older install
left. `bundle` builds HEAD's tree with `pnpm install --frozen-lockfile` and
`pnpm build` and zips everything but the build's inputs (`src/`,
`package.json`, `pnpm-lock.yaml`, `tsconfig.json`, `vite.config.ts`) plus
`dist/custom.js`. `verify` compares the bundle against the same list, checks
`dist/` holds only `custom.js`, and runs `install.py` and `t3-custom check`
with `PATH` set to an empty directory; the stand-in for T3's executable names
Node by its absolute path.

**Edit-and-see loop.** `pnpm watch --outDir "<install folder>"` rebuilds into
an installed copy on each save and the injector reapplies it. Checked by
hand: a touch of `src/main.ts` rebuilt into a scratch folder, keeping its
`config.json` (`emptyOutDir: false`).

**CI.** The `extension` job installs `t3-extension/` from its lockfile, runs
`pnpm typecheck` and `pnpm build`, then `node run-tests.mjs`.
`scripts/ci-jobs.sh` is unchanged. The `changes` job now sets up Node and
pnpm, because `t3-extension-release.test.sh`, which it runs, builds a real
bundle. That adds the setup and one cached `pnpm install` to every CI run. The
release workflow sets up Node and pnpm before `bundle`.

**Tests.** The only changed assertion is the installed-file list in
`test-update.mjs`, which drops `host-adapter.js`. Setup changed where it named
the files: `test-extension.mjs` wraps `T3Custom.start` instead of
`createHostAdapter` to expose the api, `test-update.mjs` puts the build in its
bundles, and `test-contract.mjs` inlines `src/host-adapter.js` with `export`
stripped. New checks: `install.py` with no build refuses and writes nothing;
an old `host-adapter.js` is removed; the release test bundles this checkout's
extension, checks the contents, runs `verify`, and checks that `verify`
refuses a bundle that carries `src/`.

Run here: `pnpm typecheck` and `pnpm build`; `node run-tests.mjs` ("All 3
tests passed: test-extension.mjs, test-fixture-desk.mjs, test-update.mjs");
`scripts/t3-extension-release.test.sh` (all 16 ok);
`scripts/ci-jobs.test.sh`; `git diff --check`.

Not verified: `test-contract.mjs` against a real T3 (it needs one; I checked
only that the inlined adapter still parses), the CI and release workflows
themselves (they run on the pull request), and Windows or macOS. `pnpm
typecheck` checks only `src/main.ts` and `vite.config.ts`: the moved
JavaScript gives 389 errors under `--checkJs`, nearly all implicit `any`,
which belong to the items that convert it.
