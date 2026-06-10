# Markdown-First Record Store

This document defines the target persistence architecture for Atelier after the
Markdown-only canonical state cutover. It refines the storage contract in
[Canonical Export And Rebuild Layout](../spec/storage/export/rebuild/canonical-layout.md)
by separating canonical record ownership from rebuildable query indexes and
local-only runtime state.

## Direction

Atelier's durable project state lives in Markdown record files under
`.atelier-state/`. SQLite remains valuable, but as a rebuildable projection and
runtime store rather than a second canonical copy of the same facts.

The target architecture has three explicit components:

| Component | Owns | Does not own |
| --- | --- | --- |
| `RecordStore` | Canonical Markdown record discovery, parsing, validation, ID allocation, deterministic writes, atomic file replacement, and known-ID mutations. | Global query planning, local work/session association, or long-lived caches. |
| `ProjectionIndex` | Rebuildable SQLite indexes derived from `RecordStore`: issue lists, ready queries, reverse links, graph traversal, search, validation lookups, and Mission Control query inputs. | Canonical record mutation or facts that cannot be recreated from Markdown. |
| `RuntimeState` | Local-only state under `.atelier/`: current work association, sessions used by that association, local agent identity, UI caches, and other machine-specific data. | Durable project records, dependencies, typed links, evidence metadata, or workflow policy. |

Successful canonical mutations must write Markdown first. A command may refresh
the projection in the same operation, but durability must not depend on a later
SQLite export. Query commands may use SQLite after a cheap freshness check and
must either refresh the projection or fail with an actionable recovery message.

## Write Path

Known-ID mutations such as issue update, close, dependency edits, labels, typed
links, and future mission/plan/evidence commands follow this order:

1. Load and validate the target record from `RecordStore`.
2. Apply the domain mutation to the in-memory record.
3. Validate record-local invariants and affected graph references.
4. Render deterministic Markdown and replace the record file atomically.
5. Refresh the affected `ProjectionIndex` rows, or mark the projection stale
   with enough metadata for the next query to repair it.

New-record creation allocates a project-scoped random ID through `RecordStore`,
checks for local file collisions across all record kinds, writes the Markdown
record, and then indexes it. The allocator must not rely on a SQLite sequence as
the source of canonical identity.

During the staged migration, first-class non-issue record kinds are registered
centrally in code with their canonical directory, schema, and schema version.
Export, rebuild, and link validation must consume that registry instead of
carrying command-local record kind lists. The registered canonical kinds are
missions, milestone checkpoint records, plans, and evidence; workflow validator
records are recognized as a future kind but do not yet have a canonical
`.atelier-state/` directory.

The first issue-focused `RecordStore` slice is implemented as a testable file
API for `.atelier-state/issues/*.md`. It owns issue record discovery, canonical
path validation, schema and front matter parsing, deterministic rendering, ID
collision checks across canonical directories, and atomic issue file replacement.
During the remaining migration, existing commands may still mutate SQLite first,
but canonical issue export and rebuild should route issue Markdown rendering and
loading through `RecordStore`.

`atelier export` remains a compatibility and repair command during migration.
Its target role is to re-render canonical records, remove obsolete derived files,
and check deterministic output, not to be the normal path that makes a mutation
durable.

## Query Path

Query commands use `ProjectionIndex` when they need global state:

- `issue list`, `issue ready`, search, dependency views, and graph traversal;
- workflow validator lookup and transition checks;
- Mission Control projections and terminal UI inputs;
- lint rules that need reverse links or whole-project consistency.

Before reading the projection, commands check source freshness using record path,
size, modified time as a hint, and content hashes when needed. If stale records
are found, a query command may perform a targeted reindex. If targeted reindex is
not implemented for the affected record kind, it must recommend `atelier rebuild`
or run the equivalent safe rebuild path itself when the command contract permits.

Read-only commands must not silently answer from a stale projection when the
result could affect orchestration, validation, or closeout decisions.

The first `ProjectionIndex` slice stores canonical record-source provenance in
local SQLite table `projection_index_sources`. Each entry records the relative
`.atelier-state/` path, file size, modified-time hint, SHA-256 content hash, and
index timestamp for files under canonical record directories such as `issues/`,
`missions/`, `milestones/`, `plans/`, and `evidence/`. Content hash is
authoritative; size and mtime exist only as cheap diagnostics and future
optimization hints. The table is runtime projection metadata, not canonical
state, and is recreated by `atelier rebuild`. Root-level derived compatibility
files such as `manifest.json` and `graph.json` are not query-projection sources.
Issue activity sidecars are not indexed in this table because recent activity
previews read those canonical files directly; rebuild still validates sidecar
schema and subject references.

During the staged migration, `atelier export` also refreshes this metadata after
it writes canonical Markdown from SQLite so compatibility workflows remain
queryable. Issue query surfaces (`issue list`, `issue ready`, `issue search`,
`issue show`, `issue blocked`, `issue related`, `issue impact`, `issue next`,
and `issue tree`) check the metadata before reading SQLite whenever
`.atelier-state/` exists. If a canonical source changed, disappeared, appeared
without being indexed, or lacks metadata, the command fails with actionable
`atelier rebuild` guidance instead of returning stale projection results.

