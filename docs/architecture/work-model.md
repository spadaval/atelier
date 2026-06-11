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
- Issue: the actual accountability unit for work. Tasks, bugs, decisions,
  reviews, validations, and closeouts are issue-shaped until a more specific
  first-class record exists.
- Workflow: the policy for how records move between states.
- Workflow validator: a transition check attached to workflow policy. A
  validator allows or rejects a transition and returns an actionable failure
  reason. Validators are not milestone fields.
- Evidence: durable proof that work, review, or validation happened. Evidence
  can validate an issue, epic, mission, or a specific milestone validation
  criterion.

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
evidence validates milestone.validation_criteria[N]
workflow transition uses validator
validator evaluation produces evidence or a machine-readable result
```

Dependencies remain separate:

```text
issue blocks issue
decision blocks epic
validator failure blocks transition
```

Mission work and mission blockers are distinct. `mission advances issue` means
the issue or epic is part of the mission's execution/progress graph. `mission
blocked_by issue` means the issue, decision, or validation item is gating the
mission but is not necessarily ordinary mission scope.

## Agent Workflow

An agent tasked with a mission should be able to:

1. Read the mission for intent, constraints, active milestones, current risks,
   and relevant plans.
2. Inspect the current milestone to understand the checkpoint state being
   pursued and the validation criteria that must eventually be proven.
3. Select a ready issue or epic slice that advances the mission and contributes
   to the milestone.
4. Follow the issue workflow: claim, implement or validate, record notes, attach
   evidence, and close only when validators allow the transition.
5. Leave enough evidence that another agent can verify what changed, which
   criteria it supports, and what remains.

`atelier worktree for <issue-id>` creates or locates a Git worktree using the
configured branch/path policy, rebuilds local SQLite state from
`.atelier-state/`, runs `worktree_setup` hooks from `atelier.workflow.yaml`, and
records the issue/branch/worktree association in local runtime state.
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
