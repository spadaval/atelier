# Markdown-First Record Store

This document defines Atelier's target persistence architecture: durable record
files, concrete domain services, and a disposable SQLite domain cache. The
historical export/rebuild specification is migration background only; this
document and [SQLite Domain Cache Schema](sqlite-runtime-schema.md) define the
target contract.

## Direction

Atelier's durable project state lives in tracked Markdown record files under
`.atelier/`. SQLite remains valuable as a rebuildable lazy domain cache, not a
second project-state model or a copy of complete record content.

The target architecture has three explicit layers:

| Layer | Owns | Does not own |
| --- | --- | --- |
| Record files / `RecordStore` | Markdown record discovery, parsing, validation, ID allocation, deterministic writes, atomic file replacement, and known-ID mutations. | Global query planning, runtime-only checkout/session context, or cache rows. |
| Concrete domain types | Typed record semantics, record-local invariants, and complete record-file content. | Generic escaped payloads or a universal record object schema. |
| SQLite domain cache | Rebuildable selected facts for work queues, ready queries, traversal, search, validation, and Mission Control inputs. | Record-file mutation, complete Markdown bodies, or facts that cannot be recreated from record files. |

Ignored local diagnostics, lock files, and UI caches may exist beside these
components, but they are not SQLite tracker state and must not define durable
project records or current work.

Successful durable mutations write record files first and invalidate the cache
facts derived from the changed source. They do not synchronously rebuild or
refresh query rows as a write-path requirement. Query commands go through
`CacheManager`, which opens the cache on demand, checks source freshness, and
repairs changed records lazily before answering. User-facing recovery names the
record-file or workflow problem; operators do not maintain cache state.
[SQLite Domain Cache Schema](sqlite-runtime-schema.md) is authoritative for
cache tables and the shared full/incremental indexing pipeline.

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
5. Invalidate the affected source metadata, or otherwise leave enough source
   information for `CacheManager` to detect the change on the next cache-backed
   query.

Mutation handlers do not call a refresh-after-write helper. They render
confirmation from the record they just wrote and leave `state.db` detectably
stale. A mutation that needs indexed preconditions obtains a fresh cache before
writing; only output that genuinely needs cache-derived graph context performs
an explicit CacheManager read afterward. Multi-record bundle, relationship,
review, and evidence writes therefore complete before one later lazy repair.

New-record creation allocates a project-scoped random ID through `RecordStore`,
checks for local file collisions across all record kinds, writes the Markdown
record, and leaves it discoverable by the next lazy cache-freshness check. The
allocator must not rely on a SQLite sequence as the source of durable identity.

First-class non-issue record kinds are registered centrally in code with their
record directory, schema, and schema version. Rebuild, cache indexing, and link
validation consume that registry instead of carrying command-local record-kind
lists. The active v1 non-issue kinds are evidence and review. Missions use the
issue record contract and are issue records with `issue_type: "mission"`. Plan,
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

The issue-focused `RecordStore` API owns `.atelier/issues/*.md`: issue record
discovery, path validation, schema and front matter parsing, deterministic
rendering, ID collision checks across record directories, and atomic issue file
replacement. Normal commands use these APIs directly. Export from SQLite is
not a target write, validation, or recovery path.

## Canonical Field Ownership

This section defines the durable field contract for Markdown-first records. Each
field belongs to exactly one of these classes:

| Class | Meaning |
| --- | --- |
| Required | Must be present in canonical Markdown for that record kind. Missing data is a lint/rebuild error. |
| Optional | May be present when the record needs the field. Omit it instead of writing an empty migration placeholder unless the record kind says otherwise. |
| Derived | Never authored directly in a record file. Domain services or cache queries compute it from durable fields. |
| Migration input | Historical input that must be converted before target readers accept it; target readers and writers do not preserve or fall back to it. |
| Forbidden | Not allowed in canonical Markdown because another field or section already owns the meaning. |

Common front matter ownership for first-class records (`issue`, `mission`, and
`evidence`) is:

