# Canonical Record And Rebuild Layout

This document defines the target `.atelier/` canonical record tree. Tracked
Markdown records under `.atelier/` are the committed rebuild source for the
local SQLite projection at `.atelier/runtime/state.db`; runtime and cache files remain
ignored local state.

## Goals

- Canonical rendering is deterministic for the same logical state.
- Canonical records are sufficient to rebuild SQLite for all canonical records.
- Every record carries schema and version metadata.
- `lint` validates `.atelier/` Markdown directly, `doctor` reports local
  projection/runtime health, and hidden/admin deterministic-renderer or rebuild
  diagnostics may verify migration internals.
- Git merges happen through tracked `.atelier/` record files, not through
  SQLite.

## Directory Layout

```text
.atelier/
  config.toml
  state.db
  issues/
    atelier-z1p8.md
    atelier-z1p8.activity/
      20260611T160930000000Z.md
  missions/
    atelier-k7mq.md
  evidence/
    atelier-n8da.md
  runtime/
    agent.json
    locks/
    diagnostics/
  cache/
```

Tracked canonical paths are `config.toml`, `issues/`, `missions/`,
`evidence/`, and canonical activity sidecars.
`.atelier/runtime/state.db`, `.atelier/runtime/`, and `.atelier/cache/` are ignored.
Copied rule trees, editor integration files, hook scaffolding, and UI caches
are not project tracker state unless a future contract explicitly promotes a
file into the tracked config surface.

## Project Config

`.atelier/config.toml` is tracked project state. It identifies the Atelier
project config schema, project slug, canonical state root, review mode, and
review provider settings. Local runtime directory, runtime SQLite path, cache
directory, and compatibility-state paths are implementation-owned local
defaults, not committed project policy.

Review provider integration is optional tracked project configuration. The
current implementation ships a Forgejo provider, so repositories that use
`atelier review` commands or review validators with Forgejo configure the
remote identity in `.atelier/config.toml`:

```toml
[review]
mode = "provider"
provider = "forgejo"

[review.providers.forgejo]
host = "forge.example.test"
owner = "workspace"
repo = "atelier"
# token lives in ~/.config/atelier.toml
```

The admin token value stays in the user-global config file:

```toml
schema = "atelier.user_config"
schema_version = 1

[review.providers.forgejo]
admin_token = "..."
```

The project config parser rejects missing Forgejo remote fields, empty values,
obsolete sudo-user mappings, and committed provider secrets with errors that
name the required key. Workflow action role attribution, including Forgejo
role-author mappings used by review artifact actions, belongs in
`.atelier/workflow.yaml`.

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
| Issue | `atelier-z1p8` | `.atelier/issues/atelier-z1p8.md` |
| Mission objective | `atelier-k7mq` | `.atelier/issues/atelier-k7mq.md` with `issue_type: "mission"` |
| Evidence | `atelier-n8da` | `.atelier/evidence/atelier-n8da.md` |

IDs use `<project-slug>-<random-base36>`. The project slug is lowercase ASCII
and normally matches the repository or workspace slug. The random suffix is
lowercase base36 and defaults to four characters; allocators must retry on local
collisions and may support longer suffixes through repository policy. IDs are
global across record kinds in one projection. Record kind is metadata, not part
of identity. Rebuild must preserve existing valid IDs and export must never
renumber records.

## Markdown Record Layout

Issues, missions, and evidence are Markdown records with YAML front matter
followed by the human-readable body.

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
| `relationships` | object | Sorted relationship buckets: `attachments`, `blocks`, `children`, and `relates`. |

The body is the canonical rich-text description. Rebuild stores it as the
record body without front matter.

`relationships` is the canonical record relationship model. `blocks` stores
operational blocker edges from the source record to blocked issue targets.
`children` stores record-kind child edges such as issue hierarchy. `attachments`
stores supporting evidence records with a `role`. `relates`
stores peer semantic relationships with a `type`; mission work and direct
mission blockers use this typed relationship surface. Rebuild derives issue
readiness, hierarchy, and runtime relation indexes from these buckets;
`depends_on` is a query/display concept derived as the inverse of `blocks`.
Proof roles such as `validates` are reserved for evidence attachments and are
not valid issue-to-issue `relates` types.

## Direct Edit Contract

Direct edits are a supported operator and agent workflow:

1. Edit tracked Markdown under `.atelier/` using the deterministic layout in
   this document.
