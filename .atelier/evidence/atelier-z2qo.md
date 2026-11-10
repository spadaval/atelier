---
created_at: "2026-11-10T16:56:27.298396523+00:00"
id: "atelier-z2qo"
evidence_type: "validation"
commit: "74f8450a9bc4ace627cb396b377084dbc490abc6"
result: "pass"
captured_at: "2026-11-10T16:56:27.298315803+00:00"
target:
  kind: "issue"
  id: "atelier-5fsj"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-5fsj"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Measured shrink of commands.py on the frozen inputs: each piece removed in turn and counted.py rerun (16 variants), then the removable set together. Removed: $(...) handling, tee, cp/mv and apply_patch writes, the record-commit hash, wrapper option arguments and # comments; together they move atelier by 21 of 4,640 atelier calls (confusion +0.07 per 100, process share -0.07 points), astromech by 4 calls and 2 threads with a first edit, life-on-rails process share by 0.02 points, and no median of calls before first edit. The final regenerated counted.json is within the orchestrator's bounds against the round 4 output in every repository."
updated_at: "2026-11-10T16:56:27.440353877+00:00"
---

Measured shrink of commands.py on the frozen inputs: each piece removed in turn and counted.py rerun (16 variants), then the removable set together. Removed: $(...) handling, tee, cp/mv and apply_patch writes, the record-commit hash, wrapper option arguments and # comments; together they move atelier by 21 of 4,640 atelier calls (confusion +0.07 per 100, process share -0.07 points), astromech by 4 calls and 2 threads with a first edit, life-on-rails process share by 0.02 points, and no median of calls before first edit. The final regenerated counted.json is within the orchestrator's bounds against the round 4 output in every repository.
