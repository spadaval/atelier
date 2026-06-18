# ADR 0010: Session-As-Issue-Events And PR Coordination Boundaries

## Status

Accepted.

## Context

Atelier is using derived session views, Forgejo pull request integration,
built-in pull request links, and PR-backed workflow gates. These concepts can easily
collapse into older runtime session/current-work behavior or into a PR-driven
workflow where remote review actions move Atelier issues directly.

The product contract needs four boundaries to stay clear before implementation:

- sessions are derived issue-scoped worker/reviewer/validator attempts from
  canonical issue activity and explain bounded work and handoff context;
- current work remains issue workflow state in the checkout;
- pull requests are review artifacts linked to issues or epics; and
- workflow validators read PR state without performing PR actions.

Without a durable decision, implementation could recreate hidden active
pointers, overload evidence attachments as mutable PR state, or let `atelier pr`
commands transition issues as a side effect.

## Decision

1. Sessions are derived issue activity views.
   A session view carries role, linked issue or epic context, lifecycle,
   serial, and bounded activity reconstructed from canonical issue events. It
   does not define current work, and routine issue execution does not require a
   session view to be created or mutated.

2. Current work remains workflow-derived.
   The current-work set is still the set of canonical issue records in the
   checkout whose workflow status is `in_progress`, interpreted with branch and
   mission context. Inspecting a derived session view must not close, block, or
   unassign an issue.

3. Forgejo PRs are review artifacts.
   `atelier pr` commands may open, inspect, comment on, review, and summarize
   Forgejo pull requests for linked issues. They do not start, close, block, or
   otherwise transition Atelier workflow.

4. Forgejo sudo identity is command authorship, not Atelier authorship.
   Repository config may map Atelier roles to Forgejo users for sudo-mode PR
   operations. That mapping controls remote PR authorship and review identity;
   it does not replace Atelier evidence producers, activity actors, or session
   attempts.

5. `pull_request` is a built-in issue artifact link.
   The active PR link belongs in the canonical `pull_request` field on the
   branch-owning issue or epic, not in generic attachments, evidence payloads,
   or a workflow-defined typed-field registry. Canonical storage is the
   normalized PR number. Forgejo host, owner, repo, and branch expectations are
   derived from project config and workflow branch policy.

6. PR validators are read-only workflow gates.
   Validators such as `linked_pr_merged` inspect the active `pull_request` link
   and remote Forgejo state, then report pass/fail guidance. They do not mutate
   Forgejo, write PR comments, merge PRs, or change Atelier issue status.

## Alternatives Considered

### Use Sessions As The Current-Work Source

This would make a session feel like a claim or active pointer. It was rejected
because current work is already recoverable from committed issue workflow state,
while sessions are derived issue-event views rather than workflow drivers.

### Store PR Links As Evidence Attachments

This would reuse the existing evidence graph. It was rejected because a PR link
is mutable review-artifact state that validators need to read repeatedly, while
evidence records prove claims with stable observations and transcripts.

### Let PR Commands Drive Issue Transitions

This would make a merged PR close an issue directly. It was rejected because
Atelier workflow transitions must remain explicit, inspectable, and governed by
repository workflow policy. PR state can satisfy a validator, but the operator
still runs the Atelier transition.

### Encode Forgejo Role Mapping In Sessions

This would couple remote authorship to whoever owns a local session. It was
rejected because role-to-user sudo mapping is repository configuration for
Forgejo operations, while sessions are derived inspection views and may be
absent.

## Consequences

- Product docs and help must teach `session` as a derived inspection surface
  over issue activity, not a replacement for `status`, `start`, or issue
  transitions.
- Product docs and help must teach `pr` as a review-artifact surface whose
  next steps point back to issue or epic transition readiness.
- Workflow schema version 3 does not define a top-level typed-field registry.
  Strict validation accepts the built-in `pull_request` field as a positive PR
  number and rejects unknown issue fields rather than accepting arbitrary JSON
  payloads.
- `linked_pr_merged` and related validators should produce actionable failure
  guidance naming the PR state to fix and the command surface that can inspect
  it.
- Evidence records remain the proof mechanism for validation transcripts,
  review summaries, and residual risk; they do not become the mutable PR state
  store.
