---
created_at: "2026-10-02T17:20:23.165454487+00:00"
id: "atelier-kf2p"
evidence_type: "benchmark"
commit: "8b59ec7594f1868a10246f317aea467349f8c7b3"
result: "fail"
captured_at: "2026-10-02T17:20:23.165385524+00:00"
path: "/tmp/budget-initial-cap12-readout.json"
agent_identity: "budget_audit"
target:
  kind: "issue"
  id: "atelier-8ud1"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-8ud1"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "FAIL: temporary initial fused raw and typed parser cap of 12 workers versus immutable f5d8ece1. Measured commit: 8b59ec7594f1868a10246f317aea467349f8c7b3, a rejected candidate and nonancestor of current production. Preserve all 80 samples and tails: 20 isolated batch measurements plus 60 normal CLI measurements, with 10 alternating pairs each for successful exact-owner land, exact-owner merge, and unchanged ordinary issue-list control. All 3 fixed dd67510 oracles, raw stdout/stderr/exits, and complete fixture/source/artifact fences PASS.\n\nBatch wall median/max: default 16.967/18.809 ms versus cap 12 18.347/20.941 ms; median paired saving -0.494 ms, 5 wins and 5 reversals. Inner aggregate CPU median: 149.436 to 112.108 ms; median paired CPU saving 30.951 ms, 9 wins and 1 reversal. CPU saving is distinct from wall latency and does not establish an allocator or futex cause.\n\nLand median/max: 107.150/124.248 to 125.964/134.264 ms; median paired saving -14.772 ms, 3 wins and 7 reversals. Merge: 111.781/143.504 to 113.292/124.591 ms; median paired saving -0.884 ms, 5 wins and 5 reversals. Unchanged ordinary-load control: 26.642/38.210 to 28.647/33.159 ms; median paired saving -1.763 ms, 2 wins and 8 reversals.\n\nDISCARD: no reliable wall improvement. The +19 production-line prototype was never merged into root. This receipt is historical evidence of a rejected nonancestor candidate, not current-root or full 44-read S4 acceptance. It cannot satisfy current-root closure or freshness. S4 and atelier-95bu remain open. Do not extend the worker sweep or infer a contention cause.\n\nArtifacts: /tmp/budget-initial-cap12-timing.json and /tmp/budget-initial-cap12-timing-raw/; /tmp/budget-initial-cap12-readout.json; /tmp/budget-initial-cap12-proof.json; /tmp/budget-initial-cap12-probe-proof.json; /tmp/atelier-cap12-rootquiet-fence.json. Preserve all preparation failures."
updated_at: "2026-10-02T17:20:23.166083951+00:00"
---

FAIL: temporary initial fused raw and typed parser cap of 12 workers versus immutable f5d8ece1. Measured commit: 8b59ec7594f1868a10246f317aea467349f8c7b3, a rejected candidate and nonancestor of current production. Preserve all 80 samples and tails: 20 isolated batch measurements plus 60 normal CLI measurements, with 10 alternating pairs each for successful exact-owner land, exact-owner merge, and unchanged ordinary issue-list control. All 3 fixed dd67510 oracles, raw stdout/stderr/exits, and complete fixture/source/artifact fences PASS.

Batch wall median/max: default 16.967/18.809 ms versus cap 12 18.347/20.941 ms; median paired saving -0.494 ms, 5 wins and 5 reversals. Inner aggregate CPU median: 149.436 to 112.108 ms; median paired CPU saving 30.951 ms, 9 wins and 1 reversal. CPU saving is distinct from wall latency and does not establish an allocator or futex cause.

Land median/max: 107.150/124.248 to 125.964/134.264 ms; median paired saving -14.772 ms, 3 wins and 7 reversals. Merge: 111.781/143.504 to 113.292/124.591 ms; median paired saving -0.884 ms, 5 wins and 5 reversals. Unchanged ordinary-load control: 26.642/38.210 to 28.647/33.159 ms; median paired saving -1.763 ms, 2 wins and 8 reversals.

DISCARD: no reliable wall improvement. The +19 production-line prototype was never merged into root. This receipt is historical evidence of a rejected nonancestor candidate, not current-root or full 44-read S4 acceptance. It cannot satisfy current-root closure or freshness. S4 and atelier-95bu remain open. Do not extend the worker sweep or infer a contention cause.

Artifacts: /tmp/budget-initial-cap12-timing.json and /tmp/budget-initial-cap12-timing-raw/; /tmp/budget-initial-cap12-readout.json; /tmp/budget-initial-cap12-proof.json; /tmp/budget-initial-cap12-probe-proof.json; /tmp/atelier-cap12-rootquiet-fence.json. Preserve all preparation failures.
