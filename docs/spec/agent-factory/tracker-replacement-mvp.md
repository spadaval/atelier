# Agent Factory Tracker Replacement MVP

This document defines the minimum Atelier contract required before this
repository and Agent Factory can replace Beads for durable work tracking. It is
not a full Beads compatibility promise. The bar is the smallest human command,
storage, projection, drill-down, and failure-mode surface that lets agents plan,
claim, update, validate, and hand off work without private chat state.

Current storage architecture is superseded by
[ADR 0002: Markdown-First Record Store](../../adr/0002-markdown-first-record-store.md).
Historical MVP language about SQLite-to-export freshness remains useful as
cutover record, but future work should treat Markdown records as canonical and
SQLite as ProjectionIndex plus RuntimeState.

## Cutover Rule

Atelier is the live tracker for `/root/atelier` after `atelier-z1p.4` cutover.
The replacement-critical rows in the MVP matrix were implemented, tested, and
exercised in a real Agent Factory workflow before switching the repository
binding. Beads is retained only as an archived recovery source.

The cutover must prove:

- Current Beads records are imported into Atelier or explicitly waived.
- `.atelier/` is current, deterministic, and rebuildable from a clean
  checkout-like state.
- Agent-facing text output is usable without JSON parsing.
- Agent-facing workflows use focused text output plus committed tracker state
  instead of command-result JSON.
- Missing IDs, invalid dependencies, stale exports, and unhealthy tracker state
  fail with actionable diagnostics.
- `AGENTFACTORY.md` routes normal repository tracker work through Atelier
  commands. Global Agent Factory procedure updates are tracked separately by
  `atelier-z1p.5`.

## Command-Result JSON Migration

Earlier cutover work required a command-result `--json` envelope for Agent
Factory parity. That contract is retired. Atelier command results are
human-first, quiet output remains intentionally terse, and durable
machine-readable state lives in `.atelier/`, rebuildable projection files,
authored JSON inputs, and diagnostic logging surfaces such as
`--log-format json`.

Scripts and agents migrating away from command-result JSON should use these
supported replacements:

- quiet acknowledgements for simple composition and exit-status checks;
- canonical `.atelier/` Markdown records for durable state;
- rebuildable local projections refreshed by `atelier rebuild`;
- freshness and health commands such as `atelier export --check`,
  `atelier lint`, and `atelier doctor`;
- focused drill-down commands such as `atelier issue show <id>`,
  `atelier mission show <id>`, `atelier issue list --ready`, and dependency commands;
- documented authored JSON inputs or derived projection files only when a
  specific spec defines the contract.

Full human detail output is not a stable API. Automation must not scrape section
headings, table layout, prose, or whole command reports as if they were a
versioned machine-readable schema.

Local command diagnostics are also not a replacement for the retired
command-result JSON envelope. They may record redacted command families,
durations, exit status, and phase timings for local performance analysis, but
they must stay outside `.atelier/` and must not become the Agent Factory
automation contract.

Historical error codes from the replacement MVP remain useful vocabulary for
human diagnostics and workflow validators:

| Code | Use |
| --- | --- |
| `not_found` | A requested issue, mission, dependency, or state file is absent. |
| `invalid_input` | Flags, status values, issue types, priorities, labels, or IDs are invalid. |
| `invalid_dependency` | A dependency would reference a missing record, duplicate an edge, or create a cycle where cycles are forbidden. |
| `blocked` | A command cannot proceed because open blockers exist. |
| `stale_export` | SQLite, derived projections, and `.atelier/` differ during `export --check` or a cutover check. |
| `schema_mismatch` | Rebuild/import encountered unsupported projection schema or version. |
| `dirty_tracker` | Tracker state has unexported or unpushed changes that must be resolved before handoff. |
| `storage_error` | SQLite, file IO, or manifest validation failed. |

Issue detail views should expose the high-value fields from this historical
inventory when they are actionable in the current workflow, and should point to
drill-down commands for related records instead of dumping every field:

| Field | Rule |
| --- | --- |
| `id` | Stable project-scoped random string ID such as `atelier-z1p8`. There is no second primary identity or legacy numeric alias after cutover. |
| `title` | Non-empty human title. |
| `description` | String or `null`; maps Beads description. |
| `acceptance_criteria` | String or array preserving imported acceptance text. |
| `status` | Stable status string such as `open`, `in_progress`, or `closed`. |
| `issue_type` | Stable type string such as `epic`, `task`, `feature`, `bug`, `validation`, `closeout`, or `spike`. Use `task` for work whose deliverable is an ADR, spec, context, or target-state update. |
| `priority` | Stable priority value comparable for ready ordering. |
| `labels` | Sorted array of strings. |
| `parent` | Parent ID or `null`. |
| `dependencies` | Array of direct blockers with IDs and status. |
| `dependents` | Array of records this issue blocks. |
| `notes` | String or chronological array preserving durable handoff notes. |
| `assignee` | Claimed actor or `null`. |
| `owner` | Accountable owner or `null`. |
| `created_at` | UTC RFC 3339 timestamp when known. |
| `updated_at` | UTC RFC 3339 timestamp. |
| `started_at` | UTC RFC 3339 timestamp or `null`. |
| `closed_at` | UTC RFC 3339 timestamp or `null`. |
| `close_reason` | String or `null`. |

## MVP Matrix

Rows marked required are blockers for repository cutover. Optional rows may be
implemented earlier, but they are not allowed to delay the first Beads
replacement unless they become necessary to satisfy a required row.

The `Machine-readable replacement` column names the supported replacement for
the retired command-result JSON envelope. It is not a requirement to emit a
JSON result from the command itself.

