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
    id: "atelier-axfa"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-fcb7"
    type: "advances"
  - kind: "issue"
    id: "atelier-pz1y"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Write the converter, and rehearse it and the new binary on all five repositories"
updated_at: "2026-11-10T01:29:40.242882394+00:00"
---

## Goal

Prove the converter and the new binary on real data before the cutover depends on them.

## Done when

- [ ] Provisional: `scripts/cutover/convert_records.py` writes a repository's whole new `.atelier/`, including `config.toml` from `workflow.yaml` with no CI workflow where the repository has no Actions; run twice it writes the same bytes, and `--verify` checks a tree against a fresh run. It and `atelier-next` pass `check` and complete one item on a scratch clone of each of the five repositories (the mission criterion `rehearsal`). The Report lists, for each repository, where its CI or `AGENTS.md` names Atelier and whether its forge requires a pull request for the base.

## Context

Preliminary. This step is planned in full at its boundary, with one planning review, as `docs/strategies/redesign-2-core.md` says. Target design: `docs/strategies/redesign-product.md` and `docs/strategies/redesign-architecture.md`.

## Unknowns

- What the conversion does with records the new model has no place for, such as evidence and decision records on open items.
