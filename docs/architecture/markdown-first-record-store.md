# Markdown-First Record Store

This document defines the target persistence architecture for Atelier after the
Markdown-only canonical state cutover. It refines the storage contract in
[Canonical Record And Rebuild Layout](../spec/storage/export/rebuild/canonical-layout.md)
by separating canonical record ownership from rebuildable query indexes and
local-only runtime state.

## Direction

Atelier's durable project state lives in tracked Markdown record files under
`.atelier/`. SQLite remains valuable, but as a rebuildable projection and
runtime store rather than a second canonical copy of the same facts.

The target architecture has three explicit components:

| Component | Owns | Does not own |
| --- | --- | --- |
| `RecordStore` | Canonical Markdown record discovery, parsing, validation, ID allocation, deterministic writes, atomic file replacement, and known-ID mutations. | Global query planning, local work/session association, or long-lived caches. |
| `ProjectionIndex` | Rebuildable SQLite indexes derived from `RecordStore`: issue lists, ready queries, reverse links, graph traversal, search, validation lookups, and Mission Control query inputs. | Canonical record mutation or facts that cannot be recreated from Markdown. |
| `RuntimeState` | Local-only ignored state under `.atelier/runtime/` and `.atelier/cache/`: current work association, sessions used by that association, local agent identity, locks, diagnostics, UI caches, and other machine-specific data. | Durable project records, dependencies, typed links, evidence metadata, or workflow policy. |

Successful canonical mutations must write Markdown first. A command may refresh
the projection in the same operation, but durability must not depend on a later
SQLite export. Query commands may use SQLite after a cheap freshness check, but
cache refresh is transparent product behavior; user-facing recovery should name
record or workflow repairs rather than ask operators to maintain projection
state.

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
`.atelier/` directory.

The first issue-focused `RecordStore` slice was implemented as a testable file
API for the legacy `.atelier/issues/*.md` layout. The single-tree
migration keeps the same responsibilities while moving the target path to
`.atelier/issues/*.md`: issue record discovery, canonical path validation,
schema and front matter parsing, deterministic rendering, ID collision checks
across canonical directories, and atomic issue file replacement.

`atelier export` remains a compatibility and repair command during migration.
Its target role is to re-render canonical records, remove obsolete derived files,
and check deterministic output, not to be the normal path that makes a mutation
durable.

## Canonical Field Ownership

This section defines the durable field contract for Markdown-first records. Each
field belongs to exactly one of these classes:

| Class | Meaning |
| --- | --- |
| Required | Must be present in canonical Markdown for that record kind. Missing data is a lint/rebuild error. |
| Optional | May be present when the record needs the field. Omit it instead of writing an empty compatibility placeholder unless the record kind says otherwise. |
| Derived | Never authored directly in canonical Markdown. Commands, rebuild, or projections compute it from canonical fields. |
| Compatibility-only | Allowed only while migration residue still exists. New writers must stop creating it, and cleanup work may delete it once readers no longer depend on it. |
| Forbidden | Not allowed in canonical Markdown because another field or section already owns the meaning. |

Common front matter ownership for first-class records (`issue`, `mission`,
`plan`, `milestone`, and `evidence`) is:

| Field | Class | Notes |
| --- | --- | --- |
| `schema`, `schema_version`, `id`, `title`, `status`, `created_at`, `updated_at`, `labels`, `relationships` | Required | Shared record identity, lifecycle, timestamps, labels, and typed links. |
| Canonical file path | Derived | Computed from record kind plus `id`; it is never duplicated in front matter. |
| Status category, ready/done grouping, priority display label, reverse-link views, and query/projection rows | Derived | Commands and projections compute these from durable fields. |
| Generic escaped payload fields such as `data` or `data_json` | Compatibility-only | Legacy migration residue. New canonical writers must not introduce new generic payload blobs. |
| Duplicate convenience links such as `validates`, `targets`, `missions`, `contributing_work`, `depends_on`, or `blocked_by` | Forbidden unless a record-kind contract explicitly assigns them | Use `relationships` as the canonical cross-record surface. |