| Agent Factory operation | Beads command today | Required Atelier equivalent | Required text behavior | Machine-readable replacement | Required | Owner |
| --- | --- | --- | --- | --- | --- | --- |
| Inspect assigned bead before work | `bd show <id>` | `atelier issue show <id>` | Print title, status, type, priority, owner/assignee, parent, blockers, dependents, description, acceptance criteria, notes, and close reason when present. Missing IDs name the requested ID. | Focused drill-down command for operators; canonical `.atelier/issues/<id>.md` plus activity sidecars for durable fields. | Yes | `atelier-z1p.3` |
| Claim assigned work | `bd update <id> --claim` | `atelier issue update <id> --claim` or `atelier issue claim <id>` | Print the claimed ID, previous assignee, new assignee, and status transition. Reclaim by same actor is idempotent. | Quiet acknowledgement with changed ID/fields; canonical issue record records durable owner/assignee state. | Yes | `atelier-z1p.3` |
| Append durable handoff notes | `bd update <id> --append-notes "..."` | `atelier issue update <id> --append-notes "..."` or `atelier issue note <id> "..."` | Print the ID and note timestamp. Do not require an editor. | Quiet acknowledgement plus `.atelier/issues/<id>.activity/` or the canonical note field, depending on the notes model in use. | Yes | `atelier-z1p.3` |
| Update title/body/priority/labels/parent | `bd update <id> --title ... --description ... --priority ... --label ... --parent ...` | `atelier issue update <id> ...` plus label/parent flags | Print changed fields and the ID. Invalid values are rejected with actionable text. | Quiet acknowledgement and canonical issue record; invalid values fail with actionable diagnostics and non-zero exit status. | Yes | `atelier-z1p.3` |
| Move issue workflow status | `bd update <id> --status ...` | `atelier issue transition <id> <transition>` | Print the applied transition and resulting status, or rejected workflow gates with actionable text. | Canonical issue record plus transition activity sidecar. | Yes | `atelier-z1p.3` |
| Close work with reason | `bd close <id> --reason "..."` | `atelier issue close <id> --reason "..."` | Print closed ID and reason. Refuse closure when required blockers or workflow validators remain, unless an explicit force flag is supported and logged. | Quiet acknowledgement and canonical close metadata; `atelier lint` or workflow validation owns machine-checkable closure defects. | Yes | `atelier-z1p.3` |
| Reopen accidentally closed work | `bd reopen <id>` | `atelier issue transition <id> reopen` | Print the applied transition and previous close reason when present. | Workflow transition acknowledgement and canonical issue record with reopened state. | Yes | `atelier-z1p.3` |
| Find ready executable work | `bd ready` | `atelier issue list --ready` | List todo-category issues with no open blockers, sorted by priority then updated age or documented deterministic tie-breaker. Show blockers count when no work is ready. | Focused queue command backed by ProjectionIndex rebuilt from `.atelier/`; scripts may use IDs from quiet output for the next drill-down command. | Yes | `atelier-z1p.3` |
| List/filter work | `bd list --status=open` | `atelier issue list --status todo` | Print compact rows with ID, status, priority, type, title, and assignee. | Focused queue command backed by ProjectionIndex; durable fields remain in canonical records. | Yes | `atelier-z1p.3` |
| Search work by text | `bd search "<topic>"` | `atelier issue search "<topic>"` | Print ranked matches with ID, title, status, and short excerpt when available. | Focused search command backed by ProjectionIndex; scripts should use returned IDs for follow-up drill-down instead of parsing excerpts. | Yes | `atelier-z1p.3` |
| Create normal task/feature/bug/validation/closeout beads | `bd create ...` | `atelier issue create ...` | Print new ID and title. All required fields must be accepted by flags or stdin, not an editor. | Quiet acknowledgement with new ID; canonical record is the durable created state. | Yes | `atelier-z1p.3` |
| Create parent/child hierarchy | `bd update <child> --parent <epic>` and `bd children <epic>` | `atelier issue update <child> --parent <parent>` and `atelier issue children <parent>` or `atelier issue tree` | Parent update prints child and parent IDs. Children/tree output distinguishes hierarchy from blocking dependencies. | Parent mutation uses quiet acknowledgement and canonical links/parent fields; children/tree are focused drill-down commands backed by ProjectionIndex. | Yes | `atelier-z1p.3` |
| Add/remove blocking dependency | `bd dep add <blocked> <blocker>` and `bd dep remove <blocked> <blocker>` | `atelier issue block <blocked> <blocker>` and `atelier issue unblock <blocked> <blocker>` or `atelier dep add/remove` | Print blocked ID and blocker ID. Duplicate adds and missing removes must be idempotent or report clear no-op behavior. | Quiet acknowledgement and canonical typed relation state; invalid edges fail with actionable diagnostics and non-zero exit status. | Yes | `atelier-z1p.3` |
| List blocked work | `bd blocked` | `atelier issue blocked` | Print open issues grouped or annotated by open blockers. | Focused queue command backed by ProjectionIndex; use `issue show` for a selected record's durable details. | Yes | `atelier-z1p.3` |
| Validate tracker records | `bd lint` and `bd lint <id>` | `atelier lint` and `atelier lint <id>` or documented equivalent | Print pass/fail summary and each actionable defect with record ID. | Supported health/check command. Exit status and finding text are the stable automation boundary unless a future diagnostic format is documented. | Yes | `atelier-z1p.3` |
| Check tracker installation/health | `bd doctor` / `bd ping` | `atelier doctor` or `atelier status --check` | Print runtime DB path, state path, schema version, export freshness, and rebuild readiness. | Supported health/check command. Exit status and named paths/state are the stable automation boundary unless a future diagnostic format is documented. | Yes | `atelier-z1p.3` |
| Export durable state | `bd export -o .beads/issues.manual.jsonl` | `atelier export` and `atelier export --output .atelier` | Print records written, output path, and whether derived files were regenerated. | Canonical `.atelier/` records and derived projections; `export --check` verifies freshness. | Yes | `atelier-ywow` closed; parity polish in `atelier-z1p.3` |
| Check export freshness before handoff | No exact Beads equivalent; Agent Factory uses backup plus Dolt status | `atelier export --check` | Print whether `.atelier/` is current. Stale files name changed, missing, or unexpected paths. | Supported freshness gate. Exit status, stale path names, and canonical records are the automation boundary. | Yes | `atelier-ywow` closed; parity polish in `atelier-z1p.3` |
| Rebuild runtime state after checkout | `bd bootstrap` / Dolt sync | `atelier rebuild` | Print source state dir, runtime DB path, record counts, and schema validation result. | Rebuildable ProjectionIndex and RuntimeState generated from `.atelier/`; the rebuilt SQLite database is local runtime state. | Yes | `atelier-fq9y` closed; cutover proof in `atelier-z1p.4` |
| Import current Beads data | `bd export` then `bd import` | `atelier import-beads .beads/issues.manual.jsonl` or `atelier import --format beads-jsonl ...` | Print imported, skipped, lossy, and failed counts. Preserve or report every source ID. | One-way import input and canonical `.atelier/` output; import reports are migration evidence, not an ongoing command-result API. | Yes | `atelier-z1p.2` |
| Push/pull tracker state | `bd dolt pull`, `bd dolt push`, `bd dolt status` | Git plus `.atelier/`: `git status`, `git pull`, `atelier rebuild`, `atelier export --check`, `git push` | Agent docs must state that committed `.atelier/` is the durable sync surface and SQLite is local runtime state. | Git moves committed canonical records; `atelier rebuild` recreates local projections; health/check commands detect dirty projection state. | Yes | `atelier-z1p.4` |
| Preserve manual backup before cutover | `.beads/issues.manual.jsonl` | Archived Beads export plus Atelier import report | Print backup path and source record count. | Archived migration artifact and import evidence only. It is not part of normal Atelier automation after cutover. | Yes | `atelier-z1p.2`, `atelier-z1p.4` |
| Record comments separately from notes | `bd comment` / `bd comments` | `atelier issue comment` / future notes model | Text must show chronological comments if used by Agent Factory. | Canonical activity sidecars or future notes records. A command-result JSON comment envelope is not required for the first cutover. | No for first cutover if Beads notes are preserved | Later feature bead |
| Worktree creation and assignment | `bd worktree` | Future `atelier worktree` | Not required for first cutover; agents can use normal Git worktrees. | Not required. | No | Deferred |
| Mission Control dashboards | None in Beads MVP | Future `atelier mission-control` or UI | Not required for first cutover. | Not required. | No | Deferred |
| Workflow validators and evidence enforcement | `bd gate` and local conventions | Future Atelier workflows, validators, and evidence records | Not required beyond lint/close diagnostics for first cutover. | Not required beyond MVP errors. | No | Deferred |
| Agent run/session accounting | Beads audit/session-adjacent features | Future Atelier run records | Not required for first cutover. | Not required. | No | Deferred |

