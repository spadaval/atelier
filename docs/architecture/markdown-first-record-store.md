# Markdown-First Record Store

This document defines the target persistence architecture for Atelier after the
Markdown-only canonical state cutover. It refines the storage contract in
[Canonical Record And Rebuild Layout](../spec/storage/export/rebuild/canonical-layout.md)
by separating canonical record ownership from rebuildable query indexes and
ignored local diagnostics/cache files.

## Direction

Atelier's durable project state lives in tracked Markdown record files under
`.atelier/`. SQLite remains valuable, but as a rebuildable projection rather
than a second canonical copy of the same facts.

The target architecture has two explicit tracker-state components:

| Component | Owns | Does not own |
| --- | --- | --- |
| `RecordStore` | Canonical Markdown record discovery, parsing, validation, ID allocation, deterministic writes, atomic file replacement, and known-ID mutations. | Global query planning, runtime-only checkout/session context, or long-lived caches. |
| `ProjectionIndex` | Rebuildable SQLite indexes derived from `RecordStore`: issue lists, ready queries, reverse links, graph traversal, search, validation lookups, and Mission Control query inputs. | Canonical record mutation or facts that cannot be recreated from Markdown. |

Ignored local diagnostics, lock files, and UI caches may exist beside these
components, but they are not SQLite tracker state and must not define durable
project records or current work.

Successful canonical mutations must write Markdown first. A command may refresh
the projection in the same operation, but durability must not depend on a later
SQLite export. Query commands may use SQLite after a cheap freshness check, but
cache refresh is transparent product behavior; user-facing recovery should name
record or workflow repairs rather than ask operators to maintain projection
state.

Current work in a checkout is derived from the canonical issue records in that
checkout whose workflow status is `in_progress`, interpreted alongside the
checkout's mission worktree and branch context. Because each Git worktree holds
its own tracked `.atelier/` tree on its branch, different worktrees may show
different current-work sets until Git reconciles the Markdown records.

## Write Path

Known-ID mutations such as issue update, close, dependency edits, labels, typed
links, and mission/evidence commands follow this order:

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

First-class non-issue record kinds are registered centrally in code with their
canonical directory, schema, and schema version. Export, rebuild, and link
validation must consume that registry instead of carrying command-local record
kind lists. The active v1 canonical kinds are missions and evidence. Plan,
milestone/checkpoint, workflow validator, and session/run records are deferred
until later contracts introduce them directly.

In code, the low-level `RecordStore` module is split by durable ownership:
`record_store::record_kinds` owns the kind registry and canonical path
derivation, while `record_store::relationships` owns relationship data
structures, sorting, and issue-link constructors. The top-level `RecordStore`
keeps file discovery, atomic writes, ID allocation, parsing/rendering entry
points, and known-ID mutation methods. Record-kind modules may depend on those
shared primitives, but command modules should not duplicate record-kind lists or
relationship constructors.

The first issue-focused `RecordStore` slice was implemented as a testable file
API for the legacy `.atelier/issues/*.md` layout. The single-tree
migration keeps the same responsibilities while moving the target path to
`.atelier/issues/*.md`: issue record discovery, canonical path validation,
schema and front matter parsing, deterministic rendering, ID collision checks
across canonical directories, and atomic issue file replacement.

Hidden/admin `atelier export` remains a compatibility and
deterministic-rendering command during migration. Its target role is to
re-render canonical records, remove obsolete derived files, and check
deterministic output, not to be the normal path that makes a mutation durable
or to own ignored runtime/projection repair.

## Canonical Field Ownership

This section defines the durable field contract for Markdown-first records. Each
field belongs to exactly one of these classes:

| Class | Meaning |
| --- | --- |
| Required | Must be present in canonical Markdown for that record kind. Missing data is a lint/rebuild error. |
| Optional | May be present when the record needs the field. Omit it instead of writing an empty compatibility placeholder unless the record kind says otherwise. |
| Derived | Never authored directly in canonical Markdown. Commands, rebuild, or projections compute it from canonical fields. |
| Compatibility-only | Allowed only for explicitly named non-canonical migration residue. New canonical Markdown readers and writers must not accept it. |
| Forbidden | Not allowed in canonical Markdown because another field or section already owns the meaning. |

