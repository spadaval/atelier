# Atelier Product Intent

Atelier is a verified, repo-native work tracker and an orchestrator's durable
memory. Missions, issues, evidence, and reviews live in Markdown beside
the code they describe. Git carries both the implementation and its done claim
through review and integration.

Agent Factory owns process: planning depth, assignments, independent review,
validation, and finding disposition. Atelier owns records, graph queries, and
mechanical guards at the point of action. The governing decision is
[ADR 0019](docs/adr/0019-verified-repo-native-tracker.md).

## Product principles

- Keep one durable source of truth: tracked records under `.atelier/`.
- Parse those records directly on each command. No database or query cache.
- Guard valid records, dependency closure, traceable active work, fresh honest
  evidence, independent approval, visible trivial exemptions, and dispositions
  for findings. Do not encode a complete agent choreography.
- Let agents commit records with the code. Atelier never creates commits.
- Make transitions record writes with read-only checks. Branch preparation and
  gated integration are explicit `start` and `merge` commands.
- Commit done claims on the work branch. Merge writes no tracker state, so the
  base branch sees completion together with the implementation.
- Evaluate merge gates from the base branch's reviewed policy.
- Keep review and evidence append-only and tied to full Git commit identities.
- Prefer concise command output and inspectable Markdown to a second UI or an
  external review service.

## Work and proof

A mission links its governing strategy and success criteria. An issue is an
executable slice, with a goal and observable done condition; an issue with
children is a parent, an increment that is one branch and review boundary. Native links
represent mission membership, hierarchy, blockers, and proof attachments.

Evidence reports `pass`, `fail`, `blocked`, or `not-applicable`, the commit
checked, the check performed, and its limits. A receipt's presence does not
prove success. Reviews record who examined which commit, their findings,
dispositions, and approval. Identity is self-declared; the guards prevent
routine mistakes rather than defend against malicious actors.

`status` orients a fresh session; `land` checks that the session can
be handed off with committed records and intact invariants. Harness hooks call
these commands. Cross-branch status makes in-flight work visible while keeping
each branch's records distinct.

## Boundaries

Atelier is a local CLI, not an agent runner, daemon, scheduler, review provider,
or process framework. Strategies and design documents remain ordinary files.
There are no transition actions, activity sidecars, review rooms, SQLite cache,
backup import/export path, or Mission Control TUI.

Terminal records can leave the active tree through explicit pruning; Git
history preserves their audit trail. Repository policy and Agent Factory decide
when work needs branches, independent review, validation, and cleanup.

Current contracts are in [product documentation](docs/product/index.md) and
[architecture documentation](docs/architecture/index.md). The
[rebuild strategy](docs/strategies/atelier-rebuild.md) tracks the migration;
older ADRs and dated audits preserve history rather than current behavior.
