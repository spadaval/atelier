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
status: "backlog"
title: "Write the converter, and rehearse it and the new binary on all five repositories"
updated_at: "2026-11-10T04:45:25.109580512+00:00"
---

## Goal

Prove the converter and the new binary on real data before the cutover depends on them.

## Done when

- [ ] Provisional: `scripts/cutover/convert_records.py` writes a repository's whole new `.atelier/`, including `config.toml` from `workflow.yaml` with no CI workflow where the repository has no Actions; run twice it writes the same bytes, and `--verify` checks a tree against a fresh run. On a scratch clone of each of the five repositories the converted tree passes `--verify` and the new binary's `check`, and one item goes from ready to done in the atelier clone and in one product clone (the mission criterion `rehearsal`). The Report lists, for each repository, where its CI or `AGENTS.md` names Atelier and whether its forge requires a pull request for the base.

## Context

Preliminary. This step is planned in full at its boundary, with one planning review, as `docs/strategies/redesign-2-core.md` says. Target design: `docs/strategies/redesign-product.md` and `docs/strategies/redesign-architecture.md`.

## Unknowns

- What the conversion does with records the new model has no place for, such as evidence and decision records on open items.
