---
created_at: '2026-06-11T02:43:59.824955638+00:00'
id: atelier-8bky
issue_type: mission
labels:
- mission
priority: P2
relationships:
  blocks: []
  children: []
  attachments:
  - kind: evidence
    id: atelier-np2r
    role: validates
  - kind: evidence
    id: atelier-p93l
    role: validates
  relates:
  - kind: issue
    id: atelier-0se4
    type: advances
  - kind: issue
    id: atelier-2h0d
    type: advances
  - kind: issue
    id: atelier-2q9p
    type: advances
  - kind: issue
    id: atelier-bia4
    type: advances
  - kind: issue
    id: atelier-eq2d
    type: advances
  - kind: issue
    id: atelier-esh8
    type: advances
  - kind: issue
    id: atelier-fkgl
    type: advances
  - kind: issue
    id: atelier-fspm
    type: advances
  - kind: issue
    id: atelier-iv68
    type: advances
  - kind: issue
    id: atelier-j75o
    type: advances
  - kind: issue
    id: atelier-kaei
    type: advances
  - kind: issue
    id: atelier-ky3z
    type: advances
  - kind: issue
    id: atelier-sspj
    type: advances
  - kind: issue
    id: atelier-uuhh
    type: advances
  - kind: issue
    id: atelier-vfqo
    type: advances
  - kind: issue
    id: atelier-vhxa
    type: advances
  - kind: issue
    id: atelier-vqsb
    type: advances
  - kind: issue
    id: atelier-vvlj
    type: advances
  - kind: issue
    id: atelier-xajk
    type: advances
  - kind: issue
    id: atelier-y571
    type: advances
  - kind: issue
    id: atelier-zjb5
    type: advances
schema: atelier.issue
schema_version: 1
closed_at: '2026-06-11T04:35:54.612060029+00:00'
status: closed
title: Remove JSON command output mode and focus human CLI output
updated_at: '2026-06-11T04:35:54.612060029+00:00'
---

## Description

Atelier should retire command-result JSON mode and make the default human-readable CLI the primary supported output surface. The goal is not to pour every previously available JSON field into normal output. Instead, each command should show the small set of details a human or agent needs for the immediate workflow, with clear next commands for related records, history, evidence, dependencies, and validation details.

This mission covers the product and implementation migration from dual human/JSON command output to focused human output. It should remove command-level JSON result contracts, update docs that still describe JSON as the automation interface, and strengthen human output where JSON had become a crutch for understanding state.

Scope includes command result output modes, formatter paths, CLI flags/help, tests/snapshots, docs, Agent Factory command guidance, and migration validation. Scope intentionally does not assume that diagnostic logging transport, repo-state projection files, export/check internals, or future API surfaces are removed; those boundaries must be confirmed by the inventory/policy work before implementation proceeds.

Recommended subskill: agent-factory orchestrate.

## Outcome

### Constraints

- Do not replace JSON by dumping every machine field into human views; prefer focused views and explicit drill-down commands.
- Keep quiet output minimal for scripting where a command naturally returns an ID, count, or status token.
- Preserve canonical repo-state files and rebuildable projections unless a separate architecture artifact says otherwise.

### Risks

- Existing agents, scripts, and tests may depend on --json; inventory and migration guidance must identify and update those callers.
- Removing a stable machine contract without improving human output could make orchestration harder; validation must exercise real Agent Factory workflows.

## Evidence

- Manual check: CLI help and representative commands no longer advertise or accept command-result --json mode, except for explicitly approved non-result diagnostic surfaces.
- Manual check: Human output fixtures prove selected commands expose enough actionable context without unbounded field dumps.
- Manual check: Docs and Agent Factory guidance no longer describe JSON result output as Atelier's automation contract.

## Notes

Migrated from `.atelier/missions/atelier-8bky.md` as a declared mission objective issue.
