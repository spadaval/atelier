# Canonical Record And Rebuild Layout

This document defines the target `.atelier/` canonical record tree. Tracked
Markdown records under `.atelier/` are the committed rebuild source for the
local SQLite projection at `.atelier/state.db`; runtime and cache files remain
ignored local state.

## Goals

- The export is deterministic for the same logical state.
- The export is sufficient to rebuild SQLite for all canonical records.
- Every record carries schema and version metadata.
- `export --check`, `lint`, and `rebuild` validate `.atelier/` Markdown
  directly.
- Git merges happen through tracked `.atelier/` record files, not through
  SQLite.

## Directory Layout

```text
.atelier/
  config.toml
  issues/
    atelier-z1p8.md
    atelier-z1p8.activity/
      20260611T160930000000Z.md
  missions/
    atelier-k7mq.md
  milestones/
    atelier-4x9t.md
  plans/
    atelier-p3v6.md
  evidence/
    atelier-n8da.md
  runtime/
    state.db
    agent.json
    locks/
    diagnostics/
  cache/
```

Tracked canonical paths are `config.toml`, `issues/`, `missions/`,
`milestones/`, `plans/`, `evidence/`, and canonical activity sidecars.
`.atelier/runtime/` and `.atelier/cache/` are ignored. Copied rule trees,
editor integration files, hook scaffolding, and UI caches are not project
tracker state unless a future contract explicitly promotes a file into the
tracked config surface.

## Project Config

`.atelier/config.toml` is tracked project state. It identifies the Atelier
project config schema, project slug, canonical state root, local runtime
directory, runtime SQLite path, and cache directory. Runtime and cache paths
named by the config remain local-only and must stay ignored; the config records
where they live, not their contents.

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
| Mission | `atelier-k7mq` | `.atelier/missions/atelier-k7mq.md` |
| Milestone | `atelier-4x9t` | `.atelier/milestones/atelier-4x9t.md` |
| Plan | `atelier-p3v6` | `.atelier/plans/atelier-p3v6.md` |
| Evidence | `atelier-n8da` | `.atelier/evidence/atelier-n8da.md` |

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
| `relationships` | object | Sorted relationship buckets: `blocks`, `children`, `attachments`, and `relates`. |

The body is the canonical rich-text description. Rebuild stores it as the
record body without front matter.

`relationships` is the canonical record relationship model. `blocks` stores
operational blocker edges from the source record to blocked issue targets.
`children` stores hierarchy, scope, and mission-work edges. `attachments` stores
supporting plan/evidence/milestone records with a `role`. `relates` stores peer
semantic relationships with a `type`. Rebuild derives issue readiness,
hierarchy, and runtime relation indexes from these buckets; `depends_on` is a
query/display concept derived as the inverse of `blocks`.

## Direct Edit Contract

Direct edits are a supported operator and agent workflow:

1. Edit tracked Markdown under `.atelier/` using the deterministic layout in
   this document.
2. Run `atelier lint` to validate schema, path, front matter, relationships,
   activity sidecars, and unsupported files.
3. Run the normal command that depends on the changed record, or run
   `atelier rebuild` when the local projection needs explicit refresh.

Every canonical Markdown file must use YAML front matter bounded by `---`, UTF-8
encoding, LF line endings, and exactly one trailing newline. Front matter keys
are rendered lexically by Atelier writers. Hand edits may use any YAML key order,
but `atelier lint` and repair commands may report non-canonical ordering as
format drift once the direct-edit lint slice lands.

Required common fields are `schema`, `schema_version`, `id`, `title`, `status`,
`created_at`, `updated_at`, `labels`, and `relationships`. Record-kind sections
below define additional required fields. Unknown required-field omissions are
lint/rebuild errors; unknown extra front matter keys are rejected unless the
record kind explicitly permits an extension map.

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
priority: high
relationships:
  attachments: []
  blocks: []
  children: []
  relates: []
schema: atelier.issue
schema_version: 1
status: open
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
   `atelier mission show <id>`, `atelier evidence show <id>`, or
   `atelier issue list --ready` to inspect the affected records.
4. Run `atelier rebuild` if the local projection is stale or was rebuilt from
   invalid intermediate files.
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
  `atelier issue list --ready` and targeted issue `show` output.

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
- If `.atelier/state.db` is missing or stale, rebuild it from canonical
  records instead of resolving it as a Git conflict.

