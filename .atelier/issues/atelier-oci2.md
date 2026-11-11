---
created_at: "2026-11-10T00:56:08.595125805+00:00"
id: "atelier-oci2"
issue_type: "task"
labels:
- "redesign"
workflow_branch:
  branch_base: master
  work_branch: atelier-oci2
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
status: "in_progress"
title: "Take the site out of the new workspace: no serve, no web build, and the old site goes at the switch"
updated_at: "2026-11-11T04:28:52.432379999+00:00"
---

## Goal

Take the tracker's site off the cutover's path. The human's pages become a product of their own, the Desk (ADR 0032, mission atelier-z4cr), so the `atelier` binary of the new workspace has no site and no `serve`, and the old site is deleted with the old workspace at the switch instead of being ported.

Mission 2 never copied the site into `next/`: the new workspace has four crates and none is `atelier-site`. So this item builds nothing. It proves the new binary has no site, and it corrects the two specifications, which still tell the switch and the cutover to rebuild, restart and check a site. Step F's runbook is written from those sections, so they are corrected before it is planned.

## Done when

- [x] Proved on a binary built from this branch's `next/`, with the commands and their output in the Report: `next/Cargo.toml` lists no `atelier-site` member and `cargo metadata` shows none; `atelier --help` lists no `serve`; no file under `next/` has a build script or a dependency that runs Node or pnpm; and `cargo build --locked` in `next/` succeeds with `node` and `pnpm` absent from `PATH`. The CI job `next` already builds without a Node step, and the Report cites it.
- [x] `docs/strategies/redesign-product.md` no longer gives the new tool a site: the Commands heading line does not count `serve`; the removed-commands row for `brief` and `debrief` says a mission is read with `atelier show <mission>` until the Desk has its page; the Merge section's sentence on an approved item names `status` and `show` and no site; and the table of four missions says mission 3 takes the site out. Its Words, Lifecycle and Review sections are not changed.
- [x] `docs/strategies/redesign-architecture.md` no longer gives the new workspace a site: the sentence above the crate table, which says the new workspace has the crates the old one has less `atelier-workflow`, says it has four and names `atelier-site` as left out too; the crate table and the dependency direction have no `atelier-site`; the rows of "Where the code is today" that say a piece is copied "at the site step" say it is not carried over; the Site section says in a few sentences that the tracker has no site, the old one is deleted with the old workspace, and the Desk replaces it in its own mission; the sentences that name the site as a reader of records or of the log's data either drop it or name the Desk; and the open question about brief and debrief view code is removed.
- [x] The Transition section of the architecture specification matches: part 3 has no "the site is rebuilt" bullet; the fallback release still has to serve the old site, since a rollback restores it; the cutover's step 3 stops and disables the `atelier-serve` service instead of restarting it and checking the site answers; and the rollback's step 3 reinstalls the fallback, enables and starts `atelier-serve` again, and checks the old site answers. No other step of the Transition is reworded here; step F's plan settles the rest.
- [x] The Rust tests that parse strategy files pass: the Report names the `cargo nextest run -j 4` filter that covers them and its result. The `last_changed` lines of both documents are left alone, since atelier-1oel edits the same two documents on another branch and the line would conflict; the orchestrator writes them once.
- [x] The old workspace's site is not changed: `git diff --stat master...HEAD -- crates/ .github/ dist-workspace.toml` is empty.

## Scope

`docs/strategies/redesign-product.md`, `docs/strategies/redesign-architecture.md` and this item's record. Nothing under `crates/`, `next/`, `.github/` or `scripts/`, and no other doc.

Three things this item's preliminary lines named are not done here, because each belongs to the change that writes the file:

- The CI and release workflows without the old site's jobs, and a release build that needs no Node, are step F's (atelier-u1ko), which replaces those files at the switch.
- "No instruction sends the human a site page" is step A's for the skill (atelier-orb2) and step F's for the `AGENTS.md` texts.
- The Desk's crate, app and CI jobs do not exist in the tree yet. If mission atelier-z4cr adds them before this item merges, they are left alone.

## Context

Mission atelier-qr5s, step E of `docs/strategies/redesign-3-cutover.md`, which changed on 2026-11-11 at Sumeet's direction from "rebuild the site on the new records". Deferred planning finding atelier-4pz2 (`atelier review show atelier-4pz2`) names the Transition lines that still expect a site; this item fixes those lines, and step F's plan fixes atelier-wd1i's Done when. The `atelier-serve` unit is `~/.config/systemd/user/atelier-serve.service`.

Expected size: about 60 changed lines across the two documents.

## Notes

- 2026-11-10 [note] Handoff: approved in round 1 (atelier-ze4k, GPT-6.1 Sol) at e1886010; no findings. Outcome, proof and open items are in the Report.

## Report

