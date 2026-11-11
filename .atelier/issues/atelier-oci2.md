---
created_at: "2026-11-10T00:56:08.595125805+00:00"
id: "atelier-oci2"
issue_type: "task"
labels:
- "redesign"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-mlwx"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-nafc"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-qr5s"
    type: "advances"
  - kind: "issue"
    id: "atelier-yptz"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Take the site out of the new workspace: no serve, no web build, and the old site goes at the switch"
updated_at: "2026-11-11T02:46:34.061046172+00:00"
---

## Goal

Take the tracker's site off the cutover's path. The human's pages become a product of their own, the Desk (ADR 0032, mission atelier-z4cr), so the `atelier` binary of the new workspace has no site and no `serve`, and the old site is deleted with the old workspace at the switch instead of being ported.

Mission 2 never copied the site into `next/`: the new workspace has four crates and none is `atelier-site`. So this item builds nothing. It proves the new binary has no site, and it corrects the two specifications, which still tell the switch and the cutover to rebuild, restart and check a site. Step F's runbook is written from those sections, so they are corrected before it is planned.

## Done when

- [ ] Proved on a binary built from this branch's `next/`, with the commands and their output in the Report: `next/Cargo.toml` lists no `atelier-site` member and `cargo metadata` shows none; `atelier --help` lists no `serve`; no file under `next/` has a build script or a dependency that runs Node or pnpm; and `cargo build --locked` in `next/` succeeds with `node` and `pnpm` absent from `PATH`. The CI job `next` already builds without a Node step, and the Report cites it.
- [ ] `docs/strategies/redesign-product.md` no longer gives the new tool a site: the Commands heading line does not count `serve`; the removed-commands row for `brief` and `debrief` says a mission is read with `atelier show <mission>` until the Desk has its page; the Merge section's sentence on an approved item names `status` and `show` and no site; and the table of four missions says mission 3 takes the site out. Its Words, Lifecycle and Review sections are not changed.
- [ ] `docs/strategies/redesign-architecture.md` no longer gives the new workspace a site: the crate table and the dependency direction have no `atelier-site`; the rows of "Where the code is today" that say a piece is copied "at the site step" say it is not carried over; the Site section says in a few sentences that the tracker has no site, the old one is deleted with the old workspace, and the Desk replaces it in its own mission; the sentences that name the site as a reader of records or of the log's data either drop it or name the Desk; and the open question about brief and debrief view code is removed.
- [ ] The Transition section of the architecture specification matches: part 3 has no "the site is rebuilt" bullet; the fallback release still has to serve the old site, since a rollback restores it; the cutover's step 3 stops and disables the `atelier-serve` service instead of restarting it and checking the site answers; and the rollback's step 3 reinstalls the fallback, enables and starts `atelier-serve` again, and checks the old site answers. No other step of the Transition is reworded here; step F's plan settles the rest.
- [ ] Both documents' `last_changed` lines say what this change did. `cargo nextest run -p atelier-cli -j 4` passes for the tests that parse strategy files, or the Report names the filter that covers them and its result.
- [ ] The old workspace's site is not changed: `git diff --stat master...HEAD -- crates/ .github/ dist-workspace.toml` is empty.

## Scope

`docs/strategies/redesign-product.md`, `docs/strategies/redesign-architecture.md` and this item's record. Nothing under `crates/`, `next/`, `.github/` or `scripts/`, and no other doc.

Three things this item's preliminary lines named are not done here, because each belongs to the change that writes the file:

- The CI and release workflows without the old site's jobs, and a release build that needs no Node, are step F's (atelier-u1ko), which replaces those files at the switch.
- "No instruction sends the human a site page" is step A's for the skill (atelier-orb2) and step F's for the `AGENTS.md` texts.
- The Desk's crate, app and CI jobs do not exist in the tree yet. If mission atelier-z4cr adds them before this item merges, they are left alone.

## Context

Mission atelier-qr5s, step E of `docs/strategies/redesign-3-cutover.md`, which changed on 2026-11-11 at Sumeet's direction from "rebuild the site on the new records". Deferred planning finding atelier-4pz2 (`atelier review show atelier-4pz2`) names the Transition lines that still expect a site; this item fixes those lines, and step F's plan fixes atelier-wd1i's Done when. The `atelier-serve` unit is `~/.config/systemd/user/atelier-serve.service`.

Expected size: about 60 changed lines across the two documents.
