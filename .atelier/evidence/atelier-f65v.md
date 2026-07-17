---
created_at: "2026-07-17T00:15:04.941126249+00:00"
id: "atelier-f65v"
evidence_type: "validation"
captured_at: "2026-07-17T00:15:04.941083910+00:00"
target:
  kind: "issue"
  id: "atelier-t876"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-t876"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "PASS ignored/skipped-test freshness audit. cargo nextest run --profile extended --run-ignored=only selected zero tests across all 9 binaries and reported 781 skipped because the workspace contains no ignored tests; nextest exit 4 is its no-tests condition, not a test failure. Evidence atelier-4os0 contains the exact command and transcript. The ordinary full suite independently ran all 781 discovered tests with zero skipped or failed in atelier-o65z. Classification: not-applicable for executing ignored tests because none exist; pass for confirming there are no stale ignored tests. Residual risk: none."
updated_at: "2026-07-17T00:15:04.943043474+00:00"
---

PASS ignored/skipped-test freshness audit. cargo nextest run --profile extended --run-ignored=only selected zero tests across all 9 binaries and reported 781 skipped because the workspace contains no ignored tests; nextest exit 4 is its no-tests condition, not a test failure. Evidence atelier-4os0 contains the exact command and transcript. The ordinary full suite independently ran all 781 discovered tests with zero skipped or failed in atelier-o65z. Classification: not-applicable for executing ignored tests because none exist; pass for confirming there are no stale ignored tests. Residual risk: none.
