# Work Model

Atelier separates intent, checkpoint state, work execution, workflow control, and
proof. These concepts should not collapse into one issue hierarchy.

## Concepts

- Mission: a long-running objective with intent, scope, constraints, current
  health, linked milestones, active epics, plans, risks, and evidence.
- Milestone: a validated checkpoint state inside a mission. A milestone defines
  desired state, scope boundaries, validation criteria, accepted evidence, and
  completion state. It is not a work container or super-epic.
- Epic: a coordinated work package. Epics group implementation, documentation,
  review, validation, and closeout tasks that deliver a coherent change.
- Issue: the actual accountability unit for work. Tasks, bugs, reviews,
  validations, closeouts, and artifact updates are issue-shaped until a more
  specific first-class record exists.
- Workflow: the policy for how records move between states.
- Workflow validator: a transition check attached to workflow policy. A
  validator allows or rejects a transition and returns an actionable failure
  reason. Validators are not milestone fields.
- Evidence: durable proof that accountable work, review, validation, or closeout
  happened. Normal evidence attaches to issue-shaped work because issues own
  accountability. Parent readiness is derived from linked implementation,
  review, validation, and closeout evidence rather than from direct proof pasted
  onto the parent objective.

## Evidence Records

Evidence records are structured proof envelopes, not free-form notes with an
optional attachment. The canonical record must expose enough metadata for a
later operator to answer what was proven, by whom, for which accountable work,
and what remains risky.

Required or expected fields are:

| Field | Contract |
| --- | --- |
| `id` | Canonical evidence ID. |
| `targets` | Accountable target IDs. Version 1 normally uses `issue/<id>` for implementation, review, validation, or closeout work. |
| `proof_scope` | The local Outcome line, parent validation criterion, workflow validator, audit row, or review claim being proven. |
| `kind` | Evidence type such as `test`, `validation`, `review`, `audit`, `transcript`, `artifact`, or `migration`. |
| `result` | `pass`, `fail`, `blocked`, `deferred`, or `not-applicable`. |
| `summary` | Human-readable result summary. |
| `commands` | Optional command transcript metadata: argv or rendered command, exit status, success flag, timestamp, and bounded stdout/stderr summaries. |
| `artifacts` | Optional repository paths, external URIs, sizes, and hashes when payloads are available. |
| `agent_identity` | Producer or validator identity, using the local agent identity when available. |
| `independence_level` | `implementer`, `peer`, `independent`, `closeout`, or `adversarial`. |
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
before closeout.

Durable issue notes are for handoff context, caveats, skipped optional checks,
and trivial proof that does not need a separate artifact. First-class evidence
records are required for non-trivial command transcripts, test results,
migration proof, process-policy changes, workflow validation, closeout audits,
and any `fail`, `blocked`, `deferred`, or `not-applicable` classification that
future workers must inspect.

Separate validation or closeout issues are required when the implementer should
not validate the claim alone: migrations, public command or API contracts,
docs/help parity, stale-test risk, cross-cutting workflow or persistence
behavior, Agent Factory process changes, and all epic or mission closeouts.

Missions are objectives, not work logs. A mission may retain direct evidence
links only for legacy imports, migration notes, or a final closeout artifact
whose accountable owner is also a linked closeout issue. Normal mission
readiness is computed from closed linked work, attached evidence on that work,
and the closeout audit that maps parent validation lines to the child proof.

The detailed routing table lives in
[Validation](../architecture/quality/validation.md). Product docs should point
to that router instead of defining a second proof model.

## Parent Coverage Summaries

Issue closeout reads evidence attached to that issue and checks whether the
issue's `Evidence` section has objective proof. Parent records summarize
coverage instead of owning every proof detail.

An epic coverage summary maps each epic Outcome line to child issues and their
attached evidence IDs. The summary classifies each line as `covered`,
`missing`, `failed`, `blocked`, `deferred`, or `not-applicable`, and names
residual risks plus follow-up IDs. It may use stable claim anchors for
automation-heavy, high-risk, or repeated audit work, but ordinary issues should
not need line-level claim plumbing.

A mission closeout summary maps mission validation expectations and linked epic
outcomes to implementation, review, validation, and closeout issues. The
mission derives readiness from closed linked work, coverage classifications,
attached evidence, current tracker health, and independent closeout validation.
Direct mission evidence is not the normal coverage source.

Existing prose evidence migrates as structured evidence with the original text
as `summary`, best-effort `kind` and `result`, the linked issue as `targets`,
`proof_scope: legacy-prose`, `independence_level: implementer` unless a
validator is clear, and a residual risk noting any missing command transcript,
artifact, or independent reviewer identity.

## Mission Sizing

Missions are goal records, not task records. A mission should describe the
desired end state and the durable context needed to coordinate work toward that
state: intent, constraints, risks, checkpoint milestones, plans, validation
expectations, and evidence.