| Field | Class | Notes |
| --- | --- | --- |
| `schema`, `schema_version`, `id`, `title`, `status`, `created_at`, `updated_at`, `labels`, `relationships` | Required | Shared record identity, lifecycle, timestamps, labels, and typed links. |
| Canonical file path | Derived | Computed from record kind plus `id`; it is never duplicated in front matter. |
| Status category, ready/done grouping, priority display label, reverse-link views, and query/cache rows | Derived | Domain services and domain-shaped cache queries compute these from durable fields. |
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
| Migration input | Legacy SQLite `description` values and unstructured imported prose must be converted to required record-file sections before target readers accept them. |
| Forbidden | Front matter or duplicate body fields named `description`, `outcome`, `acceptance`, `evidence_required`, `depends_on`, or `blocked_by`. |

Issue priority is durable front matter owned by the issue record. The current
canonical vocabulary is `P0`, `P1`, `P2`, and `P3`; human-facing terms such as
`critical`, `high`, `medium`, and `low` are derived presentation labels, not
separate canonical fields.

Issue status is durable workflow state owned by `.atelier/workflow.yaml`. The
repository-defined vocabulary includes `draft`, `plan_review`, `ready`,
`todo`, `in_progress`, `blocked`, `review`, `publish_review`, `validation`,
`done`, `closed`, and `superseded`. Human-ready groupings such as `todo`,
`active`, `blocked`, and `done` are derived categories, not alternate stored
tokens; `plan_review`, `review`, and `validation` are workflow statuses in the
active category.

### Mission Records

| Slice | Ownership |
| --- | --- |
| Required front matter | Common fields only. Mission semantics do not own extra scalar or JSON payload keys in front matter. |
| Optional front matter | None in V1 beyond record-generic labels and relationships. |
| Required body | `## Intent`, `## Constraints`, `## Risks`, and `## Validation`. |
| Optional body | `## Terminal Notes` and `## Notes`. |
| Derived | Linked work from `relationships.relates[]` entries with `type: advances`; direct mission blockers from `relationships.relates[]` entries with `type: blocked_by`; mission evidence coverage from incoming evidence links with `role: validates`; current mission-plan review state from canonical revision-bound review events. |
| Migration input | None. |
| Forbidden | Escaped mission `data` payloads, front matter keys such as `constraints`, `risks`, `validation`, `work`, `plans`, `milestones`, `evidence`, `blockers`, or `terminal_notes`, and any second relationship surface for work, blockers, plans, checkpoints, or evidence. Mission prose may reference plan/checkpoint Markdown by path, but must not become a shadow graph. |

Mission objective status is type-aware issue workflow state. This repository's
accepted target planning path is `draft`, `plan_review`, `ready`, and
`in_progress`, with terminal states declared by workflow policy. Mission-plan request,
authorship/material-edit attribution, finding, change-request, resolution,
approval, and migration events are tracked canonical history linked to the
mission in its `.atelier/issues/<mission-id>.activity/` sidecar stream; they do
not create first-class plan records, a mutable review snapshot, or use the
code-review artifact field.

### Deferred Plan Records

Plan records are not active v1 canonical Markdown records. Execution intent
that must survive the current chat should be an ordinary Markdown artifact or
prose referenced from a mission, epic, issue, or evidence body. There is no
`.atelier/plans/` directory, plan status lifecycle, or `plans.*` cache
table in the v1 target contract.

### Evidence Records

| Slice | Ownership |
| --- | --- |
| Required front matter | Common fields plus `evidence_type`, `captured_at`, and any validating targets expressed as `relationships.attachments[]` entries with `role: validates`. |
| Optional front matter | `command`, `artifact`, `agent_identity`, `independence_level`, `proof_scope`, `residual_risks`, and `follow_up_ids`. |
| Required body | Human-readable proof summary and any important limits not already captured in front matter. |
| Optional body | Additional bounded transcript excerpts, audit notes, or artifact context. |
| Derived | Evidence coverage views for issues and missions; command success summaries from structured transcript metadata; reverse lookup of which records this evidence validates. |
| Migration input | None. |
| Forbidden | Escaped `data` payloads and separate `targets` or `validates` front matter arrays, because validating links belong in `relationships.attachments[]`. |

Evidence `status` is the canonical proof result token. The target vocabulary is
`pass`, `fail`, `blocked`, `deferred`, `not_applicable`, or `informational`.
Records that still mirror result-like fields inside escaped `data` are invalid
canonical Markdown and must be migrated before rebuild/lint can pass.

### Deferred Checkpoint Records

