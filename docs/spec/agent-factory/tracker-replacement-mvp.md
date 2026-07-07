# Agent Factory Tracker Replacement MVP

> Historical replacement specification. Its `ProjectionIndex`, projection
> refresh, export, and rebuild command descriptions record the migration plan;
> they are not current operator guidance. Current storage behavior is defined
> by [ADR 0017](../../adr/0017-sqlite-domain-cache-and-hard-removal.md),
> [Markdown-First Record Store](../../architecture/markdown-first-record-store.md),
> and [SQLite Domain Cache Schema](../../architecture/sqlite-runtime-schema.md).

This document records the minimum Atelier contract used when this repository
and Agent Factory replaced Beads for durable work tracking. It is not a Beads
compatibility promise or the owner of the current CLI. The current command
contract lives in [CLI Surface Tiers](../../product/cli-surface.md); the
`atelier-c0mp` issue-inventory and Mission Overview contract further defines
the current inventory and dashboard split. The historical bar was the smallest
human command, storage, projection, drill-down, and failure-mode surface that
let agents plan, start, update, validate, and hand off work without private chat
state.

Current storage architecture is superseded by
[ADR 0002: Markdown-First Record Store](../../adr/0002-markdown-first-record-store.md)
and then by ADR 0017. Historical MVP language about projection freshness remains
useful only as a cutover record; current work uses durable record files plus the
lazy SQLite domain cache, with `atelier check` as the normal committed-state
validation surface.

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
- `AGENTS.md` routes normal repository tracker work through Atelier
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
current supported replacements:

- quiet acknowledgements for simple composition and exit-status checks;
- canonical `.atelier/` Markdown records for durable state;
- rebuildable local projections refreshed transparently by normal commands, with
  `atelier check --fix` reserved for explicit repair of ignored local state;
- committed-state and health checks through `atelier check` and checkout
  orientation through `atelier status`;
- focused drill-down commands such as `atelier issue show <id>`,
  `atelier issue transition <objective-id>`, `atelier issue list`,
  `atelier work ready`, `atelier work blocked`, and record-specific
  `atelier issue link` or `atelier issue unlink` commands;
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

The following error-code vocabulary is a historical cutover inventory, not a
promise that current commands emit these exact codes or expose the retired
diagnostic commands that once produced them:

| Code | Use |
| --- | --- |
| `not_found` | A requested issue, mission, dependency, or state file is absent. |
| `invalid_input` | Flags, status values, issue types, priorities, labels, or IDs are invalid. |
| `invalid_dependency` | A dependency would reference a missing record, duplicate an edge, or create a cycle where cycles are forbidden. |
| `blocked` | A command cannot proceed because open blockers exist. |
| `stale_export` | Historical name for a cutover-era mismatch between SQLite, derived projections, and `.atelier/`. Current committed-state validation uses `atelier check`. |
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
| `issue_type` | Stable type string such as `epic`, `task`, `feature`, `bug`, `validation`, or `spike`. Use `task` for work whose deliverable is an ADR, spec, context, or target-state update. |
| `priority` | Stable priority value comparable for ready ordering. |
| `labels` | Sorted array of strings. |
| `parent` | Parent ID or `null`. |
| `dependencies` | Array of direct blockers with IDs and status. |
| `dependents` | Array of records this issue links. |
| `notes` | String or chronological array preserving durable handoff notes. |
| `assignee` | Claimed actor or `null`. |
| `owner` | Accountable owner or `null`. |
| `created_at` | UTC RFC 3339 timestamp when known. |
| `updated_at` | UTC RFC 3339 timestamp. |
| `started_at` | UTC RFC 3339 timestamp or `null`. |
| `closed_at` | UTC RFC 3339 timestamp or `null`. |
| `close_reason` | String or `null`. |

## MVP Matrix

Rows marked required were blockers for repository cutover. Optional rows could
be implemented earlier, but were not allowed to delay the first Beads
replacement unless they became necessary to satisfy a required row.