When `atelier lint` reports invalid canonical Markdown, fix the Markdown rather
than trusting the current SQLite projection. SQLite is rebuildable; the
canonical Markdown record tree is the durable review surface.

## Issues

Path: `.atelier/issues/<record-id>.md`

Issue front matter adds:

| Field | Type | Rule |
| --- | --- | --- |
| `priority` | string | Stable priority value, such as `P1`. |
| `issue_type` | string | `task`, `feature`, `story`, `bug`, `validation`, `closeout`, or `spike`. Use `task` for work whose deliverable is an ADR, spec, context, or target-state update. |

Issue front matter does not carry large human-authored acceptance or proof
text. The canonical issue schema removes the legacy `acceptance` and
`evidence_required` arrays; canonical readers must reject those keys once the
section parser enforcement slice lands. Acceptance intent is authored in the
`Outcome` body section, and proof requirements are authored in the `Evidence`
body section.

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
- `Evidence` is required for every issue and describes the proof artifacts,
  commands, file contents, rejected commands, screenshots, lint/export checks,
  or evidence records needed to show the outcome was met. When no proof artifact
  is meaningful, the section must explicitly say why it is not applicable.
- `Notes` is optional and carries handoff context, sequencing notes, caveats,
  or non-contract background.

Every present recognized section must contain non-whitespace content before the
next recognized section or end of file. Empty required sections are rebuild and
lint errors; an empty `Notes` section is also an error and should be omitted
instead. Duplicate recognized headings are rebuild and lint errors. Any content
before the first recognized heading is rejected, including comments, prose,
lists, or blank-looking non-whitespace content. Blank lines immediately after
front matter and before `## Description` are allowed.

## Missions

Path: `.atelier/missions/<record-id>.md`

Current staged support writes mission records with common front matter fields
(`schema`, `schema_version`, `id`, `title`, `status`, `created_at`,
`updated_at`, `relationships`) plus a quoted JSON `data` field. The JSON object carries
`constraints`, `risks`, `validation`, `milestones`, `plans`, `evidence`, and
`work`. The body carries the mission summary or objective text.

The target expanded front matter remains `constraints`, `milestones`, `plans`,
`validation_expectations`, and `current_risks`. ID arrays are sorted lexically;
text arrays preserve author order.

## Milestones

Path: `.atelier/milestones/<record-id>.md`

Milestone front matter adds `desired_state`, `scope`,
`validation_criteria`, `accepted_evidence`, `completion_state`, `missions`, and
`contributing_work`. ID arrays are sorted lexically; text arrays preserve author
order. Milestones are checkpoint records, not work containers; epics and issues
link to them as contributing work.

## Plans

Path: `.atelier/plans/<record-id>.md`

Current staged support writes plan records with common front matter fields plus
a quoted JSON `data` field containing `revision` and `revisions`. The body is
the latest durable execution intent.

The target expanded front matter remains `owner`, `applies_to`, `revision`,
`supersedes`, and `drift_status`. `applies_to` and `supersedes` are sorted
lexically. Plan body is the durable execution intent.

## Evidence

Path: `.atelier/evidence/<record-id>.md`

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

Path: `.atelier/cache/mission-control.json` or another documented cache path.

`mission-control.json` is derived, not a rebuild source. Target rebuild and lint
must ignore cache projections for canonical state reconstruction and regenerate
them from canonical records when Mission Control projection work lands in
Milestone 6.

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

`atelier export` remains the deterministic repair/check surface for canonical
records, and normal durable writes target `.atelier/` directly.

`atelier rebuild` recreates `.atelier/state.db` from tracked
`.atelier/` canonical records and may create ignored runtime/cache directories
in a fresh checkout. Backup export formats are no longer command surfaces;
predecessor imports use `atelier import-beads`.

Rebuild also recreates local `ProjectionIndex` source metadata in SQLite. The
metadata records canonical file paths, size and modified-time hints, and content
hashes so query commands can detect stale projections before reading SQLite.
This metadata is intentionally not tracked and can be discarded with
`.atelier/state.db`. Issue activity sidecars are canonical files
but are read directly by history/show commands, so they are validated by rebuild
rather than tracked as query-projection sources.

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