Milestone/checkpoint records are not active v1 canonical Markdown records.
Checkpoint criteria and target-state prose may live in missions, epics, issues,
or evidence, but there is no `.atelier/milestones/` directory, milestone
completion state, or `milestones.*` cache table in the v1 target contract.

### Activity Sidecars

Activity sidecars are canonical durable history owned by the activity sidecar
API in `src/activity.rs`, not by `RecordStore` and not by the SQLite domain
cache. They are durable Markdown files, but they are not first-class
records that share the common `title`/`status` contract.

| Slice | Ownership |
| --- | --- |
| Required front matter | `schema`, `schema_version`, `id`, `subject_kind`, `subject_id`, `event_type`, `actor`, `created_at`, and `summary`. |
| Optional front matter | Event-specific lightweight fields such as `evidence_id`, `result`, `field`, `old`, or `new` when the event kind needs them. |
| Required body | User-authored text or lightweight event detail. Empty body is allowed only when the event-specific front matter fully carries the event payload. |
| Derived | Sidecar path from `subject_id` plus timestamp ID; chronological ordering; recent-activity previews. |
| Migration input | SQLite comments and imported close reasons must be converted to sidecars before target readers accept them. |
| Forbidden | `relationships`, `labels`, `priority`, or generic record payload blobs. Activity sidecars are event logs, not another record graph. |

### Runtime, Cache, Config, And Provenance

| Surface | Ownership |
| --- | --- |
| Tracked config | `.atelier/config.toml` is the only durable config record in this scope. Required fields are the project config schema/version, `project_slug`, and `[paths].state_root`; optional `[prune]` fields own project cleanup defaults. |
| Migration config | No migration-state or legacy SQLite path is tracked in project config. |
| Local domain-cache state | `.atelier/runtime/state.db`, `.atelier/runtime/`, `.atelier/cache/`, lock files, diagnostics, and UI caches are ignored machine-local artifacts. SQLite tables under `state.db` are rebuildable cache state, not non-Markdown tracker facts. |
| Cache source metadata | `record_source_index` rows, record identity, file size hints, mtimes, optional hashes, and reindex timestamps are derived SQLite metadata, not record-file fields. |
| Forbidden durable provenance | Runtime branch names, worktree paths, session IDs, lock ownership, local diagnostic output, and cache payloads must not be promoted into canonical record front matter without a separate artifact update. |

### Manual Classification Check

Manual classification run date: 2026-06-13. This is a representative check of
current committed records against the target contract above.

| Sample | Record kind | Result | Notes |
| --- | --- | --- | --- |
| `.atelier/issues/atelier-x45p.md` | Issue | Pass | Uses required issue front matter plus `Description`/`Outcome` and optional `Evidence`; blocker intent lives in `relationships.blocks`; durable priority/status tokens are `P1` and `todo`. |
| `.atelier/issues/atelier-man9.md` | Mission objective | Pass | Uses `schema: "atelier.issue"` with `issue_type: "mission"` and `relationships.relates[]` `type: advances` links for work. No escaped JSON mission payload remains. |
| `.atelier/evidence/atelier-06rb.md` | Evidence | Fail (forbidden payload residue present) | The record uses canonical `relationships.attachments[] role=validates`, but it still stores proof metadata in escaped `data` instead of owned first-class fields such as `evidence_type`, `captured_at`, and `proof_scope`. |
| `.atelier/issues/atelier-0001.activity/20260611T204233793564Z.md` | Activity sidecar | Pass | Uses required activity front matter. Event payload keys `field`, `old`, and `new` are acceptable event-specific detail, not a second relationship or status model. |
| `.atelier/config.toml` | Project config/runtime boundary | Pass | Tracks record-path ownership and prune defaults without committed runtime/cache or migration-state path settings. |
| `.atelier/plans/` | Plan | Deferred | No active v1 plan record table exists; planning intent is ordinary Markdown or prose referenced from accountable records. |
| `.atelier/milestones/` | Milestone | Deferred | No active v1 milestone record table exists; checkpoint intent is ordinary Markdown or prose referenced from accountable records. |

## Query Path

Query commands use `CacheManager` when they need selected facts across many
records:

- `atelier work ready`, `atelier work blocked`, `atelier issue list`, objective
  detail views, dependency views, and graph traversal;
- workflow validator lookup and transition checks;
- Mission Control and terminal UI inputs;
- lint rules that need reverse links or whole-project consistency.

