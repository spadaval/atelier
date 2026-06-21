---
created_at: '2026-06-13T20:35:22.674703945+00:00'
id: atelier-man9
issue_type: mission
labels:
- mission
priority: P2
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: issue
    id: atelier-cve1
    type: advances
  - kind: issue
    id: atelier-foy0
    type: advances
  - kind: issue
    id: atelier-lpnr
    type: advances
  - kind: issue
    id: atelier-p2m2
    type: advances
  - kind: issue
    id: atelier-yqg9
    type: advances
schema: atelier.issue
schema_version: 1
closed_at: '2026-06-14T00:28:17.702989645+00:00'
status: closed
title: Stabilize and simplify Atelier CLI and tracker model
updated_at: '2026-06-14T00:28:17.702989645+00:00'
---

## Description

Make Atelier simple, powerful, and consistent after recent rapid CLI work. The mission audits and removes command ambiguity, normalizes canonical Markdown/data model shape, and reduces code/module complexity that makes ordinary work harder to reason about.

## Outcome

### Constraints

- Commands must have one clear purpose, focused default output, and explicit drill-down flags or subcommands for extra detail.
- Canonical Markdown must be readable and logically structured without generic escaped payload fields for normal records.
- Do not preserve compatibility aliases, fallback readers, or old-command shims unless a human explicitly asks for a compatibility window.
- Contract and schema decisions land as durable docs/ADRs before dependent implementation.

### Risks

- Some awkward implementation may be a symptom of an unresolved product contract; implementation work should not hide those decisions.
- The repo has substantial dirty work already, so workers must preserve unrelated changes and isolate stabilization edits.

## Evidence

- Manual check: CLI closeout includes help transcripts proving each visible command has a clear purpose, predecessor surfaces are removed or explicitly classified, default output is focused, and drill-down remains available.
- Manual check: Data-model closeout includes representative issue, mission, plan, and evidence Markdown records that are readable, deterministic, linted, rebuildable, and free of unowned random fields.
- Manual check: Architecture closeout includes residue searches and review evidence proving dead inherited code, god modules, and misplaced boundaries were removed or assigned explicit follow-up work.
- Manual check: Final mission audit maps each outcome to linked work and evidence, then records atelier lint, atelier export --check, atelier doctor, focused CLI tests, and git diff --check proof.

## Notes

Migrated from `.atelier/missions/atelier-man9.md` as a declared mission objective issue.