### Relationships

`relationships` is the only canonical cross-record field. Bucket direction is
always from the source record to the target record named in that entry.

| Bucket | Required/optional | Canonical meaning | Derived or forbidden companions |
| --- | --- | --- | --- |
| `attachments` | Required bucket; entries optional | Supporting links with a `role`, such as `planned_by`, `has_checkpoint`, or `validates`. Evidence targets live here with `role: validates`. | `targets`, `validates`, `plans`, `milestones`, and direct evidence ID arrays are forbidden duplicates. |
| `blocks` | Required bucket; entries optional | Sequencing blockers from the source record to blocked issue-like work. | Inverse `depends_on` and `blocked_by` views are derived only. |
| `children` | Required bucket; entries optional | Structural hierarchy when the source owns the child record. | Mission execution work must not be authored here; use `relates` with `type: advances`. |
| `relates` | Required bucket; entries optional | Peer semantic links with a precise `type`, such as `advances`, `blocked_by`, `related`, `derived_from`, or `supersedes`. | Ad hoc link arrays or prose-only link inventories are forbidden duplicates. |

### Issue Records

| Slice | Ownership |
| --- | --- |
| Required front matter | Common fields plus `priority` and `issue_type`. |
| Optional front matter | None in V1 beyond record-generic labels and relationships. |
| Required body | `## Description`, `## Outcome`, and `## Evidence`. |
| Optional body | `## Notes`. |
| Derived | Workflow status category from `.atelier/workflow.yaml`; display priority bucket; inverse blockers/parents; recent activity preview from sidecars. |
| Compatibility-only | Legacy SQLite `description` mirror columns; imported prose that has not yet been normalized into the required sections. |
| Forbidden | Front matter or duplicate body fields named `description`, `outcome`, `acceptance`, `evidence_required`, `depends_on`, or `blocked_by`. |

Issue priority is durable front matter owned by the issue record. The current
canonical vocabulary is `P0`, `P1`, `P2`, and `P3`; human-facing terms such as
`critical`, `high`, `medium`, and `low` are derived presentation labels, not
separate canonical fields.

Issue status is durable workflow state owned by `.atelier/workflow.yaml`. The
current repository-defined values are `todo`, `in_progress`, `blocked`,
`review`, `validation`, `done`, and `archived`. Human-ready groupings such as
`todo`, `active`, `blocked`, `review`, `validation`, and `done` are derived
categories, not alternate stored tokens.

### Mission Records

| Slice | Ownership |
| --- | --- |
| Required front matter | Common fields only. Mission semantics do not own extra scalar or JSON payload keys in front matter. |
| Optional front matter | None in V1 beyond record-generic labels and relationships. |
| Required body | `## Intent`, `## Constraints`, `## Risks`, and `## Validation`. |
| Optional body | `## Closeout Notes` and `## Notes`. |
| Derived | Linked work from `relationships.relates[]` entries with `type: advances`; direct mission blockers from `relationships.relates[]` entries with `type: blocked_by`; mission evidence coverage from incoming evidence links with `role: validates`. |
| Compatibility-only | Escaped mission `data` payloads and front matter keys such as `constraints`, `risks`, `validation`, `work`, `plans`, `milestones`, `evidence`, `blockers`, or `closeout_notes`. |
| Forbidden | Any second relationship surface for work, blockers, plans, checkpoints, or evidence. Mission prose must not become a shadow graph. |

Mission status is mission-lifecycle state, not issue workflow state. The current
durable vocabulary is `draft`, `ready`, `active`, and `closed`.

### Plan Records

| Slice | Ownership |
| --- | --- |
| Required front matter | Common fields plus `revision`. |
| Optional front matter | `owner`, `applies_to`, `supersedes`, and `drift_status`. `applies_to` and `supersedes` are sorted lexically. |
| Required body | The durable execution intent in Markdown prose. V1 does not require a section grammar for plan bodies. |
| Optional body | Any additional Markdown structure inside that execution intent. |
| Derived | Superseded-by views from other plans' `supersedes` arrays; applicability rollups from `applies_to` plus `relationships`; plan freshness summaries from `drift_status`. |
| Compatibility-only | Quoted JSON `data` payloads that carry `revision` or `revisions`. |
| Forbidden | Generic front matter arrays such as `revisions`, `steps`, or `targets` when their meaning is already owned by `revision`, the body, or `relationships`. |