The application boundary separates discovery from access. Constructing a
`CacheManager`, asking for repository/cache paths, or inspecting cache health
does not create or open SQLite. `get_cache(Decision)` is the only normal path
for decision-bearing reads and returns only after freshness repair or rebuild;
`get_cache(Orientation)` may return the last good cache solely for read-only
orientation when canonical records are invalid, and must name that degraded
state. Health commands use the explicit raw inspection/open boundary when they
own repair policy. Missing cache files, cache schema-version mismatches, and
corrupt cache files are disposable-state rebuild signals, not migrations or
operator-managed errors.

The degraded orientation allowance is intentionally narrow: checkout status
and single-record detail views may use it. Lists, ready/blocker calculations,
transition options, workflow/evidence/review lookups, history/graph traversal,
and branch decisions use `Decision` access and fail rather than expose known-
stale rows. Central CLI dispatch selects these policies through named
application accessors; command modules do not open cache databases for indexed
read acquisition. Lint, export-check, doctor, and rebuild remain explicit
health/repair consumers because they must diagnose invalid canonical state
rather than have freshness acquisition reject it first.

`CacheManager` discovers supported record-file sources and compares them with
`record_source_index`. File size and modified time are candidate-selection
hints. A candidate whose hints changed is parsed directly; an implementation
may use a retained content hash to avoid unnecessary parsing when filesystem
metadata is noisy, but hashing is never required before parsing a changed
candidate. Added and deleted paths are detected by comparing complete source
sets rather than looking only at existing cache rows.

For a changed source, `CacheManager` calls the concrete record parser and the
same domain-specific indexer used by a full rebuild. It replaces all cache rows
owned by that record in one transaction. Deleted sources remove their owned
rows in one transaction. If a record kind cannot be repaired incrementally, the
manager runs the safe rebuild path when the command contract permits. Read-only
commands must not silently answer from known-stale cache facts when the result
could affect orchestration, validation, or closeout decisions.

The cache exposes domain-shaped read models only. The required families are
issue summaries, labels, blockers and typed issue relations; evidence summaries
and evidence targets; native review-room workflow facts; and source-freshness
metadata. Their physical table contract and query justification live in
[SQLite Domain Cache Schema](sqlite-runtime-schema.md). There is no universal
`records` table, `record_labels` satellite, `record_links` graph, or generic
payload row in the target architecture.

Detail paths such as `atelier issue show`, `atelier issue show
<objective-id>`, and `atelier evidence show` may use the cache to find an ID or
related candidates, then load complete content from the corresponding record
file before rendering. Activity history and recent-activity previews read
activity sidecars directly. Frequent polling surfaces may use small domain
cache rows for candidate lists without treating complete record objects or
Markdown bodies as cached UI state.

## Hidden Rebuild Diagnostic And Freshness

`atelier rebuild` recreates `.atelier/runtime/state.db` from record files
discovered under tracked `.atelier/` record directories. Cache schema/version
changes replace the local database; they do not migrate or preserve SQLite
rows. No durable project fact may exist only in the database.

Issue activity history is durable sidecar state under
`.atelier/issues/<issue-id>.activity/`. Each activity entry is a Markdown
file named with a UTC microsecond timestamp ID:
`YYYYMMDDTHHMMSSffffffZ.md`. If multiple entries share the same timestamp,
writers append deterministic `-01`, `-02`, and later suffixes while refusing to
overwrite an existing file.

Decision: activity sidecar APIs belong in `atelier-records`, not in
`atelier-sqlite`. `atelier-records::activity` is the storage boundary for the
sidecar schema and filesystem operations; higher layers may wrap those APIs for
use-case orchestration, but the SQLite domain cache must not read or
write `.atelier/issues/<id>.activity/*.md` as a comment adapter.

Ownership is intentionally split:

- `src/activity.rs` owns sidecar schema, parsing, ID allocation, atomic
  create-new writes, ordering, and validation.
- `atelier-app` may expose command-oriented issue-note, import, history, and
  evidence-attachment workflows that coordinate activity writes with workflow
  checks and rendered outcomes.
- `src/commands/activity_log.rs` is a thin CLI adapter that converts command
  events into sidecar events. Its cwd-based `.atelier` discovery is tolerated
  only at the command boundary for callers that do not already carry a
  `StorageLayout`.
