# Bundle JSON Contract

Bundles are one-shot authored graph deltas. They create records from a file;
tracked Markdown becomes the durable state after apply. Bundles are not backup
exports, desired-state manifests, or a second tracker.

```text
atelier bundle preview <file>
atelier bundle apply <file> --yes
```

Preview validates and prints a deterministic non-mutating plan. Apply requires
`--yes`, validates the same input and complete staged canonical graph, then
installs records. Failed validation leaves the live tree unchanged. The input
file is not silently consumed, moved, or deleted.

## Envelope and references

The top-level object has `schema: "atelier.bundle"`, `schema_version: 2`, a
non-empty `title`, and `resources`. Optional `description` and `metadata`
describe the input. Unknown fields and unsupported versions are rejected.
Version 2 is create-only: resource `operation` is omitted or `create`.
Version 1 is rejected because its issue body and evidence contract differ.

`resources.issues` and `resources.evidence` are supported. Missions are issues
with `issue_type: "mission"`; a separate `resources.missions` array is rejected.
Plans and milestones are ordinary prose, not bundle resources.

Each resource has a unique ASCII `client_ref`, matching
`^[A-Za-z][A-Za-z0-9._:-]*$`. References contain exactly one of:

```json
{ "client_ref": "slice.implementation" }
```

```json
{ "id": "atelier-z1p8" }
```

Client references resolve inside the same file; IDs resolve to existing records.
Client references never become canonical record IDs.

## Issues

| Field | Contract |
| --- | --- |
| `client_ref`, `title` | Required local reference and non-empty title. |
| `body` | Required typed Markdown body, using the same shape as CLI `--body`. |
| `issue_type` | Repository-supported type; default `task`. |
| `priority` | `critical`, `high`, `medium`, or `low`; default `medium`. |
| `status` | Optional initial status; defaults to the selected workflow's initial status. |
| `strategy` | Optional object with `path`; no revision number. |
| `labels` | Optional string array. |
| `parent` | Optional parent issue reference; any work item can be a parent, one level deep. |
| `blocked_by`, `blocks` | Optional reference arrays for sequencing: `blocked_by` names work this issue waits for, `blocks` names work that waits for it. These are the `atelier issue link --role` names. |
| `advances` | Optional mission membership reference array. On a mission it names work in its scope; on other issues it names the mission they advance. Each link must join exactly one mission. |
| `notes` | Optional ordered notes with `body`, optional `author`, and optional RFC 3339 `created_at`. |
| `operation` | Omitted or `create`. |

Work item bodies contain Goal and Done when, optionally Scope, Context,
Unknowns, and Design. Missions contain Outcome
and Success criteria or link a strategy path. Do not author the retired
Description/Outcome/Evidence issue schema. The old bundle issue fields
`description`, `outcome`, and `evidence` are rejected; use `body`.

Missions have no parent or children. Work items may be standalone, parents, or
children of a parent; a child cannot have children. Use `advances`
for mission scope, not `parent`. Initial execution states must satisfy the
complete staged workflow and dependency rules; draft planning does not bypass
record integrity.

## Evidence

| Field | Contract |
| --- | --- |
| `client_ref`, `title` | Required local reference and title. |
| `evidence_type` | Proof classification, such as `test`, `log`, `report`, or `benchmark`. |
| `result` | `pass`, `fail`, `blocked`, or `not-applicable`. |
| `commit` | Required full Git commit SHA for the checked state. |
| `body` | Method, claim, artifacts, and limits. |
| `validates` | References to work supported by the receipt. |
| `artifact` | Optional repository path or external URI. |
| `operation` | Omitted or `create`. |

A bundle-authored receipt follows the same honesty and freshness rules as
[CLI evidence](../../product/validation.md). A non-pass receipt is not passing
proof. Adding evidence does not silently close work.

## Validation and installation

Validation checks the envelope, supported resources, unique client references,
reference resolution, body shape, types, statuses, hierarchy, blockers, and the
complete staged graph. Errors name the rejected field, reference, record, or
`client_ref` where available. No invalid reference or cycle may be installed.

Parent references normalize to parent-to-child relationships. Both `blocked_by` and `blocks`
normalize to blocker-to-blocked relationships. `advances` is stored on both
records, as `atelier issue link` stores it, so a mission listing a parent and the
parent listing the mission are the same mission-to-work edge; declaring both is a
duplicate. `validates` retains its semantic role. Duplicate normalized edges are
errors.

Preview and apply report the same normalized plan in deterministic order; apply
replaces client references with allocated IDs. Preview reserves no IDs. Apply
holds the canonical mutation lock from snapshot through staged installation,
rejects live-tree drift, and backs up issue and evidence directories as one
rollback unit. Invalid paths, symlinks, and special entries are rejected.

Cleanup failure after installation reports the retained backup or stage path.
Do not retry a successful create-only bundle: inspect the created IDs, run
`atelier check`, and resolve the named retained directory. Reapplication is not
an idempotent update operation.

Fixtures:

- [Valid input](fixtures/valid-bundle.json)
- [Invalid input](fixtures/invalid-bundle.json)
- [Preview output](fixtures/dry-run-preview.txt): current human CLI output, not a
  JSON response contract.

The valid input is self-contained: it creates a mission and a parent feature, direct
task and validation children, a dependency, and an evidence attachment. Run preview
in a fresh Git repository initialized with `atelier init`; it needs no existing
record IDs. Apply additionally requires `--yes`.

The example evidence commit is an illustrative full-length hexadecimal ID.
Preview and apply validate its syntax; freshness gates cannot count a commit
that is unavailable in the repository as passing proof. For real proof, use the
full commit actually checked. New bundle-authored issues must first be applied
and committed before fresh evidence can validate their committed acceptance.

The invalid fixture deliberately uses the unsupported `resources.milestones`
field. It demonstrates input rejection rather than a valid planning shape.
