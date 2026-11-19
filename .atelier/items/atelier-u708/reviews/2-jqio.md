---
schema: atelier.review
schema_version: 3
round: 2
commit: de60a8f097cd1c2097698265c44bbfd0dffc68b6
reviewers:
- thread:delegated-task:command%3Amcp%3Ac8935948-40b5-4d71-9502-f3f27fd405ab%3Adelegate-task%3Ae0ed5863-f2bb-4430-b99b-4eb2f56c5a57
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-19T04:14:44.790391724Z
---

## Summary

Complexity: medium. Separate worker pools are straightforward, but provider failure classification and result reuse must agree with native-child records, setup identity and table aggregation.

Value: high. Parallel execution directly reduces the time needed for the current matrix, and the saved 32-run benchmark demonstrates the benefit.

Risk of merging: high. Misclassifying provider failures or reusing incompatible grades can silently bias an eval's conclusions. The three earlier defects now have regression coverage and are fixed.

The value warrants the change. I found no substantially smaller design that delivers the required concurrency, isolation and resumption. Approve this head with no new fix-now findings.

I reviewed `master...de60a8f097cd1c2097698265c44bbfd0dffc68b6` and the changes since round 1, checked each Done when line and the Report, and read the surrounding environment, delegation, native-child conversion, judging and aggregation code. `python3 -m unittest test_run test_confine` passed all 100 tests. These exercise the independent pool caps, overlapping thread runs with separate work directories, locks, logs, homes and sign-in links, unchanged per-run concurrency, provider limits and recovery, setup changes and matrix-local retry suppression. `git diff --check master...de60a8f097cd1c2097698265c44bbfd0dffc68b6` passed. Additional model-free probes confirmed that a native child's `stream_error` remains a limit after a parent's `UserMessage`, and clears after the child's subsequent completed reply.

I independently inspected the saved benchmark at `~/.cache/af-evals/runs/u708-parallel/`: 32 runs, 18 passes, 14 failures, no invalids, 612 seconds across agents and judges, 4,337 summed first-agent seconds, and a matrix peak of 19 agents. No first-agent, delegated-child or native-child transcript was classified as provider-limited; no top-level error or failed-turn event appeared, and all 32 Claude limit events were allowed. The load log supports peaks of 12.93 for the 1-minute load and 6.48 for the 5-minute load, with at least 6,434,352 KiB available memory. The current reuse check keeps all 32 runs when given their original setup, binary and resolved skill commit.

I did not start another real matrix, induce a real provider limit, or rerun the scenarios' fixture suites. PR #241's evals check was still running when inspected. The documented fixed-port interference remains the deferred finding 1-611l.4; this round does not change that assessment.

## Finding 1: Native Codex child usage limits are lost and become reusable task failures
Recommendation: fix now
Was: 1-611l.1
Response: fixed in 38d3ec4b3. A `spawn_agent` subagent's transcript in `threads/` is rebuilt from its session record and keeps no errors, so `run_limit` now also reads the records in `sessions/`, where `provider_limit` takes an `error` or `stream_error` event_msg as a limit and a later completed item, not the parent's message, as a recovery. Two tests run `sol` end to end with a parent that ends normally: a subagent that ends on its usage limit makes the run invalid, and one that recovers keeps its grade.
Now: fixed

`run_limit` now reads the copied native-child records in `sessions/`, and `provider_limit` handles their error events independently of the parent's transcript. Both end-to-end native-child regressions pass: a terminal usage limit makes the run invalid despite the parent finishing normally, and a recovered child retains its grade. The additional parent-input probe also preserves the limit until the child responds.

## Finding 2: Reuse compares the setup name instead of its actual roles and judges
Recommendation: fix now
Was: 1-611l.2
Response: fixed in 38d3ec4b3. `kept_runs` takes the resolved setup and keeps a run only if its saved `roles` and `judges` equal the setup's now; a test changes a role's effort and a judge's model under the name `sol` and sees the run made again. The 32 real runs are still kept.
Now: fixed

`kept_runs` now compares saved roles and judges with the resolved setup as well as its name. The tests cover changing a reviewer's effort and a judge's model, including the `main` path making a run again under the same setup name. All pass, and the unchanged real matrix remains reusable.

## Finding 3: Table aggregation mistakes independent matrices for retries
Recommendation: fix now
Was: 1-611l.3
Response: fixed in 38d3ec4b3. `table.py` drops a remade null only within its own `results.jsonl`; a test reads two matrices' files in both orders and keeps the null.
Now: fixed

`table.read` applies retry suppression separately to each input results file before concatenating them. Its regression passes in both input orders for independent matrices and still removes a superseded null within one matrix. `rows` no longer suppresses results from different matrices after concatenation.