The Beads column and the cutover ownership/proof columns are historical. The
Atelier column has been updated to the current supported path so this matrix is
not mistaken for live guidance to removed commands. The `Machine-readable
replacement` column names the durable or composable replacement for the retired
command-result JSON envelope; it does not require commands to emit JSON.

| Agent Factory operation | Beads command at cutover | Current Atelier path | Required text behavior | Machine-readable replacement | Required at cutover | Historical owner |
| --- | --- | --- | --- | --- | --- | --- |
| Inspect assigned bead before work | `bd show <id>` | `atelier issue show <id>` | Print title, status, type, priority, owner/assignee, parent, blockers, dependents, description, acceptance criteria, notes, and close reason when present. Missing IDs name the requested ID. | Focused drill-down command for operators; canonical `.atelier/issues/<id>.md` plus activity sidecars for durable fields. | Yes | `atelier-z1p.3` |
| Begin assigned work | `bd update <id> --claim` | Inspect `atelier issue transition <id>`, then execute the configured start transition, such as `atelier issue transition <id> start`. | Print the applied transition and resulting status or actionable blockers. Repeating a transition that no longer applies reports current state rather than creating a parallel claim. | Canonical issue status and transition activity define current work; there is no claim or session pointer. | Yes | `atelier-z1p.3` |
| Append durable handoff notes | Beads update notes flag | `atelier issue note <id> "..."` | Print the ID and note timestamp. Do not require an editor. | Quiet acknowledgement plus `.atelier/issues/<id>.activity/`. | Yes | `atelier-z1p.3` |
| Update title/priority/labels/parent | `bd update <id> --title ... --priority ... --label ... --parent ...` | `atelier issue update <id> ...` plus label/parent flags; edit rich body sections in `.atelier/issues/<id>.md` | Print changed fields and the ID. Invalid values are rejected with actionable text. | Quiet acknowledgement and canonical issue record; invalid values fail with actionable diagnostics and non-zero exit status. | Yes | `atelier-z1p.3` |
| Move issue workflow status | `bd update <id> --status ...` | `atelier issue transition <id> <transition>` | Print the applied transition and resulting status, or rejected workflow gates with actionable text. | Canonical issue record plus transition activity sidecar. | Yes | `atelier-z1p.3` |
| Close work with reason | `bd close <id> --reason "..."` | Inspect `atelier issue transition <id>`, then execute the configured close transition, such as `atelier issue transition <id> close --reason "..."`. | Print the applied transition and reason. Refuse closure when required blockers or workflow validators remain. | Canonical close metadata and transition activity; `atelier check` and transition validators own machine-checkable defects. | Yes | `atelier-z1p.3` |
| Reopen accidentally closed work | `bd reopen <id>` | Inspect `atelier issue transition <id>`, then execute the configured reopen transition when one is available. | Print the applied transition and previous close reason when present. | Workflow transition acknowledgement and canonical issue record with reopened state. | Yes | `atelier-z1p.3` |
| Find ready executable work | `bd ready` | `atelier work ready` | List todo-category issues with no open blockers, sorted by priority then updated age or documented deterministic tie-breaker. Show blockers count when no work is ready. | Focused ready-work view backed by ProjectionIndex rebuilt from `.atelier/`; scripts may use IDs from quiet output for the next drill-down command. | Yes | `atelier-z1p.3` |
| List/filter work | `bd list --status=open` | `atelier issue list --status todo` | Print compact rows with ID, status, priority, type, title, and assignee. | Focused inventory command backed by ProjectionIndex; durable fields remain in canonical records. | Yes | `atelier-z1p.3` |
| Find work without a known ID | `bd search "<topic>"` | There is no root search command. Use metadata filters on `atelier issue list`, then `atelier issue show <id>` for the selected record. | Print a bounded flat inventory with stable IDs and exact metadata; detail remains a separate drill-down. | Canonical issue records plus the bounded inventory projection; full-text search is not part of the current public surface. | Yes | `atelier-z1p.3` |
| Create normal task/feature/bug/validation/closeout beads | `bd create ...` | `atelier issue create ...` | Print new ID and title. All required fields must be accepted by flags or stdin, not an editor. | Quiet acknowledgement with new ID; canonical record is the durable created state. | Yes | `atelier-z1p.3` |
| Create parent/child hierarchy | `bd update <child> --parent <epic>` and `bd children <epic>` | `atelier issue update <child> --parent <parent>` for mutation and `atelier issue show <objective-id>` for inspection | Parent update prints child and parent IDs. Detail output distinguishes hierarchy from blocking dependencies. | Parent mutation uses quiet acknowledgement and canonical links/parent fields; detail is a focused drill-down command backed by ProjectionIndex. | Yes | `atelier-z1p.3` |
| Add/remove blocking dependency | `bd dep add <blocked> <blocker>` and `bd dep remove <blocked> <blocker>` | `atelier issue link <blocked> <blocker> --role blocked_by` and `atelier issue unlink <blocked> <blocker> --role blocked_by` | Print blocked ID, blocker ID, and relationship role. Duplicate adds and missing removes must be idempotent or report clear no-op behavior. | Quiet acknowledgement and canonical typed relation state; invalid edges fail with actionable diagnostics and non-zero exit status. | Yes | `atelier-z1p.3` |
| List blocked work | `bd blocked` | `atelier work blocked` | Print bounded open work with active blockers and IDs for drill-down. | Focused operational view backed by ProjectionIndex; use `atelier issue show <id>` for durable details. | Yes | `atelier-z1p.3` |
| Validate tracker records | `bd lint` and `bd lint <id>` | `atelier check` and `atelier check <id>` | Print a pass/fail summary and each actionable defect with its record ID or path. | Exit status and named findings are the supported health-check boundary. | Yes | `atelier-z1p.3` |
| Check tracker installation/health | `bd doctor` / `bd ping` | `atelier status` for orientation and `atelier check` for committed-state health; use `atelier check --fix` only when ignored local state needs repair. | Print tracker state and actionable recovery without making local projection mechanics a routine operator concern. | Exit status, named paths/findings, and canonical records are the supported boundaries. | Yes | `atelier-z1p.3` |
| Persist durable state | `bd export -o .beads/issues.manual.jsonl` | No export step is required. Mutating commands write canonical `.atelier/` Markdown directly; validate it with `atelier check`. | Mutations name the changed record; `atelier check` names invalid tracked records or workflow configuration. | Canonical `.atelier/` records are the durable state. Low-level deterministic renderers are not normal workflow. | Yes | `atelier-ywow` closed; parity polish in `atelier-z1p.3` |
| Check committed state before handoff | No exact Beads equivalent; Agent Factory used backup plus Dolt status | `atelier check` plus `git status --short --branch` | Report committed-state defects and show any uncommitted tracker changes. | Exit status, named findings, canonical records, and Git state are the handoff boundary. | Yes | `atelier-ywow` closed; parity polish in `atelier-z1p.3` |
| Repair runtime state after checkout | `bd bootstrap` / Dolt sync | Normal commands refresh derived state; when repair is explicitly required, use `atelier check --fix`. | Keep ProjectionIndex and RuntimeState mechanics out of the normal workflow, and name the repair path only for degraded local state. | ProjectionIndex and RuntimeState remain rebuildable from canonical `.atelier/` records. | Yes | `atelier-fq9y` closed; cutover proof in `atelier-z1p.4` |
| Import current Beads data | `bd export` then `bd import` | `atelier init --import-beads` imports the standard repo-local `.beads/issues.manual.jsonl` migration input. | Print imported, skipped, lossy, and failed counts. Preserve or report every source ID. Standalone predecessor-import helpers are hidden migration tooling, not current workflow guidance. | One-way import input and canonical `.atelier/` output; import reports are migration evidence, not an ongoing command-result API. | Yes | `atelier-z1p.2` |
| Push/pull tracker state | `bd dolt pull`, `bd dolt push`, `bd dolt status` | Git plus `.atelier/`: `git status`, normal Git pull/merge, `atelier check`, and `git push`. | Agent docs state that committed `.atelier/` is the durable sync surface and SQLite is local derived state. | Git moves committed canonical records; normal commands refresh projections and `atelier check` detects invalid committed state. | Yes | `atelier-z1p.4` |
| Preserve manual backup before cutover | `.beads/issues.manual.jsonl` | Archived Beads export plus Atelier import report | Print backup path and source record count. | Archived migration artifact and import evidence only. It is not part of normal Atelier automation after cutover. | Yes | `atelier-z1p.2`, `atelier-z1p.4` |
| Record comments separately from notes | `bd comment` / `bd comments` | `atelier issue note` | Text must show chronological comments if used by Agent Factory. | Canonical activity sidecars or future notes records. A command-result JSON comment envelope is not required for the first cutover. | No for first cutover if Beads notes are preserved | Later feature bead |
| Worktree creation and assignment | `bd worktree` | Use ordinary Git worktrees when isolation is needed; no Atelier worktree command is part of the current surface. | Not required for first cutover. | Git owns worktree state. | No | Deferred |
| Mission dashboards | None in Beads MVP | `atelier work missions` for the cross-mission overview, `atelier work mission <mission-id>` for one mission, and `atelier work epic <epic-id>` for one epic. | Not required for first cutover; these bounded views now own current coordination. | Canonical records and ProjectionIndex-backed views. | No | Deferred at cutover; now specified |
| Workflow validators and evidence enforcement | `bd gate` and local conventions | `atelier issue transition <id>` exposes validators and executable transitions; `atelier evidence record` captures proof. | Not required for first cutover; now part of workflow-backed operation. | Canonical workflow policy, transition activity, and evidence records. | No | Deferred at cutover; now specified |
| Agent run/session accounting | Beads audit/session-adjacent features | No current run or session command. `atelier status` and active-category issue records define current work; long-term run records remain deferred. | Not required for first cutover and still not a current command surface. | Canonical issue status and activity, without a parallel runtime pointer. | No | Deferred |

