# ADR 0012: Transition Effects And Review Artifact Boundary

## Status

Accepted.

## Context

ADR 0011 defines review modes and keeps review merge separate from Atelier
workflow transitions. Workflow schema work now needs a way for an explicit issue
transition to perform small configured integration work, especially opening or
linking the review artifact when a transition moves work into review.

Without a durable boundary, implementation could make review commands transition
issues, make provider PR aliases return, hide issue close behind review merge,
or introduce broad workflow hooks before the transition contract is stable.

## Decision

1. A transition effect is part of an explicit issue transition.
   Effects run only after required fields and validators pass. The transition
   is successful only when required effects and the final canonical status write
   succeed. Effects are not background hooks or provider webhooks.

2. The v1 effect set is review artifact open/link only.
   `review_artifact` with `action: open` creates or locates the configured
   review artifact for the branch-owning issue or epic and writes the canonical
   `review` field. `review_artifact` with `action: link` accepts an existing
   configured-provider artifact identifier or URL and writes the same canonical
   link when workflow policy allows linking.

3. Review mode authority follows ADR 0011.
   In room mode, the effect creates or locates a native review room under
   `.atelier/reviews/<id>.yaml`. In provider mode, it opens or links the
   configured provider review artifact and stores the normalized structured
   `review` field. The effect does not create a second review backend.

4. Review commands remain artifact commands.
   `atelier review` may inspect, comment, approve, request changes, resolve, or
   merge review artifacts according to the configured mode. It must not start,
   close, block, or otherwise transition Atelier issues. Review merge can
   satisfy validators, but the operator still runs the explicit workflow
   transition.

5. Failure semantics are explicit.
   Effect preflight failure leaves canonical issue status and review links
   unchanged. Local write failure names the failed canonical write and does not
   claim transition success. External provider failure leaves the transition
   incomplete and prints provider recovery guidance. Retry is idempotent:
   matching existing links or artifacts are reused, while ambiguous provider
   state routes to explicit link recovery.

## Non-Goals

- Review merge, approval, change requests, comments, or finding resolution as
  transition effects.
- Hidden issue close, implicit workflow status changes, or review-command
  authority over issue workflow.
- Compatibility aliases for old `atelier pr` commands.
- Broad automation hooks, arbitrary shell hooks, or provider webhooks in v1.

## Consequences

- Workflow schema work must model transition effects directly rather than
  encoding review artifact behavior as validators, descriptions, or aliases.
- Transition planner and CLI rendering work must show effect preflight,
  execution, failure, retry, and recovery text in transition/status output.
- Review provider work can implement open/link for the configured mode without
  widening `atelier review` into workflow authority.
- Downstream implementation issues blocked by this contract are `atelier-68sm`
  for workflow schema effects and `atelier-wxj5` for review/transition
  documentation alignment. Execution-engine issues should consume the schema
  vocabulary established there.

## Rejected Alternatives

### Let Review Commands Move Issue Workflow

Rejected. That would collapse review artifact state into workflow authority and
contradict ADR 0011.

### Use Validators To Create Review Artifacts

Rejected. Validators decide whether a transition is allowed and should remain
read-oriented. Creating or linking a review artifact is transition work after
validators pass.

### Add General Workflow Hooks Now

Rejected. The current product need is a narrow review artifact open/link
contract. General hooks would introduce ordering, security, retry, and audit
semantics beyond this slice.
