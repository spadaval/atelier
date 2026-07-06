# Bundle JSON Contract

This document defines the target JSON contract for one-shot bundle files.
Bundles are agent-authored graph deltas that create multiple Atelier records in
one reviewed operation. They are input files, not durable source of truth,
backup exports, or long-lived desired-state manifests. After a successful apply,
canonical Markdown under `.atelier/` is the durable state described in
[Canonical Record And Rebuild Layout](../storage/export/rebuild/canonical-layout.md).

The contract covers `schema_version: 1`. Future versions may add operations or
record kinds, but commands must reject unknown future versions unless an
explicit migration or compatibility mode supports them.

## Command Surface

Bundles are read from real filesystem paths. The command does not read bundle
JSON from stdin.

The public v1 command shape is:

```text
atelier bundle preview <file>
atelier bundle apply <file> --yes
```

`preview` validates the input and prints a deterministic non-mutating summary.
`apply` validates the same contract, requires `--yes`, writes canonical records,
refreshes rebuildable projection state, and prints the created IDs and
relationships. Mutating apply must fail before any write when validation fails.

The input file is intentionally temporary. The file can be archived or consumed
only through an explicit option after a successful apply; failed preview or
apply never deletes, moves, or rewrites the input.

## Document Shape

Required top-level fields:

| Field | Type | Rule |
| --- | --- | --- |
| `schema` | string | Must be `atelier.bundle`. |
| `schema_version` | integer | Must be `1` for this contract. |
| `title` | string | Human-readable bundle title. |
| `resources` | object | Resource arrays keyed by resource kind. |

Optional top-level fields:

| Field | Type | Rule |
| --- | --- | --- |
| `description` | string or null | Summary of the bundle's intent. |
| `metadata` | object | Free-form JSON metadata used only for preview/apply diagnostics unless a later issue defines persistence. |

Top-level object keys should be serialized lexically in fixtures and generated
previews. Commands must not rely on object key order when parsing.

## Operations

Version 1 is create-only. A resource may omit `operation`, in which case the
operation is `create`, or it may set `"operation": "create"` explicitly.
Commands must reject `update`, `delete`, `replace`, `upsert`, and any other
operation until a later schema version accepts the operation and defines its
conflict behavior.

Existing records may be referenced with durable IDs, but v1 bundle resources do
not mutate existing records.

## References

Every authored resource that may be referenced inside the same file has a
`client_ref`.

Client refs:

- Are unique across all `resources` arrays in the file.
- Use stable ASCII identifiers matching `^[A-Za-z][A-Za-z0-9._:-]*$`.
- Are local to one input file and are never stored as canonical record IDs.
- Must be included in validation errors for any resource-local problem.

Reference values use one of these object shapes:

```json
{ "client_ref": "issue.bundle-contract" }
```

```json
{ "id": "atelier-z1p8" }
```

`client_ref` references must resolve to a resource in the same file. `id`
references must resolve to an existing durable record unless the field
explicitly allows planned future references. A reference object must contain
exactly one of `client_ref` or `id`.

## Resources

`resources` contains arrays for first-class resource kinds. Missing arrays are
treated as empty arrays. Version 1 supports these resource keys:

| Key | Record kind | Notes |
| --- | --- | --- |
| `issues` | Issue | Required implementation target for v1 apply. |
| `missions` | Deferred alias | Rejected in the fixed domain model; author mission-shaped objectives as `issues` with `issue_type: "mission"`. |
| `evidence` | Evidence | May be implemented after issue creation, using the same create-only operation contract. |

First-class plans and milestones are not legal v1 bundle resources. Plans are
ordinary Markdown artifacts referenced from accountable issues, missions, or
evidence. Milestone/checkpoint semantics are deferred and are not a bundle v1
validation-data destination.

Implementations may reject non-issue resource creation until the corresponding
first-class record work lands, but validation errors must still identify the
JSON path and `client_ref`.

