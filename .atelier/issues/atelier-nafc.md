---
created_at: "2026-11-10T00:56:08.858805140+00:00"
id: "atelier-nafc"
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
    id: "atelier-1sbw"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-axfa"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-dxrj"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-fcb7"
    type: "advances"
  - kind: "issue"
    id: "atelier-pz1y"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-t2jo"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-ym3z"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-zrss"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Rehearse the converter and the new binary on all five repositories, with one item taken to done in two"
updated_at: "2026-11-10T21:49:02.946155827+00:00"
---

## Goal

Prove the converter and the new binary on real data before the cutover depends on them: every repository converts and checks clean, and a whole item goes from ready to done on converted records in two of them. This is the mission criterion `rehearsal`.

## Done when

- [ ] On a fresh scratch clone of each of the five repositories (atelier, astromech, demiurge, life-on-rails, cosplay-studio), `scripts/cutover/convert_records.py` converts, the result is committed in the clone, `--verify` passes, and the new binary's `check` passes.
- [ ] In each converted clone, `atelier setup` installs the Git hooks, a commit of a deliberately broken record is refused by the pre-commit hook, and `status` and `list` run; the Report gives the time each took, with the number of local branches, and says whether reading started items from their branches is fast enough in the repository with the most.
- [ ] One item goes from ready to done in the converted atelier clone and in one converted product clone, by the commands alone: `start`, a small real change, `submit`, `review` and `verdict approve` by a second thread identity, and `merge`, with that repository's converted merge checks running. In each clone the forge's remote is a local bare repository and `gh` is the test double, so the forge path runs and nothing reaches the network.
- [ ] The Report lists, for each repository, where its CI workflows and its `AGENTS.md` name Atelier or one of its commands, and whether its forge requires a pull request to change the base, as facts mission 3's cutover needs. It gives counts, file names and process facts, and nothing of any private repository's content.
- [ ] Each defect the rehearsal finds in the converter or the new binary is fixed on this item's branch when it is small and inside the specifications, or filed as an item of this mission and named in the Report. The rehearsal is run again from fresh clones after the last fix.

## Scope

The rehearsal, and small fixes to `scripts/cutover/convert_records.py` or `next/` that it shows are needed. The four product repositories under `/home/supadava/projects/` are read-only and some are private: work only on scratch clones under `/home/supadava/.cache/atelier-fcb7/`, whose remotes are replaced by local bare repositories before any command of the new binary runs in them; commit nothing from them here; send nothing from them to any outside service; and report process facts, never their content. The new binary is run from `next/target/` and installed nowhere, and no real repository's hooks, records or configuration are touched. Rolling a cutover back, the runbook and the replacement `AGENTS.md` texts are mission 3's. No file under `crates/` is changed.

## Context

Design: `docs/strategies/redesign-architecture.md`, Transition ("2. Rehearse") and Testing. The converter is atelier-1sbw; `check` and the hooks are atelier-pz1y, `setup` is atelier-ym3z. The strategy's Risks name the one performance question: reading started items from their branches on every `status` may be slow in the repository with the most branches, and if it is, the fix is a cache inside one command's run, not a stored index.

This is the second of step E's two items in `docs/strategies/redesign-2-core.md`, and the mission's last.