Plan status owns the lifecycle of the plan record itself. Exact plan status
tokens may evolve with the dedicated plan command surface, but they must remain
plan-specific lifecycle values rather than borrowed issue workflow aliases.

### Evidence Records

| Slice | Ownership |
| --- | --- |
| Required front matter | Common fields plus `evidence_type`, `captured_at`, and any validating targets expressed as `relationships.attachments[]` entries with `role: validates`. |
| Optional front matter | `command`, `artifact`, `agent_identity`, `independence_level`, `proof_scope`, `residual_risks`, and `follow_up_ids`. |
| Required body | Human-readable proof summary and any important limits not already captured in front matter. |
| Optional body | Additional bounded transcript excerpts, audit notes, or artifact context. |
| Derived | Evidence coverage views for issues, missions, plans, or milestones; command success summaries from structured transcript metadata; reverse lookup of which records this evidence validates. |
| Compatibility-only | Escaped `data` payloads carrying fields such as `kind`, `result`, `path`, `uri`, `producer`, `output`, `exit_code`, or `target`. |
| Forbidden | Separate `targets` or `validates` front matter arrays, because validating links belong in `relationships.attachments[]`. |

Evidence `status` is the canonical proof result token. The target vocabulary is
`pass`, `fail`, `blocked`, `deferred`, `not_applicable`, or `informational`.
Older records that still mirror result-like fields inside escaped `data` are
compatibility residue rather than a second canonical status surface.

### Milestone Records

| Slice | Ownership |
| --- | --- |
| Required front matter | Common fields plus `desired_state`, `scope`, `validation_criteria`, `accepted_evidence`, and `completion_state`. |
| Optional front matter | None beyond record-generic labels and relationships in V1. |
| Required body | Milestone narrative and checkpoint context in Markdown prose. |
| Optional body | Additional headings or notes that explain the checkpoint without duplicating front matter fields. |
| Derived | Mission membership and contributing work from canonical relationships instead of separate scalar arrays; acceptance rollups from `accepted_evidence` plus validating evidence links. |
| Compatibility-only | None yet in committed canonical records; this kind is still target-state contract work. |
| Forbidden | Separate `missions` or `contributing_work` front matter arrays when the same links are already modeled through `relationships`. |

Milestone `status` owns the record lifecycle. `completion_state` is separate and
describes checkpoint acceptance, not authoring lifecycle or issue workflow
progress.

### Activity Sidecars

Activity sidecars are canonical durable history, but they are not first-class
records that share the common `title`/`status` contract.

| Slice | Ownership |
| --- | --- |
| Required front matter | `schema`, `schema_version`, `id`, `subject_kind`, `subject_id`, `event_type`, `actor`, `created_at`, and `summary`. |
| Optional front matter | Event-specific lightweight fields such as `evidence_id`, `result`, `field`, `old`, or `new` when the event kind needs them. |
| Required body | User-authored text or lightweight event detail. Empty body is allowed only when the event-specific front matter fully carries the event payload. |
| Derived | Sidecar path from `subject_id` plus timestamp ID; chronological ordering; recent-activity previews. |
| Compatibility-only | Legacy SQLite `comments` rows and imported close-reason history not yet converted into sidecars. |
| Forbidden | `relationships`, `labels`, `priority`, or generic record payload blobs. Activity sidecars are event logs, not another record graph. |

### Runtime, Cache, Config, And Provenance

