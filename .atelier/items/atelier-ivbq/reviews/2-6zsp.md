---
schema: atelier.review
schema_version: 3
round: 2
commit: 6dd7fc218fbbd9786ab1a4aaf7de8c1926281624
reviewers:
- thread:delegated-task:command%3Amcp%3A570d4615-b9d9-43d7-b838-51d47e80cbc3%3Adelegate-task%3Aec69df6c-e11e-4188-ab74-e4158a0c920d
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-11T16:03:41.606113996Z
---

## Summary

Complexity: low. The changes fit in one sitting and add no command, record field or workflow rule.

Value: medium. These messages guide threads through the existing implementation, review and decision paths.

Risk of merging: medium. Incorrect guidance can strand a waiting question, and the shared checklist parser also feeds the Desk.

The value warrants the size. Updating atelier-records is justified because it owns the Done when parser; an output-only fix would duplicate that parsing. No smaller change appears to achieve the same results.

Reviewed the full change from master to 6dd7fc218fbbd9786ab1a4aaf7de8c1926281624, the changes since round 1, the relevant role, question, finding-chain and Markdown parsing code, the changed tests, both specification passages and the Report. The six Done when lines are satisfied. Ran cargo fmt -- --check, cargo clippy -j 4 --all-targets -- -D warnings and cargo nextest run -j 4, sequentially. All passed, including all 292 tests. git diff --check on the reviewed change also passed.

Independently reproduced both earlier findings' fixed behavior with the built CLI in a disposable repository, using ordinary start, submit, review, verdict, escalate, status and decide commands. A finished reviewer sees an unrelated waiting question, receives the decide command without a stop or takeover instruction, and can answer it. A wrapped Done when entry beginning its continuation with #123 is printed whole. The regression tests also cover a question about that reviewer's own finding, where deciding is prohibited, and the mission lead's guidance before and after answering.

Checked that escalation selects an open unasked finding and follows its carried-finding chain when looking for an existing question. The tests cover the third unsuccessful round, guidance on and away from the branch, and disappearance after asking and after answering without settling the finding. The handoff and specifications agree on starting a fresh thread without waiting to be asked, including when the reader is a child.

I did not repeat the Report's source-swap experiments. By inspection, the new assertions distinguish the former behavior; both round 2 regressions also passed in the full suite and in the independent CLI exercise. I did not exercise live Codex or Claude Code handoffs. Their wording and the identity tests were checked. PR #142 names the reviewed commit; its CI test job was still running when checked. No new findings.

## Finding 1: Show waiting questions to eligible deciders after their review has finished
Recommendation: fix now
Was: 1-o9m9.1
Response: fixed in 6b5ec644d
Now: fixed

Verified in the status path, the new regression test and an independent CLI reproduction. The question is shown and the stop instruction is removed only when the finished reviewer may decide it. No takeover is offered.

## Finding 2: Preserve wrapped text beginning with an issue reference
Recommendation: fix now
Was: 1-o9m9.2
Response: fixed in 6b5ec644d
Now: fixed

Verified in the parser and status regression tests and an independent CLI reproduction. The heading check now recognizes one to six hash marks followed by a space or end of line, so #123 remains continuation text.