All six Done when lines hold. Most of the specification text they ask for was already on `master` before this branch started: the planning commits 63af769d, 07418b1b and cffbca9c rewrote the Commands line, the `serve` row, the crate table's dependency line, the "Where the code is today" rows, the Site section, the open questions and the Transition's part 3, cutover step 3 and rollback step 3. This branch makes the four corrections that were still missing, checks the rest against each line, and adds the binary proof. The proof: `cargo build --locked -j 4` in `next/` with no `node`, `npm` or `pnpm` on `PATH` finished with exit 0; `atelier --help` of that build lists no `serve`; `cargo metadata` shows four packages and no `atelier-site`; and `cargo nextest run -j 4 -p atelier-app -E 'test(this_repositorys_strategies_parse_without_problems)'` passed, 1 test.

Line by line:

1. **The binary has no site.** `next/Cargo.toml` lists four members: `atelier-app`, `atelier-cli`, `atelier-core`, `atelier-records`. `cargo metadata --no-deps --locked` in `next/` gives `['atelier-app', 'atelier-core', 'atelier-records', 'atelier-cli']`, and in the full resolve `atelier-site` is absent. `next/target/debug/atelier --help` lists `status check setup show list new set ready drop close start submit escalate review verdict decide lead merge help`; `grep -c -w serve` on it prints 0. `find next -path next/target -prune -o -name build.rs -o -name package.json -o -name 'pnpm-*'` finds nothing, and no `Cargo.toml` or `.rs` file under `next/` names node, npm, npx or pnpm. The third-party crates with build scripts in the resolve (anyhow, libc, serde, proc-macro2 and the like) are the usual Rust ones; none runs Node, which the build below shows. Build: a `PATH` of `~/.cargo/bin` plus a directory linking every `/usr/bin` entry except node, nodejs, npm, npx, pnpm, pnpx, corepack and yarn (the shell printed `node: absent; pnpm: absent; npm: absent`), then `cargo build --locked -j 4` in `next/`: `Finished dev profile ... in 9.46s`, exit 0. CI's `next` job (`.github/workflows/ci.yml:81`) has no Node step: checkout, Rust toolchain, nextest, `rm -rf ../crates`, `cargo fmt -- --check`, `cargo build --locked --all-targets`, `cargo nextest run --locked`.
2. **Product specification.** The Commands line already reads "Eighteen commands, plus `init` and `setup`. The tool has no site", and its eighteen are the tables' commands with no `serve`. Changed here: the `brief` and `debrief` row now says "until it has its mission page, a mission is read with `atelier show <mission>`"; the Merge section's approved-item sentence names `status` and `show` only, dropping "and on the Desk"; the four-missions table's mission 3 row says "the site taken out, the old workspace deleted with it". Words, Lifecycle and Review are untouched (`git diff master -- docs/strategies/redesign-product.md` changes lines 17, 405 and 462 only).
3. **Architecture specification.** Changed here: the sentence above the crate table now says the workspace has "four crates, those of the old one less `atelier-workflow` and `atelier-site`". Already on `master` and checked: the crate table and the dependency line name no `atelier-site`; no row of "Where the code is today" says "at the site step" (the site and brief/debrief rows say "Not carried over"); the Site section says the tool has no site, the old one goes with the old workspace, and the Desk replaces it in mission atelier-z4cr; the record-reader and log sentences name the Desk; and Open questions has no brief or debrief question.
4. **Transition.** Already on `master` and checked, nothing changed: part 3's site bullet says the site is taken out, not rebuilt; the fallback release bullet still says it must "serve the site as the installed binary does"; cutover step 3 says to "stop and disable the `atelier-serve` service, since the new binary has no `serve`"; rollback step 3 says to "reinstall the fallback release, ... enable and start `atelier-serve` again, and check the site answers". This closes the Transition half of deferred finding atelier-4pz2; atelier-wd1i's Done when is step F's.
5. **Strategy tests.** The one test that reads this repository's strategy files is `criteria::tests::this_repositorys_strategies_parse_without_problems` in `crates/atelier-app/src/criteria.rs`. `cargo nextest run -j 4 -p atelier-app -E 'test(this_repositorys_strategies_parse_without_problems)'`: 1 passed, 127 skipped. Neither specification has a Success Criteria heading, so the test reads both and parses neither; it guards the strategies that have one. Both `last_changed` lines are unchanged.
6. **Old workspace untouched.** `git diff --stat master...HEAD -- crates/ .github/ dist-workspace.toml` prints nothing, and neither does the same against the working tree.

### Deviations

The change is 7 lines in the two documents, not about 60, because the planning commits had already made most of the corrections the Done when lines describe. This branch confirms each against the text.

### Open items

- The Site section says the Desk's crate and app "are built in this workspace", while the crate sentence now counts four crates. Both hold today, since the Desk's crate does not exist yet; mission atelier-z4cr updates the count when it adds one.
- The new binary's help has no `log` and no `init`, which the product specification's Commands line counts. That is outside this item.