| Surface | Ownership |
| --- | --- |
| Tracked config | `.atelier/config.toml` is the only durable config record in this scope. Required fields are the project config schema/version, `project_slug`, and `[paths].state_root`, `runtime_dir`, `runtime_database`, and `cache_dir`. |
| Compatibility-only config | `[paths].compatibility_state_root` remains tracked only while `.atelier-state/` compatibility flows still exist. |
| Local runtime state | `.atelier/runtime/state.db`, `.atelier/runtime/`, `.atelier/cache/`, lock files, diagnostics, local agent identity, sessions, and work associations are ignored machine-local state. |
| Projection provenance | `projection_index_sources` rows, file size hints, mtimes, hashes, and reindex timestamps are derived SQLite metadata, not canonical Markdown fields. |
| Forbidden durable provenance | Runtime branch names, worktree paths, session IDs, lock ownership, local diagnostic output, and cache payloads must not be promoted into canonical record front matter without a separate artifact update. |

### Manual Classification Check

Manual classification run date: 2026-06-13. This is a representative check of
current committed records against the target contract above.

| Sample | Record kind | Result | Notes |
| --- | --- | --- | --- |
| `.atelier/issues/atelier-x45p.md` | Issue | Pass | Uses required issue front matter plus `Description`/`Outcome`/`Evidence`; blocker intent lives in `relationships.blocks`; durable priority/status tokens are `P1` and `todo`. |
| `.atelier/missions/atelier-man9.md` | Mission | Pass | Uses mission required sections and `relationships.relates[]` `type: advances` links for work. No escaped JSON mission payload remains. |
| `.atelier/evidence/atelier-06rb.md` | Evidence | Fail (compatibility residue present) | The record uses canonical `relationships.attachments[] role=validates`, but it still stores proof metadata in escaped `data` instead of owned first-class fields such as `evidence_type`, `captured_at`, and `proof_scope`. |
| `.atelier/issues/atelier-0001.activity/20260611T204233793564Z.md` | Activity sidecar | Pass | Uses required activity front matter. Event payload keys `field`, `old`, and `new` are acceptable event-specific detail, not a second relationship or status model. |
| `.atelier/config.toml` | Project config/runtime boundary | Pass with compatibility note | Correctly tracks canonical path ownership and local runtime/cache locations. `compatibility_state_root` remains compatibility-only until old `.atelier-state/` flows are removed. |
| `.atelier/plans/` | Plan | Defer | No committed plan records exist yet, so the contract is target-state only in this manual check. |
| `.atelier/milestones/` | Milestone | Defer | No committed milestone records exist yet, so the contract is target-state only in this manual check. |

## Query Path

Query commands use `ProjectionIndex` when they need global state:

- `issue list`, `issue list --ready`, search, dependency views, and graph traversal;
- workflow validator lookup and transition checks;
- Mission Control projections and terminal UI inputs;
- lint rules that need reverse links or whole-project consistency.

Before reading the projection, commands check source freshness using record path,
size, modified time as a hint, and content hashes when needed. If stale records
are found, a query command may perform a targeted reindex. If targeted reindex is
not implemented for the affected record kind, it must run the equivalent safe
rebuild path itself when the command contract permits. Low-level rebuild
commands may remain for debug and repair, but ordinary product workflows should
not require them.

Read-only commands must not silently answer from a stale projection when the
result could affect orchestration, validation, or closeout decisions.

The first `ProjectionIndex` slice stores canonical record-source provenance in
local SQLite table `projection_index_sources`. Each entry records the relative
`.atelier/` path, file size, modified-time hint, SHA-256 content hash, and
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
queryable. Ordinary projection-backed read surfaces (`issue list`,
`issue list --ready`, `issue search`, `issue show`, `issue blocked`,
`issue related`, `issue impact`, `issue next`, `issue tree`, dependency lists,
and tracker lint) check the metadata before reading SQLite whenever
canonical records exist. If a canonical source changed, disappeared, appeared
without being indexed, or lacks metadata, the command first validates
`.atelier/`; when validation succeeds it automatically rebuilds the local
SQLite projection and answers the query. Invalid Markdown, schema drift,
conflicting records, missing required canonical state, or rebuild failures still
block completion paths. Orientation and repair surfaces should degrade at the
record level where possible instead of collapsing into cache-maintenance errors.

The projection is metadata-only in the target design: it may keep fields needed
to find, sort, filter, traverse, and validate records, but it must not be
treated as a full Markdown mirror for detail rendering. Current SQLite columns
are classified as follows:

