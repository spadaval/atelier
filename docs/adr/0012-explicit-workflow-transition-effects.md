# ADR 0012: Explicit Workflow Transition Effects

## Status

Superseded by
[ADR 0013](0013-workflow-transition-actions-and-branching.md), which replaces
transition effects with transition actions and makes branch operations explicit
workflow actions.

## Context

Atelier workflow transitions already own issue status changes, required fields,
validators, branch policy, and activity records. Review commands own review
artifacts in either native room mode or provider mode. The next product slice
needs an explicit way for a successful issue transition to do bounded work such
as opening or linking the configured review artifact, without letting
`atelier review` commands start, close, block, validate, or otherwise move
Atelier issue workflow as a hidden side effect.

Without a declared transition-effect contract, implementation could either hide
workflow mutations inside review commands or grow broad automation hooks that
are hard to preview, validate, and recover.

## Decision

1. Workflow policy may declare transition effects.
   Effects are inline transition entries in `.atelier/workflow.yaml`. They are
   parsed and validated with the transition, rendered in transition planning
   output, and run only by explicit `atelier issue transition` execution after
   required fields and validators pass.

2. Validators and effects have separate authority.
   Validators decide whether a transition may proceed and must not mutate
   provider or tracker state. Effects run after readiness succeeds and perform
   the bounded work declared by the transition. Static transition descriptions
   remain human guidance only.

3. Version 1 effects are intentionally small.
   The built-ins are:
   - `issue_status_write`: write canonical issue status and transition activity.
   - `owner_branch_commit`: commit tracker state on the workflow-derived owner branch.
   - `owner_branch_integrate`: integrate the owner branch to the configured base.
   - `review.open`: open or reuse the configured review artifact and
     write the branch owner's `review` field.

4. Review artifact effects respect ADR 0011.
   They use the configured review mode. In room mode they create or reuse a
   native room. In provider mode they create, fetch, or link the configured
   provider artifact. They do not approve, comment, request changes, resolve
   findings, merge, close issues, add `pr` aliases, or bypass workflow policy.

5. Failure and retry behavior must be inspectable.
   Transition preflight stops before mutation when required fields, source
   status, validators, review mode, or effect configuration is invalid. Local
   write, provider, commit, and integration failures report the failed effect,
   preserved state, and next inspection commands. Effects must be idempotent
   enough for retry after an already-created artifact, already-written link,
   already-applied status/activity entry, or already-made branch commit.

## Consequences

- `atelier issue transition <id> --options` must render validators and planned
  effects as separate sections.
- Successful transition output must name applied and skipped effects separately
  from validator results.
- Review commands remain review-artifact operations; issue status changes stay
  behind explicit issue transition commands.
- Schema, engine, CLI, docs, and validation work must implement declared
  effects directly rather than adding compatibility aliases, hidden fallbacks,
  or generic hook systems.

## Rejected Alternatives

### Let Review Commands Drive Issue Workflow

Rejected. This would hide workflow state changes behind review operations and
contradict ADR 0011's review merge boundary. Review state may satisfy
validators or be prepared by effects, but issue workflow authority remains
explicit.

### Add A General Automation Hook System

Rejected for v1. Hooks would broaden the contract beyond workflow policy and
make transition output harder to preview and recover. The product needs a small
declared effect set first.

### Keep PR Compatibility Aliases

Rejected. The current product policy prefers hard removal of old command
surfaces unless a human asks for a compatibility window. The review artifact
effects use `review` terminology only.
