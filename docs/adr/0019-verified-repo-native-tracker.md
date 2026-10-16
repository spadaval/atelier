# ADR 0019: Verified Repo-Native Tracker

## Status

Accepted (2026-09-30, human-directed).

Supersedes [ADR 0006](0006-agent-guidance-ownership-boundary.md),
[ADR 0007](0007-mission-workspaces-and-epic-review-branches.md),
[ADR 0010](0010-session-aware-pr-coordination-boundaries.md),
[ADR 0011](0011-native-review-modes-and-room-authority.md),
[ADR 0012](0012-explicit-workflow-transition-effects.md),
[ADR 0013](0013-workflow-transition-actions-and-branching.md),
[ADR 0014](0014-status-role-attribution-replaces-sessions.md), and
[ADR 0017](0017-sqlite-domain-cache-and-hard-removal.md).
Amends [ADR 0004](0004-work-lock-sync-policy.md),
[ADR 0005](0005-repo-owned-issue-workflow-state.md),
[ADR 0015](0015-missions-are-declared-workflow-policy.md), and
[ADR 0016](0016-canonical-work-branches-and-mission-integration.md) where they
describe transition actions, review providers, or the cache.
[ADR 0018](0018-strategy-draft-epics-and-adaptive-expansion.md) stands, except
that strategy is linked by path without a revision number.

## Context

Atelier was designed to make agents follow a process by owning that process:
role guidance, a transition engine with validators and side-effecting actions
(branch preparation, tracker commits, push, provider review, merge, sync), a
Forgejo provider and native review rooms, a SQLite domain cache, and
per-transition activity files.

Agent Factory has since been rewritten to own process directly and to scale
ceremony with risk. Measured against it, Atelier's machinery:

- produced tracker-only commits for about 80% of commits since July 2026;
- merged non-trivial code to `master` without independent review, because
  ordinary task close integrated locally;
- normalized every evidence result to "recorded" and counted manual evidence
  as passing, so gates checked presence rather than truth;
- needed atomic multi-step actions mainly because merges wrote tracker state;
- kept a cache that is slower when warm (250–310 ms per command) than parsing
  every record directly (62 ms for 973 issues).

The repo-native record model, the work graph, and command-captured evidence
remain valuable and are not available in hosted trackers.

## Decision

1. **Ownership.** Agent Factory owns process. Atelier owns durable records,
   graph queries, and mechanical guards for a small set of invariants. It
   surfaces guidance at the point of action but does not define the process.

2. **Invariants, not choreography.** Atelier enforces only rules that protect
   an invariant: valid records and a consistent graph; nothing done while its
   dependencies are open; traceable active work; evidence tied to a commit
   with an explicit result; a current approval from a non-author before merge;
   an explicit, visible trivial exemption; a disposition for every finding;
   cleanup of branches created for the work. Statuses, transitions, graph
   shape, and planning depth are agent-owned.

3. **Transitions write records and run read-only checks.** There are no
   transition actions. Side effects live in explicit commands: `atelier start`
   prepares the owner branch and records it; `atelier merge` checks the gates
   and merges. Atelier never creates commits; agents commit records with the
   code they describe.

4. **Done claims travel with the code.** The done claim is committed on the
   work branch and reviewed with the code. Merging writes no tracker state, so
   the base branch shows an item done exactly when its code is merged.
   Merge-time facts are derived from Git. Post-approval commits that only add
   review or evidence records or change in-scope statuses do not make an
   approval stale; changes to code, policy, or acceptance criteria do. A
   clean merge of the base into the branch does not (atelier-n7g1). Every
   change is checked when it lands, so history that passed the gates holds by
   induction: `land` checks only the claims and review history that changed
   since the branch base or the upstream, or every claim after a policy
   change, and `atelier doctor` checks everything (atelier-37uq).

5. **Policy is code.** `.atelier/workflow.yaml` is reviewed like any other
   change. Gates are evaluated from the base branch's policy, so a branch
   cannot loosen the gates it is merged under. Invalid policy falls back to
   the last valid policy with a visible error.

6. **Reviews are in-repo records.** Reviews, approvals pinned to a commit, and
   findings with dispositions are append-only Markdown records. There is no
   external review provider. Identity is self-declared.

7. **No cache.** Commands parse record files directly. Terminal records leave
   the active tree and remain in Git history.

## Consequences

- The Forgejo provider, review rooms, transition actions, most validators,
  the SQLite crate, activity files, `history`, and `man` are removed with no
  compatibility window.
- Product docs, `PRODUCT_INTENT.md`, and `CONTEXT.md` must be rewritten to
  this model; until then they describe the superseded design.
- Repositories using the old model need a one-time migration.
- Reading a PR on a provider's viewer is no longer integrated; reviews happen
  through Atelier records and plain Git.

## Alternatives Considered

- **GitHub or Forgejo Issues.** Rejected for now: tracker state would not be
  versioned with the code, branch-scoped plans and PR-visible roadmaps would
  be lost, and evidence would be unstructured comments.
- **Keep transition actions for convenience.** Rejected: their main purpose
  was atomicity across Git and the tracker, which decision 4 makes
  unnecessary, and partial failures were a recurring source of insane state.
- **Rewrite from scratch.** Rejected: the record store, IDs, locking, graph,
  and evidence capture are shared by both models and well tested.
- **Move storage to a database (as Beads moved to Dolt).** Rejected: it gives
  up PR-visible, diffable records, the property this project exists for.
