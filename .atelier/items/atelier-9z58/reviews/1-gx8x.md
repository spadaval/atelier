---
schema: atelier.review
schema_version: 3
round: 1
commit: f01c93bac90a1d060b6cf6231e1f065036f4b0e9
reviewers:
- thread:delegated-task:command%3Amcp%3A673b5e94-a49b-41a0-a463-cf1a6cfbaecf%3Adelegate-task%3A5d88e823-e94a-4b82-a406-cb43161327ff
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-18T05:10:31.470016413Z
---

## Summary

Complexity: low. Four existing consumers now inspect the repository error already present in their responses; there is no new state or API.

Value: medium. These native views are in use, and an unreadable repository must not look empty.

Risk of merging: medium. The rendering changes reach users, but failures are visible and the change is easy to revert.

The value warrants the change. I found no fix-now or deferred findings.

Reviewed `master...f01c93bac90a1d060b6cf6231e1f065036f4b0e9`, the Done when lines, the Report, the four changed consumers, and their surrounding data flow. Compared the fixtures with the successful-response failure shapes in `server.rs` and `server/views.rs`. The thread view names inbox repository failures and suppresses its empty-thread fallback; the glance names motion failures and omits In motion and Next up. The sidebar names its repository failure and clears the note after recovery. Settings names `/api/repos` failures and marks the Desk line as an error.

Audited the extension's read sites against the response types and handlers. Project home reads missions, roadmap, history and individual missions, which fail through HTTP statuses rather than successful responses with repository errors. Its existing failure handling and fixtures therefore cover the applicable case without an added motion or inbox read. The Report correctly identifies the remaining Desk glance fault and the pages that already show repository errors. Suppressing the glance while any inbox repository is unreadable is conservative but consistent with the existing failed-request rule and explicitly reported.

Validation: `pnpm typecheck`, `pnpm build`, all seven tests in `node run-tests.mjs`, and `git diff --check master...f01c93bac90a1d060b6cf6231e1f065036f4b0e9` passed. This includes the new successful-response inbox and motion failure fixtures and sidebar failure/recovery assertions. The tests used the existing Desk binary and app in this checkout. Settings' repository-error rendering was checked by inspection; I did not run a live T3 session or independently repeat the author's source-reversion checks. CI's extension job was pending when checked.
