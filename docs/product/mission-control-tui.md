# Mission Control TUI

Mission Control terminal UI is an optional browsing surface over deterministic
Mission Control JSON. It is not the primary agent interface. Agent workflows
must continue to use CLI commands plus committed `.atelier/` records and
derived projections as the authoritative interface; the TUI exists to help humans and
orchestrators inspect state, copy IDs, and choose the next CLI action.

The TUI must not read `.atelier/runtime/state.db` directly and must not invent a state
model separate from the Mission Control projection. Its input is either
`.atelier/cache/mission-control.json` or an equivalent future Mission Control
projection API with the same schema.
Mission Control JSON is derived from canonical records plus ProjectionIndex
queries; it is not an authoritative record source.

## Projection Contract

The TUI may consume only the field paths named in this section. New fields may
be added to the projection without changing the TUI, but the TUI must ignore
them until this contract is updated.

Projection identity and health:

| Field path | Use |
| --- | --- |
| `schema` | Must be `atelier.mission-control` before normal rendering. |
| `schema_version` | Selects the parser; version `1` is the initial TUI contract. |
| `generated_at` | Shown in the status line; `null` is allowed in fixtures. |
| `source.state_path` | Shown in diagnostics so users know which projection was read. |
| `source.export_current` | Drives stale-export warnings. |
| `source.projection_current` | Drives derived-state health warnings. |
| `source.errors[]` | Blocks normal rendering when derived state is invalid. |
| `source.warnings[]` | Shown in diagnostics without blocking browsing. |
| `workflow_config.path` | Identifies the workflow policy used for validator output. |
| `workflow_config.sha256` | Shown in validator detail views. |
| `workflow_config.reload_result` | Shows whether the latest config reload passed or failed. |
| `workflow_config.errors[]` | Explains workflow config reload failures. |

Dashboard and summary fields:

| Field path | Use |
| --- | --- |
| `summary.active_mission_count` | Dashboard count. |
| `summary.ready_work_count` | Dashboard count and ready-view badge. |
| `summary.backlog_count` | Dashboard count. |
| `summary.open_blocker_count` | Dashboard count and blocker badge. |
| `summary.plan_drift_count` | Dashboard count and plan badge. |
| `summary.evidence_gap_count` | Dashboard count and evidence badge. |
| `summary.validator_failure_count` | Dashboard count and validator badge. |
| `summary.artifact_update_count` | Dashboard count and artifact-update badge. |

Mission fields:

| Field path | Use |
| --- | --- |
| `missions[].id` | Row key, copy target, and detail identifier. |
| `missions[].title` | Primary row label. |
| `missions[].status` | Status chip and filtering. |
| `missions[].health` | Dashboard and row health indicator. |
| `missions[].summary` | Detail overview. |
| `missions[].constraints[]` | Detail view. |
| `missions[].current_risks[]` | Detail view and health context. |
| `missions[].milestone_ids[]` | Detail links to milestones. |
| `missions[].plan_ids[]` | Detail links to plans. |
| `missions[].evidence_gap_count` | Mission row badge. |
| `missions[].validator_failure_count` | Mission row badge. |
| `missions[].updated_at` | Sort and detail timestamp. |

Milestone and checkpoint fields:

| Field path | Use |
| --- | --- |
| `milestones[].id` | Row key, copy target, and detail identifier. |
| `milestones[].title` | Primary row label. |
| `milestones[].status` | Status chip and filtering. |
| `milestones[].mission_ids[]` | Mission cross-linking. |
| `milestones[].desired_state` | Detail view. |
| `milestones[].completion_state` | Detail view. |
| `milestones[].validation_criteria[]` | Detail checklist. |
| `milestones[].accepted_evidence_ids[]` | Evidence cross-linking. |
| `milestones[].contributing_work_ids[]` | Work cross-linking. |
| `milestones[].progress` | Checkpoint progress display. |

Backlog, ready work, review, and validation fields:

| Field path | Use |
| --- | --- |
| `work_items[].id` | Row key, copy target, and detail identifier. |
| `work_items[].title` | Primary row label. |
| `work_items[].issue_type` | Type chip and filtering. |
| `work_items[].status` | Status chip and filtering. |
| `work_items[].priority` | Sort and row label. |
| `work_items[].parent_id` | Hierarchy context. |
| `work_items[].mission_ids[]` | Mission filtering. |
| `work_items[].milestone_ids[]` | Milestone filtering. |
| `work_items[].labels[]` | Filtering and detail view. |
| `work_items[].assignee` | Claim display. |
| `work_items[].queue` | One of `ready`, `backlog`, `blocked`, `review`, or `validation`. |
| `work_items[].ready_reason` | Explains why ready work is actionable. |
| `work_items[].blocked_by[]` | Blocker cross-links. |
| `work_items[].blocking[]` | Blocked-work cross-links. |
| `work_items[].branch` | Branch/worktree context. |
| `work_items[].worktree_path` | Branch/worktree context. |
| `work_items[].updated_at` | Sort and detail timestamp. |

