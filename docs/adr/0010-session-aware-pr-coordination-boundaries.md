# ADR 0010: Session-Aware PR Coordination Boundaries

## Status

Accepted.

## Context

Atelier is adding durable session records, Forgejo pull request integration,
typed issue fields, and PR-backed workflow gates. These concepts can easily
collapse into older runtime session/current-work behavior or into a PR-driven
workflow where remote review actions move Atelier issues directly.

The product contract needs four boundaries to stay clear before implementation:

- sessions explain bounded work and handoff context;
- current work remains issue workflow state in the checkout;
- pull requests are review artifacts linked to issues; and
- workflow validators read PR state without performing PR actions.

Without a durable decision, implementation could recreate hidden active
pointers, overload evidence attachments as mutable PR state, or let `atelier pr`
commands transition issues as a side effect.

## Decision

1. Sessions are durable, optional coordination records.
   A session records role, linked work context, lifecycle, and bounded activity
   for handoff. It does not define current work, and routine issue execution
   does not require a session.

2. Current work remains workflow-derived.
   The current-work set is still the set of canonical issue records in the
   checkout whose workflow status is `in_progress`, interpreted with branch and
   mission context. Ending, deleting, or abandoning a session must not close,
   block, or unassign an issue.

3. Forgejo PRs are review artifacts.
   `atelier pr` commands may open, inspect, comment on, review, and summarize
   Forgejo pull requests for linked issues. They do not start, close, block, or
   otherwise transition Atelier workflow.

4. Forgejo sudo identity is command authorship, not Atelier authorship.
   Repository config may map Atelier roles to Forgejo users for sudo-mode PR
   operations. That mapping controls remote PR authorship and review identity;
   it does not replace Atelier evidence producers, activity actors, or session
   participants.

5. `forge_pr` is a typed issue field.
   The active PR link belongs in workflow-policy-owned typed issue fields, not
   in generic attachments or evidence payloads. Evidence remains the durable
   proof envelope for transcripts and validation results.

6. PR validators are read-only workflow gates.
   Validators such as `linked_pr_merged` inspect the configured `forge_pr`
   field and remote Forgejo state, then report pass/fail guidance. They do not
   mutate Forgejo, write PR comments, merge PRs, or change Atelier issue status.

## Alternatives Considered

### Use Sessions As The Current-Work Source

This would make a session feel like a claim or active pointer. It was rejected
because current work is already recoverable from committed issue workflow state,
while sessions are optional coordination metadata.

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
Forgejo operations, while sessions are handoff records and may be absent.

## Consequences

- Product docs and help must teach `session` as optional durable handoff
  metadata, not a replacement for `status`, `start`, or issue transitions.
- Product docs and help must teach `pr` as a review-artifact surface whose
  next steps point back to issue transition readiness.
- Workflow schema version 2 can define typed fields such as `forge_pr`; strict
  validation must reject unknown typed fields rather than accepting arbitrary
  JSON payloads.
- `linked_pr_merged` and related validators should produce actionable failure
  guidance naming the PR state to fix and the command surface that can inspect
  it.
- Evidence records remain the proof mechanism for validation transcripts,
  review summaries, and residual risk; they do not become the mutable PR state
  store.
