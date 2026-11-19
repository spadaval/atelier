---
schema: atelier.review
schema_version: 3
round: 1
commit: df406468a821979bbccedfb6c267f7280e58c628
reviewers:
- thread:delegated-task:command%3Amcp%3A086109c9-66b0-4adf-ae7a-e15a0ae9afe7%3Adelegate-task%3A209c02b8-03f9-49ce-acd7-2fe622fb48e4
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-19T00:07:22.861403510Z
---

## Summary

Complexity: medium. The runner adds role-specific commands and homes, cross-agent delegation, and per-thread accounting to an existing harness.

Value: high. Running the two hosts' actual role assignments and reporting their cost is necessary to compare their eval results.

Risk of merging it: high. Incorrect telemetry can silently misattribute behavior and bias the later graders that consume these facts.

The value warrants the change. I found no substantially smaller approach that meets the role-routing and accounting requirements, but the call log must preserve the identity of a Codex subagent before it becomes grading input.

I read the full diff against master at df406468a821979bbccedfb6c267f7280e58c628, checked the setups and scenario roles against every Done when line, and traced staging, delegation, the two-agent limit, validity checks, token totals, and table aggregation. `evals/agent-factory/test.sh /home/supadava/.cargo/bin/atelier` passed all 49 harness tests and all nine scenario suites. `git diff master...df406468 --check` passed. I also inspected the recorded smoke result and reproduced the call-identity problem below without a model. I did not run a new paid matrix or independently verify the real model behavior of all three house roles. CI's evals job was pending when inspected.

## Finding 1: Preserve Codex subagent identities in the Atelier call log
Recommendation: fix now

`evals/agent-factory/harness/atelier_log.py:29` records only `T3CODE_THREAD_ID`. In the checked-in `sol` setup, children start with native `spawn_agent` and inherit that variable from the first agent. Their distinct identity is in `CODEX_THREAD_ID`, alongside the shared `CODEX_SESSION_ID`. Atelier explicitly handles this in `crates/atelier-core/src/identity.rs:23`: a child is `<parent>/<CODEX_THREAD_ID>`.

I installed the new wrapper over a stand-in binary that exits 1, then ran the same call once with `T3CODE_THREAD_ID=thread-parent`, `CODEX_SESSION_ID=root-session`, `CODEX_THREAD_ID=root-session` and once with the same parent/session but `CODEX_THREAD_ID=reviewer-session`. Both log entries said `"thread": "thread-parent"`. Consequently, `atelier_failed` attributes a native reviewer's failures to the implementer and cannot be matched to the corresponding child in `threads`. This affects the normal `sol` path, not just malformed setups; non-threaded runs also get a null identity because they have no `T3CODE_THREAD_ID`.

Preserve the effective calling identity, including Codex's child/session distinction, and expose enough information to join each call to its thread facts. Add a no-model root/child test; the current log test covers only a single explicit T3 thread.
