---
schema: atelier.review
schema_version: 2
id: atelier-wpu3
title: finding on atelier-a4rs @fac20029
target: atelier-a4rs
identity: GPT-6.1 Sol independent reviewer
commit: fac200290b0846185e18d061a127f2560e2c1c27
created_at: 2026-11-11T06:40:31.441971019Z
event:
  event: finding
  recommendation: fix_now
---

medium: evals/agent-factory/scenarios/next-done-when-rewrite/grade.py:74-75 accepts deleted benchmark files because None is allowed alongside the original text. bench_passes then recreates those missing files at lines 45-47, hiding their deletion. Reproduced on the copied post-atelier-psjw binary: use the existing indexed-search passing walk, run the real benchmark successfully and record its actual output, git rm -r bench and commit, tick the latency line, submit, independently approve and merge in the scratch fixture. The grader returns pass=true, merged=true and bench_changed=[], but python3 bench/run_bench.py on the finished repository exits 2 because the script is missing. This violates the scenario rule that bench/ stays unchanged and credits a finished criterion whose required command cannot run. Treat missing benchmark files as changes and add a failing walk for deletion, including the otherwise passing indexed-and-merged path. All 25 harness tests, 91 scenario tests, ten standalone setups and the brief's formatting, metadata and whitespace checks passed; this additional scripted check exposes a false pass they miss.
