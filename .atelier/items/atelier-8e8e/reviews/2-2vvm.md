---
schema: atelier.review
schema_version: 3
round: 2
commit: 3c75cee4fbd5e7a456a71253d0a698377e9a4357
reviewers:
- thread:delegated-task:command%3Amcp%3A086109c9-66b0-4adf-ae7a-e15a0ae9afe7%3Adelegate-task%3A2637b392-1728-440b-a57f-de5d79429526
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-19T00:12:41.425664327Z
---

## Summary

Complexity: medium. The existing harness gains role-specific agent commands and homes, mixed-agent delegation, and per-thread accounting.

Value: high. The evals need to run the hosts' actual role assignments and report their time and token cost.

Risk of merging it: high. Incorrect accounting silently biases the cost comparisons that this change introduces.

The value warrants the change, and I found no substantially smaller way to meet its role-routing and accounting requirements. The previous identity finding is fixed, but interrupted Codex threads still lose recorded token usage.

I read the full change against master at 3c75cee4fbd5e7a456a71253d0a698377e9a4357 and the changes since round 1. I checked the setup and judge definitions, scenario roles, per-role commands and skill staging, the two-agent limit, init checks, call attribution, token totals and table aggregation against each Done when line. `evals/agent-factory/test.sh /home/supadava/.cargo/bin/atelier` passed all 51 harness tests and all nine scenario suites; `git diff master...3c75cee4f --check` passed. I inspected the saved smoke result and reproduced the accounting problem below with the existing end-to-end fake agents. I did not run a new paid matrix or independently verify all three real house models. The PR's evals check was pending when inspected.

## Finding 1: Preserve Codex subagent identities in the Atelier call log
Recommendation: fix now
Was: 1-jgy4.1
Response: fixed in 579100db4: the log records the thread Atelier names, `<parent>/<CODEX_THREAD_ID>` for a Codex subagent, and the harness gives each call the `threads` entry that made it as `by`, with tests of the reviewer's root and child calls.
Now: fixed

The wrapper now follows `crates/atelier-core/src/identity.rs`, including the Codex child/session distinction. `atelier_calls` adds `by` with the corresponding `threads` identity, including `main` for an unthreaded first agent. The new root/child wrapper test and identity parity cases pass. The original attribution failure is resolved.


## Finding 2: Preserve recorded token usage when a Codex thread does not complete
Recommendation: fix now

`evals/agent-factory/harness/run.py:636-643` reads a delegated Codex thread's session record, discards its `tokens`, and returns only the transcript's totals. `read_transcript` collects Codex usage only from `turn.completed` (lines 406-407). A child that records token usage and then fails, times out or is killed before that event therefore reports zero, or omits the interrupted turn's usage, despite the harness already having read the actual running total. The first-agent path also discards the session totals at line 906.

I reproduced this without a model using `RunOneTest`'s existing fake agents. I changed only the fake Codex reviewer's final `turn.completed` event to `turn.failed` and made it exit 1, retaining its session record with 300 input, 200 cached input and 30 output tokens. `run_one` reported reviewer exit 1 and `{in: 0, cached_in: 0, out: 0}`, while `codex_session` returned `{in: 300, cached_in: 200, out: 30}` for the same thread. The parent completed successfully and the run remained valid, so such a run can enter the table's medians with understated cost. This also makes the mixed setup's accounting differ from native Codex subagents, whose totals come from their session records.

Preserve the available session usage for interrupted Codex threads, including the first agent, rather than replacing known consumption with zero. Add a no-model case where usage is recorded but no successful `turn.completed` follows, and check both the thread's totals and the aggregate.
