# Product

This map owns Atelier's user-visible behavior: workflow concepts, command
surface, human output, policy semantics, and Mission Control experience.
Implementation ownership, persistence boundaries, and local runtime mechanics
live in [Architecture](../architecture/index.md).

Product direction starts in [PRODUCT_INTENT.md](../../PRODUCT_INTENT.md), and
domain language lives in [CONTEXT.md](../../CONTEXT.md). Product docs refine
that direction into contracts that can be implemented and validated.

## Doctrine

- [Zen Of Atelier](zen.md): product principles for long-running
  semi-autonomous mission work, coordination, evidence, validation, and
  terminal checks.

## Workflow Model

- [Work Model](work-model.md): mission, deferred checkpoint semantics, epic,
  issue, workflow validator, and evidence relationships.
- [Validation](validation.md): product language for outcomes, proof, evidence,
  validation, terminal checks, and proof visibility in normal operator surfaces.
- [Milestone Records](milestone-records.md): deferred checkpoint semantics and
  the v1 rule that validation data stays on issues, epics, missions, and
  evidence.
- [Workflow Configuration Contract](workflow-configuration.md):
  fixed `.atelier/workflow.yaml` issue-policy path, schema, built-in validators,
  transition actions, strict errors, and starter workflow examples.
- [Work View Ordering](work-view-ordering.md): blocker-aware ordering and row
  state vocabulary for issue-like work queues.
- [Retention And Prune Policy](retention-and-prune.md): cleanup classes,
  protection rules, Git-history recovery, and explicit apply semantics.

## Interfaces

- [Development Setup](development-setup.md): required tools, fresh-checkout
  commands, local env/secrets policy, and devcontainer rationale.
- [Repository Contribution Policy](repository-contribution.md): contribution
  entry point, GitHub template classification, dependency update automation,
  and tracker-first ownership policy.
- [CLI Surface Tiers](cli-surface.md): public CLI core command surface, removed
  compatibility commands, and inherited utility disposition.
- [Command Audit](command-audit/index.md): role ownership, naming, documentation,
  output hierarchy, and role-specific guide proposal for each root command
  surface.
- [Human CLI Output](human-cli-output.md): human-readable CLI output grammar,
  formatter boundaries, color/width policy, and test expectations.
- [Mission Control TUI](mission-control-tui.md): Mission Control TUI projection
  dependencies, degradation rules, navigation model, mutation boundary, and
  fixture expectations.

## Boundary

Product docs may name implementation-backed concepts such as `ProjectionIndex`
or `RuntimeState` when the behavior is user-visible, but they should not define
storage ownership, database schema, module boundaries, or cache repair
algorithms. Those belong in architecture docs.
