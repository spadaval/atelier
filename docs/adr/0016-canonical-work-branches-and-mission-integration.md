# ADR 0016: Canonical Work Branches And Mission Integration

Status: Accepted
Date: 2026-06-30

## Context

Mission integration branch work needs durable vocabulary before implementation.
Existing docs used overlapping terms such as base branch, integration branch,
owner branch, branch target, and branch templates. That made it unclear whether
the target branch for review and sync is a configured repository default, a
mission branch, an epic branch, or a value recomputed from the current graph.

The workflow contract also needs to preserve the mission model: mission scope is
the direct `advances` links plus descendants of those roots. Parent hierarchy
alone does not define mission execution scope.

## Decision

Atelier uses these terms:

- `base_branch`: the named integration branch in `branch_policy.base_branch`.
  It is the repository default when no narrower recorded branch base applies.
- Work branch: the branch that owns mutation and review for a branch-owning
  record.
- Branch base: the recorded branch/ref and commit from which a work branch was
  prepared. Review target validation, sync, and integration use this recorded
  value rather than recomputing a target from current hierarchy.
- Mission integration branch: an opt-in work branch for a mission objective.
  It exists only when workflow validators and actions prepare or integrate it.

Work branch names are canonical and not template-driven:

```text
<issue_type>/<issue_id>
```

Examples are `mission/atelier-k7mq`, `epic/atelier-4p7q`, and
`task/atelier-z1p8`. The issue type is the workflow registry key and the issue
ID is the canonical record ID. Slugs, issue-title fragments, and configurable
branch templates are not part of the target contract.

Mission integration branches are workflow-owned, not implicit. A mission
workflow may opt in by declaring Git validators and branch actions such as
`git.prepare_branch`; without those declarations, a mission remains an objective
record and workspace boundary, not a hidden branch lifecycle.

Projection freshness is internal command-storage health. Commands may repair or
diagnose stale projections, but workflow policy must not expose projection
freshness as a configurable validator.

Starter workflow names are domain names: `mission`, `epic`, `task`,
`validation`, and `spike`. Delivery-suffixed names such as `mission_delivery`,
`epic_delivery`, and `spike_review` are rejected for target-state examples and
defaults.

## Consequences

- Product docs and schema examples must use canonical `<issue_type>/<issue_id>`
  branch names.
- Review and sync implementation must persist and read branch base context for
  workflow branches.
- Mission closeout and mission status continue to derive scope from direct
  `advances` roots plus descendants, not from parent hierarchy.
- Workflow validators stay focused on durable facts such as evidence, blockers,
  lint, Git state, and review state; projection freshness remains a command
  health concern.

## Rejected Alternatives

### Configurable Branch Templates

Rejected. Templates such as `epic/{{ issue.id }}` and slugged branch names add
policy flexibility before the workflow engine has branch-base persistence,
review retargeting, and repair behavior. Canonical names are easier to inspect,
validate, and recover.

### Implicit Mission Branches

Rejected. A mission branch that appears without workflow declarations would
hide branch lifecycle behind mission status. Mission integration branches must
be visible in transition validators, actions, and recovery output.

### Recompute Review Targets From Parent Hierarchy

Rejected. Parent relationships can change while work is in progress. The branch
base recorded when the branch is prepared is the target for later review, sync,
and integration checks.

### Use Parent Hierarchy As Mission Scope

Rejected. Mission execution scope is the direct `advances` links plus
descendants. Parent hierarchy remains structural context and must not silently
add or remove mission work.

### Workflow Validator For Projection Freshness

Rejected. Projection freshness is rebuildable local state, not durable workflow
policy. Exposing it as a validator would make repository policy depend on a
machine-local cache implementation detail.
