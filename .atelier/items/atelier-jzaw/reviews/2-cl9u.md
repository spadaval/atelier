---
schema: atelier.review
schema_version: 3
round: 2
commit: 3bb9d98d0f6113a03bcfce541e98643deeef37df
reviewers:
- thread:delegated-task:command%3Amcp%3Ae9fbcf90-745c-45c0-a7f1-9563cdd27749%3Adelegate-task%3A8be8c32e-39f3-40ab-81f3-7f75c9e0124e
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-16T05:46:15.115982514Z
---

## Summary

Complexity: low. One optional host field and one conditional repository lookup extend the existing thread-follow path.
Value: medium. Ordinary threads with no current tie now open their project, a path in current use.
Risk of merging: medium. A navigation failure reaches users and can show unrelated work, but is visible and reversible.
The value warrants this small change. No smaller approach is needed, and no new findings remain.

Reviewed `git diff master...3bb9d98d0f6113a03bcfce541e98643deeef37df` against the Goal and every Done when line, and compared the round 2 changes with the prior reviewed commit. Checked host message validation, stable identity for repeated messages, current-tie precedence, served and unreadable repository handling, encoded route construction, stale-request guards, the host contract documentation and its test cases. The implementation meets the acceptance criteria.

In this separate worktree, `pnpm test` passed all 60 tests, `pnpm build` passed, and `pnpm e2e --workers=2` passed all 118 desktop and phone tests. E2E used `TMPDIR=/home/supadava/.cache/atelier-review-jzaw-r2-tmp` and a separate preview port. `git diff --check master...HEAD` passed. The actual T3 host integration was not exercised; the host extension is outside this item's scope.

## Finding 1: Fall back to the inbox when the repository-list fetch rejects
Recommendation: fix now
Was: 1-8m4g.1
Response: fixed in a1fb38672
Now: fixed

`desk/src/components/Shell.tsx` catches rejection of the repository-list read locally and treats it as no readable list. Once the ties have been read, execution still reaches the inbox fallback and navigation. Non-OK responses retain the same fallback, and a failed ties read still leaves the page without the last thread's line. The added host contract test switches from an implementer's item to an untied thread, aborts `/api/repos` with `connectionreset`, and verifies the inbox and its line. It passed on desktop and phone.