| Table or field | Classification | Target ownership |
| --- | --- | --- |
| `issues.id`, `title`, `status`, `issue_type`, `priority`, `parent_id`, `created_at`, `updated_at`, `closed_at` | Projection metadata | Keep as the issue list, ready-work, graph, workflow, and Mission Control summary index. These fields are small and commonly used for sorting and filtering. |
| `issues.description` | Removal candidate | Canonical Markdown body owned by `RecordStore`; detail views should load it from `.atelier/issues/*.md`. It remains only as migration compatibility until write paths stop depending on it. |
| `labels.issue_id`, `labels.label` | Projection metadata | Keep for queue filters, ownership labels, and Mission Control facets. |
| `dependencies.blocker_id`, `dependencies.blocked_id` | Projection metadata | Keep as derived graph edges for ready queries and workflow checks. |
| `relations.issue_id_1`, `relations.issue_id_2`, `relations.relation_type`, `relations.created_at` | Projection metadata | Keep as derived typed issue-link edges for traversal and impact views. |
| `records.id`, `records.kind`, `records.title`, `records.status`, `records.created_at`, `records.updated_at` | Projection metadata | Keep or replace with a narrower cross-record metadata index for missions, milestones, plans, and evidence. |
| `records.body`, `records.data_json` | Removal candidate | Rich first-class record content owned by `RecordStore`; command detail views should load selected Markdown records instead of rendering these columns. |
| `record_links.source_kind`, `source_id`, `target_kind`, `target_id`, `relation_type`, `created_at` | Projection metadata | Keep as derived cross-record graph edges for workflow validation and Mission Control rollups. |
| `projection_index_sources.path`, `size_bytes`, `modified_micros`, `sha256`, `indexed_at` | Projection metadata | Keep as rebuildable freshness metadata. Hash is authoritative; size and modified time are optimization hints. |
| `sessions.*`, `work_associations.*` | Runtime state | Keep local-only under `.atelier/runtime/`; rebuild may recreate these tables empty or preserve them through runtime-specific paths, but they are not durable project facts. |
| `comments.content`, `comments.kind`, `comments.created_at` | Compatibility residue | Legacy SQLite notes retained for imports and repositories not yet migrated to activity sidecars. New issue activity detail is canonical Markdown sidecar state. |
| Dropped `token_usage`, `time_entries`, `milestones`, `milestone_issues` | Compatibility removal | Already removed from the active schema after their command surfaces or replacement record forms superseded them. |

Representative detail paths for this boundary are `atelier issue show`,
`atelier mission show`, `atelier plan show`, and `atelier evidence show`: the
commands use SQLite to resolve requested IDs, relationships, and graph/runtime
metadata, then load the selected Markdown payload from `RecordStore` before
rendering. `atelier issue search` also matches issue titles and bodies from
canonical issue files, with comment/note text read from canonical activity
sidecars instead of legacy SQLite `comments.content`. This allows frequent
polling surfaces such as Mission Control to use small SQLite rows for candidate
lists without treating every Markdown body or record payload as cached UI state.

## Rebuild And Freshness

`atelier rebuild` recreates the canonical portion of
`.atelier/runtime/state.db` from Markdown records discovered under tracked
`.atelier/` record directories. It ignores local-only runtime state except
where runtime tables must be recreated empty or migrated for schema
compatibility.

Issue activity history is canonical sidecar state under
`.atelier/issues/<issue-id>.activity/`. Each activity entry is a Markdown
file named with a UTC microsecond timestamp ID:
`YYYYMMDDTHHMMSSffffffZ.md`. If multiple entries share the same timestamp,
writers append deterministic `-01`, `-02`, and later suffixes while refusing to
overwrite an existing file.

Activity front matter uses `schema: "atelier.activity"` and
`schema_version: 1` with these required fields:

- `id`: timestamp activity ID matching the file name.
- `subject_kind`: `issue` in V1.
- `subject_id`: canonical issue ID.
- `event_type`: one of `comment`, `note`, `handoff`, `plan`,
  `close_reason`, `status_changed`, `field_changed`, `work_started`,
  `work_finished`, or `evidence_attached`.
