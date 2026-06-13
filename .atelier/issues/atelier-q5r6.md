---
created_at: "2026-06-13T17:36:35.295164413+00:00"
id: "atelier-q5r6"
issue_type: "epic"
labels:
- "epic"
- "validation"
- "workflow"
priority: "P0"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-09sx"
  - kind: "issue"
    id: "atelier-2bpd"
  - kind: "issue"
    id: "atelier-ewpk"
  - kind: "issue"
    id: "atelier-fyms"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Epic: Validate and close repo-defined workflow rollout"
updated_at: "2026-06-13T17:37:03.146921348+00:00"
---

## Description

Validate that repo-owned issue workflows are now enforced and remove the stale
surfaces that previously made workflow policy look more implemented than it
was. This epic owns the cleanup, independent validation, known-gap closure, and
mission closeout path after the contract and implementation epics land.

Children own the executable work: `atelier-ewpk` removes misleading legacy
workflow surfaces, `atelier-fyms` independently validates the finished behavior,
`atelier-09sx` tracks the original docs-overclaim bug, and `atelier-2bpd`
performs final mission closeout.

## Outcome

- Normal docs, help, Agent Factory guidance, and next-action surfaces describe
  workflow init/check/migration, configured transitions, start, close, and
  abandon without presenting `workflow validate`, `finish`, or root
  `atelier.workflow.yaml` hooks as the normal workflow.
- Independent validation covers starter policy, workflow check, migration,
  start, blocked transitions, close with evidence, lightweight spike close,
  archive, missing YAML, and unmigrated-record failures.
- Mission closeout either closes `atelier-09sx` as resolved by enforced
  repo-owned workflow behavior or leaves it open with a precise residual gap.

## Evidence

- Residue-scan and help/doc transcript evidence from `atelier-ewpk` classifies
  remaining legacy references as removed, historical, or deferred.
- First-class validation evidence from `atelier-fyms` records scenario
  transcripts, pass/fail classifications, residual risks, and follow-up IDs.
- Closeout evidence from `atelier-2bpd` maps mission validation criteria,
  child work, and evidence records line by line, then includes final
  `atelier lint`, `atelier export --check`, `atelier doctor`, focused workflow
  tests, and `git diff --check` proof or explicit deferrals.
