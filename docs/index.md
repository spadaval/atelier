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
- [Agent Factory](../skills/agent-factory/SKILL.md): planning,
  implementation, review, validation, and handoff process. Its one copy is
  `skills/agent-factory` in this repository, with its evals in
  `evals/agent-factory` ([ADR 0029](adr/0029-agent-factory-lives-in-atelier.md));
  agents load the installed skill.
- [Rebuild strategy](strategies/atelier-rebuild.md): intent, criteria, scope,
  and authority of the rebuild, completed with mission
  [atelier-kgcr](../.atelier/issues/atelier-kgcr.md) on 2026-10-15. Open
  missions are listed by `atelier work missions`.
- [ADRs](adr/0019-verified-repo-native-tracker.md): decision history; consult each
  decision's status and the supersession lists of ADR 0019 and
  [ADR 0028](adr/0028-simpler-lifecycle.md#supersession) before applying older rules.
  [ADR 0030](adr/0030-a-read-only-site-a-session-database-and-t3-code.md)
  allows a read-only site for humans, a host-wide session database for
  attribution, and a dependency on T3 Code, with records in Git still the only
  source of truth for work.
  [ADR 0031](adr/0031-a-forge-carries-branches-and-ci.md) lets a forge carry
  branches and report CI, with review staying in the repository.
  [ADR 0032](adr/0032-the-desk-is-the-humans-product.md), proposed, makes
  the human's pages a product of their own, the Desk, and removes the site
  from `atelier`. Its intent is [product/desk.md](product/desk.md).
- [Development setup](product/development-setup.md): local tools and verification.
- [Repository contribution](product/repository-contribution.md): ownership and
  integration policy.

Dated [quality audits](architecture/quality/index.md), provenance notes, and
`docs/references/` preserve observations about earlier designs. They do not
replace the current guides.