- `RecordStore` owns first-class issue, evidence, and review record files;
  mission objectives use the issue record contract with
  `issue_type: "mission"`. Rebuild and cache-source coverage therefore spans
  issue, evidence, and review. `RecordStore` must not absorb activity event
  payloads or project activity into record `relationships`.
- `rebuild`, `check`, `history`, import conversion, issue note commands, issue
  detail views, and tests consume sidecars through
  `atelier-records::activity` directly or through app-level workflows built on
  that API.
- `atelier-sqlite` owns domain-cache queries only and has no activity-sidecar
  read or write adapter.
- The domain cache does not index sidecar payloads as source rows.

Activity front matter uses `schema: "atelier.activity"` and
`schema_version: 1` with these required fields:

- `id`: timestamp activity ID matching the file name.
- `subject_kind`: `issue` in V1.
- `subject_id`: canonical issue ID.
- `event_type`: one of `comment`, `note`, `handoff`, `plan`,
  `close_reason`, `status_changed`, `field_changed`, `work_started`,
  `work_finished`, `work_abandoned`, `evidence_attached`,
  `transition_applied`, `transition_blocked`, or `mission_plan_review`.
- `actor`: user or agent identity that produced the event.
- `created_at`: RFC3339 timestamp.
- `summary`: one-line event summary.

The Markdown body stores user-authored text or lightweight event details.
Evidence remains a rich first-class record under `.atelier/evidence/`;
issue activity records only lightweight `evidence_attached` references such as
`evidence_id` and `result` so operators can follow up with
`atelier evidence show`.

`transition_applied` activities may carry a strictly typed
`workflow_transition` front-matter object naming the transition and exact
from/to statuses. A mission `start` from `ready` to `in_progress` additionally
carries `mission_plan_start`, bound to the current graph revision and the exact
prior approval activity ID. Canonical validation accepts active mission state
only when this receipt resolves to that approval. Free-form Markdown body text,
including text shaped like transition fields, is never workflow authority.

`mission_plan_review` activities additionally require a strictly typed
`mission_plan_review` front-matter object. Its event kind is one of `request`,
`material_edit_attribution`, `finding`, `change_request`, `resolution`,
`approval`, or `legacy_grandfather`. Every kind names the versioned graph
revision it concerns; requests record the complete author/material-editor
provenance snapshot, attributions chain a new revision to its predecessor,
findings and change requests carry stable decision IDs and affected issue or
dependency-path IDs, resolutions target those IDs, and approval identity comes
from the activity actor. Actor identities use
`actor-v1:<authenticated-authority>/<immutable-subject>`. The authority is a
canonical lowercase DNS-style namespace whose authentication layer owns the
immutable subject mapping; display names are not actor IDs. Identities must be
NFC, contain no whitespace, controls, or Unicode
`Default_Ignorable_Code_Point` characters, and use one canonical spelling, so
zero-width or decomposed Unicode aliases cannot create false independence.

Public review mutations validate every affected ID and dependency-path node
against the exact canonical mission graph before allocating or writing an
activity file. The same validator runs during canonical rebuild. Material edits
made while a mission is in `plan_review` are resubmitted with a typed
`material_edit_attribution` that links the prior provenance revision to the
current revision and adds the authenticated editor. Resolutions retain the
target decision's original graph revision across that rework boundary; they do
not rewrite the decision or make a stale approval current.

A legacy grandfather is limited to cutover status `in_progress`, migration ID
`independent-mission-plan-review-v1`, and activity actor
`actor-v1:atelier.local/mission-review-migration`. The tracked
`.atelier/mission-plan-review-cutover.yaml` manifest records the exact cutover
time and a sorted eligibility entry for each mission, graph revision, status,
and receipt activity ID. The `legacy_grandfather` event is the exactly-once
receipt: its timestamp, activity ID, mission, revision, status, migration ID,
and versioned receipt digest must all match that manifest. Missing, late,
duplicate, wrong-mission, or forged receipts fail rebuild rather than project
authority. Cutover and activity timestamp values use canonical UTC RFC3339
precision no finer than microseconds; canonical activity rendering always uses
six fractional digits. Activity producers truncate their timestamps to that
precision before ID allocation, while cutover manifests reject finer precision
before receipt hashing and already-constructed noncanonical records are rejected
before emission or load. Thus the manifest, digest, rendered activity, and
rebuild comparison cannot disagree. A valid receipt is
fresh only while the mission still has the exact eligible status and graph
revision. Lists that represent sets are sorted and
unique so rendering and rebuild are deterministic. Malformed payloads, missing
provenance, non-independent approval, unresolved blocking decisions at approval
time, and plan-review events attached to non-mission issues make canonical
rebuild fail.

