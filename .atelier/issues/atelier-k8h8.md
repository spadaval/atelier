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
  - kind: "issue"
    id: "atelier-zqee"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-23T03:51:45.869560247+00:00"
status: "done"
title: "Run parallel work through review and merge without churn"
updated_at: "2026-10-23T03:51:45.869560247+00:00"
---

## Outcome

An orchestrator running several work branches at once can see which branches are ready to land, and what landing each one would take. It brings the base into a branch only when that branch is next, and can let `atelier merge` merge a clean branch into a base that has moved, after checking the merged tree. Records that many branches link to stop conflicting. Lockfile and other generated-file changes still get re-review and re-validation; this mission adds no path exemptions.

The rest of the cycle gets cheaper too. Approvals stay current across changes that do not touch reviewed code. Review effort matches the change. A step's records reach the base in one commit. Pushing and starting work cannot quietly go wrong, and merge refuses what land would refuse.

## Success criteria

- S1: `status` shows each waiting branch's readiness against the current base: commits behind, whether it merges cleanly or which files conflict, approval state, whether proof is at the branch head, and which waiting branches share files (atelier-ftqn, atelier-tciw). Shown by tests.
- S2: The orchestrate guidance in Agent Factory says to update a waiting branch only when it is next to land, and to check generated files mechanically (atelier-w56o). Shown by the vendored and upstream skill text.
- S3: A base-policy merge strategy lets `atelier merge` merge a clean branch into the current base after `merge_checks` pass on the merged tree, keeping a clean merge's approval. Whether it becomes the default is the human's decision (atelier-ydv0). Shown by merge tests, including a lockfile that git merges cleanly but `cargo metadata --locked` rejects.
- S4: Linking new work to a shared mission or blocker leaves that record unchanged (atelier-ihss). Shown by a test.
- S5: An owned branch does not trust local base commits that no land has checked (atelier-kops). Shown by a land test.
- S6: In an orchestrated run with at least five parallel branches, catch-up merges average at most one per landed branch. The 2026-10-16 baseline is astromech-cmhv's 16 catch-up merges, half of them needing a re-review. Shown by a count from the run's git history, recorded as a dated note.
- S7: An approval stays current when a follow-up is linked to its mission and across a close commit that touches only the item's record. A planning commit that adds an issue to a mission re-checks no done claim in that mission (atelier-iaas, atelier-n642, atelier-ss25). Shown by merge and land tests.
- S8: Review costs no more ceremony than the change needs: low-priority follow-ups can share one branch and one review, `atelier review request` writes the reviewer brief, and a follow-up records where it was found (atelier-u6f3, atelier-rc8p, atelier-tq1j). Review keeps one standard for code and prose. Shown by CLI tests and the Agent Factory review guidance.
- S9: A step's records reach the base in one commit, and a trivial change lands on the base with a visible exemption, with no issue or work branch (atelier-4ixu, atelier-mbrf). Shown by tests.
- S10: `land --push` pushes only after a passing land, and pushing warns about base commits made by another owner. `start` takes an explicit base from any checkout, refuses a branch whose type prefix does not match the issue, and warns when another live session holds the item (atelier-ecah, atelier-mvzi, atelier-yc4d, atelier-mhx0, atelier-81e5). Shown by CLI tests.
- S11: `merge` refuses a done claim that `land` will refuse, runs its checks in the merged checkout's own target directory, and cleans up merged-tree checkouts after an interrupted merge. Land's per-review history walks are batched, and find review records created inside a merge commit (atelier-uw4x, atelier-isyf, atelier-rcnk, atelier-7b6e, atelier-as7i). Shown by merge and land tests.

## Background

This mission comes from the 2026-10-16 transcript retrospective. In astromech, feature/astromech-cmhv merged main into itself 16 times while it waited 4.5 hours to land, and 8 of those merges needed a fresh independent review. The cause was fast-forward-only merging combined with updating every waiting branch after each landing. Clean base merges have kept approvals since atelier-n7g1, and record-only conflicts since atelier-1isk; the remaining churn is conflict resolutions and lockfile regeneration, which rightly need review. The model is a local GitHub: Atelier writes a commit only when merging. Mission integration branches (atelier-w0su) are related but deferred by the human; the item sits in this mission's backlog without a criterion.

On 2026-10-22 the human widened this mission to the whole review-and-land cycle. It took the 2026-10-17 retrospective's branch, review, merge, land and start items, which had no mission, so S7-S11 were added.

## Notes

