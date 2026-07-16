# Bundle JSON Contract

This document defines the target JSON contract for one-shot bundle files.
Bundles are agent-authored graph deltas that create multiple Atelier records in
one reviewed operation. They are input files, not durable source of truth,
backup exports, or long-lived desired-state manifests. After a successful apply,
record files under `.atelier/` are the durable state described in
[Markdown-First Record Store](../../architecture/markdown-first-record-store.md).

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
`apply` validates the same contract, requires `--yes`, writes record files,
invalidates the affected domain-cache facts for later lazy repair, and prints
the created IDs and relationships. Mutating apply must fail before any write
when validation fails.

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
| `status` | string | Optional initial state. Defaults to repository policy, normally `todo`. Apply accepts statuses only when the complete staged canonical graph satisfies workflow policy. |

Bundles do not carry typed workflow-transition or mission-plan-review receipts.
Use initial and other non-executable statuses (normally `todo` for work items
and `draft` for missions) when creating records. A mission in `ready` or
`in_progress` is rejected unless the staged canonical state already contains
the required current authorization, which create-only v1 bundles cannot add.
After apply, use `atelier issue plan-review` and `atelier issue transition` to
enter executable mission states. Apply also rejects graph changes that would
make an existing executable mission's approval stale or introduce an
unsatisfied direct or transitive blocker.

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
records, no dependency edges, no relationships, no notes, and no cache changes.

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
canonical records or SQLite. Its human-readable summary reports record counts
by kind, note count, and the complete normalized relationship plan. Preview
relationships use `client_ref` for records authored in the bundle and canonical
IDs for existing records. Records are sorted by kind and `client_ref`; graph
edges are sorted by their normalized source, target, and role tuple.

Parent references normalize to `parent -> child (parent)`. Both `depends_on`
and `blocks` normalize to `blocker -> blocked (blocks)`. `advances` and
`validates` preserve their authored direction and role. A duplicate normalized
edge is a validation error rather than being silently counted or applied twice.

Validation failure under preview exits non-zero and lists validation errors in
human-readable diagnostics.

## Apply Summary And Failure Behavior

`atelier bundle apply <file> --yes` validates this v1 input contract and prints
a compact human summary with created counts by kind, note count, the complete
relationship list, and next commands. Apply executes the same normalized graph
plan shown by preview. Its relationship list has the same ordering, count, and
roles as preview, with authored `client_ref` values replaced by their allocated
durable IDs. Mutating apply persists durable records under tracked `.atelier/`,
where those mappings can be audited.

Apply builds and validates a complete staged canonical tree before installing
its record directories. The validation is the same canonical rebuild contract
used by tracker health checks, including workflow execution state, review
freshness, dependency closure, and record references. If staging or validation
fails, the live canonical tree and SQLite cache remain unchanged. Apply holds
an exclusive canonical mutation transaction from snapshot through installation,
so ordinary Atelier writers resume only after the new tree is live and apply on
top of it. A final full-tree fingerprint rejects noncooperative filesystem drift
instead of overwriting it. Installation backs up both `issues/` and `evidence/`
as one rollback unit; a failure installing either restores both. Symbolic links,
special filesystem entries, and non-directory install targets are rejected.
Once both live directories are installed, backup or staging cleanup is
post-commit housekeeping: cleanup failure emits a warning naming the retained
path but does not turn the committed apply into a failure. Backups are retained
under git-ignored runtime state. Operators must not retry that create-only
bundle. A retained backup blocks later bundle applies until `atelier check`
confirms the live tree and the named backup is removed, preventing an unsafe
retry from creating duplicates.

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
