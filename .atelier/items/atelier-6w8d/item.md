---
schema: atelier.item
schema_version: 3
id: atelier-6w8d
title: Replace the product docs, `CONTEXT.md`, `PRODUCT_INTENT.md` and the ADRs with the design record
status: in_review
mission: atelier-qr5s
labels:
- redesign
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A2074a88e-920b-49f9-bd17-caf5903cc60f%3Adelegate-task%3A9ecd8d63-8fcc-407b-8c2c-a18464f2529d
ready:
  by: sumeet
  at: 2026-11-11T15:43:38.001833021Z
trivial: null
caused_by: null
created_at: 2026-11-10T00:56:10.442628646Z
---

## Goal

Leave one current description of the system, edited in place. Today the repository describes two tools: `docs/product/`, `CONTEXT.md`, `PRODUCT_INTENT.md`, 31 ADRs and most of `docs/architecture/` describe the old tracker, which is deleted, and the two specifications under `docs/strategies/` describe the new one as a target. An agent that reads the first set is told to run commands that no longer exist. After this item the two specifications are the product page and the design record, in the present tense, and the old set is gone.

## Done when

- [x] `docs/product.md` is the product specification as it stands, edited to describe what Atelier does, not what it will do: no "target design", no four-missions table, no "today" for the old tool. Its Words, Lifecycle and Review sections keep their rules word for word, apart from tense. It says which of its commands are not built (`log`, `init`) and names the item that would build each, or drops them where no item does.
- [x] `docs/design.md` is the architecture specification edited the same way: the single design record. Its Transition section is cut to what is still to come (the work host's adoption, and deleting `scripts/cutover/` afterwards) and one paragraph on how the cutover went, on 2026-11-11. Its "Tried and removed" and "Decisions" lists are kept, and gain a line for each decision of an ADR that still governs and is not already there, ADR 0032 (the Desk is the human's product) among them. The Report lists each ADR with where its decision now lives or why it is not carried.
- [x] Deleted, with the Report naming each path: `docs/adr/`, `docs/strategies/`, `docs/product/` except the Desk's page, `CONTEXT.md`, `PRODUCT_INTENT.md`, `docs/spec/`, `docs/references/`, and every page under `docs/architecture/` that describes the old tracker or its site. Kept: the Desk's pages (`docs/architecture/desk.md`, `docs/product/desk.md`), `docs/architecture/quality/standards.md`, and the November 2026 baseline's reports (`docs/architecture/quality/scorecard-baseline-2026-11.md` and its directory), which are history the mission's criterion allows. A kept page's links are corrected to the new places.
- [x] No open mission's document is lost: each strategy file under `docs/strategies/` that an open mission's `mission.md` or `design.md` names is in that mission's directory, and the Report says which those were. A closed mission's strategy is deleted; Git keeps it.
- [x] `docs/index.md`, `README.md`, `CONTRIBUTING.md`, `AGENTS.md` and `CLAUDE.md` name only what exists: the two pages above, the Desk's pages, the skill, the evals. `CHANGELOG.md` gains a 0.5.0 entry that says in a few lines what the release is: the tracker rewritten, new records and commands, one cutover per host, and where a 0.4 repository goes for the way across (`scripts/cutover/README.md`).
- [x] The old tool's eval fixtures are gone: the eleven scenario directories that need the old binary and `green-but-wrong` are deleted, with the files two new scenarios read from them (`costly-hardening/files/`, `non-atelier-repo/setup.sh`) moved into those scenarios first; every `next-*` scenario's unit suite and `setup.sh` still pass; and `evals/agent-factory/README.md` describes only what is left and names no file that is gone. Recorded results under `evals/agent-factory/results/` are history and stay as they are.
- [x] The mission's criterion `no-old-surface` holds for this repository, and the Report gives the search and its output: for each command and record path in the product page's "What is removed" and "Today / Replaced by" tables, plus `atelier serve`, a search of `skills/`, `AGENTS.md`, `CLAUDE.md` and `docs/` finds no instruction to use it. A hit that names one as history is allowed in two places only, the design record's lists of what was tried and removed and the baseline's reports, and the Report counts those.
- [x] Every relative link in the files this item writes or keeps under `docs/`, and in `README.md`, `CONTRIBUTING.md`, `AGENTS.md`, `skills/` and `scripts/cutover/README.md`, resolves, shown by a command and its output. `git grep -n -e 'docs/strategies' -e 'docs/adr' -e 'docs/product/' -e 'CONTEXT.md' -e 'PRODUCT_INTENT.md'` over the tracked tree outside `.atelier/` and `evals/agent-factory/results/` prints nothing but the Desk's kept page, or the Report explains each line. `atelier check` passes, and so does `cargo fmt -- --check`, since comments in `crates/` may change. CI on the pull request runs after `submit` and is `merge`'s to wait for, not a line to tick here.