## Agent Factory Operation Contract

Agent Factory replacement requires these end-to-end workflows, not just isolated
commands:

1. Start gate: `atelier issue show <id>`, `atelier issue update <id>
   --claim`, `git status --short --branch`, and tracker health checks must let
   an implement worker verify scope, claim ownership, and detect stale tracker
   state.
2. Planning/orchestration: `atelier issue list --ready`, `atelier issue create`,
   parent updates, and dependency operations must let an orchestrator create and
   sequence child work without `bd`.
3. Implementation handoff: notes, close, `atelier export --check`, lint, and
   Git status must produce enough durable evidence for the next agent to resume.
4. Closeout: a closeout worker must be able to classify parent criteria in
   durable notes and close the parent or child without using Beads.
5. Recovery: from a clean checkout-like state, `atelier rebuild` must recreate
   local SQLite state from `.atelier/`, after which show/list/ready/lint
   commands behave the same as before rebuild.

## Historical Command Mappings

Atelier supports the following agent-facing command surface for the Beads subset
used by workers and orchestrators:

| Beads command | Atelier command |
| --- | --- |
| `bd show <id>` | `atelier issue show <id>` |
| `bd update <id> --claim` | `atelier issue update <id> --claim` |
| `bd update <id> --append-notes "..."` | `atelier issue update <id> --append-notes "..."` |
| `bd update <id> --title ... --description ... --priority ...` | `atelier issue update <id> --title ... --description ... --priority ...` |
| `bd update <id> --parent <parent>` | `atelier issue update <id> --parent <parent>` |
| `bd close <id> --reason "..."` | `atelier issue close <id> --reason "..."` |
| `bd ready` | `atelier issue list --ready` |
| `bd list --status=open` | `atelier issue list --status todo` |
| `bd search "<topic>"` | `atelier issue search "<topic>"` |
| `bd create ...` | `atelier issue create ...` |
| `bd dep add <blocked> <blocker>` | `atelier dep add <blocked> <blocker>` |
| `bd dep remove <blocked> <blocker>` | `atelier dep remove <blocked> <blocker>` |
| `bd lint [id]` | `atelier lint [id]` |
| `bd doctor` / `bd ping` | `atelier doctor` |

