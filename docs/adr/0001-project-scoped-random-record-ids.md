# ADR 0001: Project-Scoped Random Record IDs

## Status

Accepted.

## Context

Atelier previously documented typed numeric IDs such as `ISS-0001`,
`MIS-0001`, and `PLAN-0001`, while the CLI also exposed shorter numeric issue
references such as `#53` or `53`. That creates three user-visible identities
for a record: canonical ID, shorthand ID, and title.

Atelier is local-first and Git-merged. Agents may create records concurrently in
different worktrees. Sequential numeric allocation creates merge conflicts or
requires coordination. Semantic names are readable but drift when scope or
titles change.

## Decision

Atelier records use one canonical, human-facing ID:

```text
<project-slug>-<random-base36>
```

For this repository, examples look like:

```text
atelier-z1p8
atelier-k7mq
atelier-4x9t
```

The random suffix is lowercase base36 and defaults to four characters. The
allocator must retry on local collisions and may use a longer suffix when a
repository policy chooses to do so. The project slug scopes IDs across
repositories and makes copied records recognizable outside their original
checkout.

Record kind is metadata, not part of identity. The same ID shape is used for
issues, missions, milestones, plans, evidence, and future first-class records.
Titles are mutable display text and never identity.

## Consequences

- The canonical ID and the ID humans type are the same string.
- Exported filenames are derived directly from record IDs.
- Links use record kind plus ID for validation, but the ID string itself does
  not encode kind.
- Concurrent creation in multiple worktrees does not need a shared sequence
  counter.
- Existing numeric SQLite IDs and `ISS-0001` exported records are migrated to
  project-scoped random IDs during cutover.
- The CLI, export/rebuild, links, bulk plans, and Mission Control use the new
  ID form directly, without maintaining numeric or typed-prefix aliases.
- Importers may record external source IDs as provenance only when useful, but
  external IDs are not accepted as alternate primary references after cutover.
