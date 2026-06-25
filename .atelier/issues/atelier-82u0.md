---
created_at: "2026-06-24T19:26:22.591395612+00:00"
id: "atelier-82u0"
issue_type: "epic"
labels: []
review:
  kind: pull_request
  number: 28
  provider: forgejo
priority: "P1"
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
status: "review"
title: "Consolidate health, relationships, review, and admin escape hatches"
updated_at: "2026-06-25T01:43:10.489434546+00:00"
---

## Description

Collapse secondary command-family sprawl after the read surface contract is clear. Move toward one health/check surface, one generic relationship mutation model, a smaller review open/show/submit/resolve/merge model, provider setup under review/admin ownership, workflow-owned branch lifecycle, root `bundle` retained for graph preview/apply until a better owner is proven, and root `man` retained as smart guidance. Extract the underlying relationship, review, branch/workflow, health, bundle, and guidance decisions out of CLI command modules into domain/app services before simplifying the public commands.

Constraints:
- Start only after `atelier-55tk` settles the public command/read contract.
- Keep `bundle` as root graph preview/apply until a better owner is deliberately designed.
- Keep `prune` separate; this issue only ensures cleanup guidance does not leak into normal command workflow.
- Provider, branch repair, projection repair, and destructive maintenance paths should be hidden/admin or transition-guided unless they are necessary public workflow surfaces.

## Outcome

Secondary command families are collapsed into clearer owners: health and repair diagnostics live under one check surface, issue relationships use one generic mutation model, review commands focus on the review lifecycle, provider setup is not a top-level workflow, branch lifecycle belongs to the workflow that needs it, bundle remains the bounded owner for graph preview/apply unless a clearly better owner appears, and man remains guidance rather than workflow. CLI commands delegate policy, validation, graph, review, bundle, and health decisions to domain/app services.

## Evidence

- Focused tests or transcripts prove health/check, relationship mutation, review lifecycle, branch/workflow guidance, bundle preview/apply, and man guidance route through the new owners.
- Manual check: `rg -n 'forgejo|branch|maintenance|lint|doctor' AGENTS.md docs/product/cli-surface.md crates/atelier-cli/src/commands/man.rs` shows provider setup, branch repair, destructive maintenance, and old health commands are not taught as normal workflow.
- Domain/app tests prove relationship, review, bundle, health, and branch/workflow decisions are evaluated outside CLI renderers.