## Scope

`docs/`, the root Markdown files, `evals/agent-factory/` as the line above says, and this item's record. Beyond those, only a reference to a page this item moves or deletes, wherever a tracked file holds one: `scripts/cutover/README.md`, which names the Transition section as its protocol and must point at the design record's section on adoption; `skills/agent-factory/`; `desk/README.md`; and comments in `crates/` and `scripts/` that name a specification's path. Such an edit changes the path and nothing else. No behaviour of any code, script or workflow changes, and no skill text beyond a link. The mission records under `.atelier/missions/` are other planners' and are changed only where a link to a moved page would break.

The two pages are edited, not rewritten: a passage changes when it is no longer true or is in the wrong tense, and otherwise stays. Nothing the tool does changes; where a page and the binary disagree, the page is corrected to the binary and the Report lists the place.

## Context

Mission atelier-qr5s, step I, the last. The specifications are `docs/strategies/redesign-product.md` and `docs/strategies/redesign-architecture.md`; their `last_changed` lines were left alone during the mission and go with the front matter when the pages move. Several items edited them as they went, so they are close to the code already. The design record's own first section says it becomes "the single design record that replaces the ADRs".

This host runs a service its owner depends on: one cargo command at a time, with `-j 4`, if one is needed at all; set `TMPDIR` to a directory under `$HOME` outside any repository.

Expected size: about 9,000 lines deleted across some 90 files, and 300 to 500 changed in the two pages and the files that point at them.

## Report

The repository now describes one tool. `docs/product.md` and `docs/design.md` are the two specifications, moved from `docs/strategies/` and edited into the present tense. The old product docs, `CONTEXT.md`, `PRODUCT_INTENT.md`, the 31 ADRs, the old architecture pages and the old tool's eval fixtures are deleted: 204 files changed, 393 lines added and 15,842 deleted.

### The product page, `docs/product.md`

