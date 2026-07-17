# Repository Shape

Use this reference when installing Agent Factory or assessing whether a fresh
agent can orient, act, and validate work from durable repository sources.
Names may vary, but each responsibility needs one discoverable owner.

## Entry Map

A short root instruction file should point to, rather than duplicate:

- tracker identity and live workflow/status entry points;
- product intent and domain language;
- the documentation index;
- product behavior and operator contracts;
- architecture boundaries and dependency direction;
- architecture decision records (ADRs);
- code and quality standards;
- validation routing and executable checks; and
- durable versus ignored/rebuildable state.

When the repository supports mission planning, the tracker entry point must
also make its lifecycle capability discoverable: authored draft state,
revision-bound independent review, and the current ready state that gates
execution. A repository missing any of those capabilities is not fully
operable for that workflow; record the gap in its tracker instead of teaching a
parallel lifecycle here.

Keep this entry map compact. Repository-specific command cookbooks and product
policy belong in their owning executable help or documentation.

## Documentation Responsibilities

| Source | Owns |
| --- | --- |
| Product intent | The product's purpose, users, desired outcomes, and non-goals. |
| Domain language | Stable terms and distinctions that plans and code must use consistently. |
| Product docs | Observable behavior, operator workflows, public contracts, and user-facing semantics: what the product does. |
| Architecture docs | System boundaries, components, data ownership, dependency direction, and implementation constraints: how the product is shaped. |
| ADRs | Costly, surprising, or repeatedly contested decisions, including context, choice, consequences, and supersession state. |
| Quality standards | Language, code, testing, review, and repository conventions that apply across changes. |
| Validation router | Which checks prove which claims, where proof is recorded, independence triggers, and result classification. |
| Tracker | Durable scope, outcomes, dependencies, lifecycle state, and evidence links for current work. |

Product and architecture docs may cross-link, but they should not silently own
each other's contracts. ADRs explain why a durable choice was made; they do not
replace the current product or architecture description.

## State Boundary

Identify canonical tracked records separately from local projections, caches,
locks, logs, and generated state. Rebuildable state should be ignored and have
one documented repair owner. A fresh agent must not need private machine state
to discover current scope or the repository's governing decisions.

## Readiness Test

A fresh agent should be able to answer, from the entry map and linked sources:

1. What is this product trying to achieve?
2. What do its core terms mean?
3. Where is current work and its lifecycle state?
4. Which behavior is public, and which constraints are architectural?
5. Why were important non-obvious choices made?
6. Which standards apply, and what proves this change?
7. Which state is durable, and how is local derived state repaired?
8. For a mission draft, who may author, independently review, and begin
   execution, and where does the tracker report current approval and readiness?

A missing responsibility is a repository-readiness gap even when the code is
otherwise buildable. Record the gap in the repository's tracker instead of
embedding a private replacement inside Agent Factory.