The graph revision string is `mission-graph-v2:sha256:<digest>`. The hashed
payload uses stable-ID ordering and contains the mission and reachable authored
planning sections, direct roots, hierarchy and `advances` edges, and blocker
edges touching reviewed scope. Blocker inputs include directional canonical
`blocks` buckets and direct `blocked_by` links; the revision version changed so
an approval produced under the incomplete V1 inputs cannot silently remain
fresh. Status, timestamps, notes, evidence receipts, code-review fields,
context-only links, activity, and cache state are excluded. The current review
projection is rebuilt directly from canonical issue records and sidecars; it is
not a mutable snapshot or a SQLite activity table.

`atelier issue show` uses the same sidecars for its bounded recent activity
preview. It does not fall back to SQLite notes or comments.

Imported predecessor comments and close reasons are migration input, not a
separate durable comment store. The accepted policy is:

- New Atelier notes, comments, handoffs, work-start/work-finish events,
  resolution comments, close reasons, and evidence attachments are durable
  issue activity sidecars.
- Imported comments from Beads or older SQLite rows may be converted into
  activity sidecars by an explicit migration step. Conversion preserves original
  author and timestamp when available and records the event type as `comment`,
  `note`, or `close_reason`.
- Imported SQLite comments must be converted to activity sidecars before the
  target reader uses them. The target cache has no comments table or fallback
  reader.
- Display order is chronological by `created_at`, then activity ID, then file
  path. Duplicate timestamps are represented by deterministic filename suffixes
  rather than overwriting entries.
- Merge conflict resolution is file-level: keep both valid sidecar files when
  two agents add distinct history entries, and edit the body/front matter only
  when the same activity file conflicts.

`atelier rebuild` validates sidecars and rejects activity entries whose subject
issue is missing. Cache freshness intentionally excludes `.activity/` files;
changing only activity sidecars does not make domain-cache rows stale because
history and recent-activity views read the sidecars directly.

## Durable Mutation Contract

| Path | Classification | Notes |
| --- | --- | --- |
| `RecordStore` issue and domain APIs | Record-file storage | Concrete domain helpers allocate IDs, validate, render, atomically replace record files, remove files for deletes, and mutate relationship front matter. |
| Issue create/update/close/reopen/label/unlabel/block/unblock/relate/unrelate/subissue/quick | RecordStore-owned mutation | Public commands write issue records first and invalidate affected cache source metadata. They do not write SQLite domain rows directly. |
| Issue note/comment and lifecycle activity | Activity-sidecar mutation | Commands append activity sidecars through `atelier-records::activity`; the cache does not own or mirror their payloads. |
| Issue delete and close-all | RecordStore-owned mutation | Delete removes the record file; close-all rewrites matching issue files through the lifecycle close path. Cache repair occurs lazily. |
| `atelier issue link <id> <target> --role blocked_by` and `atelier issue unlink <id> <target> --role blocked_by` | RecordStore-owned mutation | Typed issue relationship commands mutate issue relationship front matter. Issue detail is the query surface and goes through `CacheManager`. |
| Mission objective create/update/link/block | RecordStore-owned mutation | Mission objectives are issue records with `issue_type: "mission"`; links and blockers write issue record files and relationships. |
| Plan create/revise/link | Removed/deferred | V1 plans are ordinary Markdown artifacts or prose references, not `.atelier/plans/` records. |
| Bundle apply | RecordStore-owned mutation | Bundle apply stages issue-backed missions, ordinary issues, evidence, and relationships as record files, then invalidates affected cache sources after successful writes. |
| Evidence add/attach | RecordStore-owned mutation | Evidence records and attachment links write evidence files and relationships. Issue evidence attachments also append issue activity sidecars. |
| Typed links, labels, and blockers | Record-file facts | Cache rebuild and repair derive domain-specific label, blocker, and relation rows from concrete domain records. |
| Workflow validate | Cache-backed query | Built-in validators use domain services and cache read models but do not persist validator-result records. |
| Issue transitions and branch recovery | Record files plus activity sidecars | `atelier issue transition <id>` exposes readiness and executes configured lifecycle actions. Canonical issue status remains the durable current-work source of truth; manual owner-branch commands are recovery-only after failed transition actions. |
| Diagnostics, telemetry, import, hidden renderer/cache probes, and check | Local runtime, migration, or repair | Telemetry is local. Imports must emit valid record files. Hidden probes diagnose migration or cache behavior; `atelier check` and `atelier check --fix` own normal operator health and ignored-state repair. |

