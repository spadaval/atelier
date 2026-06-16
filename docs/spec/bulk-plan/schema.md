# Bulk Plan JSON Contract

This document defines the target JSON contract for authored bulk plans. Bulk
plans are agent-authored intent files that create or connect multiple Atelier
records in one operation. They are not backup exports and do not replace the
canonical `.atelier/` record tree described in
[Canonical Record And Rebuild Layout](../storage/export/rebuild/canonical-layout.md).

The contract covers `schema_version: 1`. Future versions may add fields, but
commands must reject unknown future versions unless an explicit migration or
compatibility mode supports them.

## Purpose

A bulk plan lets an agent describe a small graph before durable IDs exist. The
file uses `client_ref` values for intra-file references, validates the complete
graph first, supports dry-run previews without state mutation, and then applies
records and relationships through normal Atelier storage.

This feature is for authored work plans, not import, restore, or explicit-ID
migration. Durable IDs are allocated by Atelier during apply and are reported in
the apply summary.

## Document Shape

Required top-level fields:

| Field | Type | Rule |
| --- | --- | --- |
| `schema` | string | Must be `atelier.bulk-plan`. |
| `schema_version` | integer | Must be `1` for this contract. |
| `title` | string | Human-readable plan title. |
| `apply` | object | Apply options. |
| `records` | object | Record arrays keyed by record kind. |

Optional top-level fields:

| Field | Type | Rule |
| --- | --- | --- |
| `description` | string or null | Summary of the authored plan's intent. |
| `metadata` | object | Free-form JSON metadata preserved only in previews unless a later issue defines persistence. |

Top-level object keys should be serialized lexically in fixtures and generated
previews. Commands must not rely on object key order when parsing.

## Apply Options

`apply` controls validation and mutation behavior:

| Field | Type | Default | Rule |
| --- | --- | --- | --- |
| `dry_run` | boolean | `false` | When `true`, validate and emit a preview without mutating canonical records or SQLite. |
| `on_conflict` | string | `fail` | `fail`, `skip_existing`, or `update_existing`. Conflict behavior applies only to explicit durable IDs or aliases when future support exists. |
| `atomic` | boolean | `true` | When `true`, validation failure or apply failure must leave no partial graph where practical. If full transactionality is impossible, the command must emit recovery guidance. |
| `export` | string | `auto` | Compatibility option: `auto`, `skip`, or `check_only`. In the target design, apply writes canonical `.atelier/` records first and refreshes projection state; compatibility readers may still use this field to decide whether to check legacy export freshness. |
| `validate_only` | boolean | `false` | When `true`, validate and emit validation output without constructing an allocation preview. |

`dry_run` and `validate_only` never mutate state. If both are true, the command
uses `validate_only` semantics and may omit allocation previews.

## References

Every authored record that may be referenced inside the same file has a
`client_ref`.

Client refs:

- Are unique across all `records` arrays in the file.
- Use stable ASCII identifiers matching `^[A-Za-z][A-Za-z0-9._:-]*$`.
- Are local to one input file and are never stored as canonical record IDs.
- Must be included in validation errors for any record-local problem.

Reference values use one of these object shapes:

```json
{ "client_ref": "issue.bulk-contract" }
```

```json
{ "id": "atelier-z1p8" }
```

`client_ref` references must resolve to a record in the same file. `id`
references must resolve to an existing durable record unless the field
explicitly allows planned future references. A reference object must contain
exactly one of `client_ref` or `id`.

## Records

`records` contains arrays for first-class record kinds. Missing arrays are
treated as empty arrays.

Supported keys for version 1:

| Key | Record kind | Notes |
| --- | --- | --- |
| `issues` | Issue | Required implementation target for the first apply slice. |
| `missions` | Mission | Contract shape for future first-class record support. |
| `milestones` | Milestone | Checkpoint records, not hierarchy containers. |
| `plans` | Plan | Durable execution intent. |
| `evidence` | Evidence | Durable validation proof records. |

Implementations may reject non-issue record creation until the corresponding
first-class record work lands, but validation errors must still identify the
JSON path and `client_ref`.