### Issues

Issue resources support hierarchy, sequencing dependencies, labels, priorities,
notes, and completion sections.

Required fields:

| Field | Type | Rule |
| --- | --- | --- |
| `client_ref` | string | Unique local reference. |
| `title` | string | Non-empty human title. |
| `issue_type` | string | `epic`, `task`, `feature`, `bug`, `validation`, `spike`, or repository-configured value. Use `task` for work whose deliverable is an ADR, spec, context, or target-state update. |
| `priority` | string | Stable priority value accepted by `atelier issue`, such as `high`, `medium`, `low`, or repository-configured values. |

Optional fields:

| Field | Type | Rule |
| --- | --- | --- |
| `operation` | string | Omitted or `create`; any other value is rejected in v1. |
| `description` | string or null | Description section body. |
| `outcome` | array of strings or string | Outcome section content. |
| `evidence` | array of strings or string | Evidence section content. |
| `labels` | array of strings | Stored sorted lexically. |
| `parent` | reference or null | Parent issue or epic. |
| `depends_on` | array of references | Records that must complete before this issue is ready. |
| `blocks` | array of references | Records blocked by this issue. |
| `notes` | array of note objects | Durable handoff notes appended in order after the issue is created. |
| `status` | string | Optional initial state. Defaults to repository policy, normally `todo`. Non-initial statuses may be rejected by workflow policy. |

`depends_on` and `blocks` describe sequencing dependencies. They must not be
used for semantic contribution, validation, duplicate, supersession, or planning
relationships.

Notes use this shape:

| Field | Type | Rule |
| --- | --- | --- |
| `body` | string | Required note body. |
| `author` | string or null | Optional author identity. |
| `created_at` | string or null | Optional UTC RFC 3339 timestamp. If omitted, apply time is used. |

### Missions

Missions are issue resources whose `issue_type` is `mission`. Bundles must not
use a separate mission resource shape or express mission execution work through
issue `parent`. Mission scope is authored through typed relationship fields
that normalize to issue `relates` entries with `type: "advances"`.

The fixed mission shape is:

| Field | Type | Rule |
| --- | --- | --- |
| `client_ref` | string | Unique local reference. |
| `title` | string | Mission title. |
| `issue_type` | string | Must be `mission`. |
| `description` | string or null | Mission intent/body text. |
| `outcome` | array of strings or string | Mission outcome section. |
| `labels` | array of strings | Stored sorted lexically. |
| `blocks` | array of references | Direct mission blockers, normalized as direct blocker relationships. |
| `advances` | array of references | Issues or epics included as mission execution work. |
| `parent` | null | Missions cannot have parents. Any non-null parent is rejected. |

### Evidence

Evidence resources describe proof:

| Field | Type | Rule |
| --- | --- | --- |
| `operation` | string | Omitted or `create`; any other value is rejected in v1. |
| `client_ref` | string | Unique local reference. |
| `title` | string | Evidence title. |
| `evidence_type` | string | `test`, `log`, `screenshot`, `report`, `benchmark`, or repository-configured value. |
| `result` | string | `pass`, `fail`, `blocked`, or `informational`. |
| `body` | string | What was proven and the limits of proof. |
| `validates` | array of references | Records or criteria this evidence validates. |
| `artifact` | string or null | Repo path or external URI. |

## Relationships

Bundles do not accept a top-level generic `links` array. Relationship intent is
authored through domain fields: issue `blocks`/`depends_on`/`advances` and
evidence `validates`. Apply normalizes those fields into canonical `.atelier/`
relationship buckets.

## Preview And Validation

Validation must complete before any mutation. A failed validation creates no
records, no dependency edges, no relationships, no notes, and no projection
changes.

Required validation checks:

- Top-level `schema` and `schema_version` are supported.
- The command input is a readable file path.
- Required top-level fields are present.
- Every resource has a supported or explicitly deferred resource kind.
- Every resource operation is omitted or `create`.
- Every resource has a valid, unique `client_ref`.
- Every reference object contains exactly one of `client_ref` or `id`.
- Every `client_ref` reference resolves inside the file.
- Every `id` reference resolves in current tracker state when required.
- Issue hierarchy does not create a parent cycle.
- Mission resources are authored as issue records with `issue_type:
  "mission"`, no parent, no children, and execution scope through `advances`
  links rather than `parent`.
- Epics have no parent; ordinary issue types may be standalone or direct
  children of epics; ordinary issue types cannot own children.
- Issue dependencies do not create a forbidden cycle.
- `depends_on`, `blocks`, and relationship fields do not create duplicate edges after normalization.
- Labels, priorities, issue types, statuses, evidence types, and relationship roles are accepted by repository policy.
- Plans and milestones are rejected as v1 resources with explicit JSON paths.

Validation errors must use JSON paths rooted at `$` and include the nearest
resource `client_ref` when one exists:

```json
{
  "client_ref": "issue.review",
  "code": "unresolved_client_ref",
  "message": "Reference does not resolve to a resource in this file",
  "path": "$.resources.issues[1].depends_on[0].client_ref"
}
```

Top-level errors that do not belong to a resource use `"client_ref": null`.

`atelier bundle preview <file>` must be deterministic and must not mutate
canonical records or SQLite. Preview output returns the same envelope style as
other agent-facing JSON commands:

```json
{
  "ok": true,
  "command": "bundle.preview",
  "data": {
    "valid": true,
    "would_create": [],
    "would_link": [],
    "would_note": [],
    "client_ref_map": {}
  },
  "warnings": []
}
```

Preview arrays are sorted deterministically by record kind, then `client_ref`,
then relationship tuple. Proposed durable IDs may be placeholder strings such
as `"<allocated:issue.bundle-contract>"`; they are preview-only and must not be
treated as reservations.

Validation failure under preview exits non-zero and lists validation errors in
human-readable diagnostics.

## Apply Summary And Failure Behavior

`atelier bundle apply <file> --yes` validates this v1 input contract and prints
a compact human summary with created counts by kind, relationship count, note
count, and next commands. Mutating apply persists durable records under tracked
`.atelier/`, where `client_ref` to durable `id` mappings can be audited.

A successful mutating apply returns:

| Field | Type | Rule |
| --- | --- | --- |
| `created` | array | Created records with `kind`, `client_ref`, and durable `id`. |
| `links` | array | Created relationship edges. |
| `notes` | array | Appended notes with target record references. |
| `client_ref_map` | object | Maps every authored `client_ref` to the durable record ID. |

Apply is atomic at the canonical-record level. If validation fails, nothing is
written. If an unexpected write failure occurs after mutation starts, the
command must stop, report the first failed operation, list any created durable
IDs, and print recovery guidance. A later issue may strengthen this into a
transactional temporary-directory swap, but v1 must never silently leave partial
state without a recovery summary.

## Idempotency And Conflicts

Bundle files are one-shot deltas. Reapplying the same successful create-only
bundle is expected to fail on duplicate titles, duplicate aliases, or other
repository policy conflicts unless a future schema adds idempotency keys.

Preview may be run repeatedly. Apply must not reserve IDs during preview. An
apply summary should include enough created-ID mapping for a later operator to
inspect whether a repeated apply is a duplicate rather than new work.

## Fixtures

Compact examples live in this directory:

- [valid-bundle.json](fixtures/valid-bundle.json): valid authored input
  containing issues, hierarchy, dependencies, labels, priorities, notes, mission
  links, evidence links, and create-only operations.
- [invalid-bundle.json](fixtures/invalid-bundle.json): invalid authored input
  designed to produce path and `client_ref` diagnostics, including rejected v1
  plan and milestone resources.
- [dry-run-preview.json](fixtures/dry-run-preview.json): deterministic preview
  output shape for the valid fixture.
