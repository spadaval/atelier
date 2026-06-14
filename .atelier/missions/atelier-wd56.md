---
created_at: "2026-06-10T20:37:05.079899571+00:00"
id: "atelier-wd56"
labels:
- "mission"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-34ap"
    type: "advances"
  - kind: "issue"
    id: "atelier-lold"
    type: "advances"
  - kind: "issue"
    id: "atelier-xf1y"
    type: "advances"
  - kind: "issue"
    id: "atelier-xz8u"
    type: "advances"
  - kind: "issue"
    id: "atelier-ytmi"
    type: "advances"
schema: "atelier.mission"
schema_version: 1
status: "closed"
title: "Simplify Atelier command surface and delete legacy command code"
updated_at: "2026-06-10T21:39:24.666159172+00:00"
---

## Intent

Simplify Atelier's CLI by deleting old compatibility commands, inherited utility surfaces, and the code that only exists to support them. The end state is a smaller command surface centered on documented core workflows, with hard removal of non-core compatibility paths instead of warning shims.

Protected core surfaces are: init, issue, dep, mission create/show/list/update, plan, evidence, link, workflow validate, work, worktree, canonical export, rebuild, import-beads, lint, doctor, and Agent Factory commands documented in AGENTFACTORY.md.

Removal should include command variants, dispatch arms, command modules, tests, docs, and safe cleanup of unused internal storage/runtime paths.

## Constraints

- Preserve documented core command surfaces and Agent Factory workflows.
- Remove old compatibility surfaces hard, without deprecation shims.
- Do not corrupt existing canonical state; runtime database cleanup must be rebuild-safe or migration-safe.

## Risks

- Deleting command code can accidentally remove helpers still used by core workflows.
- Tests and docs may still encode hidden alias behavior.

## Validation

- Primary help lists only core commands.
- Removed commands fail as unknown commands.
- Agent Factory issue workflows, mission show, export, rebuild, lint, and doctor still pass.
