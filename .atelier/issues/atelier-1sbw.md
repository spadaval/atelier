---
created_at: "2026-11-10T21:49:02.103352734+00:00"
id: "atelier-1sbw"
issue_type: "task"
labels:
- "redesign"
workflow_branch:
  branch_base: master
  work_branch: atelier-1sbw
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-fcb7"
    type: "advances"
  - kind: "issue"
    id: "atelier-jciw"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Write the converter from today's records to the new layout, with a `--verify` mode"
updated_at: "2026-11-10T22:56:25.330037055+00:00"
---

## Goal

Write the script that moves a repository from today's records to the new ones, so the cutover is one reviewed, repeatable step and not hand work in five repositories. `scripts/cutover/convert_records.py` reads a repository's old `.atelier/` and writes the whole new one: `config.toml`, `missions/` and `items/`.

## Done when

- [x] Run in a repository with the old layout, the script replaces `.atelier/` with the new layout and nothing else of the old one remains there: no `issues/`, `reviews/`, `evidence/`, `runtime/` or `workflow.yaml`. It changes no file outside `.atelier/` except the strategy files it moves, below. It makes no commit.
- [x] Every open record becomes one new record with the same ID. A mission becomes `missions/<id>/mission.md`; every other open issue becomes `items/<id>/item.md`. Done, declined and superseded records are not carried over. Status maps as: `draft` and `backlog` to `draft`; `todo` to `ready`; `in_progress` to `ready`, with each such record named in the script's output, since a branch's work is not converted. A record made ready carries `ready`, with `conversion` as who and the old record's `updated_at` as when.
- [x] An item keeps its title, priority, labels, creation time and its Goal, Done when and Scope; its other sections and its Notes follow as text, so nothing an agent wrote on an open item is lost. Its issue type becomes a label, except `task`. Its `mission` is the mission it advances, the first by ID when it names several, or, when it advances none itself, the mission its nearest parent advances; a parent that is not a mission is named in the item's text. `blocked_by` keeps the blockers that are still open.
- [x] An open decision record becomes an item labelled `needs-human` whose Goal is the question and whose Done when has one line, that the question is answered and the answer is written in the item's Report. Wherever an old record lacks a section the new format requires, the script writes that heading with one fixed line saying it was not specified, so every record it writes is valid. Evidence records, and review records of every kind, are not carried over; Git keeps them.
- [x] A mission's `mission.md` takes Why, Outcome, Success criteria, Scope and Authority from the strategy file its record links, and the rest of that file becomes `design.md` beside it; the strategy file is removed from where it was. What the mission's own record holds beyond those sections, such as its Background and its Notes, becomes `notes.md` beside them, so no mission loses text. A mission with no strategy file takes its sections from its record.
- [x] Moving a strategy file breaks no link the script can see. Relative Markdown links in the moved text are rewritten to resolve from the new place, and a test resolves each. In the records the script writes, a mention of a moved file's old path becomes its new path. Every other tracked file that still names a moved file's old path is listed in the script's output and left unchanged.
- [x] `config.toml` holds the slug, the base branch, each shell merge check and the forge from `workflow.yaml`, with `ci_workflow` only where `workflow.yaml` names one and the repository has that workflow file.
- [x] The script's output says what it did in counts: records converted by kind, records left behind by kind and status, and each thing it could not place, by ID. It exits non-zero and writes nothing when it meets an old record it cannot read.
- [x] Run twice on the same old commit it writes the same bytes: no clock, no random value and no ordering that depends on the file system. `--verify`, run in a converted tree with the old commit named, converts that commit afresh and exits non-zero, listing the differences, unless the tree's `.atelier/` and moved files match byte for byte.
- [x] Tests run the script on a small old-layout repository built by the test, with a record of each kind and status above, and check each line of this list; they run with the repository's other Python tests in CI, or the Report says how they are run.
- [x] On a scratch clone of each of the five repositories (atelier, astromech, demiurge, life-on-rails, cosplay-studio), the script converts, `--verify` passes, and the new binary built from `master` runs `status` and `list` on the converted tree with exit 0 and its `check` finds no problem, which shows every record is valid in the new format. `check` is atelier-pz1y's and is the only part of this item that waits on another: do everything else first, and take `master` in once it is there. The Report gives, for each repository, the counts the script printed and the time `status` and `list` took, and nothing of any record's content.

