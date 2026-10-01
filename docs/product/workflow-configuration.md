# Workflow Configuration

`.atelier/workflow.yaml` is reviewed repository policy. It declares issue types,
statuses and their categories, permitted transitions, branch base, and invariant
checks. Agent Factory owns process; this file must not become an agent
choreography or a way to evade review.

`.atelier/config.toml` contains project identity, the canonical state root,
prune defaults, and supported project settings. There is no review-provider
mode or global provider credential configuration.

## Status and transitions

Each issue type selects a workflow with an initial status, terminal statuses,
and named transitions. A transition declares its accepted source statuses,
target status, optional description, required fields such as `close_reason`,
and read-only checks. Categories support queries; the exact status remains the
stored lifecycle state.

The current repository has draft objectives, todo executable work, active
`in_progress` and `review`, blocked work, and terminal `done` or `superseded`.
Inspect actual policy and `atelier issue transition <id>` before executing a
transition; a todo category alone is not permission to start.

Transitions write the issue record after checks pass. They do not create
branches or commits, push, open review artifacts, or merge. `start` and `merge`
are explicit Git boundaries. Required close reasons are recorded with terminal
claims on the work branch.

## Invariant checks

Record-file validity is checked whenever a command loads the repository.
`atelier check` exposes record and graph problems. Transition checks include
parseable intent, lint with no blocking findings (`lint.none_blocking`), and
open direct or transitive blockers where policy selects them.

Merge gates protect terminal committed scope, fresh passing evidence,
independent current approval or a visible trivial exemption, finding
dispositions, and absence of open blockers. Policy can add read-only shell
checks. Checks must preserve honest results and identify the failed invariant.

## Base authority

Merge evaluates the base branch's committed policy. Editing the work branch's
policy cannot loosen that branch's merge gate. Invalid policy emits a visible
error and falls back to the last valid policy in the base's first-parent
history. Fix the invalid policy through reviewed work; fallback is not silent
acceptance of malformed configuration.

The supported integration strategy is `fast_forward`. Atelier never creates
commits. If the base has diverged, reconcile the work using ordinary Git and
renew any proof or approval made stale by the resulting changes.

Consult the workflow parser and command help for exact schema fields. See
[reviews and merge](reviews-and-merge.md) and
[record storage](../architecture/markdown-first-record-store.md) for the other
sides of this contract.
