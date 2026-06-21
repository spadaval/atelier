# ADR 0015: Missions Are Declared Workflow Policy

Status: Accepted
Date: 2026-06-21

## Context

Atelier needs mission-shaped objective work for long-running coordination, but
the old implementation made missions a special command and lifecycle universe.
Mission records had hardcoded status values, terminal checks, status output,
and close behavior outside the repository workflow policy. That duplicated the
issue workflow model and made it unclear which surface owned obligations such
as evidence, blockers, validation approval, branch state, and close readiness.

The target product principle is that Atelier hardcodes capabilities, safety
checks, and concise recovery hints. Repository workflow policy chooses which
obligations apply to a record type and transition.

## Decision

Mission behavior is represented as declared objective work under the issue and
workflow model. A repository that wants mission-shaped work declares the issue
type, statuses, transitions, validators, and actions in `.atelier/workflow.yaml`
and links accountable work with typed relationships such as `advances`,
`blocked_by`, and evidence `validates` links.

Atelier may retain built-in capabilities for objective summaries, graph
traversal, typed relationship rendering, validator evaluation, and migration of
legacy mission records. Those capabilities must not impose a hidden mission
lifecycle, mission-only close command, mission-only status gate, active mission
focus pointer, or universal evidence requirement.

The replacement command boundary is:

- `atelier status` orients the checkout and names active work.
- `atelier issue show <objective-id>` renders one objective record and its
  linked work, blockers, and proof.
- `atelier issue status <objective-id>` inspects readiness and terminal checks
  for one objective.
- Explicit issue browsing or inventory surfaces discover objective records.
- `atelier issue transition <objective-id> <transition>` applies the workflow
  policy declared for that objective type.

## Rejected Alternatives

- Keep a root `atelier mission` namespace. This preserves the old split between
  mission policy and issue policy, and it teaches operators to look outside the
  configured workflow for closeout rules.
- Keep mission-specific close and status logic in CLI command modules. This
  duplicates validator and transition behavior and makes repository policy a
  partial override instead of the authority.
- Keep an active mission focus pointer. Current work is already derived from
  committed issue status in the current checkout; a separate pointer would add
  cleanup and reconciliation semantics without owning durable work.
- Keep mandatory evidence as a universal mission or issue rule. Evidence is a
  useful validator capability, but workflow policy must decide where it is
  required.
- Keep compatibility aliases or fallback mission behavior during the cutover.
  They would leave two target states in the product and make command output
  harder to trust.

## Consequences

- Mission/objective obligations are visible in the same workflow policy that
  controls ordinary issue and epic transitions.
- Objective completion can use the same validator result and transition
  rendering path as other work.
- Legacy mission records need a direct migration or rebuild into the declared
  objective model.
- Documentation and role guidance should route operators to status, issue
  detail, issue status, issue transition options, and explicit browsing
  surfaces, not to hidden mission-specific lifecycle rules.
