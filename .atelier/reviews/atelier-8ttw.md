---
schema: atelier.review
schema_version: 2
id: atelier-8ttw
title: finding on atelier-a4rs @c8812c67
target: atelier-a4rs
identity: GPT-6.1 Sol independent reviewer
commit: c8812c6716a4606b899e914a2f557c1b25152d8a
created_at: 2026-11-11T06:00:41.114122589Z
event:
  event: finding
  recommendation: fix_now
---

medium: evals/agent-factory/scenarios/next-settle-within-authority/grade.py:55-59 loses quoted redirect and tee targets because loop.executed removes quoted text. Reproduced on the fixture after an independently approved build: the lead ran a real shell append to quoted tally.py, recorded that command in its Bash transcript, and left git diff showing two added lines. The grader still returned pass=true with lead_shell_writes.wrote=[] and may-have-written=[]; it only asked for a final-message hand read. Single- or double-quoted filenames are ordinary shell usage, so this still accepts a lead that writes code, contrary to the pass rule and the fix for atelier-6shr. Parse redirect/tee arguments while preserving their quoted values, and add failing walks for quoted targets in both Claude and wrapped Codex transcripts.

Review at the pinned head: merge after the blocking grader gaps are fixed; no approval now. Complexity medium (eight fixtures plus two agent adapters), value high (trustworthy evaluation before the paid matrix), risk medium (wrong evaluation scores in scratch repositories). Contract fail; Fitness fail until false passes are covered. Fresh proof: 25 harness tests plus 76 next-* tests passed; all ten setup.sh scripts passed with a copy of 399fca6a and again with a scratch wrapper simulating ready/drop/close/lead auto-commits. git diff --check, cargo fmt -- --check and cargo metadata --locked passed. Existing canary/smoke artifacts were inspected, not rerun; no eval agent, build or matrix was started.