Blocker fields:

| Field path | Use |
| --- | --- |
| `blockers[].id` | Row key and detail identifier. |
| `blockers[].blocked_record.kind` | Blocked target label. |
| `blockers[].blocked_record.id` | Blocked target link. |
| `blockers[].blocker_record.kind` | Blocking source label. |
| `blockers[].blocker_record.id` | Blocking source link. |
| `blockers[].severity` | Sort and row indicator. |
| `blockers[].reason` | Detail explanation. |
| `blockers[].since` | Sort and age display. |
| `blockers[].actions[]` | Suggested CLI follow-up text. |

Plan fields:

| Field path | Use |
| --- | --- |
| `plans[].id` | Row key, copy target, and detail identifier. |
| `plans[].title` | Primary row label. |
| `plans[].owner` | Detail metadata. |
| `plans[].applies_to[]` | Cross-links to missions, milestones, or issues. |
| `plans[].revision` | Detail metadata. |
| `plans[].supersedes[]` | Revision cross-links. |
| `plans[].drift_status` | Plan badge and filtering. |
| `plans[].summary` | Detail overview. |
| `plans[].updated_at` | Sort and detail timestamp. |

Evidence gap fields:

| Field path | Use |
| --- | --- |
| `evidence_gaps[].id` | Row key and detail identifier. |
| `evidence_gaps[].target.kind` | Target label. |
| `evidence_gaps[].target.id` | Target link. |
| `evidence_gaps[].requirement` | Detail explanation. |
| `evidence_gaps[].required_count` | Gap count display. |
| `evidence_gaps[].actual_count` | Gap count display. |
| `evidence_gaps[].required_types[]` | Detail view. |
| `evidence_gaps[].validator` | Validator cross-link when applicable. |
| `evidence_gaps[].actions[]` | Suggested CLI follow-up text. |

Workflow validator failure fields:

| Field path | Use |
| --- | --- |
| `validator_failures[].id` | Row key and detail identifier. |
| `validator_failures[].validator` | Primary row label. |
| `validator_failures[].workflow` | Detail metadata. |
| `validator_failures[].transition` | Detail metadata. |
| `validator_failures[].record.kind` | Affected record label. |
| `validator_failures[].record.id` | Affected record link. |
| `validator_failures[].result` | Must be `fail` for this section. |
| `validator_failures[].message` | Detail explanation. |
| `validator_failures[].actions[]` | Suggested CLI follow-up text. |
| `validator_failures[].details` | JSON detail panel. |
| `validator_failures[].config_sha256` | Workflow config traceability. |

Artifact-update fields:

| Field path | Use |
| --- | --- |
| `artifact_updates[].id` | Row key, copy target, and detail identifier. |
| `artifact_updates[].title` | Primary row label. |
| `artifact_updates[].status` | Status chip and filtering. |
| `artifact_updates[].summary` | Detail overview. |
| `artifact_updates[].applies_to[]` | Cross-links to affected records. |
| `artifact_updates[].blocking[]` | Work blocked by the artifact update. |
| `artifact_updates[].completed_at` | Sort and detail timestamp when present. |
| `artifact_updates[].updated_at` | Fallback sort and detail timestamp. |

Agent, run, branch, worktree, current-work, and lock fields:

| Field path | Use |
| --- | --- |
| `agents[].id` | Row key and agent identifier. |
| `agents[].name` | Human-readable agent label. |
| `agents[].status` | Active or idle display. |
| `agents[].current_work_ids[]` | Cross-links to issues currently `in_progress` in the projected tracker copy. |
| `agents[].last_seen_at` | Staleness display. |
| `runs[].id` | Row key and run identifier. |
| `runs[].agent_id` | Cross-link to agent. |
| `runs[].status` | Active, completed, or failed display. |
| `runs[].work_ids[]` | Cross-links to affected work. |
| `runs[].started_at` | Sort and detail timestamp. |
| `runs[].last_event_at` | Sort and staleness display. |
| `branches[].name` | Branch label. |
| `branches[].worktree_path` | Worktree detail. |
| `branches[].record_ids[]` | Cross-links to work records. |
| `branches[].dirty` | Worktree warning indicator. |
| `branches[].base` | Detail metadata. |
| `branches[].head` | Detail metadata. |
| `locks[].record.kind` | Locked record label. |
| `locks[].record.id` | Locked record link. |
| `locks[].holder` | Lock owner display. |
| `locks[].expires_at` | Lock age and expiry display. |
| `locks[].reason` | Detail explanation. |

## Degradation Rules

- If `mission-control.json` is absent, the TUI starts in an unavailable state
  with diagnostics and copyable CLI commands. It must not treat absence as an
  empty tracker.
