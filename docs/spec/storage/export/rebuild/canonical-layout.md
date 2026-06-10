# Canonical Export And Rebuild Layout

This document defines the target `.atelier-state/` projection for Milestone 2.
It is the committed rebuild source for `.atelier/state.db`; the live SQLite
database remains local runtime state.

## Goals

- The export is deterministic for the same logical state.
- The export is sufficient to rebuild SQLite for all canonical records.
- Every record carries schema and version metadata.
- `export --check` can compare SQLite state with the committed projection.
- Git merges happen through files in `.atelier-state/`, not through SQLite.

## Directory Layout

```text
.atelier-state/
  issues/
    atelier-z1p8.md
  missions/
    atelier-k7mq.md
  milestones/
    atelier-4x9t.md
  plans/
    atelier-p3v6.md
  evidence/
    atelier-n8da.md
  mission-control.json
```

## Schema Identity

Every exported file has a `schema` string and `schema_version` integer.

- `schema` uses the form `atelier.<record-kind>`.
- `schema_version` starts at `1` for the first canonical projection.
- Unknown future `schema_version` values are rebuild errors unless a migration
  explicitly supports them.
- Missing `schema` or `schema_version` fields are rebuild errors.

## Deterministic Serialization

Export writers must serialize deterministically:

- JSON object keys are emitted in lexical order.
- JSON arrays are sorted by the rules in this document, never by insertion
  order.
- Markdown files use UTF-8, LF line endings, and exactly one trailing newline.
- Markdown record front matter is YAML bounded by `---` lines.
- YAML front matter keys are emitted in lexical order.
- Timestamps are UTC RFC 3339 strings with `Z`.
- IDs are case-sensitive and filename-safe ASCII.
- Empty optional arrays are emitted as `[]`; absent optional strings are
  emitted as `null`.
- Body text is normalized to LF line endings before export.

## ID And Filename Rules

The canonical file path is derived from the record ID:

| Record kind | ID format | Path |
| --- | --- | --- |
| Issue | `atelier-z1p8` | `.atelier-state/issues/atelier-z1p8.md` |
| Mission | `atelier-k7mq` | `.atelier-state/missions/atelier-k7mq.md` |
| Milestone | `atelier-4x9t` | `.atelier-state/milestones/atelier-4x9t.md` |
| Plan | `atelier-p3v6` | `.atelier-state/plans/atelier-p3v6.md` |
| Evidence | `atelier-n8da` | `.atelier-state/evidence/atelier-n8da.md` |

IDs use `<project-slug>-<random-base36>`. The project slug is lowercase ASCII
and normally matches the repository or workspace slug. The random suffix is
lowercase base36 and defaults to four characters; allocators must retry on local
collisions and may support longer suffixes through repository policy. IDs are
global across record kinds in one projection. Record kind is metadata, not part
of identity. Rebuild must preserve existing valid IDs and export must never
renumber records.

## Markdown Record Layout

Issues, missions, milestones, plans, and evidence are Markdown records with YAML
front matter followed by the human-readable body.

```markdown
---
id: atelier-z1p8
schema: atelier.issue
schema_version: 1
...
---

Body text starts here.
```

Common required front matter:

| Field | Type | Rule |
| --- | --- | --- |
| `schema` | string | Record schema. |
| `schema_version` | integer | Record schema version. |
| `id` | string | Must match the filename stem. |
| `title` | string | Human-readable record title. |
| `status` | string | State-machine value owned by the record kind. |
| `created_at` | string | UTC RFC 3339 timestamp. |
| `updated_at` | string | UTC RFC 3339 timestamp. |
| `labels` | array | Sorted lexically. |
| `links` | array | Sorted by `type`, `target_kind`, then `target_id`. |

The body is the canonical rich-text description. Rebuild stores it as the
record body without front matter.

`links` is the canonical typed graph metadata for semantic relationships owned
by a record. Each entry contains `type`, `target_kind`, and `target_id`.
Dependencies and issue hierarchy remain in the issue-specific `depends_on`,
`blocks`, and `parent` fields because they have dedicated command semantics.
Typed relation rows are rebuilt from `links`; `export --check` validates that
link targets exist, link objects are not duplicated, and relation types are
valid.

## Issues

Path: `.atelier-state/issues/<record-id>.md`

Issue front matter adds:

| Field | Type | Rule |
| --- | --- | --- |
| `priority` | string | Stable priority value, such as `P1`. |
| `issue_type` | string | `task`, `feature`, `story`, `bug`, `validation`, `closeout`, `spike`, or `decision`. |
| `parent` | string or null | Parent mission, milestone, or issue ID. |
| `blocks` | array | Issue IDs this issue blocks, sorted lexically. |
| `depends_on` | array | Issue IDs this issue depends on, sorted lexically. |
| `acceptance` | array | Acceptance criteria strings in user-defined order. |
| `evidence_required` | array | Evidence requirements in user-defined order. |

