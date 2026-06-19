# Work Model

Atelier separates intent, accountable work execution, workflow control, and
proof. Deferred checkpoint or planning prose should not collapse into a
separate issue hierarchy.

## Concepts

- Mission: a long-running objective with intent, scope, constraints, current
  health, active epics, risks, validation expectations, and evidence. It is also
  the default shared background workspace boundary: one mission normally owns
  one shared worktree or equivalent checkout.
- Session: a derived issue-scoped worker/reviewer/validator attempt rebuilt
  from canonical issue activity. Session views are inspection-only and help
  explain handoff context without replacing issue workflow state.
- Checkpoint semantics: deferred v1 product language for intermediate target
  states. Checkpoint prose may live in missions, epics, issues, or evidence, but
  there is no active first-class milestone record table.
- Epic: a coordinated work package and the normal branch/review boundary.
  Epics group implementation, documentation, review, validation, and completion
  tasks that deliver a coherent change on one reviewable branch.
- Issue: the actual accountability unit for work. Ordinary implementation
  issues are local slices on an epic branch; tasks, bugs, reviews,
  validations, completions, and artifact updates are issue-shaped until a more
  specific first-class record exists.
- Workflow: the policy for how records move between states.
- Workflow validator: a transition check attached to workflow policy. A
  validator allows or rejects a transition and returns an actionable failure
  reason. Validators are not milestone fields.
- Transition action: configured work run by an explicit issue transition after
  required fields and validators pass. Actions may write issue status, activity,
  owner branch commits, integration results, or review artifact links, but they
  do not replace explicit review or issue commands.
- Evidence: durable proof that accountable work, review, validation, or completion
  happened. Normal evidence attaches to issue-shaped work because issues own
  accountability. Parent completion is derived from linked implementation,
  review, validation, and completion evidence rather than from direct proof pasted
  onto the parent objective.

Session views remain derived from issue activity; they are not a separate work
queue, and they do not become the source of truth for current work or
completion.

## Evidence Records

Evidence records are structured proof envelopes, not free-form notes with an
optional attachment. The canonical record must expose enough metadata for a
later operator to answer what was proven, by whom, for which accountable work,
and what remains risky.

Required or expected fields are:

| Field | Contract |
| --- | --- |
| `id` | Canonical evidence ID. |
| `targets` | Accountable target IDs. Version 1 normally uses `issue/<id>` for implementation, review, validation, or validation work. |
| `proof_scope` | The local Outcome line, parent validation criterion, workflow validator, audit row, or review claim being proven. |
| `kind` | Evidence type such as `test`, `validation`, `review`, `audit`, `transcript`, `artifact`, or `migration`. |
| `result` | `pass`, `fail`, `blocked`, `deferred`, or `not-applicable`. |
| `summary` | Human-readable result summary. |
| `commands` | Optional command transcript metadata: argv or rendered command, exit status, success flag, timestamp, and bounded stdout/stderr summaries. |
| `artifacts` | Optional repository paths, external URIs, sizes, and hashes when payloads are available. |
| `agent_identity` | Producer or validator identity, using the local agent identity when available. |
| `independence_level` | `implementer`, `peer`, `independent`, `completion`, or `adversarial`. |
| `residual_risks` | Known caveats that remain after this proof. Empty is allowed only when the producer has no known caveat. |
| `follow_up_ids` | Issue IDs for defects, deferred proof, migration cleanup, or remaining work. |
| `created_at` | Capture time. |

Manual summaries and command transcripts use the same record shape. A transcript
record fills `commands`; an audit table or screenshot fills `artifacts`; a
manual review fills `summary`, `proof_scope`, `independence_level`, and
`residual_risks`.

## Proof Routing

Atelier routes proof by risk and scope. Ordinary executable issues prove their
own Outcome on the issue: the `Evidence` section names the proof, the worker
runs the narrowest checks that support the claim, and the result is recorded
before completion. Ordinary implementation issues do not require a separate
independent review by default; the parent epic supplies the review and
validation boundary for the coherent changeset.

