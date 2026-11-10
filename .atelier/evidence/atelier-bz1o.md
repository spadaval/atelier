---
created_at: "2026-11-10T15:14:17.388924325+00:00"
id: "atelier-bz1o"
evidence_type: "validation"
commit: "134def9355d27a5760e415f0ce8dcfcb23109f9a"
result: "pass"
captured_at: "2026-11-10T15:14:17.388883399+00:00"
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
title: "The new CI job's run steps, read from ci.yml and run each with bash -e in next/ of a scratch clone (/home/supadava/.cache/atelier-fcb7/run_next_job.py): on 14022993 rm -rf ../crates, cargo fmt -- --check, cargo build --locked --all-targets and cargo nextest run --locked all passed (35 tests). On a throwaway commit e08cdd2b adding atelier-workflow = { path = \"../../../crates/atelier-workflow\" } to atelier-records and using it (it builds with crates/ present), the job failed at its first cargo step: failed to read .../crates/atelier-workflow/Cargo.toml: No such file or directory; cargo build --locked alone failed the same way."
updated_at: "2026-11-10T15:14:17.532563113+00:00"
---

The new CI job's run steps, read from ci.yml and run each with bash -e in next/ of a scratch clone (/home/supadava/.cache/atelier-fcb7/run_next_job.py): on 14022993 rm -rf ../crates, cargo fmt -- --check, cargo build --locked --all-targets and cargo nextest run --locked all passed (35 tests). On a throwaway commit e08cdd2b adding atelier-workflow = { path = "../../../crates/atelier-workflow" } to atelier-records and using it (it builds with crates/ present), the job failed at its first cargo step: failed to read .../crates/atelier-workflow/Cargo.toml: No such file or directory; cargo build --locked alone failed the same way.
