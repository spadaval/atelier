---
created_at: "2026-10-02T23:13:09.614836803+00:00"
id: "atelier-frvc"
evidence_type: "benchmark"
commit: "f337156bd9b8697aabbd83ae202c3645381c5ef0"
result: "fail"
captured_at: "2026-10-02T23:13:09.614770098+00:00"
path: "/tmp/budget-dynamic16-production/normal60-readout.json"
agent_identity: "Agent Factory budget_audit"
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
title: "Rejected dynamic16 initial-parser candidate at measured source f337156bd9b8697aabbd83ae202c3645381c5ef0. Root executed the sole 60 normal CLI comparisons (10 alternating pairs each LAND, MERGE and ordinary control); all raw outcomes, tails, original-DD oracles, preparation failures and complete source/fixture/artifact fences were retained. Timing SHA256 13452f2c0af84afb59b50a037b5ffee307f8edbabe6f2371591f89f64183740e; outer fence SHA256 f0713b33e1931684f6ff1ee6c5150a14e892d2ca597243f2e5e6ff0bf8f7662f. Candidate LAND maximum 136.886857 ms and MERGE maximum 135.444533 ms fail strict ordinary-read <100 ms S4. This was only the scoped normal60 comparison, not a full44 acceptance run; the earlier closed20 diagnostic is separate. Root disposition DISCARD: small mixed guard gains, worse maximum tails, ordinary control 8/10 losses, source +51 lines. This rejected candidate is historical and nonancestor of accepted root production; this receipt cannot establish accepted-root freshness or close issue atelier-8ud1, issue atelier-95bu, validation, or mission work."
updated_at: "2026-10-02T23:13:09.616069243+00:00"
---

Rejected dynamic16 initial-parser candidate at measured source f337156bd9b8697aabbd83ae202c3645381c5ef0. Root executed the sole 60 normal CLI comparisons (10 alternating pairs each LAND, MERGE and ordinary control); all raw outcomes, tails, original-DD oracles, preparation failures and complete source/fixture/artifact fences were retained. Timing SHA256 13452f2c0af84afb59b50a037b5ffee307f8edbabe6f2371591f89f64183740e; outer fence SHA256 f0713b33e1931684f6ff1ee6c5150a14e892d2ca597243f2e5e6ff0bf8f7662f. Candidate LAND maximum 136.886857 ms and MERGE maximum 135.444533 ms fail strict ordinary-read <100 ms S4. This was only the scoped normal60 comparison, not a full44 acceptance run; the earlier closed20 diagnostic is separate. Root disposition DISCARD: small mixed guard gains, worse maximum tails, ordinary control 8/10 losses, source +51 lines. This rejected candidate is historical and nonancestor of accepted root production; this receipt cannot establish accepted-root freshness or close issue atelier-8ud1, issue atelier-95bu, validation, or mission work.
