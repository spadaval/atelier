---
acceptance: []
created_at: "2026-06-10T00:22:41.105938194+00:00"
evidence_required: []
id: "atelier-0023"
issue_type: "task"
labels:
- "cli"
- "identity"
- "migration"
- "storage"
- "task"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-0005"
  - kind: "issue"
    id: "atelier-000u"
  - kind: "issue"
    id: "atelier-001n"
  - kind: "issue"
    id: "atelier-001u"
  - kind: "issue"
    id: "atelier-0027"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Cut over record identity to project-scoped random IDs"
updated_at: "2026-06-10T02:18:00.651875871+00:00"
---

Replace inherited numeric and typed-prefix record identity with one canonical project-scoped random ID format such as `atelier-z1p8`. This is a hard cutover, not a compatibility layer.

## Scope

- Add a record ID allocator using `<project-slug>-<random-base36>` with a four-character default suffix and collision retry.
- Use one global ID namespace across issues, missions, milestones, plans, evidence, and future first-class records.
- Migrate existing SQLite rows, `.atelier-state` filenames, front matter, graph edges, parent/dependency references, tests, fixtures, and docs from numeric/typed-prefix IDs to the new ID form.
- Remove numeric shorthand and typed-prefix style target identity from command parsing, JSON output, export, rebuild, and link APIs.
- Keep record kind as metadata and validation context, not encoded in the ID.

## Out Of Scope

- Maintaining numeric or typed-prefix aliases after cutover.
- Semantic slugs as primary identity.

## Acceptance

All user-facing commands accept and emit only project-scoped random record IDs; canonical export filenames and graph references use the same IDs; rebuild preserves IDs exactly; concurrent record creation does not rely on a shared sequence counter; existing repository state is migrated; tests and fixtures no longer depend on numeric or typed-prefix record IDs.

## Validation

- `cargo fmt -- --check`
- `cargo test`
- `git diff --check`
- `atelier lint`
- `atelier export --check`
- `atelier doctor`