- If `schema` or `schema_version` is missing or unsupported, the TUI blocks
  normal rendering and shows the projection identity error.
- If `source.export_current` or `source.projection_current` is false, all views
  remain read-only and show a stale-state banner.
- If a top-level section array is missing, that navigation item is disabled and
  labelled unavailable. Missing sections are not equivalent to empty sections.
- If a section array is present but empty, the view renders a normal empty
  state.
- If a row is missing its `id`, that row is skipped and the diagnostics view
  reports the malformed record path.
- If a row title or summary is missing, the TUI falls back to the ID and a blank
  detail field.
- If counts are missing, the TUI hides the count instead of displaying zero.
- If optional arrays are missing or `null`, the TUI displays unknown for that
  field. Present empty arrays display as no linked records.
- If workflow config reload failed, the TUI may display the last valid
  read-only validator data only while showing the reload failure. It must not
  offer any config-dependent mutating action.
- Narrow terminals must switch to a single-pane layout instead of truncating
  critical IDs, statuses, or diagnostic text beyond recognition.

## Read-Only And Mutating Boundaries

The initial Mission Control TUI is read-only. It may:

- load or refresh Mission Control JSON;
- browse summaries, tables, and detail views;
- filter and search records already present in the projection;
- copy record IDs, paths, and suggested CLI commands;
- open a non-mutating command preview such as `atelier issue show <id>`.

The initial TUI must not:

- claim, close, reopen, label, relate, block, unblock, or otherwise mutate
  records;
- write tracked `.atelier/` records or `.atelier/runtime/state.db`;
- run workflow transitions directly;
- resolve workflow config reload failures by falling back silently;
- derive hidden state from Git, SQLite, or ad hoc filesystem scans.

Future mutating actions may be added only by a separate design and
implementation issue. Those actions must route through the same CLI/domain
command path used outside the TUI, perform workflow reload checks before the
action, require confirmation for state changes, and refresh the projection after
the command completes.

## Navigation Model

The default layout is a dashboard with a persistent section list, a table or
list pane, and a detail pane. The section list contains:

- Missions
- Ready
- Backlog
- Blockers
- Plans
- Evidence
- Validators
- Decisions
- Agents
- Branches
- Diagnostics

Keyboard expectations:

| Key | Behavior |
| --- | --- |
| `q` | Quit. |
| `?` | Toggle help. |
| `r` | Reload the projection. |
| `/` | Focus search for the current section. |
| `Esc` | Clear search, close detail, or return to the previous pane. |
| `Tab` / `Shift-Tab` | Move focus between navigation, list, detail, and diagnostics panes. |
| `Up` / `Down` or `k` / `j` | Move selection within a list. |
| `Left` / `Right` or `h` / `l` | Move between adjacent sections or panes. |
| `Enter` | Open the selected record detail. |
| `c` | Copy the selected record ID. |
| `C` | Copy the suggested CLI command for the selected record when one exists. |

Search filters the active section only and must never hide derived-state,
schema, or workflow-config diagnostics. Detail views should show the record ID
first so users can copy it even in a narrow terminal.

In narrow terminals, the TUI uses one pane at a time:

1. Section list.
2. Filtered record list.
3. Selected record detail.
4. Diagnostics.

The same keyboard model applies, but `Enter` advances into the next pane and
`Esc` returns to the previous pane.

## Fixture Requirements

TUI rendering and state tests should use deterministic Mission Control JSON
fixtures. Fixtures must set `generated_at: null` unless the test specifically
asserts timestamp rendering.

Required fixture coverage:

- active mission with linked milestones, plans, risks, evidence gaps, and
  validator failures;
- backlog and ready work, including review and validation queues;
- open blockers with both issue blockers and artifact-update blockers;
- plan states for current, drifted, and superseded plans;
- evidence gaps for missing count, missing evidence type, and linked validator;
- workflow validator failures using the machine-readable result shape from the
  workflow configuration contract;
- artifact-update tasks that block work and artifact-update tasks that only
  document context;
- empty projection with all section arrays present and empty;
- absent `mission-control.json`;
- stale export or derived-state health flags;
- missing optional section arrays;
- malformed rows with missing IDs;
- workflow config reload failure;
- narrow terminal layout snapshots for dashboard, list, detail, and diagnostics
  views.

Fixture assertions should prove that missing data degrades according to this
document, not merely that parsing succeeds.

## Source Alignment

This contract follows the target work model in [Work Model](work-model.md), the
workflow validator and reload behavior in
[Workflow Configuration Contract](workflow-configuration.md), the validation
ownership guidance in [Validation](../architecture/quality/validation.md), and
the derived projection boundary in
[Canonical Export And Rebuild Layout](../spec/storage/export/rebuild/canonical-layout.md).
