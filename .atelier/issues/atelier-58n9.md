---
created_at: "2026-06-17T20:03:01.915151595+00:00"
id: "atelier-58n9"
issue_type: "epic"
labels:
- "architecture"
- "bundle"
- "declarative"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-98mo"
  children:
  - kind: "issue"
    id: "atelier-7kh1"
  - kind: "issue"
    id: "atelier-jmmn"
  - kind: "issue"
    id: "atelier-mrj5"
  - kind: "issue"
    id: "atelier-tkiw"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-17T22:17:59.511855748+00:00"
status: "done"
title: "Define and rework declarative bundle apply semantics"
updated_at: "2026-06-17T22:17:59.511855748+00:00"
---

## Description

Define the exact semantics for a declarative bundle apply feature, then move it
out from under the plan-record command surface. A bundle is an authored,
temporary package of tracker records and relationships. Atelier validates it,
previews the effect, and applies it through canonical Markdown mutations only
when explicitly confirmed.

The current implementation lives at `atelier plan apply <file.json>`, accepts a
bulk-plan schema with issues, missions, milestones, plans, and evidence, and
claims atomic behavior while writing records sequentially. That makes the
interface awkward and the safety contract underspecified. The replacement
surface should be `atelier bundle preview <file>` and
`atelier bundle apply <file> --yes`.

## Outcome

- A documented bundle contract defines file input, schema identity, allowed
  record kinds, create-only versus update semantics, dry-run/preview output,
  idempotency expectations, failure behavior, workflow defaults, and relationship
  resolution.
- The command surface is renamed away from `plan apply` to `atelier bundle`
  commands.
- `atelier bundle preview <file>` is the safe review path, and mutating apply
  requires explicit confirmation.
- The schema name is `atelier.bundle`.
- The v1 schema excludes first-class plans and milestones.
- Apply either commits all canonical writes or leaves tracker records unchanged.
- New issues created by bundle apply use workflow-aware defaults instead of a
  parallel hard-coded lifecycle.

## Evidence

- Fixture tests cover valid apply, confirmation-required apply, preview, rejected
  plan/milestone inputs, duplicate client refs, missing refs, invalid workflow
  statuses, and mid-apply failure atomicity.
- Command transcripts show the new command accepts a real file path and rejects
  stdin-style usage.
- `atelier lint`, `atelier export --check`, `atelier doctor`, and
  `git diff --check` pass.
