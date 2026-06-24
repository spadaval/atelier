---
created_at: "2026-06-24T20:37:07.897614288+00:00"
id: "atelier-5km8"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-8c91"
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-z0ll"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Make evidence list ordering and omitted wording truthful"
updated_at: "2026-06-24T20:37:07.897614288+00:00"
---

## Description

Fix evidence list browse output so bounded defaults do not hide arbitrary records while claiming omitted records are older. Current `atelier-sqlite` record listing orders by ID, but `evidence list` takes the first 20 and reports older records hidden. Audit evidence points to `crates/atelier-sqlite/src/records.rs:227`, `crates/atelier-cli/src/commands/evidence.rs:293`, and `crates/atelier-cli/src/commands/evidence.rs:318`.

Constraints:
- Default browse output must state its sort order truthfully.
- Prefer newest relevant proof first for human evidence browsing unless a product decision chooses a different explicit sort.

Risks:
- Recent validation proof can be hidden behind arbitrary ID ordering, causing operators to miss the evidence they need.

## Outcome

`evidence list` default ordering and omitted-record wording are truthful. Either evidence is ordered by captured/updated time descending for browse output, or the UI removes `older` wording and documents the actual sort/limit contract.

## Evidence

- Regression test creates evidence records with IDs and timestamps that sort differently, then proves browse output and omitted wording are truthful.
- Focused transcript shows bounded evidence list output with correct omitted-count wording.
- `git diff --check` passes for the output and test changes.
