---
created_at: "2026-10-02T05:26:34.378832615+00:00"
id: "atelier-k6wv"
evidence_type: "benchmark"
commit: "bf6bb637afd889fbef2d07c0cf90d2660d1978ee"
result: "fail"
captured_at: "2026-10-02T05:26:34.378806124+00:00"
path: "/tmp/atelier-range-ordered-timing.json"
agent_identity: "rebuild-orchestrator"
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
title: "S4 remains FAIL. Reviewed quiet whole-command measurements of production commit 22d817c9e0a1dd7a9d676546a952cd3167b57140; current HEAD has the identical crates tree. Ten alternating pairs per shape, 80 raw samples retained; fixed original-dd67510 output/exit parity and fixture digests unchanged. Candidate median/max ms: prime 103.79/124.71, valid land 141.81/161.34, merge check 152.71/202.77, mixed refusal 75.73/91.76. All failed limits and the baseline 658.86 ms outlier remain in the artifact. This is bounded performance diagnosis, not full44-shape acceptance; independent mission validation remains pending."
updated_at: "2026-10-02T05:26:34.379223425+00:00"
---

S4 remains FAIL. Reviewed quiet whole-command measurements of production commit 22d817c9e0a1dd7a9d676546a952cd3167b57140; current HEAD has the identical crates tree. Ten alternating pairs per shape, 80 raw samples retained; fixed original-dd67510 output/exit parity and fixture digests unchanged. Candidate median/max ms: prime 103.79/124.71, valid land 141.81/161.34, merge check 152.71/202.77, mixed refusal 75.73/91.76. All failed limits and the baseline 658.86 ms outlier remain in the artifact. This is bounded performance diagnosis, not full44-shape acceptance; independent mission validation remains pending.