2. Run `atelier lint` to validate schema, path, front matter, relationships,
   activity sidecars, and unsupported files.
3. Run the normal command that depends on the changed record, or run
   `atelier doctor --fix` when local projection/runtime repair is explicitly
   needed.

Every canonical Markdown file must use YAML front matter bounded by `---`, UTF-8
encoding, LF line endings, and exactly one trailing newline. Front matter keys
are rendered lexically by Atelier writers. Hand edits may use any YAML key order,
but `atelier lint` and repair commands may report non-canonical ordering as
format drift once the direct-edit lint slice lands.

Required common fields are `schema`, `schema_version`, `id`, `title`, `status`,
`created_at`, `updated_at`, `labels`, and `relationships`. Record-kind sections
below define additional required fields. Unknown required-field omissions are
lint/rebuild errors; unknown extra front matter keys are rejected unless the
record kind explicitly permits an extension map. Generic escaped payload keys
such as `data` or duplicate relationship arrays such as `targets`, `validates`,
`missions`, `contributing_work`, `depends_on`, or `blocked_by` are not part of
the target canonical layout.

Relationship buckets are the only canonical relationship surface:

```yaml
relationships:
  attachments: []
  blocks: []
  children: []
  relates: []
```

Bucket names are always present and sorted lexically. Entries are sorted by
target kind, target ID, role or type, then creation timestamp where applicable.
Operators should add dependency intent to `blocks`; `depends_on` remains a
derived display concept and must not be authored in canonical front matter.

Example issue:

```markdown
---
created_at: "2026-06-11T20:00:00Z"
id: atelier-z1p8
issue_type: task
labels:
  - docs
priority: P1
relationships:
  attachments: []
  blocks: []
  children: []
  relates: []
schema: atelier.issue
schema_version: 1
status: todo
title: "Define direct edit behavior"
updated_at: "2026-06-11T20:00:00Z"
---

## Description

Document the operator-visible behavior.

## Outcome

Operators can edit canonical issue Markdown directly and validate the result.

## Evidence

- `atelier lint atelier-z1p8` reports no findings.

## Notes

Migration context and handoff notes live here when needed.
```

Example issue activity sidecar:

```markdown
---
actor: "agent"
created_at: "2026-06-11T20:01:00Z"
event_type: note
id: "20260611T200100000000Z"
schema: atelier.activity
schema_version: 1
subject_id: atelier-z1p8
subject_kind: issue
summary: "Recorded hand-edit note"
---

The activity body contains the note text.
```

## Merge Conflict And Recovery Guidance

Canonical records are ordinary Git files, so operators resolve conflicts with
normal Git tools and then use Atelier commands to validate the result. The
standard recovery loop is:

1. Resolve file conflicts under tracked `.atelier/` record directories.
2. Run `atelier lint`.
3. Use focused drill-down commands such as `atelier issue show <id>`,
   `atelier issue status <objective-id>`, `atelier evidence show <id>`, or
   `atelier work queue --ready` to inspect the affected records.
4. Run `atelier doctor --fix` if ignored local projection/runtime state is
   stale or was rebuilt from invalid intermediate files.
5. Re-run `atelier lint` and the workflow validator for the issue, epic, or
   mission being closed.

For a single Markdown record conflict:

- Keep exactly one YAML front matter block.
- Preserve the record ID, schema, schema version, and path identity.
- Merge body text as normal prose, retaining useful intent from both sides.
- Preserve `created_at`; set `updated_at` to the later valid timestamp or to
  the timestamp of the resolving edit when the correct source timestamp is
  ambiguous.
- Preserve labels and other set-like arrays in deterministic sorted order.
- Preserve user-authored issue body sections in the order defined by the issue
  section contract. When two versions edit the same section, merge the prose
  that best preserves useful intent.

For relationship conflicts:

- Merge relationship buckets by target identity and role/type.
- Keep both edges when two branches add distinct blockers, children,
  attachments, or semantic relations.
- Remove exact duplicate edges.
- Do not author `depends_on`; express sequencing in `blocks` and let commands
  derive inverse display.
- After resolving dependency changes, inspect readiness with
  `atelier work queue --ready` and targeted issue `show` output.

For activity sidecar conflicts:

- Prefer keeping both files when two branches add different activity entries.
- If two files share the same timestamp ID but have different content, rename
  one file with the next deterministic suffix, such as
  `20260611T200100000000Z-01.md`, and update its `id` field to match.