## Scope

`scripts/cutover/convert_records.py` and its tests. It is a script for the cutover and is deleted after it; it is not part of the binary, and `next/` gains no code for the old layout. No flag but `--verify`. The four product repositories under `/home/supadava/projects/` are read-only and some are private: work only on scratch clones made with `git clone` under `/home/supadava/.cache/atelier-fcb7/`, commit nothing from them here, send nothing from them to any outside service or model prompt beyond what the work needs locally, and put only counts and timings in this repository's records. Before any command of the new binary runs in a scratch clone, the clone's remotes are removed, so nothing can be pushed. The full rehearsal (`check`, the hooks, one item from ready to done) is atelier-nafc. No file under `crates/` or `next/` is changed.

## Context

Design: `docs/strategies/redesign-architecture.md`, Records and Transition ("2. Rehearse"). The new record formats are what `next/crates/atelier-records` reads; the old ones are in each repository's `.atelier/` and described by `docs/product/` and the root `crates/atelier-records`. `docs/strategies/redesign-product.md`, "What is removed", says what has no place in the new model and why. The mission cut `merge_strategy` from `config.toml`. Deferred findings on done items are not carried over, as the architecture specification says.

This is the first of step E's two items in `docs/strategies/redesign-2-core.md`.

## Notes

- 2026-11-10 [decision] The orchestrator reworded the Done when line on the mission an item takes: an item that advances no mission itself takes the mission of its nearest ancestor that advances one. Its parent still appears in its Links section.
  Reason: Without it, a mission's lead would not see those items and close on the mission would not wait for them; 35 open items (12 in astromech, 23 in cosplay-studio) are in a mission only through their parent.
  Rejected: Leave such items with no mission and only name the parent in their Links section

## Report

