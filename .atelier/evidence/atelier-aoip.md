---
created_at: "2026-11-10T15:38:56.159019774+00:00"
id: "atelier-aoip"
evidence_type: "validation"
commit: "cd0d159306365ea3d185f65c9752480b061bcd90"
result: "pass"
captured_at: "2026-11-10T15:38:56.158940556+00:00"
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
title: "The new CI job's run steps, read from ci.yml and run each with bash -e in next/ of a scratch clone (/home/supadava/.cache/atelier-fcb7/run_next_job.py): on c2c0d713 (code at 780625d7) all passed (38 tests). On throwaway commit 5995d75f, which adds atelier-workflow = { path = \"../../../crates/atelier-workflow\" } to atelier-records and uses it (it builds with crates/ present), the job failed at its first cargo step: failed to read .../crates/atelier-workflow/Cargo.toml."
updated_at: "2026-11-10T15:38:56.298989824+00:00"
---

The new CI job's run steps, read from ci.yml and run each with bash -e in next/ of a scratch clone (/home/supadava/.cache/atelier-fcb7/run_next_job.py): on c2c0d713 (code at 780625d7) all passed (38 tests). On throwaway commit 5995d75f, which adds atelier-workflow = { path = "../../../crates/atelier-workflow" } to atelier-records and uses it (it builds with crates/ present), the job failed at its first cargo step: failed to read .../crates/atelier-workflow/Cargo.toml.
