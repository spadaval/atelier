# Markdown Record Store

Tracked Markdown under `.atelier/` is Atelier's durable source of truth.
Commands parse record files into an in-memory `RecordIndex` on every invocation.
There is no SQLite database, domain cache, daemon, or persistent query index.

## Layout

```text
.atelier/
  config.toml
  workflow.yaml
  issues/<id>.md
  evidence/<id>.md
  reviews/<id>.md
  runtime/                 # ignored local mutation lock
```

Issue and evidence Markdown use `schema_version: 2`. Older canonical versions
are rejected rather than silently converted. Workflow policy has version 3 and
project configuration version 1; these are distinct contracts. Review events
use `atelier.review` version 1. Only issue, evidence and review are record or
relationship target kinds. `workflow_validator` names mechanical workflow gate
nodes; it is not a persistent record or link target kind.

Issue kinds include missions and work items; any work item can parent others. IDs are project-scoped
random IDs such as `atelier-z1p8`, independent of record type: the configured
`project_slug`, a hyphen, and a random base36 suffix. Any valid slug prefix is
accepted when reading, so records minted under another slug stay valid. Priority is stored as `P0`–`P3`; the
[work model](../product/work-model.md#records-and-relationships) maps them to
the words commands and views use. YAML front matter
holds typed identity, lifecycle state, timestamps, relationships, strategy path,
and owner branch metadata. The Markdown body is human-readable intent and
proof. Generic extension maps and historical issue section schemas are not the
current record model.

`workflow_branch` contains `work_branch` and `branch_base` on the branch owner.
`strategy` links a path without a revision number. Relationship buckets are
`blocks`, `children`, `attachments`, and `relates`; indexes derive parent and
reverse-link views from those stored edges. An issue link is stored once, on
the record that declares it: a blocked issue stores `blocked_by` in its
`relates`, a blocker that declares the edge stores it in `blocks`, and an
`advances` or context link sits on whichever endpoint declared it. Records
from before atelier-ihss may hold a link on both records; the index reads it
as one edge.

Work item bodies require Goal and Done when; Scope, Context, Unknowns,
Design, and Background are optional. Missions carry Outcome and Success
criteria or link strategy, with optional Background. Shared Notes and Close
Reason preserve handoffs and terminal reasoning. See
[authoring](../product/mission-authoring.md) for the process-owned templates.

Evidence is an immutable Markdown receipt with explicit result
(`pass`, `fail`, `blocked`, `not-applicable`) and a full commit identity. Reviews
are append-only Markdown receipts. Review identity, approval, findings, and
dispositions are durable facts rather than provider state. Their exact fields
are owned by the typed record parsers and
[review contract](../product/reviews-and-merge.md).

## Reads and writes

`atelier-records` parses, renders, validates, and persists canonical records.
`atelier-app` builds direct indexes and graph queries. `atelier check` refuses
an invalid structural graph at the first load error; repair that error and
rerun to uncover the next one. It does not promise aggregate structural errors.
Read commands do not
repair files or consult a hidden secondary truth.

Mutating commands validate before writing. The canonical mutation lock
coordinates cooperative writers. A failed validation must leave the canonical
tree unchanged. Bundle apply stages and validates the complete candidate graph
before installing its issue and evidence directories; see the
[bundle input contract](../spec/bundle/schema.md).

Transitions write records and run read-only checks. `start` explicitly prepares
a branch. `merge` explicitly checks gates and fast-forwards; it writes no
tracker records and creates no commits. Agents commit record mutations with
the implementation. Active work must resolve to a recorded owner branch.

## Git and policy

A work branch carries its own graph and done claim. The base branch obtains
those records with the same fast-forward that integrates code. Integration
facts and author identities come from Git.

Merge policy is read from the base branch's `.atelier/workflow.yaml`, so a work
branch cannot weaken its own gate. Invalid base policy produces a visible error
and uses the last valid policy from base history. Freshness compares commits and
semantic record changes. Evidence freshness guards code, policy, and target
acceptance; approval freshness also guards relationships, metadata, and existing
proof/review rewrites. See the product contracts for each boundary.

Terminal records may be removed by explicit prune after protection checks.
Git history is the audit and recovery source. There is no tracked archive index
or per-transition activity sidecar. See
[retention](../product/retention-and-prune.md).