Durable issue notes are for handoff context, caveats, skipped optional checks,
and trivial proof that does not need a separate artifact. First-class evidence
records are required for non-trivial command transcripts, test results,
migration proof, process-policy changes, workflow validation, validation audits,
and any `fail`, `blocked`, `deferred`, or `not-applicable` classification that
future workers must inspect.

Separate validation issues are required when the implementer should
not validate the claim alone: migrations, public command or API contracts,
docs/help parity, stale-test risk, cross-cutting workflow or persistence
behavior, Agent Factory process changes, explicitly risk-escalated issue
slices, epic completions, and explicit mission-level completion or validation
claims.

Missions are coordination shells by default, not work logs. A mission may
retain direct evidence links only for legacy imports or migration notes. Normal
mission completion is computed from closed linked work, clear mission blockers,
configured health gates, and workflow approval on accountable child work;
mission `Validation` prose guides human completion and validation but is not
parsed as a coded evidence contract.
Missions keep the built-in lifecycle `draft`, `ready`, `active`, `superseded`,
and `closed`; `superseded` means another mission has replaced the execution
scope and hides from default current mission lists without claiming completion.
Atelier does not add a configurable mission workflow graph. Issues and epics
remain the workflow-owned records: they move through normal issue transitions
until a terminal done-category status is allowed.

The detailed routing table lives in
[Validation](../architecture/quality/validation.md). Product docs should point
to that router instead of defining a second proof model.

### Proof Routing Examples

Choose the proof surface by the claim being closed:

| Claim | Enough proof | Command shape | Completion implication |
| --- | --- | --- | --- |
| Handoff context, caveat, or local observation that does not satisfy an `Evidence` requirement | Issue or mission note | `atelier issue note <issue-id> "handoff context"` or `atelier mission note <mission-id> "coordination context"` | Notes help future operators, but completion validators do not treat them as claim proof. |
| Manual validation of an issue Outcome/Evidence line | First-class evidence record | `atelier evidence record --target issue/<issue-id> --kind validation "checked root help and docs examples against current CLI"` | The evidence summary should name the observed behavior and the target issue it validates. |
| Command-backed test, lint, audit, or transcript | Command-backed evidence record | `atelier evidence record --target issue/<issue-id> --kind test -- target/debug/atelier lint <issue-id>` | The record stores command metadata so reviewers do not rely on copied terminal prose. |
| Reusing an existing proof record for a second accountable target | Evidence attachment | `atelier evidence attach <evidence-id> issue <other-issue-id> --role validates` | Attachment is for reuse. New proof should still start with `evidence record`. |
| Process-policy, public command, persistence, migration, or cross-cutting workflow behavior | Independent validation issue plus evidence on that issue | Create a validation issue, run the checks from a clean checkout or independent review path, then record evidence on the validation issue. | Parent completion should map the parent claim to the independent validation issue and its evidence ID. |
| Epic or mission completion | Validation issue that maps parent claims to child proof | Record evidence on the validation issue. | Mission completion comes from closed linked work, clear blockers, configured health gates, and workflow approval on accountable child work. |

Example for ordinary documentation work:

```text
atelier issue note atelier-isd5 "Examples checked against graph, mission, issue, and evidence help."
atelier evidence record --target issue/atelier-isd5 --kind validation "operator command map examples use current record-specific command families"
atelier issue transition atelier-isd5 request_validation
```

Example for command-backed validation:

```text
atelier evidence record --target issue/atelier-zrqa --kind test -- target/debug/atelier lint atelier-zrqa
```

Example for independent completion proof:

```text
atelier evidence record --target issue/<validation-issue-id> --kind validation "mission completion validation maps mission expectations to closed linked work and evidence IDs"
atelier mission close <mission-id> --reason "linked work closed and completion proof attached"
```

## Parent Coverage Summaries

Issue completion reads evidence attached to that issue and checks whether the
issue's `Evidence` section has objective proof. Parent records summarize
coverage instead of owning every proof detail.

