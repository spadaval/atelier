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
checks at merge, using this top-level YAML shape:

```yaml
branch_policy:
  base_branch: master
  merge_strategy: fast_forward_only
merge_checks:
  - shell.check:
      command: "git diff --check HEAD^ HEAD"
```

This is a fragment of the full workflow schema, not a complete policy.
`shell.check` is accepted only in `merge_checks`, never in transition validators.
It runs in the work checkout; a zero exit status passes and output explains the
result. Checks must preserve the pinned checkout and records. Git hooks are
disabled during integration and cleanup, so put mandatory checks in this
reviewed policy surface.

## Base authority

Merge evaluates the base branch's committed policy. Commit a valid policy on
the base before beginning governed work; without any valid base policy, merge
refuses. Editing the work branch's policy cannot loosen that branch's gate.
Invalid base policy emits visible diagnostics and falls back to its last valid
policy in first-parent history. The work branch's candidate policy must still
be valid; fallback does not excuse an invalid candidate. Ordinary commands can
read working policy and visibly fall back to committed history when it is
malformed. Fix policy through reviewed work.

The supported gated integration strategy is `fast_forward_only`. Atelier never creates
commits. If the base has diverged, reconcile the work using ordinary Git and
renew any proof or approval made stale by the resulting changes.

Consult the workflow parser and command help for exact schema fields. See
[reviews and merge](reviews-and-merge.md) and
[record storage](../architecture/markdown-first-record-store.md) for the other
sides of this contract.
