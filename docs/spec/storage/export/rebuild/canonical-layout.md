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
  manifest.json
  issues/
    ISS-0001.md
  missions/
    MIS-0001.md
  milestones/
    MS-0001.md
  plans/
    PLAN-0001.md
  evidence/
    EV-0001.md
  graph.json
  mission-control.json
```

## Schema Identity

Every exported file has a `schema` string and `schema_version` integer.

- `schema` uses the form `atelier.<record-kind>`.
- `schema_version` starts at `1` for the first canonical projection.
- `format_version` in `manifest.json` identifies the projection format as a
  whole and also starts at `1`.
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
| Issue | `ISS-0001` | `.atelier-state/issues/ISS-0001.md` |
| Mission | `MIS-0001` | `.atelier-state/missions/MIS-0001.md` |
| Milestone | `MS-0001` | `.atelier-state/milestones/MS-0001.md` |
| Plan | `PLAN-0001` | `.atelier-state/plans/PLAN-0001.md` |
| Evidence | `EV-0001` | `.atelier-state/evidence/EV-0001.md` |

Numeric suffixes are zero-padded to four digits for examples and initial
allocation. Rebuild must preserve any valid existing ID with the matching prefix
and a positive decimal suffix; export must not renumber records.

## Manifest

Path: `.atelier-state/manifest.json`

The manifest is the projection inventory. Rebuild reads it first, validates the
projection format, then validates that listed record paths exist and match their
content hashes.

Required fields:

| Field | Type | Rule |
| --- | --- | --- |
| `schema` | string | Must be `atelier.manifest`. |
| `schema_version` | integer | Manifest schema version. |
| `format_version` | integer | Whole-projection format version. |
| `generated_at` | string or null | UTC timestamp for normal export; `null` in deterministic fixtures. |
| `generator` | object | Tool name and version metadata. |
| `records` | array | Sorted inventory of canonical and derived files. |

Each `records` entry contains:

| Field | Type | Rule |
| --- | --- | --- |
| `path` | string | Repository-relative path below `.atelier-state/`. |
| `kind` | string | One of `issue`, `mission`, `milestone`, `plan`, `evidence`, `graph`, `mission-control`. |
| `id` | string or null | Record ID for per-record files; `null` for aggregate files. |
| `schema` | string | Expected schema string. |
| `schema_version` | integer | Expected schema version. |
| `role` | string | `canonical` or `derived`. |
| `sha256` | string | Hex SHA-256 of the file bytes excluding `manifest.json`. |

Manifest `records` are sorted by `path`.

## Markdown Record Layout

Issues, missions, milestones, plans, and evidence are Markdown records with YAML
front matter followed by the human-readable body.

```markdown
---
id: ISS-0001
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

## Issues

Path: `.atelier-state/issues/<ISS-ID>.md`

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

Path: `.atelier-state/missions/<MIS-ID>.md`

Mission front matter adds `constraints`, `milestones`, `plans`,
`validation_expectations`, and `current_risks`. ID arrays are sorted lexically;
text arrays preserve author order.

## Milestones

Path: `.atelier-state/milestones/<MS-ID>.md`

Milestone front matter adds `desired_state`, `scope`, `required_evidence`,
`completion_state`, and `missions`. ID arrays are sorted lexically; text arrays
preserve author order.

## Plans

Path: `.atelier-state/plans/<PLAN-ID>.md`

Plan front matter adds `owner`, `applies_to`, `revision`, `supersedes`, and
`drift_status`. `applies_to` and `supersedes` are sorted lexically. Plan body is
the durable execution intent.

## Evidence

Path: `.atelier-state/evidence/<EV-ID>.md`

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

## Graph

Path: `.atelier-state/graph.json`

`graph.json` is canonical. It stores typed relationships that do not belong to
one record body and lets rebuild reconstruct graph indexes before derived
projections are generated.

Required fields:

| Field | Type | Rule |
| --- | --- | --- |
| `schema` | string | Must be `atelier.graph`. |
| `schema_version` | integer | Graph schema version. |
| `nodes` | array | Record references sorted by `kind`, then `id`. |
| `edges` | array | Typed links sorted by `source_kind`, `source_id`, `type`, `target_kind`, `target_id`. |

Each edge contains `source_kind`, `source_id`, `type`, `target_kind`,
`target_id`, and `metadata`. `metadata` is a JSON object with lexical key order.

## Mission Control Projection

Path: `.atelier-state/mission-control.json`

`mission-control.json` is derived, not a rebuild source for Milestone 2. It is
included in the manifest with `role: "derived"` when present so
`export --check` can detect stale projection output. Rebuild must ignore it for
canonical state reconstruction and regenerate it from canonical records when
Mission Control projection work lands in Milestone 6.

Until Milestone 6, the file may be absent. If present, it must carry
`schema: "atelier.mission-control"` and `schema_version: 1`.

## Rebuild Order

Rebuild proceeds in this order:

1. Read and validate `manifest.json`.
2. Read canonical Markdown records from `issues/`, `missions/`, `milestones/`,
   `plans/`, and `evidence/` in manifest path order.
3. Read `graph.json` and validate that all graph nodes and edges reference
   records loaded in step 2.
4. Recreate SQLite tables inside a transaction.
5. Regenerate derived projections such as `mission-control.json` when supported.

If any canonical file exists under `.atelier-state/` but is absent from the
manifest, `export --check` must report a stale or untracked projection error.

## Mutating Command Rollout

Milestone 2 introduces `atelier export` as the deterministic writer for
`.atelier-state/` and `atelier export --check` as the freshness gate. Explicit
backup exports remain available with `atelier export --format json` and
`atelier export --format markdown`.

Until mutating commands are wired to export automatically, commands that create,
update, close, reopen, delete, label, relate, block, comment on, or otherwise
change records must be followed by `atelier export` before committing state.
Automation work that adds post-mutation export must reuse this canonical writer
and preserve `export --check` semantics instead of duplicating serialization in
individual command handlers.

## Deferred Or Future Paths

All paths listed in `SPEC.md` are covered above. `mission-control.json` is
explicitly deferred as a derived projection until Milestone 6; its presence must
not be required to rebuild SQLite during Milestone 2.