## Missions

Path: `.atelier-state/missions/<record-id>.md`

Current staged support writes mission records with common front matter fields
(`schema`, `schema_version`, `id`, `title`, `status`, `created_at`,
`updated_at`, `links`) plus a quoted JSON `data` field. The JSON object carries
`constraints`, `risks`, `validation`, `milestones`, `plans`, `evidence`, and
`work`. The body carries the mission summary or objective text.

The target expanded front matter remains `constraints`, `milestones`, `plans`,
`validation_expectations`, and `current_risks`. ID arrays are sorted lexically;
text arrays preserve author order.

## Milestones

Path: `.atelier-state/milestones/<record-id>.md`

Milestone front matter adds `desired_state`, `scope`,
`validation_criteria`, `accepted_evidence`, `completion_state`, `missions`, and
`contributing_work`. ID arrays are sorted lexically; text arrays preserve author
order. Milestones are checkpoint records, not work containers; epics and issues
link to them as contributing work.

## Plans

Path: `.atelier-state/plans/<record-id>.md`

Current staged support writes plan records with common front matter fields plus
a quoted JSON `data` field containing `revision` and `revisions`. The body is
the latest durable execution intent.

The target expanded front matter remains `owner`, `applies_to`, `revision`,
`supersedes`, and `drift_status`. `applies_to` and `supersedes` are sorted
lexically. Plan body is the durable execution intent.

## Evidence

Path: `.atelier-state/evidence/<record-id>.md`

Evidence front matter adds:

| Field | Type | Rule |
| --- | --- | --- |
| `evidence_type` | string | `test`, `log`, `screenshot`, `report`, `benchmark`, or `external`. |
| `captured_at` | string | UTC RFC 3339 timestamp. |
| `validates` | array | Linked record IDs, sorted lexically. |
| `command` | string or null | Command that produced the evidence. |
| `result` | string | `pass`, `fail`, `blocked`, or `informational`. |
| `artifact` | string or null | Repo path or external reference. |

Evidence body summarizes what was proven and any limits of the proof.

Current staged support writes evidence records with common front matter fields
plus a quoted JSON `data` field containing `kind`, `result`, `path`, `uri`,
`producer`, and `captured_at`. The body carries the evidence summary. The
expanded front matter above remains the target shape for a later Markdown-first
RecordStore slice.

## Mission Control Projection

Path: `.atelier-state/mission-control.json`

`mission-control.json` is derived, not a rebuild source for Milestone 2.
`export --check` compares it only after Mission Control projection export lands.
Rebuild must ignore it for canonical state reconstruction and regenerate it from
canonical records when Mission Control projection work lands in Milestone 6.

Until Milestone 6, the file may be absent. If present, it must carry
`schema: "atelier.mission-control"` and `schema_version: 1`.

The TUI consumer contract for this derived projection is defined in
[Mission Control TUI](../../../../architecture/mission-control-tui.md). That
contract does not make `mission-control.json` a rebuild source.

## Rebuild Order

Rebuild proceeds in this order:

1. Discover canonical Markdown records under `issues/`, `missions/`, `plans/`,
   and `evidence/`. Milestone checkpoint records join this list when their
   command slice lands.
2. Validate each record's schema, schema version, ID, path, front matter shape,
   and body encoding.
3. Validate that `parent`, `blocks`, `depends_on`, and `links` references point
   to discovered records and that duplicate IDs or duplicate links are rejected.
4. Recreate SQLite tables inside a transaction.
5. Regenerate derived projections such as `mission-control.json` when supported.

If any unexpected canonical file exists under `.atelier-state/`, `export
--check` must report a stale or untracked projection error. `manifest.json` and
`graph.json` are not canonical source files and canonical export removes stale
copies when it writes the projection.

## Mutating Command Rollout

Milestone 2 introduces `atelier export` as the deterministic writer for
`.atelier-state/` and `atelier export --check` as the durable-state freshness
check.
`atelier rebuild` recreates `.atelier/state.db` from `.atelier-state/` and may
create the local `.atelier/` runtime directory in a fresh checkout. Explicit
backup exports remain available with `atelier export --format json` and
`atelier export --format markdown`.

This is the transitional compatibility path for the SQLite-first inherited
implementation. The target architecture is Markdown-first: mutating commands
write canonical records through RecordStore and then refresh or mark stale the
ProjectionIndex. During migration, commands that still create, update, close,
reopen, delete, label, relate, block, comment on, or otherwise change canonical
records through SQLite must be followed by `atelier export` before committing
state. Automation work must preserve `export --check` semantics instead of
duplicating serialization in individual command handlers.

## Deferred Or Future Paths

All paths listed in `SPEC.md` are covered above. `mission-control.json` is
explicitly deferred as a derived projection until Milestone 6; its presence must
not be required to rebuild SQLite during Milestone 2.