An epic coverage summary maps each epic Outcome line to child issues and their
attached evidence IDs. The summary classifies each line as `covered`,
`missing`, `failed`, `blocked`, `deferred`, or `not-applicable`, and names
residual risks plus follow-up IDs. It may use stable claim anchors for
automation-heavy, high-risk, or repeated audit work, but ordinary issues should
not need line-level claim plumbing.

A mission completion summary, when the mission needs one, lives on explicit
validation issue-shaped work. It maps explicit approval work and linked
execution status to implementation, review, and validation issues. The mission
derives completion from closed linked work, clear blockers, configured health
gates, and independent validation when configured. Direct mission evidence is
not the normal coverage source.

Existing prose evidence migrates as structured evidence with the original text
as `summary`, best-effort `kind` and `result`, the linked issue as `targets`,
`proof_scope: legacy-prose`, `independence_level: implementer` unless a
validator is clear, and a residual risk noting any missing command transcript,
artifact, or independent reviewer identity.

## Mission Sizing

Missions are goal records, not task records. A mission should describe the
desired end state and the durable context needed to coordinate work toward that
state: intent, constraints, risks, validation expectations, evidence, linked
work, optional checkpoint prose, and the shared workspace/background checkout
where the mission is executed.

A mission is large enough to require at least one epic. If the work can be
planned, claimed, implemented, validated, and closed as a single accountable
unit, it should remain an issue. If the work needs coordinated implementation,
review, validation, documentation, or migration slices under a shared objective,
the shared objective should be a mission and the executable slices should live
under one or more epics or issues linked to that mission.

## Mission Graph Shape

Prefer a shallow mission graph: the mission links to epics, validation issues, or
other root work that directly advances the objective, and those epics own their
executable child tasks. Ordinary child tasks should not also be direct mission
work unless the duplicate link is deliberate and useful for a specific
validation, migration, or emergency tracking reason.

```text
mission atelier-hy2i
  advances epic atelier-4p7q
    child task atelier-liqk
    child task atelier-qnxs
  advances epic atelier-a625
    child task atelier-oqtz
    child task atelier-qdaw
  advances validation issue atelier-mission-validation
```

In this shape, the mission carries objective scope and validation expectations,
epics group accountable work packages on reviewable branches, and child issues
execute as implementation slices under their epic. Mission status should count
each unique issue once even when a deliberate duplicate path exists, but
planners should avoid duplicate mission links by default because they make
completion state harder to scan.

## Workspace, Branch, And Review Boundaries

The default operating model separates three concerns:

- Mission: one shared worktree or background checkout for coordinated work.
- Epic: one reviewable branch or PR-equivalent changeset under that mission.
- Issue: one implementation, documentation, review, validation, migration, or
  artifact-update slice with local proof.

Per-issue worktrees and per-issue branches are exceptional isolation tools. Use
them for dirty or high-risk experiments, cross-epic conflicts, destructive
migration trials, or an explicitly assigned validation/review context. They are
not the default for every mutating subagent or every ordinary child issue.

Independent review moves to the epic by default. Ordinary implementation issues
close with their own proof, while epic completion maps child issue proof to the
parent outcome and records the review or validation judgment for the branch.

Workflow transitions may prepare review state through declared actions. For
example, an epic `request_review` transition can open or link the configured
review artifact after validators pass, then write the artifact link to the
branch-owning issue. That action prepares the review workspace only. Review
comments, approvals, change requests, finding resolution, and merge stay on
`atelier review`; issue status changes stay on `atelier issue transition`.

## Relationships

Use hierarchy for ownership and typed links for contribution, validation, and
workflow proof:

```text
mission advances issue
mission blocked_by issue
issue part_of epic
evidence validates issue
evidence validates review issue
evidence validates validation issue
workflow transition uses validator
validator evaluation produces evidence or a machine-readable result
```

Dependencies remain separate:

```text
issue blocks issue
artifact update blocks epic
validator failure blocks transition
```