Common front matter ownership for first-class records (`issue`, `mission`, and
`evidence`) is:

| Field | Class | Notes |
| --- | --- | --- |
| `schema`, `schema_version`, `id`, `title`, `status`, `created_at`, `updated_at`, `labels`, `relationships` | Required | Shared record identity, lifecycle, timestamps, labels, and typed links. |
| Canonical file path | Derived | Computed from record kind plus `id`; it is never duplicated in front matter. |
| Status category, ready/done grouping, priority display label, reverse-link views, and query/projection rows | Derived | Commands and projections compute these from durable fields. |
| Generic escaped payload fields | Forbidden | Canonical Markdown readers reject generic payload blobs; record-kind contracts must expose typed front matter and body sections instead. |
| Duplicate convenience links such as `validates`, `targets`, `missions`, `contributing_work`, `depends_on`, or `blocked_by` | Forbidden unless a record-kind contract explicitly assigns them | Use `relationships` as the canonical cross-record surface. |

### Relationships

`relationships` is the only canonical cross-record field. Bucket direction is
always from the source record to the target record named in that entry.

| Bucket | Required/optional | Canonical meaning | Derived or forbidden companions |
| --- | --- | --- | --- |
| `attachments` | Required bucket; entries optional | Supporting links with a `role`, such as `validates`. Evidence targets live here with `role: validates`. | `targets`, `validates`, `plans`, `milestones`, and direct evidence ID arrays are forbidden duplicates. |
| `blocks` | Required bucket; entries optional | Sequencing blockers from the source record to blocked issue-like work. | Inverse `depends_on` and `blocked_by` views are derived only. |
| `children` | Required bucket; entries optional | Structural hierarchy when the source owns the child record. | Mission execution work must not be authored here; use `relates` with `type: advances`. |
| `relates` | Required bucket; entries optional | Peer semantic links with a precise `type`, such as `advances`, `blocked_by`, `related`, `derived_from`, or `supersedes`. | Ad hoc link arrays or prose-only link inventories are forbidden duplicates. |

### Issue Records

| Slice | Ownership |
| --- | --- |
| Required front matter | Common fields plus `priority` and `issue_type`. |
| Optional front matter | None in V1 beyond record-generic labels and relationships. |
| Required body | `## Description` and `## Outcome`. |
| Optional body | `## Evidence` and `## Notes`; workflow policy and issue contracts may still require attached proof or authored proof prose for specific work. |
| Derived | Workflow status category from `.atelier/workflow.yaml`; display priority bucket; inverse blockers/parents; recent activity preview from sidecars. |
| Compatibility-only | Legacy SQLite `description` mirror columns; imported prose that has not yet been normalized into the required sections. |
| Forbidden | Front matter or duplicate body fields named `description`, `outcome`, `acceptance`, `evidence_required`, `depends_on`, or `blocked_by`. |

Issue priority is durable front matter owned by the issue record. The current
canonical vocabulary is `P0`, `P1`, `P2`, and `P3`; human-facing terms such as
`critical`, `high`, `medium`, and `low` are derived presentation labels, not
separate canonical fields.

Issue status is durable workflow state owned by `.atelier/workflow.yaml`. The
current repository-defined values are `todo`, `in_progress`, `blocked`,
`review`, `validation`, and `done`. Human-ready groupings such as `todo`,
`active`, `blocked`, and `done` are derived categories, not alternate stored
tokens; `review` and `validation` are workflow statuses in the active category.

### Mission Records

