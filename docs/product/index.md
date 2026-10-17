# Product Guide

Atelier keeps records and proof in Git and checks invariants. Agent Factory owns
how agents plan, execute, review, and validate work. See
[Product Intent](../../PRODUCT_INTENT.md) and
[ADR 0019](../adr/0019-verified-repo-native-tracker.md).

- [Work model](work-model.md): records, graph relationships, ownership, and
  completion.
- [Mission authoring](mission-authoring.md): concise missions, parents, and slices.
- [CLI surface](cli-surface.md): command map and ordinary work sequence.
- [Evidence and validation](validation.md): honest results, commit identity,
  freshness, and proof limits.
- [Reviews and merge](reviews-and-merge.md): approvals, findings, exemptions,
  and gated integration.
- [Workflow configuration](workflow-configuration.md): reviewed repository
  policy and read-only transition checks.
- [Sessions and hooks](sessions-and-hooks.md): status, land, setup, and branch
  orientation.
- [Mission reviews](mission-reviews.md): `debrief` and its static HTML page.
- [Human CLI output](human-cli-output.md), [inventory and mission views](issue-inventory-and-mission-overview.md),
  and [work ordering](work-view-ordering.md): reading operational views.
- [Retention and prune](retention-and-prune.md): explicit cleanup and Git recovery.
- [Development setup](development-setup.md) and
  [repository contribution](repository-contribution.md): working on Atelier.

Command help is the exact argument reference. Use the repository's debug binary
while validating the rebuild; the installed binary can still represent the old
model until installation is deliberately updated.
