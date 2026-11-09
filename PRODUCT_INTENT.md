# Atelier Product Intent

Atelier exists so that agents produce high-quality, correct code that solves
real problems, with as little red tape and overhead as possible. Nothing else
about Atelier is sacred, and this purpose governs every tradeoff.
[ADR 0028](docs/adr/0028-simpler-lifecycle.md) records it with the simpler
lifecycle that follows from it; the principles below that it supersedes
describe today's behaviour until the item that changes each lands.

Atelier is a verified, repo-native work tracker and an orchestrator's durable
memory. Missions, issues, evidence, and reviews live in Markdown beside
the code they describe. Git carries the implementation through review, and
merging it marks the work done, so done means merged.

Agent Factory owns process: planning depth, assignments, independent review,
validation, and finding disposition. Atelier owns records, graph queries, and
mechanical guards at the point of action. The governing decision is
[ADR 0019](docs/adr/0019-verified-repo-native-tracker.md).

## Product principles

- Add nothing that does not earn its place. A new gate or record type names
  the kind of defect it would have caught in real agent sessions, and a rule
  that only patches another rule's edge case is a reason to remove the first
  rule.
- Keep one durable source of truth: tracked records under `.atelier/`.
- Parse those records directly on each command. No database or query cache.
- Guard valid records, dependency closure, traceable active work, a current
  independent approval of the code, visible trivial exemptions, and a decision
  for each deferred or dismissed finding; the approval after a `fix_now`
  finding's fix resolves it. No gate reads evidence: a receipt is a note tied
  to a commit with an explicit result. A reviewer's identity is recorded, not
  compared with the work's Git authors. Valid records are guarded when they
  are written: every
  command that writes records refuses, before writing anything, records that
  would fail a rule `atelier check` applies, so an invalid record does not
  block later commands. Do not encode a complete agent choreography.
- Let agents commit records with the code. Atelier creates one commit: the
  merge commit `merge` integrates reviewed work with, which also marks that
  work done.
- Make transitions record writes with read-only checks. Branch preparation and
  gated integration are explicit `start` and `merge` commands for an issue's
  work. A change no issue owns, such as a release, docs or a mockup, needs no
  `start`: it is reviewed and merged on an independent approval of its branch
  head, under the same merge checks, and writes no issue state
  ([ADR 0028](docs/adr/0028-simpler-lifecycle.md), decision 6).
- Enforce one order where each step happens: `review request` refuses while a
  Done when line is unticked or the Report is empty, and `merge` marks the
  owner's open work done in its merge commit, so the base branch sees
  completion together with the implementation and no close step can come in
  the wrong place ([ADR 0028](docs/adr/0028-simpler-lifecycle.md), decision
  2).
- Evaluate merge gates from the base branch's reviewed policy.
- Keep review records append-only once they reach the base branch, approvals
  and trivial exemptions from when they are first committed, and review and
  evidence tied to full Git commit identities. Until a finding or another
  review record reaches the base, any agent on the branch may correct or
  delete it, each change judged against the base as it stood where the
  change joined it. On the base, the one exception is a finding's disposition, a
  field of the finding that is set and changed in place, with who set it and
  at which commit; Git history keeps earlier ones
  ([ADR 0025](docs/adr/0025-findings-carry-their-disposition.md)). A
  `fix_now` finding records none: the approval after its fix resolves it
  ([ADR 0028](docs/adr/0028-simpler-lifecycle.md) decision 5). Identity
  is self-declared, so an implementer could delete a reviewer's finding before
  merge and leave no decision on it; that cannot let unreviewed code merge,
  since approvals cannot be edited and one still has to cover the code
  ([ADR 0028](docs/adr/0028-simpler-lifecycle.md) decision 4).
- Prefer concise command output and inspectable Markdown to a second UI or an
  external review service. A static HTML review page, such as `debrief
  --html` writes, is a document a human reads, not a second UI: it holds no
  controls, and decisions are recorded with ordinary commands.

## Work and proof

A mission is a themed container for related work, with an outcome and, when
it needs them, a governing strategy and success criteria its own work can
meet. An issue is an
executable slice, with a goal and observable done condition; an issue with
children is a parent, an increment that is one branch and review boundary. Native links
represent mission membership, hierarchy, blockers, and proof attachments.

Evidence reports `pass`, `fail`, `blocked`, or `not-applicable`, the commit
checked, the check performed, and its limits. A receipt's presence does not
prove success. Reviews record who examined which commit, their findings,
with a decision for each one deferred or dismissed, and approval. A fix of a
`fix_now` finding is reviewed again before merge, and the approval of that
re-review is what resolves the finding: it may cover only the fix's diff,
and the reviewer who raised the finding may do it. Identity is self-declared;
the guards prevent routine mistakes rather than defend against malicious
actors.

`status` orients a fresh session; `land` reports whether the session can be
handed off with committed records and intact invariants, and never refuses
work in progress on a branch: it refuses only on the base, where unreviewed
code would reach it. Harness hooks call
these commands. Cross-branch status makes in-flight work visible while keeping
each branch's records distinct.

## Boundaries

Atelier is a local CLI, not an agent runner, daemon, scheduler, review provider,
or process framework. Reviews, findings and approvals are records in the
repository, never state on a forge. When a repository's policy names a forge,
Atelier may push branches and the base, open pull requests and read CI
results for a commit through it. A policy that names none gets none of these
forge operations, while its own `shell.check`s and explicit commands such as
`land --push` keep their behaviour
([ADR 0031](docs/adr/0031-a-forge-carries-branches-and-ci.md)).
Strategies and design documents remain ordinary files.
There are no transition actions, activity sidecars, review rooms, SQLite cache,
backup import/export path, or Mission Control TUI.

Terminal records can leave the active tree through explicit pruning; Git
history preserves their audit trail. Repository policy and Agent Factory decide
when work needs branches, independent review, validation, and cleanup.

Current contracts are in [product documentation](docs/product/index.md) and
[architecture documentation](docs/architecture/index.md). The
[rebuild strategy](docs/strategies/atelier-rebuild.md) tracks the migration;
older ADRs and dated audits preserve history rather than current behavior.