A mission is large enough to require at least one epic. If the work can be
planned, claimed, implemented, validated, and closed as a single accountable
unit, it should remain an issue. If the work needs coordinated implementation,
review, validation, documentation, or closeout slices under a shared objective,
the shared objective should be a mission and the executable slices should live
under one or more epics or issues linked to that mission.

## Mission Graph Shape

Prefer a shallow mission graph: the mission links to epics, closeout issues, or
other root work that directly advances the objective, and those epics own their
executable child tasks. Ordinary child tasks should not also be direct mission
work unless the duplicate link is deliberate and useful for a specific
closeout, migration, or emergency tracking reason.

```text
mission atelier-hy2i
  advances epic atelier-4p7q
    child task atelier-liqk
    child task atelier-qnxs
  advances epic atelier-a625
    child task atelier-oqtz
    child task atelier-qdaw
  advances closeout issue atelier-mission-closeout
```

In this shape, the mission carries objective scope and validation expectations,
epics group accountable work packages, and child issues execute under their
epic. Mission status should count each unique issue once even when a deliberate
duplicate path exists, but planners should avoid duplicate mission links by
default because they make readiness and closeout harder to scan.

## Relationships

Use hierarchy for ownership and typed links for contribution, validation, and
workflow proof:

```text
mission has_checkpoint milestone
mission advances issue
mission blocked_by issue
epic contributes_to milestone
issue part_of epic
issue contributes_to milestone
evidence validates issue
evidence validates review issue
evidence validates validation issue
evidence validates closeout issue
evidence validates milestone.validation_criteria[N]
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
expectations, and closeout notes live in ordered Markdown sections:

```text
## Intent
## Constraints
## Risks
## Validation
## Closeout Notes
## Notes
```

`Intent`, `Constraints`, `Risks`, and `Validation` are required. `Closeout
Notes` and `Notes` are optional. Linked work, blockers, checkpoints, plans,
evidence, and other supporting records are typed links, not prose-only lists.
`atelier mission show` and `atelier mission status` render those links as
Linked Work, Mission Blockers, Evidence, Plans, and checkpoint sections. They
count only `advances` issue links as mission work and only `blocked_by` issue
links as direct mission blockers; other precise relations remain supporting
records instead of broadening the work queue.

This abbreviated escaped-JSON shape is rejected as an authoring contract:

```markdown
---
id: "atelier-tcmr"
data: "{\"constraints\":[\"Use sectioned issue Markdown.\"],\"risks\":[\"Large rework can sprawl.\"],\"validation\":[\"Closeout requires evidence.\"],\"work\":[]}"
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
  --validation "Closeout requires linked work closed and validation evidence attached."
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

- Closeout requires linked work closed and validation evidence attached.
```

The validating evidence itself is a separate evidence record linked back to the
mission with `role: validates`; it is not copied into the mission body.

## Agent Workflow

An agent tasked with a mission should be able to:

1. Read the mission for intent, constraints, active milestones, current risks,
   and relevant plans.
2. Inspect the current milestone to understand the checkpoint state being
   pursued and the validation criteria that must eventually be proven.
3. Select a ready issue or epic slice that advances the mission and contributes
   to the milestone.
4. Follow the issue workflow: start with `atelier start <issue-id>`, implement
   or validate, record notes, attach evidence, inspect transition options with
   `atelier issue transition <id> --options`, and close only when validators
   allow the transition.
5. Leave enough evidence that another agent can verify what changed, which
   criteria it supports, and what remains.

`atelier status` is the normal current-work orientation surface. Root `atelier
issue close <issue-id> --reason "..."` is the normal completion path for
tracked work, and `atelier abandon [issue-id] --reason "..."` clears only the
local active-work association without requiring operators to discover hidden
work lifecycle helpers.

`atelier worktree for <issue-id>` creates or locates a Git worktree using the
configured branch/path policy, rebuilds local SQLite state from
tracked `.atelier/` records, and records the issue/branch/worktree association
in local runtime state. Workflow-defined hooks are deferred in v1 and are not
part of the current worktree contract.
`atelier worktree status` reports path, branch, dirty paths, ahead/behind when
an upstream exists, unpushed commit count, associated work, and canonical export
freshness when available. `atelier worktree merge` and
`atelier worktree remove` are thin Git wrappers for merging an associated branch
and cleaning up the associated worktree.

## Milestones And Validators

Milestones own validation criteria. Workflows own validators.

A milestone may say:

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
      - required_validation_criteria_satisfied
      - no_open_blockers
      - evidence_records_present
```

The validator does not define the milestone's meaning. It only enforces whether
the transition is allowed.

See [Milestone Records](milestone-records.md) for the detailed field contract,
evidence relationship, and completion-state semantics.

## Current Representation

First-class mission and milestone records make objective, checkpoint, work,
workflow, and evidence relationships explicit. Epics and tasks remain
issue-shaped accountability records linked into that graph.
