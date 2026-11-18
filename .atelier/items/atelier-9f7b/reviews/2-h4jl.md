---
schema: atelier.review
schema_version: 3
round: 2
commit: 105e4bcdbdbb7a7df1bf9f377beb77160b986c03
reviewers:
- thread:delegated-task:command%3Amcp%3A291922e7-af6e-4a51-b18e-b6eb012ff1a5%3Adelegate-task%3A705c1b19-0abf-43c0-9ed4-33b164365f0f
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-18T01:04:49.251046848Z
---

## Summary

Complexity: low. One additive thread-response field feeds the mission page's existing waiting classification through its existing full-thread reads.
Value: high. An approval-waiting thread must appear under Needs you for the mission page to serve its purpose.
Risk of merging it: medium. It changes the state presented to the human, but errors are visible and the change is easy to revert.

The value warrants the change. No smaller change meets the goal with T3's current list contract. I found no new defect.

Reviewed `git diff master...105e4bcdbdbb7a7df1bf9f377beb77160b986c03` and the changes since round 1, including the Desk's thread parsing and cache, mission-page reads, classification, sentence and actions, generated API type, fixtures and documentation. Independently ran:

- `cargo nextest run -p atelier-desk --test threads -j 2`: all 9 passed, including the running thread with one pending request, omitted-field zero and list-only null.
- `node t3-extension/test-mission.mjs`: all 14 checks passed, including the approval thread under Needs you, its absence under At work, the sentence and its button opening the thread.
- `pnpm typecheck` in `t3-extension`, `pnpm exec tsc --noEmit` in `desk`, and `cargo fmt -- --check`: passed.
- `pnpm test` in `desk`: all 57 tests passed.
- `git diff --check master...105e4bcdbdbb7a7df1bf9f377beb77160b986c03`: passed. CI run 44212929810 also passes `changes` and the Rust `test` job; Desk and extension jobs were still pending when checked.

Inspected both committed test-instance screenshots. The sidebar shows Approval, the implementer appears under Needs you with Open implementer, the sentence counts the waiting thread, and At work is empty. I did not start another real T3 instance, repeat the author's revert experiment, or rerun the whole extension suite or clippy. The focused browser and Rust tests independently cover the changed behavior.

The first Done when line is not literally met: the mission JSON does not carry the count. The Report explicitly discloses this. I verified the local T3 source: `OrchestratorMcpThreadListItem` and `listItemFromShell` omit `pendingRequestCount`, while `OrchestratorMcpThreadDetail` and `threadDetail` supply it. The mission page already requests each listed thread with `full=1`. Using those existing reads meets the user-facing goal without adding per-thread reads to the global poll, so I accept that transport adjustment. The Report accurately identifies the remaining Inbox/sidebar gap as outside this item.

## Finding 1: Remove the report's trailing blank line so CI can run
Recommendation: fix now
Was: 1-8ha7.1
Response: fixed in e6bb31940
Now: fixed

The Report now ends with a single newline, `git diff --check` passes on the reviewed head, and CI's `changes` job passes. The previous whitespace blocker is resolved.
