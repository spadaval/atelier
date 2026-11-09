---
created_at: "2026-11-09T05:26:52.216762702+00:00"
id: "atelier-zz0t"
issue_type: "validation"
labels:
- "forge"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-1gb3"
    type: "advances"
  - kind: "issue"
    id: "atelier-7o4r"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Validate the one-command merge on atelier-7o4r's real run"
updated_at: "2026-11-09T05:26:52.216762702+00:00"
---

## Goal

Independently check the mission's one-command-merge criterion on the real run: atelier-7o4r merged with `atelier merge atelier-7o4r --push` alone while another commit reached master after its CI run.

## Done when

- [ ] A validator who did not run the merge reads the complete delivery transcript for atelier-7o4r, from `atelier start` through `merge --push`'s success line, including every setup attempt, refusal and retry (the orchestrator quotes it in a note on atelier-7o4r, and the child threads' transcripts are linked), and confirms no `gh pr create`, `gh pr merge`, `git push` of the branch or `git push` of the base ran by hand at any point; every branch push, the pull request and the base push came from `atelier merge atelier-7o4r --push`.
- [ ] `gh pr view <n> --repo spadaval/atelier --json state,mergedAt,mergeCommit,headRefOid` shows the pull request merged, and origin's master contains Atelier's merge commit.
- [ ] A commit by another thread (not the session running the merge) that changes a file under `docs/strategies/` reached master after the CI run `ci.passed` accepted had completed, and before the merge; the validator names that commit, the run (its ID, head commit and the base it tested), and confirms from the merge output that `ci.passed` accepted that same run with no new run. If no other thread commits one in that window, the orchestrator has another agent make a real strategy-doc commit (such as recording progress in a mission strategy) before the merge, and the validator says so.
- [ ] The result is recorded as a validation receipt naming the criterion (`atelier evidence record --target issue/atelier-zz0t --kind validation --result pass|fail --criterion one-command-merge=pass|fail "..."`), and a failure is filed as a linked bug.
