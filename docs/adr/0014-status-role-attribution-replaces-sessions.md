# ADR 0014: Status Role Attribution Replaces Sessions

Status: Accepted
Date: 2026-06-20

## Context

Atelier briefly modeled worker, reviewer, and validator attempts as derived
sessions from issue activity. That kept PR role attribution and handoff context
separate from issue status, but it also introduced a second lifecycle beside
the workflow status graph.

The product direction is to remove magic and express behavior through tracked
configuration. Session/attempt semantics made active work harder to explain:
current work was status-derived, while role attribution was session-derived.

## Decision

Atelier removes sessions and attempts as product concepts. Active workflow
statuses may declare a `role` in `.atelier/workflow.yaml`. `atelier status` and
role guidance surface that role directly.

Mutating review commands resolve role in this order:

1. explicit `--role`;
2. linked owner issue current `status.role`;
3. failure with guidance to configure the status role or pass `--role`.

PR attribution stores the resolved role directly in `pr_attribution.role`.
Workflow action `review.open` keeps its explicit action role. Workflow action
`review.merge` uses the pre-transition issue status role and does not default
to `manager`.

## Consequences

- Current work and active role ownership are explained by the same issue status
  model.
- Role attribution is explicit policy, not hidden inference from activity
  history.
- There is no `atelier session` command, `.atelier/sessions` record kind, or
  activity `attempt` metadata.
- Historical session-oriented ADR 0010 is superseded for session/attempt
  modeling; its review-artifact boundary remains valid where not contradicted
  here.