- `actor`: user or agent identity that produced the event.
- `created_at`: RFC3339 timestamp.
- `summary`: one-line event summary.

The Markdown body stores user-authored text or lightweight event details.
Evidence remains a rich first-class record under `.atelier/evidence/`;
issue activity records only lightweight `evidence_attached` references such as
`evidence_id` and `result` so operators can follow up with
`atelier evidence show`.

`atelier issue show` uses the same sidecars for its bounded recent activity
preview when they exist, and falls back to legacy SQLite notes/comments when a
repository has not yet written activity sidecars.

Imported predecessor comments and close reasons are migration input, not a
separate durable comment store. The accepted policy is:

- New Atelier notes, comments, handoffs, work-start/work-finish events,
  resolution comments, close reasons, and evidence attachments are canonical issue
  activity sidecars.
- Imported comments from Beads or older SQLite rows may be converted into
  activity sidecars by an explicit migration step. Conversion preserves original
  author and timestamp when available and records the event type as `comment`,
  `note`, or `close_reason`.
- Legacy SQLite `comments` rows remain compatibility residue for repositories
  not yet migrated. New normal commands must not create durable project history
  only in SQLite comments.
- Display order is chronological by `created_at`, then activity ID, then file
  path. Duplicate timestamps are represented by deterministic filename suffixes
  rather than overwriting entries.
- Merge conflict resolution is file-level: keep both valid sidecar files when
  two agents add distinct history entries, and edit the body/front matter only
  when the same activity file conflicts.

`atelier export` preserves existing issue activity sidecars as canonical files
and `atelier export --check` validates them instead of reporting them as
untracked drift. `atelier rebuild` validates sidecars, rejects activity entries
whose subject issue is missing, and keeps the runtime projection rebuildable
from tracked `.atelier/` records alone.

## Durable Mutation Audit

Audit date: 2026-06-11. The current command surface has three write classes:

| Path | Classification | Notes |
| --- | --- | --- |
| `RecordStore` issue and domain APIs | Markdown-first through `RecordStore` | Issue and first-class record helpers allocate IDs, validate, render, atomically replace canonical Markdown, remove canonical files for deletes, and mutate canonical relationship front matter. |
| Issue create/update/close/reopen/comment/label/unlabel/block/unblock/relate/unrelate/subissue/quick | RecordStore-owned Markdown-first | Public commands write canonical issue Markdown or activity sidecars first, then refresh the SQLite projection. Compatibility SQLite issue mutation helpers remain only for legacy tests/import internals. |
| Issue delete and close-all | RecordStore-owned Markdown-first | Delete removes canonical issue Markdown before projection refresh. Close-all rewrites matching canonical issues through the lifecycle close path. |
| `dep add` and `dep remove` | RecordStore-owned Markdown-first | Top-level Agent Factory dependency aliases mutate canonical issue relationship front matter before projection refresh. `dep list` is query-only and checks projection freshness. |
| Mission create/update/add-work/add-blocker | RecordStore-owned Markdown-first | Mission records and cross-record links write canonical mission Markdown and relationships before projection refresh. |
| Plan create/revise/link | RecordStore-owned Markdown-first | Plan records and typed links write canonical plan Markdown and relationships before projection refresh. |
| Plan apply | RecordStore-owned Markdown-first | Bulk apply writes issue, mission, milestone, plan, evidence, and relationship records through `RecordStore`, then refreshes projection. `apply.export` no longer controls durability; `auto`, `check_only`, and `skip` all leave the projection refreshed after successful canonical writes. |
| Evidence add/attach | RecordStore-owned Markdown-first | Evidence records and attachment links write canonical evidence Markdown and relationships before projection refresh. Issue evidence attachments also write issue activity sidecars. |
| Typed record links, labels, and dependencies | RecordStore-owned Markdown-first | Rebuild derives labels, dependency edges, typed relations, hierarchy, and record links from canonical relationship front matter. |
| Workflow validate | Runtime/query-only | Built-in validators read projection state and do not persist validator-result records. `workflow_validator` is registered as a future non-canonical record kind only. |
| Work start/finish and worktree helpers | Runtime plus activity sidecars | Work associations and sessions are local runtime state in `.atelier/runtime/`. The durable part is the issue activity sidecar, which is written directly under `.atelier/issues/<id>.activity/` when the issue Markdown file exists. |
| Diagnostics, telemetry, tested marker, init, import-beads, export, rebuild, lint, doctor | Runtime, maintenance, import, or repair | Telemetry and work markers are local/runtime. `import-beads` is an external import bridge that still renders canonical state from imported SQLite rows. `export` and `rebuild` are repair/projection commands, not normal durable mutation owners. |

