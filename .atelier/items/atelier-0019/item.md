---
schema: atelier.item
schema_version: 3
id: atelier-0019
title: Garden atelier, 2026-11-11
status: in_review
mission: null
labels:
- garden
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A1e4a355d-d905-4146-a4d8-89e6984f49f1%3Adelegate-task%3A1c7e22f2-4ce0-40e1-a447-156f728619a3
ready:
  by: sumeet, through mission atelier-295u
  at: 2026-11-11T17:56:34.466554928Z
trivial: null
caused_by: null
created_at: 2026-11-11T17:56:25.650533771Z
---

## Goal

Measure atelier by `skills/agent-factory/procedures/garden.md`, fix the upkeep and signs it may fix, and bring Sumeet one page with at most three proposals. This is atelier's first run, so the window starts at the first line `atelier log` prints, 2026-11-11T15:20:09Z, and it is also the trial of the rewritten procedure for atelier-613a.

## Done when

- [x] The numbers follow garden.md's steps, and `artifacts/numbers.md` holds the log lines behind them.
- [x] At most three proposals, each one cause with its evidence counted; one that adds a rule or check names one considered for removal.
- [x] Every change on the branch is upkeep or a sign: none adds or removes a rule, check, gate, interface or default.
- [x] Each finding `atelier list --deferred` showed is settled.
- [x] The page follows garden.md's template in at most 60 lines, with Session empty.

## Scope

In: garden.md steps 1 to 7 up to the approval, for atelier only. atelier-j86z's ten entries are read as evidence and the page says which the rewrite made moot.

Out: the session and what it settles, the run's merge, closing or changing atelier-j86z, any new or removed rule, check, merge check or hook, a scorecard snapshot or a run of `scripts/scorecard/`, and other items' branches.

## Report

Window: 2026-11-11T15:20:09Z to 2026-11-11T17:56:43Z UTC, 7 items merged. Workings: `artifacts/numbers.md`, `artifacts/checklist.md`.

| Measure | This run | Last run | Baseline |
| --- | --- | --- | --- |
| Waiting to start, building, in review, changes requested, approved: median minutes (worst) | 0 (x3n5 27), 11 (6w8d 20), 6 (6w8d 13), 2 (ivbq 3), 0 (6w8d 30) | first run | not measured |
| Rounds: median (worst); take-overs; escalations; skips by rule | 2 (6w8d 3); 0; 0; `CI passed on the head` 1 | first run | no skip mechanism |
| Open items by status; blocked; unsettled deferred findings | 106 draft, 22 ready, 1 building; 8; 2, both settled here | first run | not measured |
| Defects after merge, measured 2026-11-10 | carried | first run | 15 of 277 (5.4 per 100), a floor |
| Interventions per mission, measured 2026-11-10 | carried | first run | 0 to 7, 12 missions |
| Process share, measured 2026-11-10 | carried | first run | 28.0% of commands, 21.6% of tokens |
| Confusion per 100 `atelier` calls, measured 2026-11-10 | carried | first run | 25.2 |
| Tokens per merged item, median, measured 2026-11-10 | carried | first run | 3.06 USD |
| Gate precision; review yield, measured 2026-11-10 | carried | first run | 4 of 54; 8 of 20 |
| Calls before first edit, median, measured 2026-11-10 | carried | first run | 22 |
| Rework, measured 2026-11-10 | carried | first run | 10.8% |
| Where caught: review, CI, after merge, measured 2026-11-10 | carried | first run | about 191, 6, 15 |
| Non-test lines of Rust, measured 2026-11-10 | carried | first run | 48,989 |

What means nothing yet: every log number rests on 7 items merged in under three hours of one day; x3n5's 27 minutes waiting is time blocked by 8b0a, not a stall; 6w8d's 30 minutes approved is one CI run that failed on runner DNS. The open counts are mostly the cutover's 106 converted drafts. The carried rows are the baseline itself.

Checklist: 4 met, 5 not met (4: the Desk server's build named the deleted `next/`, repaired in 94bbfa149; 5: the Desk has no format or lint command; 7: no merge runs clippy; 8: no verify skill; 9: 39 of 39 bugs lack a cause), 0 could not test.

### Worst Spots
- Navigate: in a sample of three threads (26 to 34 calls before the first edit), all three read the design record (`docs/design.md`, 1,384 lines) and the product page; both code items read `crates/atelier-cli/src/output.rs` (1,471 lines) and `crates/atelier-cli/tests/scenarios.rs` (1,376 lines).
- Change: `skills/agent-factory/procedures/readiness.md`, `crates/atelier-app/src/log.rs` and `scripts/scorecard/draw_samples.py`, 2 fix-now findings each, each from one item. Rework 10.8%, measured 2026-11-10; rework by file is not measured.
- Validate: no verify skill, so no feature has a proof command. No bug item was filed in the window, though atelier-i70l and atelier-o2vv describe defects and carry neither the `bug` label nor a cause.

### Proposals
#### P1 One reader for record Markdown
Cause: record Markdown is read by more than one parser, each with its own rules. Evidence: 3 of the window's 10 fix-now findings, atelier-ivbq 1-o9m9.2 (`body.rs` reads any leading `#` as a heading; `document.rs` does not) and atelier-x3n5 1-c6ma.1 and .2 (`draw_samples.py` matched findings and mission membership by text), and 1 deferred finding in the same script.
Moves: fix-now findings per item, and rounds (median 2). Change: give `body.rs` and `document.rs` one rule for headings and fences, and have the scorecard scripts read front matter as YAML and skip fences by that rule.

#### P2 Lint every Rust merge with clippy, and drop the merge checks CI repeats
Cause: the checks a merge runs differ from the ones `AGENTS.md` documents. Evidence: checklist line 7, clippy runs in neither `ci.yml` nor `config.toml`; 2 of 7 reviews of items that changed `crates/` ran it by hand; 2 of the 3 merge checks repeat CI steps (atelier-j86z R9).
Moves: checklist line 7, and style findings in review. Change: add `cargo clippy --all-targets -- -D warnings` to CI's Rust job. Considered removing: the `git diff --check` and `cargo fmt -- --check` merge checks, which CI already runs.

#### P3 Give every bug item a cause, or drop it
Cause: the cutover converted 39 bug items without `caused_by`, and new defects are filed unlabelled. Evidence: checklist line 9, 39 of 39; atelier-i70l and atelier-o2vv this window.
Moves: defects found after merge, the scorecard's outcome, which counts none of these today. Change: a groom pass that sets `--caused-by` or drops each, and labels new defects `bug` when filed.

atelier-j86z: the rewrite made R1, R2, R3, R4, R7 and R10 moot; R5, R6, R8 and R9 still apply, and R9 is part of P2. The session may close it.

### Fixed and Settled
- 94bbfa149: upkeep. Four lines in `desk/README.md` and `docs/architecture/desk.md` named the deleted `next/` directory and `redesign-architecture.md`.
- atelier-6w8d/3-kvll.1: dropped, because 94bbfa149 fixed it.
- atelier-x3n5/1-c6ma.3: taken into atelier-qf7u. It needs `bug` and `--caused-by atelier-x3n5` after this run merges, since `atelier set` runs only on the base.

### Responses

- 1-2bol.1: fixed in 8d7160ece

### Session
