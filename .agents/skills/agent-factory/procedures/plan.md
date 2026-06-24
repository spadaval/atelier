# Plan

Use this subskill to create, split, sequence, clarify, or repair durable work.
Planning decides what work exists, why it exists, what proves it, and what
blocks it. It is not the implementation procedure for a named code item.

## Stance

- Start from repository instructions, the tracker's current status surfaces, and
  the relevant product, architecture, ADR, and validation docs.
- In Atelier repositories, use `atelier man manager` for current command
  routing and focused `atelier issue status <objective-id>` or focused
  `show`/`list` commands for drill-down.
- Write work around desired finished state and observable proof. Avoid scripting
  private implementation steps unless that exact path is the decision being
  tracked.
- High-leverage product, architecture, persistence, security, migration,
  validation, or public-contract choices need durable artifact-update work.
  Block dependent implementation on those tasks.
- When planning work, include scoped cleanup or refactoring when adjacent code
  makes the intended change harder to understand or trust. Cleanup should be
  purposeful and bounded, not drive-by churn.
- Keep graph edits focused. Do not rewrite unrelated tracker areas while
  clarifying one workstream.

## Ready Work

An assignable item must be understandable without private chat history. It
names scope, non-scope, expected proof, dependencies, and the subskill when that
is not obvious. If the tracker has canonical sections or templates, use them
instead of inventing parallel formatting.

## Mission Creation

Create a mission only when the objective is larger than a single accountable
issue and needs at least one epic or equivalent workstream beneath it. Smaller
objectives should remain ordinary issues.

Before writing the mission, resolve the applicable source of truth:

- Product intent and target behavior from the repository product docs.
- Domain language from repository context docs.
- Current lifecycle, closeout, and command details from the tracker command
  surfaces.
- Architecture, ADR, and validation docs for decisions that affect contracts,
  persistence, workflow policy, public commands, or agent process.

If those sources conflict or leave an important product or architecture choice
open, create artifact-update work and block dependent implementation on it.
Do not bury unresolved decisions inside implementation tasks.

A mission must make the desired finished state concrete enough for another
agent to plan and validate without private context. Capture:

- The outcome the repository should have when the mission is complete.
- Constraints and explicit non-scope.
- Current risks or unknowns that could change sequencing.
- Concrete validation criteria.
- The linked epics, implementation issues, documentation work, validation work,
  migration work, review work, or audit work needed to prove the outcome.

Validation criteria must name observable proof, not broad confidence claims.
Use command output, rejected commands, help text, file content, tests, lint
checks, evidence records, screenshots, or other reproducible artifacts. A
criterion such as "the feature works end-to-end" is not ready unless it names
the scenario, command, file, or artifact that demonstrates the behavior.

For missions that touch public command behavior, workflow policy, storage or
migration contracts, agent guidance, validation rules, or multiple subsystems,
include explicit closeout coverage for an independent validation or audit issue.
Do not add audit work by rote to tiny missions; when omitting it, the mission
should still name the closeout proof that makes the omission reasonable.

## Handoff

Report items created or changed, dependency changes, unresolved choices, proof
expectations, validation or lint run, and follow-up artifact tasks.
