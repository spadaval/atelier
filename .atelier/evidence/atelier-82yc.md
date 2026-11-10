---
created_at: "2026-11-10T16:50:58.310270092+00:00"
id: "atelier-82yc"
evidence_type: "validation"
commit: "74993259cc8e7b2dec7a7ccb9dd5632c948825db"
result: "pass"
captured_at: "2026-11-10T16:50:58.310191736+00:00"
target:
  kind: "issue"
  id: "atelier-5cuc"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-5cuc"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "The CI job's next steps (rm -rf ../crates, cargo fmt -- --check, cargo build --locked --all-targets, cargo nextest run --locked) passed as written in a fresh clone of e4b28329 with run_next_job.py: 78 tests passed. git diff --stat master...HEAD -- crates/ is empty; git diff --check is clean."
updated_at: "2026-11-10T16:50:58.454704563+00:00"
---

The CI job's next steps (rm -rf ../crates, cargo fmt -- --check, cargo build --locked --all-targets, cargo nextest run --locked) passed as written in a fresh clone of e4b28329 with run_next_job.py: 78 tests passed. git diff --stat master...HEAD -- crates/ is empty; git diff --check is clean.