- 2026-10-17 [note] 2026-10-17 plan: the mission record's Outcome and S1-S6 carry the strategy (no separate file). Ready now: w56o (guidance), kops (land trust), ydv0 (merge strategy, fast_forward_only stays default; making it default is Sumeet's call), ihss (one-sided links; sequenced before atelier-dmh5, which changes the same storage). tciw stays backlog until atelier-ftqn (mission 4pzj) lands; this mission does not take ftqn. S6 needs a later orchestrated run of >=5 parallel branches and cannot be shown in this session. Items run in parallel worktrees; catch-up merges from this run are recorded as observations.
- 2026-10-17 [note] 2026-10-17 observation from this run (not S6 proof: 4 mission branches plus about 8 other missions' items landing on master in parallel). Catch-up merges per landed branch: w56o 1 (records only), kops 1 (clean), ihss 2 (54e6e0b3 conflicted in commands/issue.rs against atelier-i2qq, and git's clean merge also broke a test asserting the old '[todo]' format, caught only by rerunning the suite; 029058e7 clean), ydv0 5 (fb1d42b4 made when I judged it next, but its re-review then found more FIX NOWs and ihss landed first; b39bd830 conflicted in merge.rs; 1d90c9f5, 83ca748c and 57a4cdca clean, the last two because master moved while the final approval and merge were running). 9 catch-ups for 4 landings (2.25 average); 7 were clean textual merges that merge_commit would have skipped, 2 needed resolution review. Lessons: treat a branch as next to land only after its review has no open FIX NOW; with fast_forward_only and busy parallel missions, master can move between approval and merge, so a records-only catch-up and an immediate merge is the fastest exit (the gates kept the approval and needed no fresh proof for a records-only merge).
- 2026-10-17 [note] 2026-10-17 handoff: w56o, kops, ihss, ydv0 and zqee (skill text for merge_commit) merged and pushed; skill changes are on spadaval/.agents main (07c8d1c, 451c0c9). S2, S3, S4, S5 shown by their items' tests and reviewed text. Open: S1 (atelier-tciw) waits on atelier-ftqn in mission 4pzj; S6 needs a later orchestrated run of >=5 parallel branches (this run's counts are in the earlier note). Decision for Sumeet: whether this repo (or the default) switches branch_policy.merge_strategy to merge_commit; it needs a merge_checks shell.check that runs real proof (e.g. the test suite) to be worth it. Follow-ups filed: atelier-rcnk (leftover merged-tree checkouts), atelier-t18i (link counts and cycle error by storage).
- 2026-10-22 [note] 2026-10-22 plan: the human widened this mission and asked for it to run first. Increments: atelier-1ztv (gate agreement: as7i, n642, uw4x, iaas, ss25; S7 and S11), atelier-9g48 (branch readiness in status: ftqn, tciw; S1), atelier-c3kh (merge a moved base: p189, isyf, rcnk, 2yl7, s2ei; S3 and S11), atelier-yim7 (safe push and start: ecah, mvzi, yc4d, mhx0, 81e5; S10), atelier-zuow (record and review tooling: 4ixu, rc8p, tq1j; S8 and S9), and atelier-j8kg (review effort: 1mn2, u6f3, mbrf; backlog, after atelier-1ztv, and atelier-mbrf's rule needs the human). The first five run in parallel on disjoint files; that run is the orchestrated run S6 counts. atelier-7b6e follows atelier-as7i. atelier-ftqn moved here from atelier-4pzj because it blocks S1. Decision for the human: atelier-6mgg (merge_commit default), which blocks atelier-2yl7. Acceptance: atelier-prjr. atelier-w0su stays deferred.
- 2026-10-22 [note] 2026-10-22: the human decided review keeps one standard for code and prose. Planning text is not exempt, and records stay exempt because requiring an issue to edit issues is recursive. atelier-mbrf was reframed as an owner-free trivial exemption on the base, atelier-1mn2 was superseded, and S8 and S9 were reworded. atelier-6mgg decided: merge_commit is the default when a test check exists.
- 2026-10-22 [note] 2026-10-22 closeout: low-priority follow-ups atelier-j458 and atelier-efnf were unlinked from this mission and keep their discovered_from sources, so the debrief still lists them as found along the way; atelier-w0su (deferred by the human) was unlinked too. All planned code increments merged: c3kh, 9g48, yim7, zuow, 1ztv, tciw, s2ei, r28p, 7b6e, j8kg and fngz.

## Close Reason

### Outcome
Parallel work runs through review and merge without catch-up churn. All 11 success criteria are proven by independent acceptance validation (atelier-prjr; S8 re-validated as atelier-sig1 after atelier-aufe). In this mission's own orchestrated run, 11 branches landed with 5 catch-up merges (0.45 per branch, five branches open at once), against 2.25 per branch on 2026-10-17 and 16 on astromech-cmhv. `status` shows each waiting branch's merge readiness; merge_commit is the default with a test check; merge reruns gates when only records moved and `merge --sync` brings the base in; land and merge share one approval rule and merge refuses any done claim land would refuse; approvals survive close-only commits and follow-ups; land --push and start are safe; record commits group; review request writes the brief; trivial changes land on the base with a pinned exemption; land is about 2.9x faster.

### Deviations
Scope widened on 2026-10-22 by the human from catch-up churn to the whole review-and-land cycle (S7-S11). atelier-1mn2 was superseded and atelier-mbrf reframed when the human decided review keeps one standard for code and prose; the vendored and upstream Agent Factory skill now say so (atelier-aufe, atelier-k4xg). atelier-ftqn moved here from atelier-4pzj. atelier-w0su stayed deferred.

### Decisions
atelier-6mgg (merge_commit default) and the one-review-standard decision are on this record's notes; increment decisions are on their items (`atelier work decisions atelier-k8h8`).

### Surprises
Reviews caught land --push pushing to origin/main, record commits sweeping unrelated edits, and three ways merge and land disagreed. A pre-existing notes test failed every evening (UTC versus local date) and blocked merge checks; fixed on the base with the new trivial exemption (deb2ccb0).

### Open items
atelier-j458 (non-UTF-8 record paths), atelier-efnf (fix-now findings in branch readiness), atelier-3js6 ('just a doc' evaluation scenario), atelier-w0su (integration branches), all unlinked with their sources. Older drift between the vendored, upstream and installed Agent Factory copies. astromech needs an independent approval of the astromech-y296 resolutions before its doctor passes.