Mission work and mission blockers are distinct. `mission advances issue` means
the issue or epic is part of the mission's execution/progress graph. `mission
blocked_by issue` means the issue, artifact update, or validation item is gating the
mission but is not necessarily ordinary mission scope.

## Readable Mission Records

Mission records are meant to be reviewed by operators and agents in normal
Markdown diffs. The product contract is not an escaped `data` object in YAML.
Mission front matter carries compact identity, lifecycle state, labels, and
typed relationships. Mission narrative, constraints, risks, validation
expectations, and terminal notes live in ordered Markdown sections:

```text
## Intent
## Constraints
## Risks
## Validation
## Notes
```

`Intent`, `Constraints`, `Risks`, and `Validation` are required. `Notes` is
optional. Linked work, blockers, evidence, and other supporting records are
typed links, not prose-only lists. Checkpoint or plan references are prose or
repository paths inside those sections, not v1 relationship tables. `atelier
mission show` and `atelier mission status` render mission work, blockers, and
evidence from canonical relationships. They count only `advances` issue links
as mission work and only `blocked_by` issue links as direct mission blockers;
other precise relations remain supporting records instead of broadening the
work queue.

This abbreviated escaped-JSON shape is rejected as an authoring contract:

```markdown
---
id: "atelier-tcmr"
data: "{\"constraints\":[\"Use sectioned issue Markdown.\"],\"risks\":[\"Large rework can sprawl.\"],\"validation\":[\"Completion requires evidence.\"],\"work\":[]}"
schema: "atelier.mission"
schema_version: 1
status: "ready"
title: "Repair CLI workflow rework and validation gaps"
---

Repair CLI workflow rework and validation gaps.
```

The abbreviated readable shape keeps the mission content where reviewers can see
it and keeps relationships typed:

```bash
atelier mission create "Repair CLI workflow rework and validation gaps" \
  --body "Repair the CLI workflow and validation gaps." \
  --constraint "Use sectioned issue Markdown." \
  --risk "Large rework can sprawl." \
  --validation "Completion requires linked work closed and validation evidence attached."
```

```markdown
---
id: "atelier-tcmr"
relationships:
  attachments: []
  blocks: []
  children: []
  relates:
  - kind: "issue"
    id: "atelier-gjaz"
    type: "advances"
schema: "atelier.mission"
schema_version: 1
status: "ready"
title: "Repair CLI workflow rework and validation gaps"
---

## Intent

Repair the CLI workflow and validation gaps.

## Constraints

- Use sectioned issue Markdown.

## Risks

- Large rework can sprawl.

## Validation