### Issues

Issue records support hierarchy, sequencing dependencies, labels, priorities,
notes, and acceptance criteria.

Required fields:

| Field | Type | Rule |
| --- | --- | --- |
| `client_ref` | string | Unique local reference. |
| `title` | string | Non-empty human title. |
| `issue_type` | string | `epic`, `task`, `feature`, `bug`, `validation`, `spike`, or repository-configured value. Use `task` for work whose deliverable is an ADR, spec, context, or target-state update. |
| `priority` | string | Stable priority value accepted by `atelier issue`, such as `high`, `medium`, `low`, or `P1`. |

Optional fields:

| Field | Type | Rule |
| --- | --- | --- |
| `description` | string or null | Issue body. |
| `acceptance` | array of strings | User-defined order is preserved. |
| `evidence_required` | array of strings | User-defined order is preserved. |
| `labels` | array of strings | Stored sorted lexically. |
| `parent` | reference or null | Parent issue, epic, mission, or milestone where supported. |
| `depends_on` | array of references | Records that must complete before this issue is ready. |
| `blocks` | array of references | Records blocked by this issue. |
| `notes` | array of note objects | Durable handoff notes appended in order. |
| `status` | string | Optional initial state. Defaults to repository policy, normally `open`. |

`depends_on` and `blocks` describe sequencing dependencies. They must not be
used for semantic contribution, validation, duplicate, supersession, or planning
relationships; use the domain fields below for those relationships.

Notes use this shape:

| Field | Type | Rule |
| --- | --- | --- |
| `body` | string | Required note body. |
| `author` | string or null | Optional author identity. |
| `created_at` | string or null | Optional UTC RFC 3339 timestamp. If omitted, apply time is used. |

### Missions

Mission records use:

| Field | Type | Rule |
| --- | --- | --- |
| `client_ref` | string | Unique local reference. |
| `title` | string | Mission title. |
| `body` | string or null | Mission intent and constraints. |
| `labels` | array of strings | Stored sorted lexically. |
| `work` | array of references | Issues or epics included as mission work. |
| `plans` | array of references | Plan records linked to the mission. |
| `milestones` | array of references | Checkpoint records linked to the mission. |

### Milestones

Milestone records follow the first-class checkpoint model in
[Milestone Records](../../product/milestone-records.md):

| Field | Type | Rule |
| --- | --- | --- |
| `client_ref` | string | Unique local reference. |
| `title` | string | Milestone title. |
| `desired_state` | string | Observable checkpoint state. |
| `scope` | array of strings | Included, excluded, or deferred boundaries in author order. |
| `validation_criteria` | array of strings | Ordered acceptance criteria for the checkpoint. |
| `missions` | array of references | Missions this checkpoint advances. |
| `contributing_work` | array of references | Issues, epics, reviews, validations, or closeouts that contribute. |

Milestones are not parent work queues. Use hierarchy for issue ownership and
mission `work`, plan `applies_to`, evidence `validates`, or milestone
`contributing_work` for other relationships.

### Plans

Plan records describe durable execution intent:

| Field | Type | Rule |
| --- | --- | --- |
| `client_ref` | string | Unique local reference. |
| `title` | string | Plan title. |
| `body` | string | Durable plan text. |
| `owner` | string or null | Accountable owner. |
| `applies_to` | array of references | Missions, milestones, issues, or epics this plan applies to. |
| `supersedes` | array of references | Prior plans superseded by this plan. |

### Evidence

Evidence records describe proof:

| Field | Type | Rule |
| --- | --- | --- |
| `client_ref` | string | Unique local reference. |
| `title` | string | Evidence title. |
| `evidence_type` | string | `test`, `log`, `screenshot`, `report`, `benchmark`, or `external`. |
| `result` | string | `pass`, `fail`, `blocked`, or `informational`. |
| `body` | string | What was proven and the limits of proof. |
| `validates` | array of references | Records or criteria this evidence validates. |
| `artifact` | string or null | Repo path or external URI. |