- The front matter and its `last_changed` line are gone. The "target design" paragraph and the four-missions table are replaced by three sentences: what the page is, where the design record is, and that the tool was rewritten to the page and every repository on this host moved onto it on 2026-11-11.
- Why is in the past tense, and Who it is for says backward compatibility "was not a goal: the cutover converted the records once".
- Commands now say that seventeen commands are built, plus `setup`. `log` is not built yet, and atelier-8b0a, in mission atelier-295u, adds it; its row in the Anyone table says so. `init` is dropped: no item builds it (atelier-t3ko is about the old tool's `init`). The Desk link points at `product/desk.md`.
- Moved to the design record, so that the old commands are named only where the mission's criterion allows: the "Today / Replaced by" table of the nine `work` views, and the "What is removed" table. The product page keeps one sentence pointing at them.
- Present tense where the text spoke of the redesign as future: the Scorecard ("Atelier is judged on these"), and `log`'s part of it ("once built"). The Out of scope line on the change view no longer says "this work".
- Words, Lifecycle and Review are byte for byte as they were. The command `diff <(section old) <(section new)` printed nothing for each of the three sections.
- The page checked against the binary: every flag in the Commands tables matches `atelier <command> --help`, and the `submit` handoff and the wrong-role line match `crates/atelier-app/src/say.rs`. I found no disagreement to correct.

### The design record, `docs/design.md`

- Its front matter is gone. The opening paragraph says this is the one design record, that it replaced the ADRs on 2026-11-11, and that it is corrected where it disagrees with the binary.
- The rule for the work, why a full rewrite, and "Where the code is today" (now "What came from the old code") are in the past tense. The table's column is "Lines in the old tool".
- Corrected to the code:
  - Workspace and crates described `next/`. It now describes the root workspace with crates under `crates/`, `scripts/ci-jobs.sh`'s two jobs (`rust` and `desk`) as they stand, `target/debug/atelier`, and `cargo install`. One paragraph records how `next/` was used.
  - "Built so far:" and "Missions join with their commands." are cut from Threads and roles, since the mission commands exist.
  - The Log section opens with a line saying it is not built yet (atelier-8b0a), and Testing's `log` line says "Once `log` is built".
  - The Forge and Site sections now describe what was copied in the past tense. The Site section names no command.
- Transition is cut to three parts. First, one paragraph on 2026-11-11: the switch merged; one run of `cutover.sh` converted the five repositories within four minutes, with no stop and no rollback; `--verify` matched in each; and the counts were 133/7, 240/4, 19/1, 47/3 and 37/7. These come from `~/.cache/atelier-cutover/journal`, with 85 starts and 85 dones and no restart, and from `report/*.counts` and `*.verify`. Then two parts still to come: the work host's adoption, rewritten to stand alone and pointing at `scripts/cutover/README.md#another-host-adopts`, and deleting `scripts/cutover/` afterwards. The line about CI's fallback is gone, because atelier-2mlh merged the change that removed it (d0f75b44).
- Tried and removed keeps its table, and gains the product page's "What is removed" table and the old-views table. The `serve` row now says "(see Decisions)" in place of the ADR link.
- Decisions keeps every line. Its header now says how ADR lines are marked, and it gains twelve lines carried from the ADRs, listed below.

### Where each ADR's decision now lives

| ADR | Where its decision is now, or why it is not carried |
| --- | --- |
| 0001 record IDs | Records, IDs: `<slug>-<4 characters>` |
| 0002 Markdown-first store | Records (Markdown with front matter in the tree). Its SQLite index is under Tried and removed |
| 0003 evidence artifacts | Not carried: evidence records are removed (Tried and removed); artifacts sit beside the item (product, Proof and validation) |
| 0004 work lock sync | Not carried: status is a stored field, and "There is no local state file" (Threads and roles) |
| 0005 repo-owned workflow state | Status is a stored field (Decisions); the configurable workflow went (Tried and removed) |
| 0006 agent guidance ownership | Superseded by 0019 before this item; principle 8, judgment in the skill and procedure in the tool, holds what remains |
| 0007 mission workspaces, epic branches | Superseded by 0019; one branch per item (product, Records) |
| 0009 virtual workspace root | Workspace and crates |
| 0010 session-aware PR coordination | Superseded by 0019; sessions as records are under Tried and removed |
| 0011 review modes and rooms | Superseded by 0019; review rooms are under Tried and removed |
| 0012 transition effects | Superseded by 0019; transition actions are under Tried and removed |
| 0013 transition actions | Superseded by 0019; the same |
| 0014 status role attribution | Superseded by 0019; roles come from the command a thread enters with (product, Words; Threads and roles) |
| 0015 missions as workflow policy | Not carried: the workflow file went |
| 0016 canonical work branches | Product, Records: "An item's branch is named for its ID" |
| 0017 SQLite cache | Superseded by 0019; Tried and removed, first row |
| 0018 strategies, draft epics | Strategy files are removed (Tried and removed); a mission's documents live in its directory. Adaptive expansion is the product Lifecycle's "later work can wait as drafts" |
| 0019 verified repo-native tracker | Not carried as a line. Its ownership split is replaced by product principles 3 and 8; its invariants are the act table and the Review section |
| 0020 Agent Factory targets Atelier only | Carried through 0029's line (the skill steps aside outside Atelier); its decision 3 is principle 8 |
| 0021 parent issues | Not carried: parents are removed (Tried and removed) |
| 0022 status orients, output is dense | Two new Decisions lines: dense Markdown with no JSON or YAML mode, with piping treated as a missing view; and repository instructions in `AGENTS.md`, imported by `CLAUDE.md`, with commands printing state and not instructions |
| 0023 skill not vendored | Superseded by 0029 |
| 0024 missions as themed containers | New line: criteria are optional, and each is achievable by the mission's own work. "Themed container" is not carried: the Words define a mission by its end, and a theme as a label |
| 0025 findings carry disposition | Not carried: replaced by recommendations and `Decided:` in the review file (Records) |
| 0026 evidence only what nothing records | Not carried: evidence records are removed |
| 0027 proof has a time | Superseded by 0028 before this item; after-merge checks are gone |
| 0028 simpler lifecycle | Its approval rule is "The approval covers the head" (Status). Its one meaning per word is the product Words. Two new lines: no regrowth, and one binary per host reading the old form until every repository moves. Loose ends and the light path are under Tried and removed |
| 0029 Agent Factory lives in Atelier | New line: one copy in `skills/agent-factory` with its evals, installed with `npx skills add`, nothing in `crates/` reads it, and it steps aside outside Atelier. `doctor` and `setup --link` are not carried, since both commands are gone |
| 0030 site, session database, T3 | Three new lines, merged with 0032: records are the only truth, and guards consult only Git, the records and CI; the host's SQLite database, with each kind's one writer and each command writing its own attribution row from its session's environment, not through an MCP server (decision 7); and what live pages and the feed may read. Its site is under Tried and removed |
| 0031 forge carries branches and CI | New line: a forge carries branches and reports CI, review stays in the repository, and GitHub works through `gh`. The mechanics are in Merge and The forge |
| 0032 the Desk is the human's product | Two new lines: two products and a separate build; and the Desk acts only through the core. With 0030: one direction of dependency; the database and its writers (decision 6); live pages reading only records, T3 Code's MCP server and the database, never T3 Code's own database, with only the feed reading other sources, one adapter per source, with provenance (decision 7); and conversations staying in T3 Code (decision 8) |

### Deleted

- `docs/adr/`: all 31 ADRs, 0001 to 0032 (there is no 0008).
- `docs/strategies/`: `agent-factory-adaptive-missions.md`, `atelier-rebuild.md`, `desk.md`, `forge-delivery.md`, `lighter-workflow.md`, `mission-review-boundary.md`, `redesign-1-baseline.md`, `redesign-2-core.md`, `release-distribution.md`, `review-and-introspection.md`, `simpler-lifecycle.md`. The two specifications moved to `docs/product.md` and `docs/design.md`, and `fleet-dashboard.md` moved as below.
- `docs/product/`, all but `desk.md`: `cli-surface.md`, `development-setup.md`, `human-cli-output.md`, `index.md`, `issue-inventory-and-mission-overview.md`, `mission-authoring.md`, `mission-reviews.md`, `repository-contribution.md`, `retention-and-prune.md`, `reviews-and-merge.md`, `sessions-and-hooks.md`, `validation.md`, `work-model.md`, `work-view-ordering.md`, `workflow-configuration.md`, `zen.md`.
- `CONTEXT.md`, `PRODUCT_INTENT.md`, `docs/spec/` (the bundle schema and its fixtures, and `agent-factory/tracker-replacement-mvp.md`), and `docs/references/` (six pages).
- `docs/architecture/`: `index.md`, `markdown-first-record-store.md`, `provenance.md`, `site.md`, `source-layout.md`.
- `docs/architecture/quality/`: `index.md`, `validation.md`, `architecture-quality.md`, `rust-quality-hazard-scans.md`, and the dated old-tool audits (`adaptive-mission-gap-analysis.md`, `agent-factory-atelier-validation.md`, `agent-readiness-audit-2026-06-13.md`, `app-cli-boundary-audit-2026-06-17.md`, `beads-replacement-closeout.md`, `cli-zen-audit-2026-06-14.md`, `codex-mission-log-insights-2026-06-14.md`, `mission-log-insights.md`, `stabilization-closeout-inventory-2026-06-13.md`). The first four describe the old tracker: evidence receipts, graph links, `fuzz/` and a `test-hooks` feature, which do not exist.

Kept: `docs/product/desk.md`, `docs/architecture/desk.md`, `docs/architecture/quality/standards.md`, `scorecard-baseline-2026-11.md` and `scorecard-baseline/`. The kept pages' changes:

- The Desk's two pages: the ADR 0032 links point at `../design.md#decisions`. The API page's status line no longer names the deleted Desk strategy, and its link example names `docs/design.md`. Three phrases that named an old command or a "loose end" now name them in other words, so the criterion's search finds nothing there: "The old tool's site" for `atelier serve`, the old tool's mission brief and debrief, and "the follow-up item atelier-qxmr".
- `standards.md`: its false lines are corrected, which is more than a link change. There is no `test-hooks` feature. `human-cli-output.md` and the hazard-scans page are deleted. Atelier does make commits now, and there is no evidence or `disposition`. Its documentation rule pointed at Product Intent, Context and ADRs, and now points at the two pages.
- The baseline: the product specification links point at `product.md#scorecard`. The link to its closed mission's strategy is now `git show 96d02deb7:docs/strategies/redesign-1-baseline.md`, as is the same pointer in `scripts/scorecard/README.md`.

### Open missions' documents

Only one strategy file was named by an open mission's `mission.md` or `design.md`: `fleet-dashboard.md`. It is named in the front matter of atelier-bnek's `design.md` ("split out of fleet-dashboard.md"), and it moved to `.atelier/missions/atelier-bnek/fleet-dashboard.md`. It was the strategy of the closed site mission atelier-yfqu, so it could have been deleted. I moved it because the rule reads "names". atelier-295u and atelier-qr5s link to the two specifications. Their links in `design.md` and `notes.md` now point at `../../../docs/design.md` (with the same anchors), `../../../docs/product.md`, and, for ADR 0032, `../../../docs/design.md#decisions`. The other strategies belonged to closed missions, among them the Desk (atelier-z4cr) and the baseline. `desk/README.md` names the Desk's criteria without the deleted path.

### Root files and references

- `docs/index.md` is rewritten. It names the two pages, the Desk's two pages, the skill, the evals, the standards and the baseline (marked as history).
- `README.md`: a present description, no `atelier serve`, no Node or pnpm for building from source, no link to the deleted setup page, and links to the two pages and `desk/`.
- `CONTRIBUTING.md`: the current loop (`status`, `new`, `start`, `submit`, `merge`), with no evidence and no deleted links.
- `AGENTS.md` and its reviewed copy `scripts/cutover/agents/atelier.md`: the Sources lines name `docs/product.md`, `docs/design.md` and the Desk's two pages. `CLAUDE.md` is `@AGENTS.md` and unchanged.
- `CHANGELOG.md` gains a 0.5.0 entry, dated 2026-11-11 like the `v0.5.0` tag. Its 0.3.0 entry's ADR 0019 link is now plain text.
- Path-only changes to `docs/design.md` or `docs/product.md`: the comments in `Cargo.toml`, `crates/atelier-app/src/skip.rs`, `crates/atelier-cli/src/{main,check,merge,setup}.rs`, `crates/atelier-records/src/lib.rs`, and `scripts/cutover/{convert_records.py,cutover.sh,rollback.sh}`. `scripts/cutover/README.md` now links the design record's Transition section and names adoption as what is still to come.

### Evals

- `costly-hardening/files/` moved to `next-costly-hardening/files/`, and `non-atelier-repo/setup.sh` to `next-non-atelier-repo/repo.sh`. Each setup now reads its own copy (`files=$here/files`, `bash "$here/repo.sh"`).
- Then I deleted `green-but-wrong/` and the eleven old scenarios: `abort-unreachable`, `costly-hardening`, `cross-repo-regression`, `done-when-rewrite`, `non-atelier-repo`, `plan-defect`, `rare-edge-chain`, `refused-land-push`, `settle-within-authority`, `trivial-typo`, `vacuous-test-review`. atelier-wl6u, which ports two of them, reads them from Git (at 96d02deb7).
- `README.md` lists only the ten `next-*` scenarios, with one paragraph on where the "from X" names came from. It drops the BenchFlow section and the link to `maintainers/skill-evaluation.md`, which the cutover had removed. Its examples use the default `--skill-dir` and two `next-*` scenarios. `results/` is unchanged.

### Validation

Each `next-*` unit suite, with the installed 0.5.0 binary as `NEXT_ATELIER` and `TMPDIR` under `~/.cache`:

```text
next-costly-hardening: Ran 8 tests  OK      next-decide: Ran 12 tests  OK
next-done-when-rewrite: Ran 12 tests  OK    next-loop: Ran 11 tests  OK
next-loop-changes: Ran 10 tests  OK         next-non-atelier-repo: Ran 6 tests  OK
next-rebuttal-kept: Ran 11 tests  OK        next-refused-push: Ran 9 tests  OK
next-settle-within-authority: Ran 11 tests  OK   next-vacuous-test-review: Ran 9 tests  OK
```

Each `setup.sh` run alone into a fresh directory exited 0 and made `repo/`, for all ten.

The links: a checker of every relative link and its `#anchor` (GitHub heading slugs, code ignored), run over `docs/`, `README.md`, `CONTRIBUTING.md`, `AGENTS.md`, `CLAUDE.md`, `CHANGELOG.md`, `skills/`, `scripts/cutover/README.md`, `evals/agent-factory/README.md` and `desk/README.md`. It reports a missing file and a missing anchor when given a test file holding both.

```text
99 relative links in 29 files, 0 broken
```

`git grep -n -e 'docs/strategies' -e 'docs/adr' -e 'docs/product/' -e 'CONTEXT.md' -e 'PRODUCT_INTENT.md' -- ':!.atelier' ':!evals/agent-factory/results'` prints these lines, each explained:

- The Desk's kept page, `docs/product/desk.md`: `AGENTS.md:61`, `desk/README.md:4` and `scripts/cutover/agents/atelier.md:61`.
- Test fixtures, which are data and not references, and which this item may not change: `desk/e2e/fixtures.ts` (172, 451, 458, 460, 463, 465), `desk/src/lib/links.test.ts` (8, 10, 21), `scripts/ci-jobs.test.sh` (20, 50, 52) and `scripts/cutover/test_convert_records.py` (18 lines).
- History pointers into Git for the deleted baseline strategy: `docs/architecture/quality/scorecard-baseline-2026-11.md:276` and `scripts/scorecard/README.md:6`.
- The design record's Tried and removed row "Strategy files in `docs/strategies/`" (`docs/design.md:1209`).

`no-old-surface`: one search per command and record path from the two tables (`workflow.yaml`, evidence, `.atelier/issues`, `.atelier/reviews`, `--issue-type`, `atelier issue`, loose ends, `bundle`, `migrate`, `doctor`, `prune`, `land`, `merge`'s six flags, `brief`, `debrief`, `serve`, the nine `work` views, the `review` subcommands, `docs/strategies`, `docs/adr`), plus `atelier serve` and `CONTEXT.md` and `PRODUCT_INTENT.md`, over `skills/`, `AGENTS.md`, `CLAUDE.md` and `docs/`. It finds no instruction to use any of them. Its hits per file:

```text
      2 docs/architecture/quality/scorecard-baseline-2026-11.md
      4 docs/architecture/quality/scorecard-baseline/defect-method.md
      7 docs/architecture/quality/scorecard-baseline/defects.md
      5 docs/architecture/quality/scorecard-baseline/rubric.md
    100 docs/architecture/quality/scorecard-baseline/samples/findings.json
     20 docs/design.md
      1 skills/agent-factory/roles/lead.md
```

- The 118 baseline hits are its reports, which are history.
- The 20 `docs/design.md` hits are lines 1192 to 1226, all inside Tried and removed (1176 to 1227).
- The `lead.md` hit is a false match of `review finding` in "a review finding's fix included".

So the history hits are 20 in the design record's lists and 118 in the baseline, and no other file has any.

`atelier check`: "Every record is valid: 133 items, 7 missions and config.toml." `cargo fmt -- --check` and `git diff --check` pass.

After round 2's approval, atelier-ivbq merged its own edits to the two specifications on master, at their old paths, and `merge` refused the conflict. I merged master in (6a7f29688) and kept ivbq's additions on the moved pages. On the product page these are the wrapped Done when line, `escalate --finding`, the visitor's questions and the no-repository exit in What `status` prints, and the longer `submit` handoff, with its link pointed at `design.md`. In the design record they are the reader's-job sentences, the no-repository line and the `escalate --finding` line in Threads and roles, without "Built so far:" and "Missions join with their commands.", which this item had cut. Afterwards Words, Lifecycle and Review were still unchanged. The checks above were rerun on the merge with the same results, at the new line numbers.

For the reviewer:

- Two Decisions lines carry ADR 0028 rules that only `PRODUCT_INTENT.md` and the ADR held: no regrowth, and one binary per host. Check that their wording does not go beyond the ADR.
- The "Reason" column of the twelve new lines is drawn from each ADR's Context.

### Responses

- 1-rzbt.1: fixed in 9e0e1d3a5
- 1-rzbt.2: fixed in 9e0e1d3a5
