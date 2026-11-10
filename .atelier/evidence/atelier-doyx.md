---
created_at: "2026-11-10T15:36:12.470419720+00:00"
id: "atelier-doyx"
evidence_type: "validation"
commit: "1a14e2ff239af8db7aa0bd143e0ddb479084d583"
result: "pass"
captured_at: "2026-11-10T15:36:12.470378914+00:00"
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
title: "The new CI job's run steps, read from ci.yml and run each with bash -e in next/ of a scratch clone (/home/supadava/.cache/atelier-fcb7/run_next_job.py): on 3d21d702 all passed (38 tests). On throwaway commit 0d73a30b, which adds atelier-workflow = { path = \"../../../crates/atelier-workflow\" } to atelier-records and uses it (it builds with crates/ present), the job failed at its first cargo step: failed to read .../crates/atelier-workflow/Cargo.toml."
updated_at: "2026-11-10T15:36:12.611841516+00:00"
---

The new CI job's run steps, read from ci.yml and run each with bash -e in next/ of a scratch clone (/home/supadava/.cache/atelier-fcb7/run_next_job.py): on 3d21d702 all passed (38 tests). On throwaway commit 0d73a30b, which adds atelier-workflow = { path = "../../../crates/atelier-workflow" } to atelier-records and uses it (it builds with crates/ present), the job failed at its first cargo step: failed to read .../crates/atelier-workflow/Cargo.toml.
