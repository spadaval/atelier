---
created_at: "2026-06-08T17:33:27+00:00"
id: "atelier-000o"
issue_type: "task"
labels:
- "fork"
- "spec"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-000e"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Decide Atelier CLI binary and alias naming"
updated_at: "2026-06-08T19:47:39+00:00"
---

## Description

Resolve the SPEC.md open question: should the binary be `atelier`, a shorter alias such as `atl`, or both? This choice blocks broad rename work because command names, tests, docs, package metadata, and install expectations depend on it.

## Outcome

Outcome was not specified in the legacy issue record.

## Evidence

Evidence was not specified in the legacy issue record.

## Notes

### Resolution

Use `atelier` as the primary and only Milestone 1 CLI binary name. Do not add an `atl` short alias yet, and do not preserve an installed `chainlink` compatibility shim unless a later migration bead proves a concrete compatibility need.

### Rationale

The fork establishment milestone needs a clear product identity before package metadata, command help, tests, generated resources, and docs are renamed. `atelier` is explicit, matches the product name, and avoids early cross-platform alias/install complexity. A short alias can be added later as an additive convenience without changing the canonical command or file layout.

### Alternatives Considered

- `atelier` only: chosen for Milestone 1.
- `atl` only: rejected because it hides the product identity during the fork rename.
- `atelier` with `atl` alias: deferred until command shape and install story are stable.
- Temporary `chainlink` compatibility shim: deferred/rejected for Milestone 1 because this fork is not yet supporting external Chainlink users.
