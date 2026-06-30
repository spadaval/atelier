# ADR 0013: Workflow Transition Actions And Branching

## Status

Accepted. Supersedes
[ADR 0012](0012-explicit-workflow-transition-effects.md). Updated by
[ADR 0016](0016-canonical-work-branches-and-mission-integration.md) for
canonical work branch names, branch base terminology, and opt-in mission
integration branches.

## Context

ADR 0012 introduced explicit transition effects to keep review setup, branch
work, and workflow mutation out of hidden review-command side effects and broad
automation hooks. That direction was correct, but the term `effects` makes the
contract sound incidental and implementation-local. It also left branch policy
easy to split into a separate lifecycle system, such as `epic.branch.owner:
self` fields, mission-only branch rules, or command-specific branch recipes.

The workflow policy now needs one durable model that operators can preview from
`atelier issue transition <id>`: the transition's validators explain
readiness, its actions explain the bounded work that will happen, and branch
context is computed from the work graph plus repository branch policy.

## Decision

1. Transition actions replace transition effects.
   The workflow schema uses `actions` for configured transition work. The old
   `effects` key, effect terminology, and effect aliases are not part of the
   target contract unless a later human-approved migration explicitly creates a
   compatibility window. An action is not a generic extension point; it is one
   of Atelier's built-in transition action families with validated parameters.

2. Validators and actions have separate authority.
   Validators are read-only checks that decide whether a transition may
   proceed. Actions run only after the source status, required fields, and
   validators pass. Static transition descriptions remain guidance text; they
   do not mutate records, branches, review artifacts, or providers.

3. Branch operations are transition actions.
   Workflow branch policy defines the base branch and merge strategy, while
   work branch names use the canonical `<issue_type>/<issue_id>` form. The
   workflow engine derives the owner branch from the work graph: child issues
   use the nearest parent epic branch, standalone issues use an issue-type
   branch, epics use an epic branch, and missions use a mission integration
   branch only when configured actions opt in. Branch mutation is still
   explicit transition work, not a separate branch lifecycle surface: actions
   prepare an owner branch when needed, commit transition tracker state on that
   owner branch, or integrate the owner branch to its recorded branch base when
   the declaring transition says to do so.

4. The v1 built-in action families are exact and closed.
   Version 1 recognizes only these configured action families:
   - `git.prepare_branch`: create or check out the workflow-derived owner branch.
   - `tracker.commit`: commit the transition's canonical tracker changes on the
     workflow-derived owner branch.
   - `git.push`: push the workflow-derived owner branch to the configured
     review provider remote.
   - `review.merge`: ask the active review authority to merge or record merge
     completion for the branch owner's review artifact.
   - `git.sync`: synchronize the local base branch after provider-owned merge
     completion.
   - `branch_integrate`: integrate the owner branch to its recorded branch base
     using the configured merge strategy for local review-room workflows only.
   - `review.open`: open or reuse the branch owner's configured review
     artifact and write the canonical `review` field.

5. Minimal workflow-engine behavior remains intrinsic.
   The workflow engine intrinsically loads the strict `.atelier/workflow.yaml`
   policy, selects the workflow for the issue type, checks source status and
   required fields, evaluates validators, computes branch context from
   `branch_policy` and the work graph, plans the ordered action list, applies
   the canonical issue status transition and transition activity record, and
   reports failure or retry guidance. Provider mutation, review artifact setup,
   Git branch preparation, branch commits, and branch integration happen only
   through declared actions.

6. Actions must be previewable, idempotent, and recoverable.
   Transition option output names planned actions separately from validators.
   A failed action reports the action family, preserved state, affected branch
   or review artifact when known, and the next inspection or retry command.
   Retrying a transition must tolerate an already-created branch, already-made
   branch commit, already-created review artifact, or already-written review
   link when the state matches the requested action.

## Consequences

- Docs, help, schema examples, and implementation work should use `actions`
  terminology for target workflow policy.
- ADR 0012 remains historical context only; it must not be cited as the live
  schema contract.
- Branch setup, commit, and integration behavior is visible in transition
  planning output instead of being inferred from static docs or command names.
- Review commands remain review-artifact operations. Issue workflow authority
  stays behind explicit workflow transition commands and configured actions.
- Implementation slices must reject unknown action families, obsolete `effects`
  keys, broad hook definitions, and generic capability declarations.

## Rejected Alternatives

### Keep Generic Capabilities

Rejected. A generic `capabilities`, plugin, shell-command, or provider-action
registry would make the workflow schema an automation platform before Atelier
has a bounded recovery model. Version 1 needs a closed action set whose
mutation surfaces can be previewed, validated, retried, and tested directly.

### Keep A Separate Branch Policy Lifecycle

Rejected. A separate branch lifecycle section or command cookbook would make
operators infer which branch action applies after a transition has already been
selected. Branch names and merge strategy belong in branch policy, but branch
mutation belongs in declared transition actions that appear in transition
planning output.

### Make Missions The Only Branch Boundary

Rejected. Missions own shared worktrees, not the reviewable branch boundary.
Epics are the normal branch and review boundary beneath a mission, standalone
issues may own issue branches, and ordinary child issues contribute proof on
the parent epic branch.

### Add Arbitrary Hooks

Rejected. Hooks would run outside the explicit transition contract and make it
harder to explain whether validators, review commands, provider calls, branch
operations, or issue status writes changed state. Future extension points need
a separate decision with concrete failure, preview, and recovery behavior.
