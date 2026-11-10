---
created_at: "2026-11-10T19:04:28.457275352+00:00"
id: "atelier-5qiq"
evidence_type: "validation"
commit: "2a970de858554b83349ee118e1683dcfb3393f72"
result: "pass"
captured_at: "2026-11-10T19:04:28.457234596+00:00"
target:
  kind: "issue"
  id: "atelier-xtvh"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-xtvh"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "On 2a970de8, the CI job's next steps run as written in a scratch clone (run_next_job.py, RUSTC_WRAPPER empty after sccache failed on a cold build): rm -rf ../crates, cargo fmt -- --check, cargo build --locked --all-targets, cargo nextest run --locked (121 passed); all steps passed."
updated_at: "2026-11-10T19:04:28.604976501+00:00"
---

On 2a970de8, the CI job's next steps run as written in a scratch clone (run_next_job.py, RUSTC_WRAPPER empty after sccache failed on a cold build): rm -rf ../crates, cargo fmt -- --check, cargo build --locked --all-targets, cargo nextest run --locked (121 passed); all steps passed.
