---
created_at: "2026-10-17T01:44:18.384856120+00:00"
id: "atelier-k8h8"
issue_type: "mission"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-ihss"
    type: "advances"
  - kind: "issue"
    id: "atelier-kops"
    type: "advances"
  - kind: "issue"
    id: "atelier-tciw"
    type: "advances"
  - kind: "issue"
    id: "atelier-w56o"
    type: "advances"
  - kind: "issue"
    id: "atelier-ydv0"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Integrate parallel work branches without catch-up churn"
updated_at: "2026-10-17T05:22:38.506429195+00:00"
---

## Outcome

An orchestrator running several work branches at once can see which branches are ready to land, and what landing each one would take. It brings the base into a branch only when that branch is next, and can let `atelier merge` merge a clean branch into a base that has moved, after checking the merged tree. Records that many branches link to stop conflicting. Lockfile and other generated-file changes still get re-review and re-validation; this mission adds no path exemptions.

## Success criteria

- S1: `status` shows each waiting branch's readiness against the current base: commits behind, whether it merges cleanly or which files conflict, approval state, whether proof is at the branch head, and which waiting branches share files (atelier-tciw). Shown by tests.
- S2: The orchestrate guidance in Agent Factory says to update a waiting branch only when it is next to land, and to check generated files mechanically (atelier-w56o). Shown by the vendored and upstream skill text.
- S3: A base-policy merge strategy lets `atelier merge` merge a clean branch into the current base after `merge_checks` pass on the merged tree, keeping a clean merge's approval. Whether it becomes the default is the human's decision (atelier-ydv0). Shown by merge tests, including a lockfile that git merges cleanly but `cargo metadata --locked` rejects.
- S4: Linking new work to a shared mission or blocker leaves that record unchanged (atelier-ihss). Shown by a test.
- S5: An owned branch does not trust local base commits that no land has checked (atelier-kops). Shown by a land test.
- S6: In an orchestrated run with at least five parallel branches, catch-up merges average at most one per landed branch. The 2026-10-16 baseline is astromech-cmhv's 16 catch-up merges, half of them needing a re-review. Shown by a count from the run's git history, recorded as a dated note.

## Background

This mission comes from the 2026-10-16 transcript retrospective. In astromech, feature/astromech-cmhv merged main into itself 16 times while it waited 4.5 hours to land, and 8 of those merges needed a fresh independent review. The cause was fast-forward-only merging combined with updating every waiting branch after each landing. Clean base merges have kept approvals since atelier-n7g1, and record-only conflicts since atelier-1isk; the remaining churn is conflict resolutions and lockfile regeneration, which rightly need review. The model is a local GitHub: Atelier writes a commit only when merging. Mission integration branches (atelier-w0su) are related but deferred by the human.

## Notes

- 2026-10-17 [note] 2026-10-17 plan: the mission record's Outcome and S1-S6 carry the strategy (no separate file). Ready now: w56o (guidance), kops (land trust), ydv0 (merge strategy, fast_forward_only stays default; making it default is Sumeet's call), ihss (one-sided links; sequenced before atelier-dmh5, which changes the same storage). tciw stays backlog until atelier-ftqn (mission 4pzj) lands; this mission does not take ftqn. S6 needs a later orchestrated run of >=5 parallel branches and cannot be shown in this session. Items run in parallel worktrees; catch-up merges from this run are recorded as observations.