## Relationships

Bulk plans do not accept a top-level generic `links` array. Relationship intent
is authored through domain fields: issue `blocks`/`depends_on`, mission `work`,
mission `plans`, mission `milestones`, milestone `contributing_work`, plan
`applies_to`, plan `supersedes`, and evidence `validates`. Apply normalizes
those fields into canonical `.atelier/` `relationships` buckets.

## Validation

Validation must complete before any mutation. A failed validation creates no
records, no dependency edges, no relationships, no notes, and no export changes.

Required validation checks:

- Top-level `schema` and `schema_version` are supported.
- Required top-level fields are present.
- Every record has a valid, unique `client_ref`.
- Every reference object contains exactly one of `client_ref` or `id`.
- Every `client_ref` reference resolves inside the file.
- Every `id` reference resolves in current tracker state when required.
- Issue hierarchy does not create a parent cycle.
- Issue dependencies do not create a forbidden cycle.
- `depends_on`, `blocks`, and relationship fields do not create duplicate edges after
  normalization.
- Labels, priorities, issue types, statuses, evidence types, and relationship roles are
  accepted by repository policy.
- Dry-run, validate-only, and export options are compatible.

Validation errors must use JSON paths rooted at `$` and include the nearest
record `client_ref` when one exists:

```json
{
  "client_ref": "issue.review",
  "code": "unresolved_client_ref",
  "message": "Reference does not resolve to a record in this file",
  "path": "$.records.issues[1].depends_on[0].client_ref"
}
```

Top-level errors that do not belong to a record use `"client_ref": null`.

## Dry-Run Output

Dry-run output must be deterministic and must not mutate canonical records or
SQLite. A dry run should return the same envelope style as other
agent-facing JSON commands:

```json
{
  "ok": true,
  "command": "bulk-plan.apply",
  "data": {
    "dry_run": true,
    "valid": true,
    "would_create": [],
    "would_update": [],
    "would_link": [],
    "would_note": [],
    "client_ref_map": {}
  },
  "warnings": []
}
```

Preview arrays are sorted deterministically by record kind, then `client_ref`,
then link tuple. Proposed durable IDs may be placeholder strings such as
`"<allocated:issue.bulk-contract>"`; they are preview-only and must not be
treated as reservations.

Validation failure under dry run exits non-zero and lists validation errors in
human-readable diagnostics.

## Apply Summary

The staged `atelier plan apply` implementation accepts this v1 input contract
and prints a compact human summary with `applied`, `dry_run`, `validate_only`,
record counts by kind, relationship count, and next commands. Mutating apply
persists durable records under tracked `.atelier/`, where `client_ref` to
durable `id` mappings can be audited. Dry-run output reports preview counts
without reserved IDs. Validate-only output reports that validation completed
without allocating a preview.

A successful mutating apply returns:

| Field | Type | Rule |
| --- | --- | --- |
| `dry_run` | boolean | `false`. |
| `created` | array | Created records with `kind`, `client_ref`, and durable `id`. |
| `updated` | array | Updated records when update behavior is supported. |
| `links` | array | Created relationship edges; retained in the summary for compatibility with existing apply output. |
| `notes` | array | Appended notes with target record references. |
| `client_ref_map` | object | Maps every authored `client_ref` to the durable record ID. |
| `export` | object | Export action, result, and changed paths when `apply.export` is not `skip`. |

If the command cannot guarantee full transactionality for a supported apply
mode, the summary must include enough mapping and recovery detail for a later
agent to repair or resume safely.

## Fixtures

Compact examples live in this directory:

- [valid-bulk-plan.json](fixtures/valid-bulk-plan.json): valid authored input
  containing issues, hierarchy, dependencies, relationships, labels, priorities,
  notes, plan and milestone references, and apply options.
- [invalid-bulk-plan.json](fixtures/invalid-bulk-plan.json): invalid authored
  input designed to produce path and `client_ref` diagnostics.
- [dry-run-preview.json](fixtures/dry-run-preview.json): deterministic dry-run
  output shape for the valid fixture.
