---
schema: atelier.item
schema_version: 3
id: atelier-n4g7
title: Move the T3 Code extension into this repository with its history
status: ready
mission: atelier-m8ye
labels: []
priority: medium
blocked_by: []
implementers: []
ready:
  by: 'sumeet, in conversation on 2026-11-16: do this, and the rest of the mission'
  at: 2026-11-16T19:26:24.749789035Z
trivial: null
caused_by: null
created_at: 2026-11-16T19:25:26.767907122Z
---

## Goal

Put the extension's code where the Desk it depends on is changed, so one item
and one review cover both sides of their contract. It lives in
`github.com/spadaval/t3-code-atelier`, and every change to it so far also
changed the Desk here.

## Done when

- [ ] The extension's files are under `t3-extension/`, brought in with their history from `spadaval/t3-code-atelier` at `a242410` (branch `main`), so `git log --follow t3-extension/custom.js` reaches its first commit. None of that repository's tags comes with it: fetch with `--no-tags`, since its `v0.1.0` matches the release workflow's tag pattern.
- [ ] The tree under `t3-extension/` equals that commit's tree apart from line endings and trailing whitespace made consistent, and the Report gives the command a reviewer runs to see that. `git diff --check` passes on the merged tree.
- [ ] `scripts/ci-jobs.sh` does not run the `rust` or `desk` job for a change under `t3-extension/` alone, and `scripts/ci-jobs.test.sh` covers it.
- [ ] `node --check` passes on each script, and `node t3-extension/test-extension.mjs` passes on this host with `BROWSER=/usr/bin/google-chrome` and `DESK=http://127.0.0.1:3780`, run from its new place.
- [ ] `AGENTS.md` names `t3-extension/` and the command that tests it, the extension's own `AGENTS.md` is folded into its `README.md` or kept as a file the root one links, and its README no longer speaks of a separate repository or of committing to `main`.
- [ ] `docs/design.md` records the decision in its Decisions table: the extension is in this repository and outside the `atelier` release, as the Desk is (ADR 0032), and `atelier` still has no Node step in its build or release.

## Scope

In: `t3-extension/`, `scripts/ci-jobs.sh` and its test, `AGENTS.md`, `README.md`, `docs/design.md`.

Out: a CI job for the extension (atelier-7tg6); its release (atelier-451s); how it installs and updates (atelier-wlyn); any change to what the extension does.

## Context

A clone of the extension is at `/tmp/claude-1000/-home-supadava-projects-atelier/efd339ea-b573-40a9-99be-465d565b2949/scratchpad/repo` if it still exists; otherwise clone `git@github.com:spadaval/t3-code-atelier.git`. It was last edited on Windows, and its installed copies there differed from the commit in line endings. `needs_rust` in `scripts/ci-jobs.sh` ends in `return 0`, so a new top-level folder runs the Rust job until it is named there.