All eleven Done when lines hold. `scripts/cutover/convert_records.py` (standard library only) converts a scratch clone of each of the five repositories. On each, `--verify` passes, a second run from the same old commit gives the same bytes, and the new binary built from this branch after merging `master` at 3d58a88b (atelier-pz1y's `check`) runs `check` ("Every record is valid"), `status` and `list` with exit 0. Every record the script wrote (539 in all) reads back byte for byte through the new binary's writer: `atelier set` with an unchanged value on each left `git diff` empty. `--verify` exits 1 and names the file when one byte of a converted item changes. `scripts/cutover/test_convert_records.py` has 14 tests, one or more per Done when line, on an invented old-layout repository. Each of 34 deliberate breakages of the script failed the test for its rule: statuses, labels, mission choice and the nearest-ancestor rule, blockers, decisions, section demotion, link rewriting, config, determinism, `--verify` and unreadable records. The tests are not in CI, and neither are `scripts/scorecard/`'s. Run them with `TMPDIR=<a directory outside any Git repository> python3 -m unittest discover scripts/cutover`. One test also runs `next/target/debug/atelier status` and `list` on the converted fixture, and skips when that binary is not built.

| Repository | Missions | Items (ready) | From decisions | Closed left | Reviews / evidence files left | Strategies moved | Convert | `check` | `status` | `list` |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| atelier | 9 | 172 (32) | 2 | 374 | 1397 / 819 | 6 | 0.20 s | 0.06 s | 0.33 s | 0.29 s |
| astromech | 4 | 240 (85) | 2 | 403 | 2855 / 801 | 3 | 0.28 s | 0.07 s | 0.43 s | 0.41 s |
| demiurge | 1 | 19 (2) | 2 | 135 | 410 / 105 | 1 | 0.09 s | 0.03 s | 0.04 s | 0.03 s |
| life-on-rails | 3 | 47 (23) | 2 | 56 | 224 / 173 | 3 | 0.12 s | 0.03 s | 0.07 s | 0.06 s |
| cosplay-studio | 7 | 37 (30) | 1 | 54 | 304 / 126 | 1 | 0.08 s | 0.03 s | 0.05 s | 0.05 s |

Other counts the script printed, in the same order: blocked_by entries dropped because the blocker is closed: 16, 104, 3, 8, 8. Records made ready from `in_progress`: 0, 1, 0, 0, 0. Records given a Links section: 72, 83, 9, 19, 34. Items given their nearest ancestor's mission: 0, 12, 0, 0, 23. Tracked files that still name a moved strategy's old path: 3, 1, 4, 0, 3.

### Decisions

- Old strategy files put their sections at `#`. Their mission sections are demoted one level into `mission.md` (`## Why`, and `### ` inside), matched without regard to case (`Success Criteria`). `design.md` keeps the rest of the file as written, front matter included.
- A section that both the strategy and the mission's record have is taken from the strategy, and the record's copy goes to `notes.md`, so nothing is lost; atelier-fcb7's own Outcome is one.
- A mention of a moved strategy's old path becomes its mission's directory, `.atelier/missions/<id>/`, since the file's text is now in two files there. A link with a fragment goes to the file that holds that section, and a same-file anchor that crosses from `mission.md` to `design.md` gains the file name.
- A relative link is rewritten only when it resolved from the old place. Old records write repository-root links (`docs/...`) that never resolved from `.atelier/issues/`: 16 in atelier and astromech, left as written. Astromech's strategies hold 8 links that were already broken, and they stay pointed where they pointed.
- The old tool reads `advances` as one edge that either end may hold, and `blocks` as the other side of `blocked_by`, so the script does too. A relationship no new field holds (a parent, `related`, `discovered_from`, a second mission, a blocker that is a mission) is listed in a `## Links` section at the end of the record, with the other record's title when it exists.
- A decision record becomes an item labelled `decision` and `needs-human`: the type label the item rule gives every type but `task`, plus the label the decision rule names.
- `--verify` makes a `git clone --shared` of the repository in `$TMPDIR`, checks out the named commit there, converts it, and compares with every file under `.atelier/`, ignored by Git or not (atelier-t4hu), plus the absence of each moved strategy. The new binary's `check`, `status` and `list` leave no file there.
- A relative link is found in all three Markdown forms: inline, inline with a title, and a reference definition (atelier-muf9).
- Front matter is written as `serde_yaml` and `chrono` write it (plain or single-quoted strings, `Z` for UTC, fractions of 0, 3, 6 or 9 digits), so the new tool's first write of a converted record changes only the field it sets.

### Surprises

- Old missions also hold `advances` entries naming their items, and old times carry trailing zeros (`.379046000`) that chrono drops; both were found by the round trip and the surveys, and both are handled.
- In two repositories, open items belong to a mission in the old tool only because their parent advances it (12 in astromech, 23 in cosplay-studio). The orchestrator reworded the Done when line so that such an item takes its nearest ancestor's mission (decision note on this item). The script walks up the first parent by ID, through closed ancestors too, and lists each such item with the ancestor it took the mission from. The parent stays in the item's Links section.
- One repository has records whose ID prefix is not its configured slug. The new validator accepts any slug-shaped prefix, so they convert under their IDs.

### Open items

- The tracked files that still name a moved strategy's old path (11 in all, listed by the script) are the cutover runbook's to update or leave.
- A strategy that was alone in its directory leaves that directory empty on disk (life-on-rails); Git does not track empty directories.
- For the rehearsal and the cutover: run the script from inside the repository, on a clean checkout of the base, with Python 3.11 or later and `git` on the PATH; commit its output, then run `--verify <the base commit before the conversion>`. It makes no commit and does not touch remotes.
