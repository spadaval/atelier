# SQLite Domain Cache Schema

Atelier uses one ignored local SQLite database at `.atelier/runtime/state.db`.
It is a disposable domain cache: record files under `.atelier/` are the
durable project state, and rebuilding or deleting the database must never lose
tracked work.

This is deliberately not a generic record graph and not a partial mirror of
record-file objects. A cache row exists only when it supports a common command
operation: filtering, sorting, graph traversal, validation, or a bounded
lookup. Detail rendering loads the complete record file through `RecordStore`.

## Cache Read Models

The target cache has the following domain-shaped tables. Column names are
illustrative ownership, not a promise that every implementation query uses the
same physical index.

| Table | Cached facts | Why it is cached |
| --- | --- | --- |
| `issue_index` | `id`, `title`, `status`, `issue_type`, `priority`, timestamps, and parent/structural identity when applicable | Work queues, ready-work filtering, status/priority ordering, workflow checks, and Mission Control summaries. |
| `issue_label_index` | issue ID and label | Label filters and facets. |
| `issue_block_index` | blocker ID and blocked ID | Ready-work calculation, dependency views, cycle checks, and impact traversal. |
| `issue_relation_index` | source issue, target issue, relation type, and ordering/timestamp facts | Typed-link traversal and impact views. |
| `evidence_index` | evidence ID, status, evidence type, captured time, and other explicitly query-worthy evidence metadata | Evidence coverage and closeout/validation lookup. |
| `evidence_target_index` | evidence ID, target kind/ID, and attachment role | Reverse evidence coverage for issues, epics, and missions. |
| `review_room_index` | native room ID, owning issue ID, and normalized room state facts required by workflow validation | Native-room review/validation lookup rebuilt from tracked `.atelier/reviews/<id>.yaml` files. |
| `record_source_index` | relative record-file path, record kind/ID, size hint, modified-time hint, optional content hash, and indexed time | Freshness detection, deleted-source repair, and targeted reindexing. |

The index may add normal relational indexes for those access paths. It must not
introduce a universal `records` table, generic JSON payload column, generic
label/link satellites, or a table that needs to understand every record kind.
New cached facts require a named domain read model and a documented query need.

## Record Files Retain Complete Content

Record files retain complete domain content: authored body sections; full
relationship entries and their typed metadata; optional fields; evidence prose
and proof context; activity sidecars; and any field used only to render one
record in detail. SQLite may cache a small search token or a normalized value
when a documented lookup needs it, but it does not own an alternate body,
escaped payload, or complete object representation.

`RecordStore` parses and validates those concrete record types. Cache rows are
derived read models, so commands that need a complete record must load its
record file after the cache identifies it.

`review_room_index` is supported only for native rooms. Its discovery path is
the tracked `.atelier/reviews/<id>.yaml` tree; `index_review_room` parses each
room YAML artifact during both full rebuild and incremental repair, and its
source row lives in `record_source_index`. The durable issue review link and
the room YAML file are sufficient to rebuild this index.

Hosted Forgejo/provider responses are not rows in the SQLite domain cache and
are not indexed by its rebuild/repair pipeline. The durable issue review link
identifies the external artifact, but the provider's current response state is
external and is fetched by the provider adapter when a validator needs it. An
adapter may keep a bounded, ignored provider-response cache outside
`.atelier/runtime/state.db`, keyed by provider and normalized review identity,
with a short TTL or single-command lifetime. It must be invalidated on explicit
refresh, review-link change, expiry, or provider error, and it is discarded on
rebuild. A validation path needing current provider state refreshes it or
reports the provider error rather than accepting an expired response.

## One Indexing Pipeline

Each record type owns one indexing function, for example
`index_issue`, `index_evidence`, and `index_review_room`. A full rebuild enumerates
record files and invokes those same functions; incremental repair invokes the
same function only for a changed or missing record-file source. Full and
incremental paths must therefore produce identical rows for the same project
state.

The indexer replaces all rows owned by its record type and ID in one SQLite
transaction, then updates `record_source_index`. A deleted source removes its
owned rows and source row in that transaction. Indexers never synthesize
durable facts absent from record files.

## Freshness, Repair, And Fallback

Before a cache-backed answer, a command discovers the complete set of supported
record-file sources and compares that set with `record_source_index` before
choosing candidates for targeted reindexing. This comparison detects added
sources with no row and indexed sources whose files were deleted; it must not
depend only on already-indexed candidates. Size and modified time are cheap
hints for selecting candidates. A candidate whose hints changed is parsed and
reindexed without requiring a content hash first. Implementations may retain or
compute a hash to suppress reparsing caused by noisy metadata, but hashing is
an optional optimization rather than a freshness precondition. Missing,
unknown, changed, deleted, corrupt, or schema-incompatible cache state is not a
user-managed workflow condition.

Commands repair the smallest safe unit lazily: reindex the affected record
type/record file, or remove rows for a deleted source. If targeted repair is
unavailable, cannot parse the record file, or cannot establish a consistent
result, the command falls back to the safe full rebuild when its contract
allows. Commands whose result controls orchestration, validation, or closeout
must not answer from known-stale cache rows; they rebuild, report a record-file
error, or fail with actionable repair guidance.

A bounded incremental repair owns one outer SQLite transaction for the complete
candidate set; the per-domain indexer transactions join that boundary. Any
later parse/index failure or full-rebuild decision rolls back every earlier
candidate row and `record_source_index` update before fallback starts. Full
rebuild writes a temporary cache and swaps it into place only after success, so
if incremental repair and full rebuild both fail, degraded orientation reopens
the exact prior last-good cache rather than a partially repaired hybrid.

## Versioning And Migration

The database has a cache schema version. A version mismatch, corrupt database,
or incompatible cache layout is handled by dropping and rebuilding the local
database from record files. It is not a durable data migration and carries no
legacy read compatibility requirement.

This experimental project permits hard removal in the implementation migration:

- remove projection terminology and APIs where they describe the cache as a
  second project-state model;
- remove canonical/SQLite dual-write or SQLite-first mutation paths;
- remove generic-record tables and generic payload/link/label cache paths;
- remove obsolete compatibility schema, migrations, and readers instead of
  preserving aliases or shims.

The only required migration safety property is that committed record files can
build the new cache. `atelier check --fix` and the rebuild path recreate ignored
local state; they do not rewrite record files merely to accommodate cache
schema change.