## Agent Factory Operation Contract

Agent Factory replacement requires these end-to-end workflows, not just isolated
commands:

1. Start gate: `atelier issue show <id>`, `atelier issue transition <id>`, the
   configured start transition, `git status --short --branch`, and `atelier
   check` let an implement worker verify scope, enter active work, and detect
   invalid tracker state without a parallel claim or session pointer.
2. Planning/orchestration: `atelier work ready`, `atelier issue create`,
   parent updates, and typed `atelier issue link` or `atelier issue unlink`
   operations let an orchestrator create and sequence child work without `bd`.
3. Implementation handoff: `atelier issue note`, the configured close
   transition, `atelier check`, `atelier evidence record`, and Git status
   produce enough durable evidence for the next agent to resume.
4. Terminal validation: a validation worker must be able to classify parent
   criteria in durable evidence or notes and execute the configured close
   transition without using Beads.
5. Recovery: from a clean checkout-like state, `atelier check --fix` repairs
   ignored local projection/runtime state from `.atelier/`, after which
   show/list/work/check commands behave the same as before repair.

## Historical Command Mappings

This table is a historical Beads-to-Atelier cutover crosswalk. The Beads column
is archival, not callable Atelier guidance. The Atelier column has been updated
to the current public workflow; removed claim, root-search, top-level dependency,
direct-close, lint, doctor, export, rebuild, standalone predecessor-import, and
session paths are not compatibility aliases.

