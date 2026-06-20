# ADR 0006: Agent Guidance Ownership Boundary

## Status

Accepted.

## Context

Atelier currently uses Agent Factory as the primary agent entry point, with
more tactical repository guidance repeated or routed through surfaces such as
`atelier man <role>`, `atelier status`, mission status, workflow policy, and product
docs.

That split has become awkward. Agent Factory is useful because it carries
portable orchestration discipline: role separation, one-subskill delegation,
model-routing judgment, independent review and validation posture, and
coordinated mission execution. Atelier is useful because it is executable and
repo-owned: it can inspect the current checkout, active mission, active work,
ready queue, tracker freshness, workflow policy, and local runtime health
before deciding what guidance to show.

The repository needs one clear authority for tactical operator guidance.
Duplicating command cookbooks and workflow policy across Agent Factory prompts,
repository instructions, static docs, and executable CLI output creates drift.
It also makes repo-owned process changes harder to validate because some
important instructions live outside the repository.

## Decision

Atelier is the primary repo-owned operational entry point for this repository.
Agent Factory remains the portable orchestration discipline.

1. Atelier owns repository-specific executable guidance.
   Repo-specific command selection, workflow recovery, tracker state
   explanation, mission and issue drill-down, evidence routing, health checks,
   readiness cues, branch/review/provider routing, and closeout diagnostics
   belong in Atelier-owned surfaces:
   `atelier man <role>`, `atelier status`, mission and issue commands, workflow
   policy, product docs, validation docs, and command help.

2. Agent Factory owns portable coordination behavior.
   Agent Factory keeps guidance for role assignment, one-subskill delegation,
   subskill selection, model-routing rationale, when to delegate, independent
   review or validation posture, dissent preservation, and orchestration
   discipline that should travel across repositories.

3. Repository instructions identify source locations, not workflow policy.
   `AGENTS.md` identifies authoritative repository sources, tracker identity,
   durable versus runtime tracker state, and the small set of repository-specific
   constraints needed before invoking Atelier-owned guidance. It should not
   restate a full command or policy contract, and it should not decide whether
   a work item needs a review artifact, which provider to use, or which branch
   action closes the work.

4. Tactical guidance should be executable before it is copied into prompts.
   When a repo-specific instruction can depend on current tracker state,
   workflow policy, active mission, active work, or repository health, prefer an
   Atelier command surface over static Agent Factory text.

5. Role-scoped guidance belongs in Atelier when it helps operators.
   `atelier man <role>` is the repo-owned guide namespace for tactical role
   guidance. Valid roles are `worker`, `reviewer`, `validator`, `manager`, and
   `admin`.
   `manager` is the broad CLI role class for work coordination; `orchestrator`
   remains an Agent Factory agent type within that class.

## Consequences

- Agent onboarding starts from executable repository state instead of private
  process lore.
- Repo-owned process changes can be validated through CLI output, product docs,
  workflow policy, and tracker evidence.
- Agent Factory can stay smaller and more portable because it no longer needs
  to carry this repository's tactical command cookbook.
- Repository instructions should stay small as Atelier surfaces take over
  recurring tracker and workflow instructions.
- Agents get current process direction from Atelier command output rather than
  from static prompt lore, available Git remotes, or installed provider tools.
- Adding new context tools to Atelier is useful only when paired with removing
  duplicated tactical guidance from Agent Factory or static repository docs.

## Tradeoffs

- Moving more guidance into Atelier increases responsibility for CLI help,
  product docs, and command-surface tests.
- Keeping Agent Factory means there are still two layers, but their ownership
  is distinct: Agent Factory coordinates agents, while Atelier explains this
  repository's live operating state.
- Role-scoped CLI guidance can make Atelier more valuable, but it must stay
  concise. `atelier man <role>` filters existing commands for the operator's
  job; it does not create role-prefixed command namespaces.