| Slice | Ownership |
| --- | --- |
| Required front matter | Common fields only. Mission semantics do not own extra scalar or JSON payload keys in front matter. |
| Optional front matter | None in V1 beyond record-generic labels and relationships. |
| Required body | `## Intent`, `## Constraints`, `## Risks`, and `## Validation`. |
| Optional body | `## Terminal Notes` and `## Notes`. |
| Derived | Linked work from `relationships.relates[]` entries with `type: advances`; direct mission blockers from `relationships.relates[]` entries with `type: blocked_by`; mission evidence coverage from incoming evidence links with `role: validates`. |
| Compatibility-only | None in canonical Markdown. |
| Forbidden | Escaped mission `data` payloads, front matter keys such as `constraints`, `risks`, `validation`, `work`, `plans`, `milestones`, `evidence`, `blockers`, or `terminal_notes`, and any second relationship surface for work, blockers, plans, checkpoints, or evidence. Mission prose may reference plan/checkpoint Markdown by path, but must not become a shadow graph. |

Mission status is mission-lifecycle state, not issue workflow state. The current
durable vocabulary is `draft`, `ready`, `active`, `superseded`, and `closed`.

### Deferred Plan Records

Plan records are not active v1 canonical Markdown records. Execution intent
that must survive the current chat should be an ordinary Markdown artifact or
prose referenced from a mission, epic, issue, or evidence body. There is no
`.atelier/plans/` directory, plan status lifecycle, or `plans.*` projection
table in the v1 target contract.

### Evidence Records

| Slice | Ownership |
| --- | --- |
| Required front matter | Common fields plus `evidence_type`, `captured_at`, and any validating targets expressed as `relationships.attachments[]` entries with `role: validates`. |
| Optional front matter | `command`, `artifact`, `agent_identity`, `independence_level`, `proof_scope`, `residual_risks`, and `follow_up_ids`. |
| Required body | Human-readable proof summary and any important limits not already captured in front matter. |
| Optional body | Additional bounded transcript excerpts, audit notes, or artifact context. |
| Derived | Evidence coverage views for issues and missions; command success summaries from structured transcript metadata; reverse lookup of which records this evidence validates. |
| Compatibility-only | None in canonical Markdown. |
| Forbidden | Escaped `data` payloads and separate `targets` or `validates` front matter arrays, because validating links belong in `relationships.attachments[]`. |

Evidence `status` is the canonical proof result token. The target vocabulary is
`pass`, `fail`, `blocked`, `deferred`, `not_applicable`, or `informational`.
Records that still mirror result-like fields inside escaped `data` are invalid
canonical Markdown and must be migrated before rebuild/lint can pass.

### Deferred Checkpoint Records

Milestone/checkpoint records are not active v1 canonical Markdown records.
Checkpoint criteria and target-state prose may live in missions, epics, issues,
or evidence, but there is no `.atelier/milestones/` directory, milestone
completion state, or `milestones.*` projection table in the v1 target contract.

### Activity Sidecars

Activity sidecars are canonical durable history owned by the activity sidecar
API in `src/activity.rs`, not by `RecordStore` and not by the SQLite
projection. They are canonical Markdown files, but they are not first-class
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
| Tracked config | `.atelier/config.toml` is the only durable config record in this scope. Required fields are the project config schema/version, `project_slug`, and `[paths].state_root`. |
| Compatibility-only config | No compatibility-state path is tracked in project config. |
| Local projection/cache state | `.atelier/runtime/state.db`, `.atelier/runtime/`, `.atelier/cache/`, lock files, diagnostics, and UI caches are ignored machine-local artifacts. SQLite tables under `state.db` must be rebuildable projection state, not non-Markdown tracker facts. |
| Projection provenance | `projection_sources` rows, record identity, file size hints, mtimes, hashes, and reindex timestamps are derived SQLite metadata, not canonical Markdown fields. |
| Forbidden durable provenance | Runtime branch names, worktree paths, session IDs, lock ownership, local diagnostic output, and cache payloads must not be promoted into canonical record front matter without a separate artifact update. |

### Manual Classification Check

Manual classification run date: 2026-06-13. This is a representative check of
current committed records against the target contract above.

