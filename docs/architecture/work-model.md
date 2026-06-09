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

## Relationships

Use hierarchy for ownership and typed links for contribution, validation, and
workflow proof:

```text
mission has_checkpoint milestone
epic advances mission
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

## Milestones And Validators

Milestones own validation criteria. Workflows own validators.

A milestone may say:

```yaml
desired_state: "CLI surface is agent-native"
validation_criteria:
  - "Primary help only shows core commands"
  - "Legacy commands remain callable as hidden compatibility paths"
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
evidence relationship, completion-state semantics, and compatibility boundary
for the inherited `atelier milestone` command.

## Current Compatibility

Today, most missions, milestones, and epics are represented as issues with
labels and parent/blocking links. The inherited `atelier milestone` command is a
SQLite-backed compatibility surface and should not define the target milestone
model. First-class mission and milestone records should preserve existing issue
compatibility while making objective, checkpoint, work, workflow, and evidence
relationships explicit.
