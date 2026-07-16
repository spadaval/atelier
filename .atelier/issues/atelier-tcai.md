---
created_at: "2026-07-06T17:26:56.041037321+00:00"
id: "atelier-tcai"
issue_type: "bug"
labels:
- "bundle"
- "cli"
- "safety"
fields:
  workflow_branch:
    branch_base: master
    integration_target: master
    merge_strategy: squash
    owner_issue_id: atelier-tcai
    owner_kind: issue
    review_target: master
    work_branch: bug/atelier-tcai
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-07-16T18:30:27.719305404+00:00"
status: "done"
title: "Make bundle preview and apply share a normalized graph plan"
updated_at: "2026-07-16T18:30:27.719305404+00:00"
---

## Description

Bundle preview hardcodes a zero relationship count, while bundle apply mutates parent, dependency, block, advances, and validates edges but reports only advances and validates. The review gate therefore misrepresents the graph that apply will create. Replace the duplicated summary interpretations with one normalized, deterministic graph plan shared by preview and apply. Preserve atomic apply behavior and current canonical relationship semantics. Reconcile the durable bundle contract where its preview/apply description has drifted from the current human-output interface.

## Outcome

- After validation, bundle input is normalized into one deterministic plan containing records, notes, and every parent, dependency, block, advances, and validates edge with an unambiguous source, target, and role.
- `atelier bundle preview` reports the complete planned relationship set without mutation, and `atelier bundle apply` executes and reports the same set after client references are mapped to durable IDs.
- Preview and apply relationship counts cannot diverge because both are derived from the same planned edges rather than independent field scans.
- Focused tests cover every supported edge form, deterministic ordering, preview non-mutation, apply persistence, preview/apply parity, and export/rebuild preservation; documentation matches the surviving human-output contract.

## Evidence

- `cargo nextest run -p atelier-cli --test cli_integration bundle` covers every supported bundle edge form, deterministic preview output, preview non-mutation, duplicate normalized-edge rejection, apply persistence, authored note ordering, and export/rebuild preservation.
- `cargo check -p atelier-cli` proves the affected CLI crate compiles.
- `cargo fmt --all -- --check` and `git diff --check` prove formatting and whitespace health.
