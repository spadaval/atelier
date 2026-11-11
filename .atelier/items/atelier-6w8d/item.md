---
schema: atelier.item
schema_version: 3
id: atelier-6w8d
title: Replace the product docs, `CONTEXT.md`, `PRODUCT_INTENT.md` and the ADRs with the design record
status: draft
mission: atelier-qr5s
labels:
- redesign
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-10T00:56:10.442628646Z
---

## Goal

Leave one current description of the system, edited in place. Today the repository describes two tools: `docs/product/`, `CONTEXT.md`, `PRODUCT_INTENT.md`, 31 ADRs and most of `docs/architecture/` describe the old tracker, which is deleted, and the two specifications under `docs/strategies/` describe the new one as a target. An agent that reads the first set is told to run commands that no longer exist. After this item the two specifications are the product page and the design record, in the present tense, and the old set is gone.

## Done when

- [ ] `docs/product.md` is the product specification as it stands, edited to describe what Atelier does, not what it will do: no "target design", no four-missions table, no "today" for the old tool. Its Words, Lifecycle and Review sections keep their rules word for word, apart from tense. It says which of its commands are not built (`log`, `init`) and names the item that would build each, or drops them where no item does.
- [ ] `docs/design.md` is the architecture specification edited the same way: the single design record. Its Transition section is cut to what is still to come (the work host's adoption, and deleting `scripts/cutover/` afterwards) and one paragraph on how the cutover went, on 2026-11-11. Its "Tried and removed" and "Decisions" lists are kept, and gain a line for each decision of an ADR that still governs and is not already there, ADR 0032 (the Desk is the human's product) among them. The Report lists each ADR with where its decision now lives or why it is not carried.
- [ ] Deleted, with the Report naming each path: `docs/adr/`, `docs/strategies/`, `docs/product/` except the Desk's page, `CONTEXT.md`, `PRODUCT_INTENT.md`, `docs/spec/`, `docs/references/`, and every page under `docs/architecture/` that describes the old tracker or its site. Kept: the Desk's pages (`docs/architecture/desk.md`, `docs/product/desk.md`), `docs/architecture/quality/standards.md`, and the November 2026 baseline's reports (`docs/architecture/quality/scorecard-baseline-2026-11.md` and its directory), which are history the mission's criterion allows. A kept page's links are corrected to the new places.
- [ ] No open mission's document is lost: each strategy file under `docs/strategies/` that an open mission's `mission.md` or `design.md` names is in that mission's directory, and the Report says which those were. A closed mission's strategy is deleted; Git keeps it.
- [ ] `docs/index.md`, `README.md`, `CONTRIBUTING.md`, `AGENTS.md` and `CLAUDE.md` name only what exists: the two pages above, the Desk's pages, the skill, the evals. `CHANGELOG.md` gains a 0.5.0 entry that says in a few lines what the release is: the tracker rewritten, new records and commands, one cutover per host, and where a 0.4 repository goes for the way across (`scripts/cutover/README.md`).
- [ ] The old tool's eval fixtures are gone: the eleven scenario directories that need the old binary and `green-but-wrong` are deleted, with the files two new scenarios read from them (`costly-hardening/files/`, `non-atelier-repo/setup.sh`) moved into those scenarios first; every `next-*` scenario's unit suite and `setup.sh` still pass; and `evals/agent-factory/README.md` describes only what is left and names no file that is gone. Recorded results under `evals/agent-factory/results/` are history and stay as they are.
- [ ] The mission's criterion `no-old-surface` holds for this repository, and the Report gives the search and its output: for each command and record path in the product page's "What is removed" and "Today / Replaced by" tables, plus `atelier serve`, a search of `skills/`, `AGENTS.md`, `CLAUDE.md` and `docs/` finds no instruction to use it. A hit that names one as history is allowed in two places only, the design record's lists of what was tried and removed and the baseline's reports, and the Report counts those.
- [ ] Every relative link in the files this item writes or keeps under `docs/`, and in `README.md`, `CONTRIBUTING.md`, `AGENTS.md` and `skills/`, resolves, shown by a command and its output. `atelier check` passes.

## Scope

`docs/`, the root Markdown files, `evals/agent-factory/` as the line above says, links in `skills/agent-factory/` and `desk/README.md` that point at a moved page, and this item's record. No Rust, no workflow, no skill text beyond a link, nothing under `scripts/cutover/` or `desk/src/`. The mission records under `.atelier/missions/` are other planners' and are changed only where a link to a moved page would break.

The two pages are edited, not rewritten: a passage changes when it is no longer true or is in the wrong tense, and otherwise stays. Nothing the tool does changes; where a page and the binary disagree, the page is corrected to the binary and the Report lists the place.

## Context

Mission atelier-qr5s, step I, the last. The specifications are `docs/strategies/redesign-product.md` and `docs/strategies/redesign-architecture.md`; their `last_changed` lines were left alone during the mission and go with the front matter when the pages move. Several items edited them as they went, so they are close to the code already. The design record's own first section says it becomes "the single design record that replaces the ADRs".

This host runs a service its owner depends on: one cargo command at a time, with `-j 4`, if one is needed at all; set `TMPDIR` to a directory under `$HOME` outside any repository.

Expected size: about 9,000 lines deleted across some 90 files, and 300 to 500 changed in the two pages and the files that point at them.
