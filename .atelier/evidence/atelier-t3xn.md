---
created_at: "2026-11-10T15:29:56.902355354+00:00"
id: "atelier-t3xn"
evidence_type: "validation"
commit: "9af9d84d948cf5f80e538b847b2d5272c90de14f"
result: "pass"
captured_at: "2026-11-10T15:29:56.902314638+00:00"
target:
  kind: "issue"
  id: "atelier-mal9"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-mal9"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "The new CI job's run steps, read from ci.yml and run each with bash -e in next/ of a scratch clone (/home/supadava/.cache/atelier-fcb7/run_next_job.py): on 0bab05bc all passed (37 tests). On throwaway commit dfab2f91, which adds atelier-workflow = { path = \"../../../crates/atelier-workflow\" } to atelier-records and uses it (it builds with crates/ present), the job failed at its first cargo step: failed to read .../crates/atelier-workflow/Cargo.toml."
updated_at: "2026-11-10T15:29:57.044537897+00:00"
---

The new CI job's run steps, read from ci.yml and run each with bash -e in next/ of a scratch clone (/home/supadava/.cache/atelier-fcb7/run_next_job.py): on 0bab05bc all passed (37 tests). On throwaway commit dfab2f91, which adds atelier-workflow = { path = "../../../crates/atelier-workflow" } to atelier-records and uses it (it builds with crates/ present), the job failed at its first cargo step: failed to read .../crates/atelier-workflow/Cargo.toml.
