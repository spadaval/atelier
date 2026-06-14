---
created_at: "2026-06-13T17:36:19.801247635+00:00"
id: "atelier-fmri"
issue_type: "epic"
labels:
- "epic"
- "workflow"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-c64h"
  - kind: "issue"
    id: "atelier-lv4s"
  - kind: "issue"
    id: "atelier-oku1"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-13T18:10:51.052326785+00:00"
status: "done"
title: "Epic: Define repo-owned workflow contract"
updated_at: "2026-06-13T18:10:51.052326785+00:00"
---

## Description

Define the contract layer for repo-owned issue workflows before broad command
changes land. This epic groups the artifact updates and the first parser/checker
implementation needed to make `.atelier/workflow.yaml` a real tracked policy
source instead of a product-doc promise.

Children own the executable work: `atelier-c64h` defines the v1 YAML contract,
`atelier-oku1` records the architecture and glossary decisions, and
`atelier-lv4s` implements strict policy loading and checking against the
documented contract.
- Product, architecture, and domain docs define the v1 issue-workflow contract
  using `.atelier/workflow.yaml`, status categories, built-in validators,
  guidance, terminal states, and issue-type workflow selection.
- The implementation can load and check the documented policy file without
  adding custom issue types, custom validators, hooks, waivers, expression
  engines, or workflow projection tables.
- Downstream transition, migration, orientation, cleanup, validation, and
  closeout issues can cite this epic's child proof instead of relying on private
  planning context.
- Child evidence for `atelier-c64h`, `atelier-oku1`, and `atelier-lv4s` maps the
  documented contract to implemented parser/checker behavior.
- Epic closeout note or evidence classifies every Outcome bullet as covered,
  blocked, deferred, or not applicable before the epic closes.
- `atelier lint` and `atelier export --check` pass after the child records and
  contract artifacts are updated.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