| Sample | Record kind | Result | Notes |
| --- | --- | --- | --- |
| `.atelier/issues/atelier-x45p.md` | Issue | Pass | Uses required issue front matter plus `Description`/`Outcome` and optional `Evidence`; blocker intent lives in `relationships.blocks`; durable priority/status tokens are `P1` and `todo`. |
| `.atelier/missions/atelier-man9.md` | Mission | Pass | Uses mission required sections and `relationships.relates[]` `type: advances` links for work. No escaped JSON mission payload remains. |
| `.atelier/evidence/atelier-06rb.md` | Evidence | Fail (forbidden payload residue present) | The record uses canonical `relationships.attachments[] role=validates`, but it still stores proof metadata in escaped `data` instead of owned first-class fields such as `evidence_type`, `captured_at`, and `proof_scope`. |
| `.atelier/issues/atelier-0001.activity/20260611T204233793564Z.md` | Activity sidecar | Pass | Uses required activity front matter. Event payload keys `field`, `old`, and `new` are acceptable event-specific detail, not a second relationship or status model. |
| `.atelier/config.toml` | Project config/runtime boundary | Pass | Tracks canonical path ownership without committed runtime/cache or compatibility-state path settings. |
| `.atelier/plans/` | Plan | Deferred | No active v1 plan record table exists; planning intent is ordinary Markdown or prose referenced from accountable records. |
| `.atelier/milestones/` | Milestone | Deferred | No active v1 milestone record table exists; checkpoint intent is ordinary Markdown or prose referenced from accountable records. |

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

The projection stores canonical record-source provenance in local SQLite table
`projection_sources`. Each entry records the relative `.atelier/` path, record
kind/id, file size, modified-time hint, SHA-256 content hash, and index timestamp
for files under canonical record directories such as `issues/`, `missions/`,
and `evidence/`. Unchanged size and mtime are accepted
as the fast path; when either stat changes, Atelier hashes only that candidate.
If the hash still matches, the source metadata row is refreshed without
reindexing the record. The table is projection metadata, not canonical state,
and is recreated by `atelier rebuild`. Root-level derived compatibility
files such as `manifest.json` and `graph.json` are not query-projection sources.
Issue activity sidecars are not indexed in this table because recent activity
previews read those canonical files directly; rebuild still validates sidecar
schema and subject references.

During the staged migration, `atelier export` also refreshes this metadata after
it writes canonical Markdown from SQLite so compatibility workflows remain
queryable. Ordinary projection-backed read surfaces (`issue list`,
`issue list --ready`, `issue search`, `issue show`, `issue blocked`,
`issue status`, dependency lists, and tracker lint) check the metadata before reading SQLite whenever
canonical records exist. If a canonical source changed, disappeared, appeared
without being indexed, or lacks metadata, the command first attempts targeted
repair for small first-class record changes by parsing only the changed Markdown
record and replacing rows owned by that record. Broad changes, issue graph
changes, missing metadata, unsupported paths, invalid Markdown, schema drift,
conflicting records, missing required canonical state, or targeted repair
failures fall back to the safe rebuild path or block completion paths.
Orientation and repair surfaces should degrade at the record level where
possible instead of collapsing into cache-maintenance errors.

The projection is metadata-only in the target design: it may keep fields needed
to find, sort, filter, traverse, and validate records, but it must not be
treated as a full Markdown mirror for detail rendering. Current SQLite columns
are classified as follows:

| Table or field | Classification | Target ownership |
| --- | --- | --- |
| `issues.id`, `title`, `status`, `issue_type`, `priority`, `parent_id`, `created_at`, `updated_at`, `closed_at` | Projection metadata | Keep as the issue list, ready-work, graph, workflow, and Mission Control summary index. These fields are small and commonly used for sorting and filtering. |
| `issues.description` | Derived search index / removal candidate | Canonical Markdown body sections are owned by `RecordStore`; detail views load them from `.atelier/issues/*.md`. Rebuild currently stores derived section text here only for legacy search projection, not as a full Markdown body mirror. |
| `labels.issue_id`, `labels.label` | Projection metadata | Keep for queue filters, ownership labels, and Mission Control facets. |
| `dependencies.blocker_id`, `dependencies.blocked_id` | Projection metadata | Keep as derived graph edges for ready queries and workflow checks. |
| `relations.issue_id_1`, `relations.issue_id_2`, `relations.relation_type`, `relations.created_at` | Projection metadata | Keep as derived typed issue-link edges for traversal and impact views. |
| `records.kind`, `records.id`, `records.title`, `records.status`, `records.created_at`, `records.updated_at`, `records.source_path` | Projection metadata | Covered cross-record metadata index for missions and evidence. Full bodies and typed detail remain owned by Markdown. |
| `record_labels.kind`, `record_labels.id`, `record_labels.label` | Projection metadata | Covered label index for first-class non-issue records. |
| `evidence.*` | Projection metadata | Narrow typed satellites for query-worthy fields. They are derived from Markdown and do not store generic JSON payloads. Plan and milestone satellites are not active v1 storage targets. |
| `record_links.source_kind`, `source_id`, `target_kind`, `target_id`, `relation_type`, `created_at` | Projection metadata | Keep as derived cross-record graph edges for workflow validation and Mission Control rollups. |
| `projection_sources.path`, `kind`, `id`, `size_bytes`, `modified_micros`, `sha256`, `indexed_at` | Projection metadata | Keep as rebuildable freshness metadata. Kind/id allow deleted-source repair; size and modified time are the fast path, with hash used for changed candidates. |
| `sessions.*`, `work_associations.*`, `runtime_metadata` | Removed local-only SQLite state | These tables are not part of the target schema because SQLite tracker state must be rebuildable from canonical Markdown. |
| `comments.content`, `comments.kind`, `comments.created_at` | Compatibility residue | Legacy SQLite notes retained for imports and repositories not yet migrated to activity sidecars. New issue activity detail is canonical Markdown sidecar state. |
| Dropped `token_usage`, `time_entries`, `milestones`, `milestone_issues` | Compatibility removal | Already removed from the active schema after their command surfaces or replacement record forms superseded them. |

Representative detail paths for this boundary are `atelier issue show`,
`atelier issue status <objective-id>`, and `atelier evidence show`: the
commands use SQLite to resolve requested IDs, relationships, and graph/runtime
metadata, then load the selected Markdown payload from `RecordStore` before
rendering. `atelier search` also matches issue titles and bodies from
canonical issue files, with comment/note text read from canonical activity
sidecars instead of legacy SQLite `comments.content`. This allows frequent
polling surfaces such as Mission Control to use small SQLite rows for candidate
lists without treating every Markdown body or record payload as cached UI state.

## Rebuild And Freshness

`atelier rebuild` recreates the canonical portion of
`.atelier/runtime/state.db` from Markdown records discovered under tracked
`.atelier/` record directories. It drops local-only SQLite state rather than
preserving non-Markdown tracker facts.

Issue activity history is canonical sidecar state under
`.atelier/issues/<issue-id>.activity/`. Each activity entry is a Markdown
file named with a UTC microsecond timestamp ID:
`YYYYMMDDTHHMMSSffffffZ.md`. If multiple entries share the same timestamp,
writers append deterministic `-01`, `-02`, and later suffixes while refusing to
overwrite an existing file.

Decision: canonical activity sidecar APIs belong in `atelier-records`, not in
`atelier-sqlite`. `atelier-records::activity` is the storage boundary for the
sidecar schema and filesystem operations; higher layers may wrap those APIs for
use-case orchestration, but the rebuildable SQLite projection must not read or
write `.atelier/issues/<id>.activity/*.md` as a comment adapter.

Ownership is intentionally split:

- `src/activity.rs` owns sidecar schema, parsing, ID allocation, atomic
  create-new writes, ordering, and validation.
- `atelier-app` may expose command-oriented issue-note, import, history, and
  evidence-attachment workflows that coordinate activity writes with
  projection refresh, workflow checks, and rendered outcomes.
- `src/commands/activity_log.rs` is a thin CLI adapter that converts command
  events into sidecar events. Its cwd-based `.atelier` discovery is tolerated
  only at the command boundary for callers that do not already carry a
  `StorageLayout`.
- `RecordStore` owns first-class issue, mission, and evidence records. It must
  not absorb activity event payloads or project activity into record
  `relationships`.
