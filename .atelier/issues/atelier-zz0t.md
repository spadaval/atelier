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

- [ ] A validator who did not run the merge reads the merge transcript (the orchestrator's session, quoted in atelier-7o4r's notes or Report) and confirms no `gh` or `git push` ran by hand for the branch, the pull request or the base between the start of `merge --push` and its success line.
- [ ] `gh pr view <n> --repo spadaval/atelier --json state,mergedAt,mergeCommit` shows the pull request merged, and origin's master contains the merge commit.
- [ ] `git log` shows a commit that reached master after the branch's CI run started and before the merge, and `ci.passed` accepted the earlier run without a new one.
- [ ] The result is recorded as a validation receipt naming the criterion (`atelier evidence record --target issue/atelier-zz0t --kind validation --result pass|fail --criterion one-command-merge=pass|fail "..."`), and a failure is filed as a linked bug.
