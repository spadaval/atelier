---
schema: atelier.review
schema_version: 3
round: 3
commit: 6641686e98730cfd86cc897631010eac35b331a4
reviewers:
- thread:delegated-task:command%3Amcp%3A086109c9-66b0-4adf-ae7a-e15a0ae9afe7%3Adelegate-task%3Afaa26aca-d535-430e-8fc0-334b6dfe6780
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-19T00:17:00.301436399Z
---


## Summary

Complexity: medium. The existing harness gains role-specific commands and homes, mixed-agent delegation, and per-thread accounting.

Value: high. Comparing the hosts' eval results needs their actual role assignments, with time and token costs reported per cell.

Risk of merging it: high. Incorrect accounting or thread attribution would silently bias the comparisons and later graders that read these records.

The value warrants the change. I found no substantially smaller approach that meets the role-routing and accounting requirements. The carried token-accounting finding is fixed, and I found no new fix-now problem.

I read the full change against master at 6641686e98730cfd86cc897631010eac35b331a4 and the changes since round 2. I checked both setups and their judges, every scenario's initial role, command construction, scratch homes and skill staging, delegation and the two-agent limit, every thread's init checks, call identities, token totals, and table aggregation against the Done when lines. `evals/agent-factory/test.sh /home/supadava/.cargo/bin/atelier` passed all 53 harness tests and all nine scenario suites. The failed-turn tests cover both a delegated Codex reviewer and a Codex first agent, retaining the session's recorded usage in the thread and aggregate totals. Additional no-model assertions checked table medians, null exclusion, and compatibility with old results; `git diff master...6641686e --check` passed.

I also inspected the saved smoke result and delegate transcripts, and checked that its two logged Atelier calls match the result. I did not run a new paid matrix or independently verify all three real house models. The saved direct-delegation work directory has been removed, so I could not recheck that Codex child's raw session record. The PR's evals check was still running when inspected.

## Finding 1: Preserve recorded token usage when a Codex thread does not complete
Recommendation: fix now
Was: 2-2vvm.2
Response: fixed in 0d68b7013: every Codex thread, the first agent included, takes its tokens from its session record's running total, with tests of a reviewer thread and a first agent whose turn fails after usage is recorded.
Now: fixed

`codex_record` distinguishes a missing usage record from recorded consumption. Both `thread_facts` and the first-agent path now prefer its running total, falling back to transcript usage only when the record has none. Native Codex subagents still use their own session totals. The two new end-to-end tests record usage and then fail the turn without `turn.completed`; both pass and retain the expected tokens. The original loss of known usage is resolved.