## Rebuild And Freshness

`atelier rebuild` recreates the canonical portion of `.atelier/state.db` from
Markdown records discovered under `.atelier-state/`. It ignores local-only
runtime state except where runtime tables must be recreated empty or migrated
for schema compatibility.

Issue activity history is canonical sidecar state under
`.atelier-state/issues/<issue-id>.activity/`. Each activity entry is a Markdown
file named with a UTC microsecond timestamp ID:
`YYYYMMDDTHHMMSSffffffZ.md`. If multiple entries share the same timestamp,
writers append deterministic `-01`, `-02`, and later suffixes while refusing to
overwrite an existing file.

Activity front matter uses `schema: "atelier.activity"` and
`schema_version: 1` with these required fields:

- `id`: timestamp activity ID matching the file name.
- `subject_kind`: `issue` in V1.
- `subject_id`: canonical issue ID.
- `event_type`: one of `comment`, `note`, `handoff`, `decision`, `plan`,
  `close_reason`, `status_changed`, `field_changed`, `work_started`,
  `work_finished`, or `evidence_attached`.
- `actor`: user or agent identity that produced the event.
- `created_at`: RFC3339 timestamp.
- `summary`: one-line event summary.

The Markdown body stores user-authored text or lightweight event details.
Evidence remains a rich first-class record under `.atelier-state/evidence/`;
issue activity records only lightweight `evidence_attached` references such as
`evidence_id` and `result` so operators can follow up with
`atelier evidence show`.

`atelier issue show` uses the same sidecars for its bounded recent activity
preview when they exist, and falls back to legacy SQLite notes/comments when a
repository has not yet written activity sidecars.

`atelier export` preserves existing issue activity sidecars as canonical files
and `atelier export --check` validates them instead of reporting them as
untracked drift. `atelier rebuild` validates sidecars, rejects activity entries
whose subject issue is missing, and keeps the runtime projection rebuildable
from `.atelier-state/` alone.

The explicit one-off migration path for old local SQLite comments is
`scripts/migrate_sqlite_comments_to_activity.py`. Operators run it manually with
`--repo <path>` and may use `--dry-run` before writing. The script migrates
comments and close reasons only, refuses to overwrite existing activity IDs,
skips equivalent already-migrated entries on repeated runs, and prints a
conversion summary. It is intentionally not a normal `atelier migrate` command.

`atelier doctor` reports both canonical projection health and runtime-state
health. A repository can be rebuild-ready even when optional runtime state is
absent. A runtime-state failure should not imply canonical record loss unless
Markdown validation also fails.

`atelier export --check` verifies deterministic rendering of canonical Markdown
and known derived projections. In the Markdown-first model, it should not depend
on SQLite being the freshest source of record facts. During migration it may
compare SQLite-derived rendering against Markdown, but any such comparison is a
compatibility check, not the target ownership model.

## Runtime State Boundary

Runtime state remains useful for coordination and operator ergonomics:

- current work/session association;
- local claim helper state used by core workflows;
- cached projection metadata;
- UI state and terminal-view caches.

Runtime state may reference canonical record IDs, but those references are local
and disposable unless a future durable record explicitly captures them. A fresh
checkout must be able to rebuild canonical query behavior from `.atelier-state/`
without copying `.atelier/state.db`.

## Migration Plan

Staged implementation note: missions, plans, evidence, workflow validator
results, and cross-record links currently use first-class SQLite tables and
deterministic Markdown export/rebuild support where they mutate durable records.
That makes mission, plan, evidence, and link records durable and rebuildable,
but their normal mutation path still uses SQLite followed by projection refresh.
This is an accepted intermediate state until the `RecordStore` write path owns
all record mutations directly.

The migration should proceed in small slices:

1. Introduce a `RecordStore` module that can load, validate, render, and write
   issue Markdown records using the existing canonical layout.
2. Extract `ProjectionIndex` responsibilities from rebuild and query commands,
   keeping SQLite schema migrations and query code explicit.
3. Add projection freshness metadata and command behavior for stale indexes.
4. Move issue create/update/close/label/dependency/link mutations to
   Markdown-first writes.
5. Separate local runtime tables and health reporting from canonical projection
   tables.
6. Extend `RecordStore` and `ProjectionIndex` to first-class missions,
   milestones, plans, evidence, and workflow validator records as those command
   surfaces land.
7. Retire the compatibility requirement that normal mutations write SQLite first
   and then call export to become durable.

Each slice must preserve `atelier rebuild`, `atelier export --check`,
`atelier lint`, `atelier doctor`, and the agent-facing issue workflow, or state
the temporary breakage and the reconnect item that owns it.

## Non-Goals

- Do not maintain a fully equivalent SQLite and Markdown live-state sync model
  as the destination.
- Do not introduce a daemon only to keep projections fresh; add one only after
  an interactive workflow proves it needs a long-lived process.
- Do not move workflow policy into `.atelier-state/`; repository-authored policy
  remains separate from deterministic record projections.
- Do not restore `manifest.json` or `graph.json` as canonical source files.
