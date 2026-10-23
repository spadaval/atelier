---
created_at: "2026-10-22T19:58:08.033898524+00:00"
id: "atelier-prjr"
issue_type: "validation"
labels:
- "merge"
- "review"
workflow_branch:
  branch_base: master
  work_branch: validation/atelier-prjr
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-1ztv"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-7b6e"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-9g48"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-c3kh"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-j8kg"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
  - kind: "issue"
    id: "atelier-yim7"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-zuow"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-23T00:05:44.831883640+00:00"
status: "done"
title: "Validate atelier-k8h8 acceptance"
updated_at: "2026-10-23T00:05:44.831883640+00:00"
---

## Goal

Check atelier-k8h8's success criteria S1 and S3-S11 independently. S2 is already shown by the skill text.

## Done when

Each criterion has an explicit result and a linked evidence receipt naming it. S6 is counted from this mission's own git history: catch-up merges per landed branch across the first wave's five parallel branches, against the baseline in the mission's Background. The handoff states failures, blocked checks and proof limits.

## Context

The mission record holds the criteria and the 2026-10-17 observation note with the earlier catch-up counts.

## Notes

- 2026-10-22 [note] Validation results on master code at f3ee2581 (receipts committed in 627e91ff): S1 pass atelier-rccd; S2 pass atelier-dueg; S3 pass atelier-fzum; S4 pass atelier-9hp8; S5 pass atelier-9sv1; S6 pass atelier-7pql (11 landed branches, 5 catch-ups, average 0.45, peak 5 parallel; first wave 2/5); S7 pass atelier-vn0d; S8 FAIL atelier-om3m (Agent Factory SKILL.md Independent Review, constitution.md and orchestrate.md still require review only for code and say prose-only documentation is not code, against the 2026-10-22 one-standard decision; behaviour parts pass); S9 pass atelier-n2ql; S10 pass atelier-lw6d; S11 pass atelier-ukq9 (limit: approvals at distinct pins still cost one range walk each). Full suite with test-hooks: 894 passed, 0 skipped. Scenario scripts: /home/supadava/.cache/atelier-tmp/validate-k8h8/scripts/s1.sh..s11.sh.

## Close Reason

### Outcome
Checked atelier-k8h8's success criteria S1-S11 against master's code (f3ee2581, built in this worktree) from the CLI user's view, in scratch repositories under TMPDIR with local bare remotes. Ten pass and one fails. Each criterion has its own validation receipt on the mission: S1 atelier-rccd, S2 atelier-dueg, S3 atelier-fzum, S4 atelier-9hp8, S5 atelier-9sv1, S6 atelier-7pql, S7 atelier-vn0d, S8 atelier-om3m (fail), S9 atelier-n2ql, S10 atelier-lw6d, S11 atelier-ukq9. All but S2 and S8 are captured commands that rerun a scenario script and assert on its output. S6 counts from this repo's history: 11 branches landed with 5 catch-up merges between them, an average of 0.45, against a baseline of 16 on one branch and 2.25 on 2026-10-17. All 5 catch-ups needed conflict resolution, and five branches were open at once. The full suite with test-hooks passed: 894 tests, none skipped.

### Deviations
S2's upstream text was checked only in the local clone /home/supadava/projects/agents-skills, at origin/main 451c0c9 as last fetched, with no network fetch. In S3's lockfile case, cargo metadata fails on a missing path dependency with a stale Cargo.lock; it is not a lock-version-only staleness. One uncommitted S11 receipt came from a broken version of its script and was deleted before commit. Early on, one scenario ran in the main checkout because TMPDIR was unset. It staged nine new files there and made no commits. I removed the files and the checkout is clean again.

### Decisions
S8 is marked fail, not pass with a limit. Its criterion names the Agent Factory review guidance as proof, and that guidance contradicts the human's one-standard decision.

### Surprises
S8: .agents/skills/agent-factory/SKILL.md, Independent Review (lines 70-80), says "Every non-trivial code change is reviewed" and "Prose-only documentation is not code". constitution.md:39 and orchestrate.md:62 and :175 say the same, and upstream has the same text. Atelier itself already refuses an unexempted prose commit on the base. Cause: defect in this change. S11: approvals at distinct pins still cost one `rev-list --parents pin..HEAD` walk each. atelier-7b6e's decision note accepted this, but I found no open follow-up for it. S10: on main, the push warning says "commits that no owner made" for commits from another owner's merged branch.

### Open items
S8 needs the Agent Factory review guidance updated in the vendored copy and upstream (atelier-k4xg covers only the upstream sync). The per-approval ancestry walks could use a follow-up.
