---
created_at: "2026-06-21T17:14:25.457730058+00:00"
id: "atelier-53bu"
issue_type: "mission"
labels:
- "mission"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-7jma"
    type: "advances"
  - kind: "issue"
    id: "atelier-9n3r"
    type: "advances"
  - kind: "issue"
    id: "atelier-f9ci"
    type: "advances"
  - kind: "issue"
    id: "atelier-nbhp"
    type: "advances"
  - kind: "issue"
    id: "atelier-ncq9"
    type: "advances"
  - kind: "issue"
    id: "atelier-vays"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-21T20:26:44.440874585+00:00"
status: "closed"
title: "Make workflow obligations explicit and minimal"
updated_at: "2026-06-21T20:26:44.440874585+00:00"
---

## Description

Remove implicit product obligations from Atelier's hardcoded mission and evidence paths. Missions should become declared workflow-owned objective work rather than a special command/lifecycle universe, and evidence should remain a useful optional capability rather than a universal tax on issue completion. The shared design principle is that Atelier hardcodes capabilities, safety checks, and simple recovery hints, while repository workflow policy chooses which obligations apply.

## Outcome

### Constraints

- Keep docs/product/zen.md central: workflow enforces the minimum, every feature justifies its cost, status surfaces guide recovery, and obsolete paths are deleted once replacements are clear.
- Unify overlapping mission-rework and evidence-requirement work under one mission so command surface, workflow policy, status/readiness, storage/projection, and validation move together.
- Keep workflow.yaml concise: it should choose validators and parameters, not carry large UI copy or reimplement built-in behavior.
- Validator failures need only simple help hints; do not introduce a large structured validator presentation framework unless a later issue explicitly proves the need.
- Keep command surfaces distinct: root status orients, issue status inspects one record, grouped issue list browses contextual work, and any flat table/inventory view is explicit rather than inferred from filters.
- Do not preserve compatibility aliases or mission/evidence fallback behavior unless a human explicitly asks for a compatibility window.

### Risks

- Removing hardcoded mission and evidence behavior can hide real readiness problems unless configured validators surface clear failures and next commands.
- The mission-rework scope and evidence-requirement scope overlap in status/readiness surfaces, so sequencing must prevent duplicate implementations.
- Existing tests, docs, and tracker records may assume mission commands, required Evidence sections, or attached proof; cleanup must remove the old path directly rather than adding shims.

## Evidence

- Manual check: Ordinary issue workflows can close work without evidence when workflow.yaml does not configure an evidence validator.
- Manual check: When a workflow configures evidence or objective validators, transition options, blocked transitions, issue show/status, and root status surface the configured validator failure with a simple help hint.
- Manual check: Mission/objective records are declared through issue/workflow policy and no longer depend on mission-only command, lifecycle, projection, or storage magic.
- Manual check: Mission/objective discovery is available through an explicit issue browsing or table/inventory surface without relying on no-argument issue status.
- Manual check: Existing mission records migrate or rebuild into the target declared-policy model without duplicate mission storage paths.
- Manual check: Root help, role guides, docs, Agent Factory guidance, and focused tests agree on the replacement command surface and no longer teach removed mission-only or mandatory-evidence behavior.
- Manual check: Independent end-to-end validation maps the mission outcomes to linked epics, focused tests, command transcripts, lint, and command-surface searches.

## Notes

Migrated from `.atelier/missions/atelier-53bu.md` as a declared mission objective issue.
