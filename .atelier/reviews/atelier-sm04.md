---
schema: atelier.review
schema_version: 2
id: atelier-sm04
title: finding on atelier-295u @1d3ffcc6
target: atelier-295u
identity: gpt-6-astra-amend2-r1
commit: 1d3ffcc648a363d934bf2b85479a58c84f4f8b0b
created_at: 2026-11-10T03:59:58.141091132Z
event:
  event: finding
  recommendation: fix_now
---

high, docs/strategies/redesign-4-gardener.md:35 and atelier-8b0a: from-log requires every non-T3 scorecard measure to come from log and list, and deletes every other script reader. Neither the specified act lines nor work lists contain source lines, deletion ancestry or defect classifications. They cannot compute non-test Rust lines, lines deleted within two weeks, or review yield; mission 1 explicitly needs a Git reader and classification for these. The mission would either fail its criterion or inflate log into the scorecard machinery Sumeet rejected. Smallest fix: limit from-log to lifecycle and thread facts, retain the existing Git readers and classification inputs for measures those facts cannot supply, and update the script-cutting instruction in the strategy and item.