Current caller map for activity sidecars:

| Caller | Destination boundary | Notes |
| --- | --- | --- |
| `atelier-records/src/activity.rs` | Record-file owner | Owns sidecar paths, schema, parsing, rendering, timestamp ID allocation, listing, and create-new writes. |
| CLI `issue note`, work lifecycle, transition, evidence attachment, and bundle note adapters | App/CLI orchestration over `atelier-records::activity` | Command code converts user actions into typed activity events; follow-on app extraction should move orchestration upward without changing the storage owner. |
| `atelier history`, issue show recent activity, Agent Factory status helpers, `rebuild`, and `check` | Read-only consumers over `atelier-records::activity` | These surfaces may combine cache rows with sidecar events, but sidecar files remain the durable history payload. |
| `import-beads` preservation notes and close reasons | App/import workflow over `atelier-records::activity` | Imported predecessor comments are migration input and should be written as activity sidecars, preserving source timestamps when available. |

## Cache Invalidation After Record Writes

`CacheManager` is the command-side cache access boundary. CLI dispatch and
domain services do not open SQLite directly. Cache-backed query commands ask
the manager for a domain read model; the manager opens the database on demand,
checks source freshness, performs targeted repair or a safe rebuild, and then
runs the query.

RecordStore-owned mutations validate and atomically write tracked `.atelier/`
record files. After a successful write they invalidate affected source metadata
or leave the changed file discoverable by the next freshness comparison. They
do not require eager index replacement before returning. A later query repairs
the smallest safe unit by invoking the same concrete parser and indexer used by
full rebuild.

If cache invalidation fails after a successful record-file write, the record
file remains durable. The command reports the local cache problem, and the next
cache-backed query detects the source mismatch rather than trusting the old
row. Export from SQLite is not a recovery path.

`atelier check` reports record-file validity, local runtime, diagnostics, and
workflow health. `check --fix` or rebuild may recreate ignored cache state, but
neither rewrites record files merely to satisfy a cache schema.

## Runtime State Boundary

Runtime state remains useful for coordination and operator ergonomics:

- ephemeral checkout context around current work;
- cache source metadata;
- UI state and terminal-view caches.

Runtime state may reference durable record IDs, but those references are local
and disposable unless a future durable record explicitly captures them. A fresh
checkout must be able to rebuild cache-backed query behavior from tracked
`.atelier/` records without copying `.atelier/runtime/state.db`.

## Migration Plan

The implementation proceeds in dependency order:

1. Introduce a `RecordStore` module that can load, validate, render, and write
   issue record files using concrete domain types.
2. Define domain-shaped SQLite schema and shared per-record indexing functions.
3. Add `record_source_index`, lazy cache opening, and targeted changed-source
   repair through `CacheManager`.
4. Route issue, mission, evidence, relationship, and activity mutations through
   record-file services and invalidate cache sources after writes.
5. Route global query and validator paths through `CacheManager` before removing
   eager refresh helpers and generic cache APIs.
6. Hard-remove universal record tables, generic label/link satellites,
   SQLite-first or dual-write mutations, compatibility readers, and obsolete
   projection terminology.

Each slice that touches persistence must preserve `atelier check`, the
agent-facing issue workflow, record-file round trips, cache freshness behavior,
and the supported rebuild diagnostic. Temporary breakage requires an explicit
reconnect owner.

## Non-Goals

- Do not maintain a fully equivalent SQLite and Markdown live-state sync model
  as the destination.
- Do not introduce a daemon only to keep the cache fresh; add one only after
  an interactive workflow proves it needs a long-lived process.
- Do not move workflow policy into ad hoc runtime files; repository-authored
  policy belongs in tracked `.atelier/config.toml` or a documented policy file
  selected by that config, separate from record files and domain-cache state.
- Do not restore `manifest.json` or `graph.json` as canonical source files.