After identity cutover, every command in this mapping uses the single
project-scoped random Atelier ID such as `atelier-z1p8`. Numeric IDs such as
`#1` or `1`, typed-prefix IDs such as `ISS-0001`, and imported predecessor IDs
are not maintained as alternate command references. Required commands use
focused human output; committed `.atelier/` records and projections are
the machine-readable state boundary.

## Cutover Status

These cutover criteria define the repository switch to Atelier:

| Blocker | Owning bead | Required proof |
| --- | --- | --- |
| Beads data import from `.beads/issues.manual.jsonl` preserves current records, relationships, statuses, labels, notes, and close metadata or reports precise loss. | `atelier-z1p.2` | Import report, count comparison, round-trip show/list/update/close validation, and fixture-based tests. |
| Agent Factory command parity covers every required MVP matrix row with focused text behavior. | `atelier-z1p.3` | Focused CLI tests and manual command transcript for ready/show/create/update/close/dependency/lint/doctor/export/rebuild. |
| Repository dogfood cutover proves Atelier is live for `/root/atelier`. | `atelier-z1p.4` | `AGENTFACTORY.md` uses Atelier commands, a real update and closeout happen through Atelier, normal repository work no longer needs `bd`, and the old archive is purged after validation. |
| Agent Factory skill docs support tracker bindings instead of hard-coding Beads. | `atelier-z1p.5` | Follow-up outside this repository cutover; skill procedures route through repository-bound tracker commands and include Atelier examples. |

Already closed prerequisites that remain part of the replacement foundation:

| Foundation | Bead | Status |
| --- | --- | --- |
| Deterministic canonical export and `export --check`. | `atelier-ywow` | Closed. |
| SQLite rebuild from `.atelier/`. | `atelier-fq9y` | Closed. |
| Milestone 2 storage closeout. | `atelier-pefi` | Closed. |

## Deferred Non-Blockers

The following capabilities are useful Atelier differentiators, but they must
not block the first replacement cutover:

- Rich Mission Control UI or dashboard projections beyond health/check output.
- Live agent process supervision.
- Long-term run/session accounting.
- First-class workflow policy engines and closure validators beyond MVP diagnostics.
- Worktree creation, branch naming, or PR automation.
- Advanced duplicate detection, semantic search, federation, external tracker
  integrations, or async coordination validators.
- Full Beads command compatibility for commands not used by Agent Factory.

## Documentation Cutover Checklist

The repository cutover updated:

- `AGENTFACTORY.md`: change tracker binding from Beads to Atelier and replace
  sync/check/export commands.
- `AGENTS.md`: replace Beads-specific agent startup instructions with Atelier
  tracker instructions.
- Repository quality docs: replace Beads validation commands with Atelier
  export/lint/doctor commands.
- Old tracker archive: purge after import, validation, and rebuild proof.

Agent Factory skill procedure changes are owned by `atelier-z1p.5`; this
repository cutover does not edit global skill docs.