- If the same sidecar file conflicts, preserve one front matter block, keep the
  original `subject_kind`, `subject_id`, and `created_at`, and merge the body
  only when both versions contain distinct useful text.
- Reject or repair any activity entry whose subject record no longer exists.

For unsupported files or stale runtime state:

- Delete or move files that do not belong in tracked canonical directories, such
  as generated caches, local databases, copied rule trees, or editor artifacts.
- Do not commit `.atelier/runtime/`, `.atelier/cache/`, SQLite databases, lock
  caches, diagnostics, or local identity.
- If `.atelier/runtime/state.db` is missing or stale, rebuild it from canonical
  records instead of resolving it as a Git conflict.

When `atelier lint` reports invalid canonical Markdown, fix the Markdown rather
than trusting the current SQLite projection. SQLite is rebuildable; the
canonical Markdown record tree is the durable review surface.

## Issues

Path: `.atelier/issues/<record-id>.md`

Issue front matter adds:

| Field | Type | Rule |
| --- | --- | --- |
| `priority` | string | Stable issue priority token. Version 1 uses `P0`, `P1`, `P2`, and `P3`; human labels such as `critical`, `high`, `medium`, and `low` are derived display text. |
| `issue_type` | string | `task`, `feature`, `epic`, `bug`, `validation`, or `spike`. Use `task` for work whose deliverable is an ADR, spec, context, or target-state update. |

Issue front matter does not carry large human-authored acceptance or proof
text. The canonical issue schema removes the legacy `acceptance` and
`evidence_required` arrays; canonical readers must reject those keys once the
section parser enforcement slice lands. Acceptance intent is authored in the
`Outcome` body section, and proof requirements are authored in the `Evidence`
body section.

Issue `status` is the workflow-owned durable token defined by
`.atelier/workflow.yaml`. In the current repository the allowed values are
`todo`, `in_progress`, `blocked`, `review`, `validation`, and `done`. Workflow
categories such as `active` or `done` are derived orientation metadata, not
alternate stored status fields; `review` and `validation` are active workflow
statuses, not separate categories.

Issue bodies are structured Markdown. The only recognized top-level issue body
headings are `## Description`, `## Outcome`, `## Evidence`, and `## Notes`.
Heading matching is exact and case-sensitive after trimming surrounding
whitespace from the heading text; headings such as `## Outcomes`,
`## evidence`, or `### Evidence` are not recognized section boundaries.
Subheadings below level two are allowed inside a section as ordinary body
content. Unknown level-two headings are rejected so durable issue records do not
hide workflow-significant authoring under unmodeled section names.

The required section policy is:

- `Description` is required for every issue and describes the current problem,
  context, or work request.
- `Outcome` is required for every issue and describes the desired finished
  world in observable terms.
- `Evidence` is optional by default. It describes proof artifacts, commands,
  file contents, rejected commands, screenshots, lint/doctor checks, explicit
  deterministic export diagnostics, or evidence records when the issue
  contract, issue type, or workflow policy explicitly requires authored proof
  prose.
- `Notes` is optional and carries handoff context, sequencing notes, caveats,
  or non-contract background.

Every present recognized section must contain non-whitespace content before the
next recognized section or end of file. Empty required sections are rebuild and
lint errors; an empty `Notes` section is also an error and should be omitted
instead. Duplicate recognized headings are rebuild and lint errors. Any content
before the first recognized heading is rejected, including comments, prose,
lists, or blank-looking non-whitespace content. Blank lines immediately after
front matter and before `## Description` are allowed.

## Mission Objectives

Path: `.atelier/issues/<record-id>.md`

Missions are issue records whose front matter declares `issue_type: "mission"`.
There is no `.atelier/missions/` directory and no `atelier.mission` schema in
the target storage contract. Mission objectives use the same issue schema,
section rules, activity sidecars, evidence links, and workflow policy as other
issues, with type-aware status and transition behavior supplied by workflow
configuration.

Mission relationship semantics are explicit:

| Mission concept | Canonical location |
| --- | --- |
| Linked execution work, including epics, tasks, reviews, validations, artifact updates, and closeouts | Mission issue `relationships.relates[]` entries with `kind: issue` and `type: advances`. |
| Direct mission blockers | Mission issue `relationships.relates[]` entries with `kind: issue` and `type: blocked_by`. Linked work item blockers remain ordinary issue dependency edges and are projected into objective status; do not duplicate them in mission prose. |
| Checkpoints | Prose or repository Markdown paths inside mission, epic, issue, or evidence bodies. No v1 milestone relationship table exists. |
| Plans | Prose or repository Markdown paths inside mission, epic, issue, or evidence bodies. No v1 plan relationship table exists. |
| Evidence | Evidence records under `.atelier/evidence/<id>.md`; the evidence record links to the mission issue with a `relationships.attachments[]` entry using `kind: issue`, the mission ID, and `role: validates`. Objective status derives incoming evidence links instead of storing evidence summaries in the mission body. |
| Supporting records that are not mission work, blockers, planning/checkpoint prose, or evidence | Mission issue `relationships.relates[]` entries with a precise built-in or configured custom context `type` such as `related`, `derived_from`, `supersedes`, or a project-specific context role; they are not counted as linked work by default. |
| Terminal notes | Workflow transition activity and optional issue notes; terminal proof remains evidence records plus workflow validation output. |

Mission work is not issue hierarchy. `relationships.blocks` keeps its common
meaning: the source record blocks the target. It is not the mission's
`blocked_by` list.
Configured custom issue link types are display/search context only. Rebuild and
lint preserve them when listed in `.atelier/config.toml`
`issue_links.custom_context_types`, and reject unconfigured custom types with
public recovery guidance.

Rejected escaped-JSON mission authoring:

```markdown
---
created_at: "2026-06-12T04:58:38Z"
id: "atelier-tcmr"
data: "{\"constraints\":[\"Use sectioned issue Markdown.\"],\"risks\":[\"Large rework can sprawl.\"],\"validation\":[\"Mission closeout requires attached evidence.\"],\"work\":[]}"
relationships:
  attachments: []
  blocks: []
  children: []
  relates: []
schema: "atelier.issue"
schema_version: 1
issue_type: "mission"
status: "ready"
title: "Repair CLI workflow rework and validation gaps"
updated_at: "2026-06-12T19:19:18Z"
---

Repair CLI workflow rework and validation gaps.
```

Readable replacement:

```markdown
---
created_at: "2026-06-12T04:58:38Z"
id: "atelier-tcmr"
labels:
  - "mission"
relationships:
  attachments:
  - kind: "evidence"
    id: "atelier-ev01"
    role: "validates"
  blocks: []
  children: []
  relates:
  - kind: "issue"
    id: "atelier-a4sn"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-gjaz"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
issue_type: "mission"
status: "ready"
title: "Repair CLI workflow rework and validation gaps"
updated_at: "2026-06-12T19:19:18Z"
---

## Description

Repair the CLI workflow and validation gaps so mission closeout can be audited
from canonical records and focused command output.

## Outcome

Mission closeout requires linked work closed, validation evidence attached,
workflow validators passing, and tracker lint checks passing.

Planning notes: see `docs/plans/cli-workflow-repair.md`.
Checkpoint criteria: closeout proof must show every linked work item closed and
mission evidence attached.
```

A validating evidence record carries this relationship fragment instead of
copying evidence into mission issue front matter or prose:

```yaml
relationships:
  attachments:
  - kind: "issue"
    id: "atelier-tcmr"
    role: "validates"
  blocks: []
  children: []
  relates: []
```

## Deferred Plan And Checkpoint Records

`.atelier/plans/` and `.atelier/milestones/` are not active v1 canonical record
directories. Plan intent and checkpoint criteria may be ordinary Markdown files
elsewhere in the repository or prose inside missions, epics, issues, and
evidence. They are referenced by path or text, not by a first-class plan or
milestone record ID.

If a later feature reintroduces first-class plan or checkpoint records, it must
add a new storage contract directly instead of relying on the legacy staged
fields described in older tracker records.

## Evidence

Path: `.atelier/evidence/<record-id>.md`

Evidence front matter adds:

| Field | Type | Rule |
| --- | --- | --- |
| `evidence_type` | string | Durable proof kind, such as `test`, `validation`, `review`, `audit`, `transcript`, `artifact`, or `migration`. |
| `captured_at` | string | UTC RFC 3339 timestamp. |
| `command` | string or null | Command that produced the evidence. |
| `proof_scope` | string or null | Claim, validator, criterion, audit row, or local outcome line being proven. |
| `agent_identity` | string or null | Producer or validator identity when known. |
| `independence_level` | string or null | Review distance such as `implementer`, `peer`, `independent`, `closeout`, or `adversarial`. |
| `follow_up_ids` | array | Related issue IDs, sorted lexically. |
| `residual_risks` | array | Remaining caveats in author order. |
| `artifact` | string or null | Repo path or external reference. |

