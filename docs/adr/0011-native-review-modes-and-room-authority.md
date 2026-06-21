# ADR 0011: Native Review Modes And Room Authority

## Status

Accepted. Amended by
[ADR 0013](0013-workflow-transition-actions-and-branching.md), which renames
workflow transition effects to transition actions.

## Context

ADR 0010 defined provider-backed pull requests as review artifacts and kept
Atelier workflow transitions separate from provider actions. The next product
step adds native review rooms while retaining Forgejo-backed review artifacts.
Without a new decision, implementation could keep `atelier pr` as the public
surface, accept both native and provider review state in one project, or store
room summaries that drift from their event history.

## Decision

1. Projects configure exactly one review mode.
   `.atelier/config.toml` selects `review.mode = "room"` for native review
   rooms or `review.mode = "provider"` for hosted PR-equivalent review
   artifacts. The modes are mutually exclusive. Forgejo remains the only
   provider implemented in this mission.

2. The public command surface is `atelier review`.
   `atelier review open/status/show/comments/comment/approve/request-changes/resolve/merge`
   operate in the configured mode. `link` is provider-only because native rooms
   are created by Atelier. The old `atelier pr` command surface is removed
   instead of kept as an alias.

3. Native rooms are canonical tracked records.
   A room is stored under `.atelier/reviews/<id>.yaml`. Current room state is
   derived from metadata plus ordered events, including comments, findings,
   approvals, change requests, resolutions, stale-approval invalidations, and
   merge. Projections may index derived state, but they must not become a
   second mutable room snapshot.

4. Issues link review artifacts through `review`.
   The canonical issue field is a structured `review` object. In room mode it
   stores `{ kind: room, id: <room-id> }`. In provider mode it stores
   `{ kind: pull_request, provider: forgejo, number: <positive integer> }`.
   Child issues inherit the nearest parent epic's review field. Legacy
   top-level `pull_request` fields are migrated to `review` and then rejected.

5. Review merge is not workflow transition authority.
   `atelier review merge` is the review artifact merge boundary. In room mode
   it enforces room approval freshness and unresolved blocking findings before
   recording a merge event. In provider mode it delegates merge or merge
   confirmation to the configured provider. It never starts, closes, blocks, or
   otherwise transitions Atelier issues. Workflow transitions continue to use
   explicit `atelier issue` commands, including objective issue transitions.

6. Workflow transition actions may prepare review artifacts.
   A successful explicit issue transition may declare a bounded action that
   opens or links the configured review artifact and writes the branch owner's
   `review` field. That setup action uses the active review mode but does not
   approve, comment, request changes, resolve findings, merge, or transition
   issues outside the declaring workflow transition.

## Consequences

- Product docs and help must describe review rooms and provider-backed review
  artifacts through `review` terminology, not active `pr` guidance.
- Workflow and record validation must reject projects that configure both room
  and provider review modes.
- Validation can prove provider parity by using the `review` commands without
  keeping `pr` compatibility shims.
- Room projections must be rebuildable from canonical YAML and event order.
- A future GitHub or GitLab provider requires a separate provider decision; it
  is not implied by the provider mode introduced here.
- Workflow action implementations must preserve this review boundary: they can
  prepare the configured artifact for a transition but cannot make review
  commands hidden issue workflow authority.

## Rejected Alternatives

### Keep `atelier pr` As A Compatibility Alias

Rejected. The repository is still a WIP product, and the standing command
policy prefers hard removal over compatibility shims unless a human explicitly
asks for a transition window.

### Allow Room And Provider Modes Together

Rejected. Two active review backends would make review inheritance, merge
authority, and validator behavior ambiguous for the same issue graph.

### Store Mutable Room Snapshots Beside Events

Rejected. Duplicate snapshots would drift from the event timeline. Derived
indexes are acceptable only when they can be rebuilt from canonical room YAML.

### Model Native Rooms As PR-Style Inline Threads

Rejected for this mission. Native rooms need findings, decisions, comments,
approval freshness, and merge authority. Full provider-style line anchoring and
thread lifecycle would broaden the scope beyond the durable review contract.

### Add A General GitHub/GitLab Provider Abstraction Now

Rejected. Provider mode keeps the product concept open, but Forgejo remains the
only provider implemented and validated in this mission.
