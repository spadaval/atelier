---
created_at: "2026-11-10T21:49:02.103352734+00:00"
id: "atelier-1sbw"
issue_type: "task"
labels:
- "redesign"
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
status: "todo"
title: "Write the converter from today's records to the new layout, with a `--verify` mode"
updated_at: "2026-11-10T21:49:02.103352734+00:00"
---

## Goal

Write the script that moves a repository from today's records to the new ones, so the cutover is one reviewed, repeatable step and not hand work in five repositories. `scripts/cutover/convert_records.py` reads a repository's old `.atelier/` and writes the whole new one: `config.toml`, `missions/` and `items/`.

## Done when

- [ ] Run in a repository with the old layout, the script replaces `.atelier/` with the new layout and nothing else of the old one remains there: no `issues/`, `reviews/`, `evidence/`, `runtime/` or `workflow.yaml`. It changes no file outside `.atelier/` except the strategy files it moves, below. It makes no commit.
- [ ] Every open record becomes one new record with the same ID. A mission becomes `missions/<id>/mission.md`; every other open issue becomes `items/<id>/item.md`. Done, declined and superseded records are not carried over. Status maps as: `draft` and `backlog` to `draft`; `todo` to `ready`; `in_progress` to `ready`, with each such record named in the script's output, since a branch's work is not converted. A record made ready carries `ready`, with `conversion` as who and the old record's `updated_at` as when.
- [ ] An item keeps its title, priority, labels, creation time and its Goal, Done when and Scope; its other sections and its Notes follow as text, so nothing an agent wrote on an open item is lost. Its issue type becomes a label, except `task`. Its `mission` is the mission it advances, the first by ID when it names several; a parent that is not a mission is named in the item's text. `blocked_by` keeps the blockers that are still open.
- [ ] An open decision record becomes an item labelled `needs-human` whose Goal is the question. Evidence records, and review records of every kind, are not carried over; Git keeps them.
- [ ] A mission's `mission.md` takes Why, Outcome, Success criteria, Scope and Authority from the strategy file its record links, and the rest of that file becomes `design.md` beside it; the strategy file is removed from where it was. A mission with no strategy file takes its sections from its record.
- [ ] `config.toml` holds the slug, the base branch, each shell merge check and the forge from `workflow.yaml`, with `ci_workflow` only where `workflow.yaml` names one and the repository has that workflow file.
- [ ] The script's output says what it did in counts: records converted by kind, records left behind by kind and status, and each thing it could not place, by ID. It exits non-zero and writes nothing when it meets an old record it cannot read.
- [ ] Run twice on the same old commit it writes the same bytes: no clock, no random value and no ordering that depends on the file system. `--verify`, run in a converted tree with the old commit named, converts that commit afresh and exits non-zero, listing the differences, unless the tree's `.atelier/` and moved files match byte for byte.
- [ ] Tests run the script on a small old-layout repository built by the test, with a record of each kind and status above, and check each line of this list; they run with the repository's other Python tests in CI, or the Report says how they are run.
- [ ] On a scratch clone of each of the five repositories (atelier, astromech, demiurge, life-on-rails, cosplay-studio), the script converts, `--verify` passes, and the new binary built from `master` runs `status` and `list` on the converted tree with exit 0, which reads every record. The Report gives, for each repository, the counts the script printed and the time `status` and `list` took, and nothing of any record's content.

## Scope

`scripts/cutover/convert_records.py` and its tests. It is a script for the cutover and is deleted after it; it is not part of the binary, and `next/` gains no code for the old layout. No flag but `--verify`. The four product repositories under `/home/supadava/projects/` are read-only and some are private: work only on scratch clones made with `git clone` under `/home/supadava/.cache/atelier-fcb7/`, commit nothing from them here, send nothing from them to any outside service or model prompt beyond what the work needs locally, and put only counts and timings in this repository's records. Before any command of the new binary runs in a scratch clone, the clone's remotes are removed, so nothing can be pushed. The full rehearsal (`check`, the hooks, one item from ready to done) is atelier-nafc. No file under `crates/` or `next/` is changed.

## Context

Design: `docs/strategies/redesign-architecture.md`, Records and Transition ("2. Rehearse"). The new record formats are what `next/crates/atelier-records` reads; the old ones are in each repository's `.atelier/` and described by `docs/product/` and the root `crates/atelier-records`. `docs/strategies/redesign-product.md`, "What is removed", says what has no place in the new model and why. The mission cut `merge_strategy` from `config.toml`. Deferred findings on done items are not carried over, as the architecture specification says.

This is the first of step E's two items in `docs/strategies/redesign-2-core.md`.