- Completion requires linked work closed and validation evidence attached.
```

The validating evidence itself is a separate evidence record linked back to the
mission with `role: validates`; it is not copied into the mission body.

## Agent Workflow

An agent tasked with a mission should be able to:

1. Read the mission for intent, constraints, current risks, validation
   expectations, and any checkpoint or plan prose.
2. Inspect linked epics, issues, and evidence to understand what has already
   been proven and what remains.
3. Select a ready issue or epic slice that advances the mission.
4. Use Atelier workflow surfaces to decide the next process step: inspect
   status, show the relevant issue, preview transitions, then follow the
   command and recovery guidance printed for the current item.
5. Leave enough evidence that another agent can verify what changed, which
   criteria it supports, and what remains.

`atelier status` is the normal current-work orientation surface. In a checkout,
current work is the set of canonical issue records in that checkout's tracked
`.atelier/` tree whose workflow status is `in_progress`. Static docs should
not tell an agent which review artifact, provider, branch action, merge action,
or completion command to use. That process guidance belongs to Atelier command
output and workflow policy for the current checkout.

Branch ownership, branch naming, base branch selection, and merge strategy are
Atelier workflow/configuration concerns. Commands that need branch context must
report the computed owner, target, and recovery path directly; agents should
not infer branch action from static work-model prose.

Completion behavior follows the owner boundary:

Atelier command output should name the workflow transition, tracker write,
review state, branch action, and recovery command that apply to the current
item. Static docs should not encode a second completion recipe beside the
configured workflow.

Close must be failure-atomic for durable workflow state. If the tracker commit,
merge, push, or configured integration step fails, the item must not appear
closed on the integration branch. The command should leave enough state for a
repair or retry command to explain which step failed.

Workflow transitions may declare transition actions. A transition action is
configured work run by the explicit issue transition after required fields and
validators pass. The transition is complete only after required actions and the
final canonical status write succeed; action output must name the affected
artifact, local record path when relevant, and retry or recovery command.

The v1 action set is limited to the built-in transition actions described by
workflow policy. Review artifact actions use the repository's configured review
mode to create, locate, or link the review artifact for the branch-owning issue
or epic, then write the canonical `review` field. They do not merge the
artifact, approve review, request changes, post comments, close hidden issues,
revive `pr` aliases, or provide a general automation-hook mechanism. Those
operations remain separate review or workflow commands.

Action failure handling must be explicit:

- Preflight failure stops before canonical status or review-link writes.
- Local write failure names the failed write and does not report transition
  success.
- External provider failure leaves the issue transition incomplete and names
  provider recovery or retry guidance.
- Retry is idempotent by reusing an existing matching review link or artifact;
  ambiguous provider state must route to an explicit link recovery path.

There is no separate durable active-pointer concept. If a worker stops without
changing the issue's durable workflow state, no extra cleanup command is
required. If the work state changed, the operator should record a note when
useful, inspect `atelier issue transition <id> --options`, and move the issue
to the next canonical workflow status instead of clearing hidden runtime state.
The former root abandon and repair cleanup flows have therefore been removed
rather than kept as target-state workflow guidance.

Different Git worktrees may legitimately show different current-work sets
because each worktree carries its own tracked `.atelier/` record copy on its
branch. Reconciliation happens through normal Git review and merge of the
canonical Markdown records, not by sharing runtime work-association rows across
checkouts. When more than one issue is `in_progress` in the same checkout,
`atelier status` and `atelier mission status` should render that set directly
rather than nominate one hidden active issue. Separate issue worktrees remain
exceptional containment for conflicting, dirty, high-risk, or explicitly
isolated slices.

`atelier worktree for-mission <mission-id>` creates or locates a mission
worktree using the configured path policy, rebuilds local SQLite state from
tracked `.atelier/` records, and reports the mission workspace association.
Explicit branch helpers such as `atelier branch for-epic <epic-id>` create or
locate reviewable branches for diagnostics, advanced repair, or manual
recovery. Routine worker guidance should come from `atelier status`, issue
detail, transition options, and any recovery text they print. Workflow-defined
general hooks are deferred in v1; transition actions are the narrow configured
integration points described by workflow policy.
`atelier worktree status` reports path, branch, dirty paths, ahead/behind when
an upstream exists, unpushed commit count, associated mission/epic/issue work,
and operator-facing health when available. `atelier worktree merge <id>`,
`atelier branch merge <id>`, and `atelier worktree remove <id>` are thin Git
wrappers for merging an associated branch and cleaning up the associated mission
worktree after branch review and cleanup are complete.

## Deferred Checkpoints And Validators

Checkpoint prose may describe validation criteria. Workflows own validators.

A mission, epic, issue, or evidence body may say:

```yaml
desired_state: "CLI surface is agent-native"
validation_criteria:
  - "Primary help only shows core commands"
  - "Legacy commands fail as unknown commands"
  - "Full test suite passes"
  - "CLI surface policy is documented"
```

A workflow may say:

```yaml
transitions:
  complete:
    validators:
      - validation.criteria_satisfied
      - blockers.none_open
      - evidence.attached
```

The validator does not define the checkpoint's meaning. It only enforces whether
the issue transition is allowed.

See [Deferred Checkpoint Semantics](milestone-records.md) for the v1 rule that
checkpoint data stays in accountable record prose and evidence.

## Current Representation

First-class mission, issue, evidence, workflow, and activity records make
objective, work, workflow, and proof relationships explicit. Epics and tasks
remain issue-shaped accountability records linked into that graph. Checkpoint
and plan data is prose or ordinary Markdown until a future contract introduces
new first-class records directly.