The remaining compatibility residue is internal: several inherited SQLite
mutation helpers are still compiled for unit tests, imports, and repair flows,
but normal public durable commands no longer call export to become recoverable.

## Projection Refresh After Canonical Writes

For RecordStore-owned mutations, Atelier uses a runtime-preserving projection
refresh after each successful canonical write. Mutation commands write
tracked `.atelier/` records first through `RecordStore`, then call
`refresh_projection_after_canonical_write` to validate canonical Markdown and
replace projection rows from that source while copying valid local session and
work-association rows forward. This preserves the clear ownership boundary:
Markdown is the durable write target, while SQLite is the disposable projection
used by subsequent query commands plus local RuntimeState.

The refresh helper deliberately runs canonical validation before replacing the
projection. If a command writes invalid Markdown, the command must fail before
answering from stale SQLite. If projection replacement fails after a successful
canonical write, the canonical files remain durable and ordinary query commands
can recover through the transparent projection repair path once the reported
record or workflow problem is fixed.

`atelier export` remains available as an explicit repair/sync command. New
durable mutation paths must not use export as the normal step that makes command
output recoverable.

The explicit one-off migration path for old local SQLite comments is
`scripts/migrate_sqlite_comments_to_activity.py`. Operators run it manually with
`--repo <path>` and may use `--dry-run` before writing. The script migrates
comments and close reasons only, refuses to overwrite existing activity IDs,
skips equivalent already-migrated entries on repeated runs, and prints a
conversion summary. It is intentionally not a normal `atelier migrate` command.

`atelier doctor` reports install, local runtime, diagnostics, and workflow
health without acting as the canonical durability gate. `atelier lint` owns
canonical Markdown validity and relationship findings. Doctor may show that
local derived state is degraded, but a repository can still have healthy runtime
state when canonical Markdown needs lint repair, and optional runtime or cache
directories may be absent.

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
checkout must be able to rebuild canonical query behavior from tracked
`.atelier/` records without copying `.atelier/runtime/state.db`.

## Migration Plan

Staged implementation note: missions, plans, evidence, workflow validator
results, and cross-record links currently use first-class SQLite tables and
deterministic Markdown export/rebuild support where they mutate durable records.
That makes mission, plan, evidence, and link records durable and rebuildable,
but their normal mutation path still uses SQLite followed by projection refresh.
This is an accepted intermediate state until the `RecordStore` write path owns
all record mutations directly.

The migration proceeded in small slices:

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
   and then call export to become durable. This is complete for normal public
   durable commands; export remains an explicit repair/sync surface and imports
   may still use export as an import bridge.

Each slice must preserve `atelier rebuild`, `atelier export --check`,
`atelier lint`, `atelier doctor`, and the agent-facing issue workflow, or state
the temporary breakage and the reconnect item that owns it.

## Non-Goals

- Do not maintain a fully equivalent SQLite and Markdown live-state sync model
  as the destination.
- Do not introduce a daemon only to keep projections fresh; add one only after
  an interactive workflow proves it needs a long-lived process.
- Do not move workflow policy into ad hoc runtime files; repository-authored
  policy belongs in tracked `.atelier/config.toml` or a documented policy file
  selected by that config, separate from deterministic record projections.
- Do not restore `manifest.json` or `graph.json` as canonical source files.