Evidence `status` is the canonical proof result token. Target values are
`pass`, `fail`, `blocked`, `deferred`, `not_applicable`, or `informational`.
Validating targets belong in `relationships.attachments[]` entries with
`role: validates`; `validates` arrays or duplicate `targets` fields are not
canonical front matter.

Evidence body summarizes what was proven and any limits of the proof.

Current staged support writes evidence records with common front matter fields
plus a quoted JSON `data` field containing `kind`, `result`, `path`, `uri`,
`producer`, and `captured_at`. The body carries the evidence summary. The
expanded front matter above remains the target shape for a later Markdown-first
RecordStore slice, and the generic `data` payload is compatibility residue.

## Mission Control Projection

Path: `.atelier/cache/mission-control.json` or another documented cache path.

`mission-control.json` is derived, not a rebuild source. Target rebuild and lint
must ignore cache projections for canonical state reconstruction and regenerate
them from canonical records when Mission Control projection work lands in
Milestone 6.

Until Milestone 6, the file may be absent. If present, it must carry
`schema: "atelier.mission-control"` and `schema_version: 1`.

The TUI consumer contract for this derived projection is defined in
[Mission Control TUI](../../../../product/mission-control-tui.md). That
contract does not make `mission-control.json` a rebuild source.

## Rebuild Order

Rebuild proceeds in this order:

1. Discover canonical Markdown records under `issues/`, `missions/`, and
   `evidence/`.
2. Validate each record's schema, schema version, ID, path, front matter shape,
   and body encoding.
3. Validate that `relationships` references point to discovered records, that
   duplicate relationships are rejected, and that `blocks` and issue
   `children` do not create cycles.
4. Recreate SQLite tables inside a transaction.
5. Regenerate derived projections such as `mission-control.json` when supported.

If any unexpected canonical file exists under tracked `.atelier/` record
directories, lint/rebuild must report an untracked or unsupported canonical file
error. `manifest.json` and `graph.json` are not canonical source files and
canonical repair removes stale copies when it writes the projection.

The staged implementation uses a registered first-class record contract for
non-issue records. Each canonical kind declares its record kind, schema,
schema version, and directory once, and export/rebuild consume that contract for
path mapping and schema validation. `workflow_validator` is a registered future
record kind but remains non-canonical until a validator-result record slice
defines its durable layout.

## Mutating Command Rollout

Hidden/admin `atelier export` remains the deterministic check surface for
canonical records during migration or targeted maintenance, and normal durable
writes target `.atelier/` directly.

Hidden/admin `atelier rebuild` recreates `.atelier/runtime/state.db` from
tracked `.atelier/` canonical records and may create ignored runtime/cache
directories in a fresh checkout. Normal operators use `doctor --fix` for
explicit ignored-state repair. Backup export formats are no longer command
surfaces; predecessor imports use `atelier import-beads`.

Rebuild and automatic refresh use an advisory lock in `.atelier/runtime/` and
write to a unique temporary database before atomically replacing `state.db`.
Concurrent rebuilds and projection-backed reads therefore serialize refresh
instead of observing a partial database or colliding on a fixed temporary file.
If lock acquisition cannot complete because another Atelier command is
rebuilding, the command reports a retry/recovery message naming the lock file.

Rebuild also recreates local `ProjectionIndex` source metadata in SQLite. The
metadata records canonical file paths, size and modified-time hints, and content
hashes so query commands can detect stale projections before reading SQLite.
This metadata is intentionally not tracked and can be discarded with
`.atelier/runtime/state.db`. Issue activity sidecars are canonical files
but are read directly by history/show commands, so they are validated by rebuild
rather than tracked as query-projection sources.

This is the transitional compatibility path for the SQLite-first inherited
implementation. The target architecture is Markdown-first: mutating commands
write canonical records through RecordStore and then refresh or mark stale the
ProjectionIndex. During migration, commands that still create, update, close,
reopen, delete, label, relate, block, comment on, or otherwise change canonical
records through SQLite may require hidden/admin deterministic rendering before
committing state. Automation work must preserve deterministic-check semantics
for retained migration diagnostics instead of duplicating serialization in
individual command handlers.

## Deferred Or Future Paths

All paths listed in `PRODUCT_INTENT.md` are covered above. `mission-control.json` is
explicitly deferred as a derived projection until Milestone 6; its presence must
not be required to rebuild SQLite during Milestone 2.
