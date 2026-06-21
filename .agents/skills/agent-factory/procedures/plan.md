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

## Handoff

Report items created or changed, dependency changes, unresolved choices, proof
expectations, validation or lint run, and follow-up artifact tasks.