| Beads command | Atelier command |
| --- | --- |
| `bd show <id>` | `atelier issue show <id>` |
| `bd update <id> --claim` | Inspect `atelier issue transition <id>`, then execute its configured start transition. |
| Beads update notes flag | `atelier issue note <id> "..."` |
| `bd update <id> --title ... --priority ...` | `atelier issue update <id> --title ... --priority ...`; edit body sections in `.atelier/issues/<id>.md` |
| `bd update <id> --parent <parent>` | `atelier issue update <id> --parent <parent>` |
| `bd close <id> --reason "..."` | `atelier issue transition <id> close --reason "..."` when `close` is the configured transition. |
| `bd ready` | `atelier work ready` |
| `bd list --status=open` | `atelier issue list --status todo` |
| `bd search "<topic>"` | Use metadata filters on `atelier issue list`, then `atelier issue show <id>`; root full-text search is not supported. |
| `bd create ...` | `atelier issue create ...` |
| `bd dep add <blocked> <blocker>` | `atelier issue link <blocked> <blocker> --role blocked_by` |
| `bd dep remove <blocked> <blocker>` | `atelier issue unlink <blocked> <blocker> --role blocked_by` |
| `bd lint [id]` | `atelier check [id]` |
| `bd doctor` / `bd ping` | `atelier status` for orientation, `atelier check` for health, and `atelier check --fix` only for explicit local repair. |

