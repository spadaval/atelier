# Documentation Map

Start with [Product Intent](../PRODUCT_INTENT.md) and
[Domain Language](../CONTEXT.md). [ADR 0019](adr/0019-verified-repo-native-tracker.md)
defines the verified repo-native tracker and supersedes older decisions about
process ownership, review providers, transition actions, and the cache.

- [Product guide](product/index.md): work records, commands, evidence, review,
  workflow policy, sessions, and cleanup.
- [Architecture guide](architecture/index.md): durable storage, crate
  boundaries, invariant checks, and engineering standards.
- [Bundle input contract](spec/bundle/schema.md): one-shot authored graph input.
- [Agent Factory](../.agents/skills/agent-factory/SKILL.md): planning,
  implementation, review, validation, and handoff process.
- [Rebuild strategy](strategies/atelier-rebuild.md): governing intent, criteria,
  scope, and authority. [Native mission](../.atelier/issues/atelier-kgcr.md) owns
  current execution state.
- [ADRs](adr/0019-verified-repo-native-tracker.md): decision history; consult each
  decision's status and ADR 0019's supersession list before applying older rules.
- [Development setup](product/development-setup.md): local tools and verification.
- [Repository contribution](product/repository-contribution.md): ownership and
  integration policy.

Dated [quality audits](architecture/quality/index.md), provenance notes, and
`docs/references/` preserve observations about earlier designs. They do not
replace the current guides.
