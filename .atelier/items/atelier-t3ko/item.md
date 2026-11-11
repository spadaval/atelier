---
schema: atelier.item
schema_version: 3
id: atelier-t3ko
title: Make a re-run of init say what it did, and init -q print the base branch
status: dropped
mission: null
labels:
- cli-output
- setup
- bug
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-17T01:10:57.979319063Z
---

## Goal

atelier-l457 made `atelier init`'s first run print the base branch it chose and the install's real next steps: read the generated policy, commit the install on the base, create the first issue. Two parts of the original report remain (checked 2026-11-07):
- A re-run on an initialized repository prints `Atelier initialized successfully!` again, with the old footer (`atelier check`, `atelier issue create`) rather than the install's steps, even when nothing changed.
- `atelier init -q` prints only the project slug; the base branch init chose is the other fact a script needs.

## Done when

- [ ] A re-run that changes nothing prints `Already initialized; nothing changed` (or lists what it reconciled) and, while the install is not yet committed, the same commit step a first run prints; it never prints `initialized successfully`.
- [ ] `atelier init -q` prints the project slug and the base branch, one per line.
- [ ] CLI tests cover a first run, a re-run with nothing changed, a re-run before the install is committed, and `-q`.

## Scope

crates/atelier-cli/src/commands/init.rs and crates/atelier-app/src/init.rs. Not the empty record directories missing from the Created list, and not a `setup claude` step, which is specific to Claude Code.

## Context

atelier-l457 (done) left the re-run footer to this item (its Decisions). The rest of the original, the install's next steps, is l457's (decision atelier-mzpn).

## Report

Dropped: `atelier init` was removed in 0.5.