After identity cutover, every command in this mapping uses the single
project-scoped random Atelier ID such as `atelier-z1p8`. Numeric IDs such as
`#1` or `1`, typed-prefix IDs such as `ISS-0001`, and imported predecessor IDs
are not maintained as alternate command references. Required commands use
focused human output; committed `.atelier/` records and projections are
the machine-readable state boundary.

## Cutover Status

These historical cutover criteria record the repository switch to Atelier.
Their evidence descriptions do not define the current CLI:

| Blocker | Owning bead | Required proof |
| --- | --- | --- |
| Beads data import from `.beads/issues.manual.jsonl` preserves current records, relationships, statuses, labels, notes, and close metadata or reports precise loss. | `atelier-z1p.2` | Import report, count comparison, round-trip show/list/update/close validation, and fixture-based tests. |
| Agent Factory command parity covers every required MVP matrix row with focused text behavior. | `atelier-z1p.3` | Historical focused CLI tests and a manual transcript covering the then-current lifecycle, relationship, health, deterministic-rendering, and projection-recovery paths. |
| Repository dogfood cutover proves Atelier is live for `/root/atelier`. | `atelier-z1p.4` | `AGENTS.md` uses Atelier commands, a real update and closeout happen through Atelier, normal repository work no longer needs `bd`, and the old archive is purged after validation. |
| Agent Factory skill docs support repository trackers instead of hard-coding Beads. | `atelier-z1p.5` | Follow-up outside this repository cutover; skill procedures route through repository tracker commands and include Atelier examples. |

Already closed prerequisites that remain part of the replacement foundation:

| Foundation | Bead | Status |
| --- | --- | --- |
| Deterministic canonical rendering and its historical freshness diagnostic. | `atelier-ywow` | Closed; not a current normal-workflow command. |
| SQLite projection recovery from `.atelier/`. | `atelier-fq9y` | Closed; current explicit local repair routes through `atelier check --fix`. |
| Milestone 2 storage closeout. | `atelier-pefi` | Closed. |

## Historical Deferred Non-Blockers

The following capabilities were explicitly non-blocking at the replacement
cutover. This is historical prioritization, not a list of current command names
or implementation status:

- Rich UI beyond the bounded `atelier work missions`, `atelier work mission`,
  and `atelier work epic` views.
- Live agent process supervision.
- Long-term run accounting; no session command or runtime current-work pointer is implied.
- First-class workflow policy engines and closure validators beyond MVP diagnostics.
- Worktree creation, branch naming, or PR automation.
- Advanced duplicate detection, semantic search, federation, external tracker
  integrations, or async coordination validators.
- Full Beads command compatibility for commands not used by Agent Factory.

## Documentation Cutover Checklist

The repository cutover updated:

- `AGENTS.md`: change tracker binding from Beads to Atelier and replace
  predecessor sync and health commands with Git, `atelier status`, and
  `atelier check`.
- `AGENTS.md`: replace Beads-specific agent startup instructions with Atelier
  tracker instructions.
- Repository quality docs: replace Beads validation commands with `atelier
  check`, workflow transitions, and evidence records.
- Old tracker archive: purge after import, validation, and projection-recovery
  proof.

Agent Factory skill procedure changes are owned by `atelier-z1p.5`; this
repository cutover does not edit global skill docs.