- `export`, `rebuild`, `lint`, `history`, import preservation, issue note
  commands, issue detail views, and tests consume sidecars through
  `atelier-records::activity` directly or through app-level workflows built on
  that API.
- `atelier-sqlite::Database` owns projection queries only. Its legacy
  `add_comment*` and `get_comments` helpers are compatibility residue until
  storage-boundary cleanup removes or confines them; they are not the target
  sidecar API for new command, import, history, or test code.
- The runtime projection does not index sidecar payloads as source rows.

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
from tracked `.atelier/` records alone. Projection freshness intentionally
does not index `.activity/` files; changing only activity sidecars should not
make the SQLite projection stale, because history and recent-activity views read
the sidecars directly.

## Durable Mutation Audit

Audit date: 2026-06-11. The current command surface has three write classes:

| Path | Classification | Notes |
| --- | --- | --- |
| `RecordStore` issue and domain APIs | Markdown-first through `RecordStore` | Issue and first-class record helpers allocate IDs, validate, render, atomically replace canonical Markdown, remove canonical files for deletes, and mutate canonical relationship front matter. |
| Issue create/update/close/reopen/comment/label/unlabel/block/unblock/relate/unrelate/subissue/quick | RecordStore-owned Markdown-first | Public commands write canonical issue Markdown or activity sidecars first, then refresh the SQLite projection. Compatibility SQLite issue mutation helpers remain only for legacy tests/import internals. |
| Issue delete and close-all | RecordStore-owned Markdown-first | Delete removes canonical issue Markdown before projection refresh. Close-all rewrites matching canonical issues through the lifecycle close path. |
| `dep add` and `dep remove` | RecordStore-owned Markdown-first | Top-level Agent Factory dependency aliases mutate canonical issue relationship front matter before projection refresh. `dep list` is query-only and checks projection freshness. |
| Mission create/update/add-work/add-blocker | RecordStore-owned Markdown-first | Mission records and cross-record links write canonical mission Markdown and relationships before projection refresh. |
| Plan create/revise/link | Removed/deferred | V1 plans are ordinary Markdown artifacts or prose references, not `.atelier/plans/` records. |
| Bundle apply | RecordStore-owned Markdown-first | Bundle apply writes issue, mission, evidence, and relationship records through staged canonical Markdown, then refreshes projection after successful writes. |
| Evidence add/attach | RecordStore-owned Markdown-first | Evidence records and attachment links write canonical evidence Markdown and relationships before projection refresh. Issue evidence attachments also write issue activity sidecars. |
| Typed record links, labels, and dependencies | RecordStore-owned Markdown-first | Rebuild derives labels, dependency edges, typed relations, hierarchy, and record links from canonical relationship front matter. |
| Workflow validate | Runtime/query-only | Built-in validators read projection state and do not persist validator-result records. `workflow_validator` is registered as a future non-canonical record kind only. |
| Work start/finish and worktree helpers | Runtime plus activity sidecars | Runtime state may cache checkout/session context in `.atelier/runtime/`, but the durable current-work source of truth is the canonical issue status in tracked Markdown. The durable side effect beyond that status is the issue activity sidecar, which is written directly under `.atelier/issues/<id>.activity/` when the issue Markdown file exists. |
| Diagnostics, telemetry, tested marker, init, import-beads, export, rebuild, lint, doctor | Runtime, maintenance, import, or repair | Telemetry and work markers are local/runtime. `import-beads` is an external import bridge that still renders canonical state from imported SQLite rows. `export` and `rebuild` are repair/projection commands, not normal durable mutation owners. |

The remaining compatibility residue is internal: several inherited SQLite
mutation helpers are still compiled for unit tests, imports, and repair flows,
but normal public durable commands no longer call export to become recoverable.

Current caller map for activity sidecars:

| Caller | Destination boundary | Notes |
| --- | --- | --- |
| `atelier-records/src/activity.rs` | Canonical owner | Owns sidecar paths, schema, parsing, rendering, timestamp ID allocation, listing, and create-new writes. |
| CLI `issue note`, work lifecycle, transition, evidence attachment, and bundle note adapters | App/CLI orchestration over `atelier-records::activity` | Command code converts user actions into typed activity events; follow-on app extraction should move orchestration upward without changing the storage owner. |
| `atelier history`, issue show recent activity, Agent Factory status helpers, `export`, `rebuild`, and `lint` | Read-only consumers over `atelier-records::activity` | These surfaces may combine projection rows with sidecar events, but sidecar files remain the canonical history payload. |
| `import-beads` preservation notes and close reasons | App/import workflow over `atelier-records::activity` | Imported predecessor comments are migration input and should be written as activity sidecars, preserving source timestamps when available. |
| `atelier-sqlite/src/comments.rs` | Removal target for `atelier-2573` | Current read/write use of `create_issue_activity` and `list_issue_activities` blurs the projection boundary and should be removed or confined to compatibility tests/import scaffolding. |

## Projection Refresh After Canonical Writes

`src/command_storage.rs` owns the command-side storage access policy. Command
routing in `src/main.rs` selects an access mode, while the command storage
boundary opens the runtime database, checks projection freshness for query and
canonical-mutation commands, transparently rebuilds missing or stale
projections, and names degraded-orientation behavior. This keeps CLI dispatch
focused on public command contracts instead of embedding projection freshness
rules in every match arm.

For RecordStore-owned mutations, Atelier refreshes the projection after each
successful canonical write. Mutation commands write
tracked `.atelier/` records first through `RecordStore`, then call
`refresh_projection_after_canonical_write` to validate canonical Markdown and
replace projection rows from that source. This preserves the clear ownership
boundary: Markdown is the durable write target, while SQLite is the disposable
covered index used by subsequent query commands.

The refresh helper deliberately runs canonical validation before replacing the
projection. If a command writes invalid Markdown, the command must fail before
answering from stale SQLite. If projection replacement fails after a successful
canonical write, the canonical files remain durable and ordinary query commands
can recover through the transparent projection repair path once the reported
record or workflow problem is fixed.

Hidden/admin `atelier export` remains available for migration compatibility and
deterministic-renderer testing. New durable mutation paths must not use export
as the normal step that makes command output recoverable, and normal ignored
runtime/projection repair belongs to `doctor --fix`.

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

- ephemeral checkout/session context around canonical current work;
- local claim-helper compatibility residue while legacy flows are being removed;
- cached projection metadata;
- UI state and terminal-view caches.

Runtime state may reference canonical record IDs, but those references are local
and disposable unless a future durable record explicitly captures them. A fresh
checkout must be able to rebuild canonical query behavior from tracked
`.atelier/` records without copying `.atelier/runtime/state.db`.

## Migration Plan

Staged implementation note: missions, evidence, workflow validator results, and
cross-record links use first-class SQLite projection tables and deterministic
Markdown export/rebuild support where they mutate durable records. Mission and
evidence records are durable and rebuildable; first-class plan and milestone
records are deferred rather than active storage targets.

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
   evidence, and workflow validator support as those command surfaces land;
   keep first-class plan and milestone records deferred until a new contract
   reintroduces them directly.
7. Retire the compatibility requirement that normal mutations write SQLite first
   and then call export to become durable. This is complete for normal public
   durable commands; export remains a hidden/admin migration or deterministic
   rendering surface, and imports may still use export as an import bridge.

Each slice that touches storage internals must preserve `atelier lint`,
`atelier doctor`, the agent-facing issue workflow, and any explicitly retained
hidden/admin projection diagnostics such as `atelier rebuild` or
`atelier export --check`; otherwise it must state the temporary breakage and the
reconnect item that owns it.

## Non-Goals

- Do not maintain a fully equivalent SQLite and Markdown live-state sync model
  as the destination.
- Do not introduce a daemon only to keep projections fresh; add one only after
  an interactive workflow proves it needs a long-lived process.
- Do not move workflow policy into ad hoc runtime files; repository-authored
  policy belongs in tracked `.atelier/config.toml` or a documented policy file
  selected by that config, separate from deterministic record projections.
- Do not restore `manifest.json` or `graph.json` as canonical source files.
