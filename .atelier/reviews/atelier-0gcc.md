---
schema: atelier.review
schema_version: 2
id: atelier-0gcc
title: finding on atelier-6sti @d3806629
target: atelier-6sti
identity: gpt-6.1-sol-r1
commit: d38066290345dce0e9c44b2dae7e462e2d37ebc7
created_at: 2026-11-10T17:20:19.929577698Z
event:
  event: finding
  recommendation: fix_now
---

medium: docs/architecture/quality/scorecard-baseline-2026-11.md:46 Frozen per-model prices do not prevent a cheaper model mix from meeting the token-cost target. The report says they do, but transcripts.Request.cost applies a different frozen rate to each model, and counted.py sums those costs per item. Fresh reproduction with identical 1M input tokens and no other tokens gives USD 2.00 for gpt-6.1-sol and USD 0.10 for gpt-6-luna, using the unchanged baseline price table. Thus a model substitution alone can exceed the proposed reduction without removing any process work. Correct the explanation to distinguish frozen price changes from model-mix changes, and state that a changed mix confounds attributing savings to the redesign; do not claim the existing script excludes this route. This is a correction to the target rationale, not a request for new measurement or script changes.
